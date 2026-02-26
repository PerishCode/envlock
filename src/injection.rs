use anyhow::{Context, Result, anyhow};

use crate::config::{CodexConfig, InjectionSpec, KubeConfig, NodeConfig};

pub fn execute_lifecycle(specs: Vec<InjectionSpec>) -> Result<Vec<(String, String)>> {
    let mut injections = build_injections(specs);

    for injection in &injections {
        injection
            .validate()
            .with_context(|| format!("{} validation failed", injection.name()))?;
    }

    let mut registered = 0usize;
    for injection in &mut injections {
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
        exports.extend(
            injection
                .export()
                .with_context(|| format!("{} export failed", injection.name()))?,
        );
    }
    Ok(exports)
}

fn shutdown_registered(injections: &mut [RuntimeInjection], registered: usize) -> Result<()> {
    for idx in (0..registered).rev() {
        injections[idx]
            .shutdown()
            .with_context(|| format!("{} shutdown failed", injections[idx].name()))?;
    }
    Ok(())
}

fn build_injections(specs: Vec<InjectionSpec>) -> Vec<RuntimeInjection> {
    let mut injections = Vec::new();
    for spec in specs {
        match spec {
            InjectionSpec::Node(cfg) if cfg.enabled => {
                injections.push(RuntimeInjection::Node(NodeInjection::new(cfg)));
            }
            InjectionSpec::Kube(cfg) if cfg.enabled => {
                injections.push(RuntimeInjection::Kube(KubeInjection::new(cfg)));
            }
            InjectionSpec::Codex(cfg) if cfg.enabled => {
                injections.push(RuntimeInjection::Codex(CodexInjection::new(cfg)));
            }
            _ => {}
        }
    }
    injections
}

enum RuntimeInjection {
    Node(NodeInjection),
    Kube(KubeInjection),
    Codex(CodexInjection),
}

impl RuntimeInjection {
    fn name(&self) -> &'static str {
        match self {
            Self::Node(inner) => inner.name(),
            Self::Kube(inner) => inner.name(),
            Self::Codex(inner) => inner.name(),
        }
    }

    fn validate(&self) -> Result<()> {
        match self {
            Self::Node(inner) => inner.validate(),
            Self::Kube(inner) => inner.validate(),
            Self::Codex(inner) => inner.validate(),
        }
    }

    fn register(&mut self) -> Result<()> {
        match self {
            Self::Node(inner) => inner.register(),
            Self::Kube(inner) => inner.register(),
            Self::Codex(inner) => inner.register(),
        }
    }

    fn export(&self) -> Result<Vec<(String, String)>> {
        match self {
            Self::Node(inner) => inner.export(),
            Self::Kube(inner) => inner.export(),
            Self::Codex(inner) => inner.export(),
        }
    }

    fn shutdown(&mut self) -> Result<()> {
        match self {
            Self::Node(inner) => inner.shutdown(),
            Self::Kube(inner) => inner.shutdown(),
            Self::Codex(inner) => inner.shutdown(),
        }
    }
}

struct NodeInjection {
    cfg: NodeConfig,
    registered: bool,
}

impl NodeInjection {
    fn new(cfg: NodeConfig) -> Self {
        Self {
            cfg,
            registered: false,
        }
    }

    fn name(&self) -> &'static str {
        "node"
    }

    fn validate(&self) -> Result<()> {
        if let Some(version) = &self.cfg.version
            && version.trim().is_empty()
        {
            return Err(anyhow!("version must not be empty"));
        }
        if let Some(registry) = &self.cfg.npm_registry
            && registry.trim().is_empty()
        {
            return Err(anyhow!("npm_registry must not be empty"));
        }
        Ok(())
    }

    fn register(&mut self) -> Result<()> {
        self.registered = true;
        Ok(())
    }

    fn export(&self) -> Result<Vec<(String, String)>> {
        let mut vars = Vec::new();
        if let Some(version) = &self.cfg.version {
            vars.push(("ENVLOCK_NODE_VERSION".to_string(), version.clone()));
        }
        if let Some(registry) = &self.cfg.npm_registry {
            vars.push(("NPM_CONFIG_REGISTRY".to_string(), registry.clone()));
        }
        Ok(vars)
    }

    fn shutdown(&mut self) -> Result<()> {
        if self.registered {
            self.registered = false;
        }
        Ok(())
    }
}

struct KubeInjection {
    cfg: KubeConfig,
    registered: bool,
}

impl KubeInjection {
    fn new(cfg: KubeConfig) -> Self {
        Self {
            cfg,
            registered: false,
        }
    }

    fn name(&self) -> &'static str {
        "kube"
    }

    fn validate(&self) -> Result<()> {
        if let Some(context) = &self.cfg.context
            && context.trim().is_empty()
        {
            return Err(anyhow!("context must not be empty"));
        }
        if let Some(namespace) = &self.cfg.namespace
            && namespace.trim().is_empty()
        {
            return Err(anyhow!("namespace must not be empty"));
        }
        Ok(())
    }

    fn register(&mut self) -> Result<()> {
        self.registered = true;
        Ok(())
    }

    fn export(&self) -> Result<Vec<(String, String)>> {
        let mut vars = Vec::new();
        if let Some(context) = &self.cfg.context {
            vars.push(("KUBECONFIG_CONTEXT".to_string(), context.clone()));
        }
        if let Some(namespace) = &self.cfg.namespace {
            vars.push(("KUBECONFIG_NAMESPACE".to_string(), namespace.clone()));
        }
        Ok(vars)
    }

    fn shutdown(&mut self) -> Result<()> {
        if self.registered {
            self.registered = false;
        }
        Ok(())
    }
}

struct CodexInjection {
    _cfg: CodexConfig,
    registered: bool,
}

impl CodexInjection {
    fn new(cfg: CodexConfig) -> Self {
        Self {
            _cfg: cfg,
            registered: false,
        }
    }

    fn name(&self) -> &'static str {
        "codex"
    }

    fn validate(&self) -> Result<()> {
        Ok(())
    }

    fn register(&mut self) -> Result<()> {
        self.registered = true;
        Ok(())
    }

    fn export(&self) -> Result<Vec<(String, String)>> {
        Ok(Vec::new())
    }

    fn shutdown(&mut self) -> Result<()> {
        if self.registered {
            self.registered = false;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn skip_disabled_injections() {
        let specs = vec![
            InjectionSpec::Node(NodeConfig {
                enabled: false,
                version: Some("22.11.0".to_string()),
                npm_registry: None,
            }),
            InjectionSpec::Kube(KubeConfig {
                enabled: true,
                context: Some("dev".to_string()),
                namespace: Some("platform".to_string()),
            }),
        ];

        let exports = execute_lifecycle(specs).expect("lifecycle should pass");
        assert_eq!(exports.len(), 2);
        assert!(exports.contains(&("KUBECONFIG_CONTEXT".to_string(), "dev".to_string())));
        assert!(exports.contains(&("KUBECONFIG_NAMESPACE".to_string(), "platform".to_string())));
    }

    #[test]
    fn fail_validation_when_node_version_is_empty() {
        let specs = vec![InjectionSpec::Node(NodeConfig {
            enabled: true,
            version: Some("   ".to_string()),
            npm_registry: None,
        })];

        let err = execute_lifecycle(specs).expect_err("empty version should fail");
        assert!(err.to_string().contains("validation failed"));
    }
}
