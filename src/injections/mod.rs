mod env;
mod symlink;

use anyhow::{Context, Result, anyhow};
use tracing::{debug, info};

use crate::profile::InjectionProfile;
use env::EnvInjection;
use symlink::SymlinkInjection;

pub fn execute_lifecycle(specs: Vec<InjectionProfile>) -> Result<Vec<(String, String)>> {
    let mut injections = build_injections(specs);
    info!(
        injection_count = injections.len(),
        "starting injection lifecycle"
    );

    for injection in &injections {
        debug!(
            injection = injection.name(),
            stage = "validate",
            "running stage"
        );
        injection
            .validate()
            .with_context(|| format!("{} validation failed", injection.name()))?;
    }

    let mut registered = 0usize;
    for injection in &mut injections {
        debug!(
            injection = injection.name(),
            stage = "register",
            "running stage"
        );
        injection
            .register()
            .with_context(|| format!("{} registration failed", injection.name()))?;
        registered += 1;
    }

    let export_result = collect_exports(&injections);
    let shutdown_result = shutdown_registered(&mut injections, registered);

    match (export_result, shutdown_result) {
        (Ok(exports), Ok(())) => Ok(exports),
        (Err(err), Ok(())) => Err(err),
        (Ok(_), Err(shutdown_err)) => Err(shutdown_err),
        (Err(export_err), Err(shutdown_err)) => Err(anyhow!(
            "{export_err}; also failed shutdown: {shutdown_err}"
        )),
    }
}

fn collect_exports(injections: &[RuntimeInjection]) -> Result<Vec<(String, String)>> {
    let mut exports = Vec::new();
    for injection in injections {
        debug!(
            injection = injection.name(),
            stage = "export",
            "running stage"
        );
        let exported = injection
            .export()
            .with_context(|| format!("{} export failed", injection.name()))?;
        debug!(
            injection = injection.name(),
            export_count = exported.len(),
            "export stage completed"
        );
        exports.extend(exported);
    }
    info!(export_count = exports.len(), "export collection completed");
    Ok(exports)
}

fn shutdown_registered(injections: &mut [RuntimeInjection], registered: usize) -> Result<()> {
    for idx in (0..registered).rev() {
        debug!(
            injection = injections[idx].name(),
            stage = "shutdown",
            "running stage"
        );
        injections[idx]
            .shutdown()
            .with_context(|| format!("{} shutdown failed", injections[idx].name()))?;
    }
    info!(registered_count = registered, "shutdown completed");
    Ok(())
}

fn build_injections(specs: Vec<InjectionProfile>) -> Vec<RuntimeInjection> {
    let mut injections = Vec::new();
    for spec in specs {
        match spec {
            InjectionProfile::Env(cfg) if cfg.enabled => {
                injections.push(RuntimeInjection::Env(EnvInjection::new(cfg)));
            }
            InjectionProfile::Symlink(cfg) if cfg.enabled => {
                injections.push(RuntimeInjection::Symlink(SymlinkInjection::new(cfg)));
            }
            _ => {}
        }
    }
    debug!(
        enabled_injections = injections.len(),
        "built enabled injections"
    );
    injections
}

enum RuntimeInjection {
    Env(EnvInjection),
    Symlink(SymlinkInjection),
}

impl RuntimeInjection {
    fn name(&self) -> &'static str {
        match self {
            Self::Env(inner) => inner.name(),
            Self::Symlink(inner) => inner.name(),
        }
    }

    fn validate(&self) -> Result<()> {
        match self {
            Self::Env(inner) => inner.validate(),
            Self::Symlink(inner) => inner.validate(),
        }
    }

    fn register(&mut self) -> Result<()> {
        match self {
            Self::Env(inner) => inner.register(),
            Self::Symlink(inner) => inner.register(),
        }
    }

    fn export(&self) -> Result<Vec<(String, String)>> {
        match self {
            Self::Env(inner) => inner.export(),
            Self::Symlink(inner) => inner.export(),
        }
    }

    fn shutdown(&mut self) -> Result<()> {
        match self {
            Self::Env(inner) => inner.shutdown(),
            Self::Symlink(inner) => inner.shutdown(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    #[test]
    fn skip_disabled_env_injection() {
        let specs = vec![
            InjectionProfile::Env(crate::profile::EnvProfile {
                enabled: false,
                vars: BTreeMap::from([("A".to_string(), "1".to_string())]),
            }),
            InjectionProfile::Env(crate::profile::EnvProfile {
                enabled: true,
                vars: BTreeMap::from([("B".to_string(), "2".to_string())]),
            }),
        ];

        let exports = execute_lifecycle(specs).expect("lifecycle should pass");
        assert_eq!(exports.len(), 1);
        assert!(exports.contains(&("B".to_string(), "2".to_string())));
    }

    #[test]
    fn fail_validation_when_env_key_is_empty() {
        let specs = vec![InjectionProfile::Env(crate::profile::EnvProfile {
            enabled: true,
            vars: BTreeMap::from([("   ".to_string(), "1".to_string())]),
        })];

        let err = execute_lifecycle(specs).expect_err("empty env key should fail");
        assert!(err.to_string().contains("validation failed"));
    }
}
