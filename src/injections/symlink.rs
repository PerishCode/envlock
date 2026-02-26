use std::path::Path;

use anyhow::{Result, bail};

use crate::config::{SymlinkConfig, SymlinkOnExist};

pub(crate) struct SymlinkInjection {
    cfg: SymlinkConfig,
    registered: bool,
    created_link: bool,
}

impl SymlinkInjection {
    pub(crate) fn new(cfg: SymlinkConfig) -> Self {
        Self {
            cfg,
            registered: false,
            created_link: false,
        }
    }

    pub(crate) fn name(&self) -> &'static str {
        "symlink"
    }

    pub(crate) fn validate(&self) -> Result<()> {
        if self.cfg.source.to_string_lossy().trim().is_empty() {
            bail!("source must not be empty");
        }
        if self.cfg.target.to_string_lossy().trim().is_empty() {
            bail!("target must not be empty");
        }
        if !self.cfg.source.exists() {
            bail!("source does not exist: {}", self.cfg.source.display());
        }
        Ok(())
    }

    pub(crate) fn register(&mut self) -> Result<()> {
        self.register_at(&self.cfg.source, &self.cfg.target, self.cfg.on_exist)?;
        self.registered = true;
        self.created_link = true;
        Ok(())
    }

    pub(crate) fn export(&self) -> Result<Vec<(String, String)>> {
        Ok(Vec::new())
    }

    pub(crate) fn shutdown(&mut self) -> Result<()> {
        if self.registered && self.cfg.cleanup && self.created_link {
            self.shutdown_at(&self.cfg.target, &self.cfg.source)?;
            self.created_link = false;
        }
        self.registered = false;
        Ok(())
    }

    fn register_at(&self, source: &Path, target: &Path, on_exist: SymlinkOnExist) -> Result<()> {
        match std::fs::symlink_metadata(target) {
            Ok(meta) => match on_exist {
                SymlinkOnExist::Error => {
                    bail!("refusing to overwrite existing file: {}", target.display())
                }
                SymlinkOnExist::Replace => {
                    if meta.file_type().is_dir() {
                        bail!("refusing to replace directory target: {}", target.display());
                    }
                    std::fs::remove_file(target)?;
                }
            },
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => {}
            Err(err) => return Err(err.into()),
        }

        if let Some(parent) = target.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::os::unix::fs::symlink(source, target)?;
        Ok(())
    }

    fn shutdown_at(&self, target: &Path, source: &Path) -> Result<()> {
        let metadata = std::fs::symlink_metadata(target)?;
        if !metadata.file_type().is_symlink() {
            bail!("refusing to remove non-symlink at {}", target.display());
        }
        let link_target = std::fs::read_link(target)?;
        if link_target != source {
            bail!(
                "refusing to remove symlink with unexpected target: {}",
                target.display()
            );
        }
        std::fs::remove_file(target)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn register_fails_when_target_exists_in_error_mode() {
        let temp = TempDir::new().expect("temp dir should be created");
        let source = temp.path().join("source.md");
        let target = temp.path().join("AGENTS.md");
        std::fs::write(&source, "content").expect("source file should be created");
        std::fs::write(&target, "existing").expect("target file should be created");

        let injection = SymlinkInjection::new(SymlinkConfig {
            enabled: true,
            source: source.clone(),
            target: target.clone(),
            on_exist: SymlinkOnExist::Error,
            cleanup: true,
        });

        let err = injection
            .register_at(&source, &target, SymlinkOnExist::Error)
            .expect_err("existing target should fail");
        assert!(
            err.to_string()
                .contains("refusing to overwrite existing file")
        );
    }

    #[test]
    fn register_and_shutdown_manage_symlink() {
        let temp = TempDir::new().expect("temp dir should be created");
        let source = temp.path().join("source.md");
        let target = temp.path().join(".codex/AGENTS.md");
        std::fs::write(&source, "content").expect("source file should be created");

        let mut injection = SymlinkInjection::new(SymlinkConfig {
            enabled: true,
            source: source.clone(),
            target: target.clone(),
            on_exist: SymlinkOnExist::Error,
            cleanup: true,
        });

        injection
            .register()
            .expect("register should create symlink");

        let metadata = std::fs::symlink_metadata(&target).expect("symlink should exist");
        assert!(metadata.file_type().is_symlink());

        injection
            .shutdown()
            .expect("shutdown should remove symlink");
        assert!(
            std::fs::symlink_metadata(&target).is_err(),
            "symlink should be removed"
        );
    }

    #[test]
    fn replace_mode_replaces_existing_file() {
        let temp = TempDir::new().expect("temp dir should be created");
        let source = temp.path().join("source.md");
        let target = temp.path().join("AGENTS.md");
        std::fs::write(&source, "content").expect("source file should be created");
        std::fs::write(&target, "existing").expect("target file should be created");

        let mut injection = SymlinkInjection::new(SymlinkConfig {
            enabled: true,
            source: source.clone(),
            target: target.clone(),
            on_exist: SymlinkOnExist::Replace,
            cleanup: true,
        });

        injection.register().expect("replace mode should succeed");
        let metadata = std::fs::symlink_metadata(&target).expect("target should exist");
        assert!(metadata.file_type().is_symlink());
    }
}
