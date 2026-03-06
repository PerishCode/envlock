use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};

const ALIAS_STORE_VERSION: u32 = 1;
const ALIAS_FILE_NAME: &str = "aliases.json";

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AliasEntry {
    pub profile: String,
}

#[derive(Debug, Clone, Default)]
pub struct AliasStore {
    aliases: BTreeMap<String, AliasEntry>,
}

#[derive(Debug, Deserialize, Serialize)]
struct AliasStoreFile {
    version: u32,
    aliases: BTreeMap<String, AliasEntry>,
}

impl AliasStore {
    pub fn load(envlock_home: &Path) -> Result<Self> {
        let path = alias_file_path(envlock_home);
        if !path.exists() {
            return Ok(Self::default());
        }

        let raw = std::fs::read_to_string(&path)
            .with_context(|| format!("failed to read alias store: {}", path.display()))?;
        let parsed: AliasStoreFile = serde_json::from_str(&raw)
            .with_context(|| format!("failed to parse alias store JSON: {}", path.display()))?;
        if parsed.version != ALIAS_STORE_VERSION {
            bail!(
                "unsupported alias store version {} (expected {})",
                parsed.version,
                ALIAS_STORE_VERSION
            );
        }

        Ok(Self {
            aliases: parsed.aliases,
        })
    }

    pub fn list(&self) -> impl Iterator<Item = (&String, &AliasEntry)> {
        self.aliases.iter()
    }

    pub fn get(&self, name: &str) -> Option<&AliasEntry> {
        self.aliases.get(name)
    }

    pub fn append(&mut self, name: String, profile: String) -> Result<()> {
        if self.aliases.contains_key(&name) {
            bail!("alias already exists: {}", name);
        }
        self.aliases.insert(name, AliasEntry { profile });
        Ok(())
    }

    pub fn save(&self, envlock_home: &Path) -> Result<PathBuf> {
        std::fs::create_dir_all(envlock_home).with_context(|| {
            format!(
                "failed to create envlock home directory: {}",
                envlock_home.display()
            )
        })?;

        let path = alias_file_path(envlock_home);
        let payload = AliasStoreFile {
            version: ALIAS_STORE_VERSION,
            aliases: self.aliases.clone(),
        };
        let json =
            serde_json::to_string_pretty(&payload).context("failed to serialize alias store")?;

        let staged = path.with_extension(format!("json.new.{}", std::process::id()));
        std::fs::write(&staged, format!("{}\n", json))
            .with_context(|| format!("failed to write staged alias store: {}", staged.display()))?;
        std::fs::rename(&staged, &path)
            .with_context(|| format!("failed to replace alias store: {}", path.display()))?;
        Ok(path)
    }
}

pub fn alias_file_path(envlock_home: &Path) -> PathBuf {
    envlock_home.join(ALIAS_FILE_NAME)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn append_and_persist_alias() {
        let temp = TempDir::new().expect("temp dir should be created");
        let mut store = AliasStore::default();
        store
            .append("work".to_string(), "profiles/work.json".to_string())
            .expect("append should succeed");
        let path = store.save(temp.path()).expect("save should succeed");
        assert!(path.exists());

        let loaded = AliasStore::load(temp.path()).expect("load should succeed");
        assert_eq!(
            loaded.get("work").map(|entry| entry.profile.as_str()),
            Some("profiles/work.json")
        );
    }

    #[test]
    fn append_rejects_duplicate_name() {
        let mut store = AliasStore::default();
        store
            .append("work".to_string(), "profiles/work.json".to_string())
            .expect("first append should succeed");
        let err = store
            .append("work".to_string(), "profiles/other.json".to_string())
            .expect_err("duplicate append should fail");
        assert!(err.to_string().contains("alias already exists"));
    }
}
