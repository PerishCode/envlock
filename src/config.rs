use std::path::Path;

use anyhow::{Context, Result};
use serde::Deserialize;

fn default_enabled() -> bool {
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
    Node(NodeConfig),
    Kube(KubeConfig),
    Codex(CodexConfig),
}

#[derive(Debug, Deserialize, Clone)]
pub struct NodeConfig {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    pub version: Option<String>,
    pub npm_registry: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct KubeConfig {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    pub context: Option<String>,
    pub namespace: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CodexConfig {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
}

pub fn load(path: &Path) -> Result<Config> {
    let raw = std::fs::read_to_string(path)
        .with_context(|| format!("failed to read config file: {}", path.display()))?;
    serde_json::from_str(&raw).with_context(|| format!("failed to parse JSON: {}", path.display()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_injections_with_defaults() {
        let raw = r#"
        {
          "injections": [
            { "type": "node", "version": "22.11.0" },
            { "type": "kube", "context": "dev", "namespace": "platform" },
            { "type": "codex" }
          ]
        }"#;

        let cfg: Config = serde_json::from_str(raw).expect("config should parse");
        assert_eq!(cfg.injections.len(), 3);

        match &cfg.injections[0] {
            InjectionSpec::Node(node) => assert!(node.enabled),
            _ => panic!("expected node injection"),
        }
        match &cfg.injections[1] {
            InjectionSpec::Kube(kube) => assert!(kube.enabled),
            _ => panic!("expected kube injection"),
        }
        match &cfg.injections[2] {
            InjectionSpec::Codex(codex) => assert!(codex.enabled),
            _ => panic!("expected codex injection"),
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
