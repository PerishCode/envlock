use anyhow::{Result, bail};

use crate::profile::EnvProfile;

pub(crate) struct EnvInjection {
    cfg: EnvProfile,
}

impl EnvInjection {
    pub(crate) fn new(cfg: EnvProfile) -> Self {
        Self { cfg }
    }

    pub(crate) fn name(&self) -> &'static str {
        "env"
    }

    pub(crate) fn validate(&self) -> Result<()> {
        for key in self.cfg.vars.keys() {
            if key.trim().is_empty() {
                bail!("env var key must not be empty");
            }
        }
        Ok(())
    }

    pub(crate) fn register(&mut self) -> Result<()> {
        Ok(())
    }

    pub(crate) fn export(&self) -> Result<Vec<(String, String)>> {
        Ok(self
            .cfg
            .vars
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect())
    }

    pub(crate) fn shutdown(&mut self) -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;

    #[test]
    fn rejects_empty_env_key() {
        let mut vars = BTreeMap::new();
        vars.insert("   ".to_string(), "x".to_string());
        let injection = EnvInjection::new(EnvProfile {
            enabled: true,
            vars,
        });
        let err = injection.validate().expect_err("empty key should fail");
        assert!(err.to_string().contains("env var key must not be empty"));
    }
}
