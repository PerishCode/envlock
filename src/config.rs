use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use path_absolutize::Absolutize;
use serde::Deserialize;

fn default_enabled() -> bool {
    true
}

fn default_cleanup() -> bool {
    true
}

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub injections: Vec<InjectionSpec>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum InjectionSpec {
    Env(EnvConfig),
    Symlink(SymlinkConfig),
}

#[derive(Debug, Deserialize, Clone)]
pub struct EnvConfig {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default)]
    pub vars: BTreeMap<String, String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SymlinkConfig {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    pub source: PathBuf,
    pub target: PathBuf,
    #[serde(default)]
    pub on_exist: SymlinkOnExist,
    #[serde(default = "default_cleanup")]
    pub cleanup: bool,
}

#[derive(Debug, Deserialize, Clone, Copy, Default)]
#[serde(rename_all = "lowercase")]
pub enum SymlinkOnExist {
    #[default]
    Error,
    Replace,
}

pub fn load(path: &Path) -> Result<Config> {
    let raw = std::fs::read_to_string(path)
        .with_context(|| format!("failed to read config file: {}", path.display()))?;
    let mut cfg: Config = serde_json::from_str(&raw)
        .with_context(|| format!("failed to parse JSON: {}", path.display()))?;
    normalize_symlink_paths(path, &mut cfg)?;
    Ok(cfg)
}

fn normalize_symlink_paths(config_path: &Path, cfg: &mut Config) -> Result<()> {
    let base_dir = config_path.parent().unwrap_or(Path::new("."));
    for injection in &mut cfg.injections {
        if let InjectionSpec::Symlink(spec) = injection {
            spec.source = normalize_path(&spec.source, base_dir)?;
            spec.target = normalize_path(&spec.target, base_dir)?;
        }
    }
    Ok(())
}

fn normalize_path(path: &Path, base_dir: &Path) -> Result<PathBuf> {
    let raw = path.to_string_lossy();
    let expanded = shellexpand::tilde(&raw);
    let expanded_path = PathBuf::from(expanded.as_ref());
    if expanded_path.is_absolute() {
        return Ok(expanded_path);
    }
    Ok(expanded_path.absolutize_from(base_dir)?.to_path_buf())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_injections_with_defaults() {
        let raw = r#"
        {
          "injections": [
            { "type": "env", "vars": { "A": "1", "B": "2" } },
            { "type": "symlink", "source": "./fixtures/agents.md", "target": "~/.codex/AGENTS.md" }
          ]
        }"#;

        let cfg: Config = serde_json::from_str(raw).expect("config should parse");
        assert_eq!(cfg.injections.len(), 2);

        match &cfg.injections[0] {
            InjectionSpec::Env(env) => {
                assert!(env.enabled);
                assert_eq!(env.vars.get("A"), Some(&"1".to_string()));
                assert_eq!(env.vars.get("B"), Some(&"2".to_string()));
            }
            _ => panic!("expected env injection"),
        }
        match &cfg.injections[1] {
            InjectionSpec::Symlink(link) => {
                assert!(link.enabled);
                assert!(matches!(link.on_exist, SymlinkOnExist::Error));
                assert!(link.cleanup);
            }
            _ => panic!("expected symlink injection"),
        }
    }

    #[test]
    fn reject_unknown_injection_type() {
        let raw = r#"
        {
          "injections": [
            { "type": "python" }
          ]
        }"#;

        let err = serde_json::from_str::<Config>(raw).expect_err("unknown type should fail");
        let msg = err.to_string();
        assert!(msg.contains("unknown variant"));
    }
}
