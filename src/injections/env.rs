use std::collections::BTreeMap;
use std::path::Path;

use anyhow::{Result, bail};

use crate::app::AppContext;
use crate::profile::{EnvOpProfile, EnvProfile};

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
        for op in &self.cfg.ops {
            match op {
                EnvOpProfile::Set { key, value } | EnvOpProfile::SetIfAbsent { key, value } => {
                    validate_key_value(key, value)?
                }
                EnvOpProfile::Prepend {
                    key,
                    value,
                    separator,
                    ..
                }
                | EnvOpProfile::Append {
                    key,
                    value,
                    separator,
                    ..
                } => {
                    validate_key_value(key, value)?;
                    if let Some(sep) = separator {
                        if sep != "os" && sep.is_empty() {
                            bail!("separator must not be empty");
                        }
                    }
                }
                EnvOpProfile::Unset { key } => {
                    if key.trim().is_empty() {
                        bail!("env var key must not be empty");
                    }
                }
            }
        }
        Ok(())
    }

    pub(crate) fn register(&mut self) -> Result<()> {
        Ok(())
    }

    pub(crate) fn export(&self, app: &dyn AppContext) -> Result<Vec<(String, String)>> {
        let resource_home = &app.config().resource_home;
        let mut env = self
            .cfg
            .vars
            .iter()
            .map(|(k, v)| (k.clone(), resolve_resource_refs(v, resource_home)))
            .collect();
        apply_ops(app, &mut env, &self.cfg.ops, resource_home)?;
        Ok(env.into_iter().collect())
    }

    pub(crate) fn shutdown(&mut self) -> Result<()> {
        Ok(())
    }
}

fn validate_key_value(key: &str, value: &str) -> Result<()> {
    if key.trim().is_empty() {
        bail!("env var key must not be empty");
    }
    if value.trim().is_empty() {
        bail!("env var value must not be empty");
    }
    Ok(())
}

fn apply_ops(
    app: &dyn AppContext,
    env: &mut BTreeMap<String, String>,
    ops: &[EnvOpProfile],
    resource_home: &Path,
) -> Result<()> {
    for op in ops {
        match op {
            EnvOpProfile::Set { key, value } => {
                env.insert(key.clone(), resolve_resource_refs(value, resource_home));
            }
            EnvOpProfile::SetIfAbsent { key, value } => {
                if !env.contains_key(key) && app.env().var(key).is_none() {
                    env.insert(key.clone(), resolve_resource_refs(value, resource_home));
                }
            }
            EnvOpProfile::Prepend {
                key,
                value,
                separator,
                dedup,
            } => {
                let sep = separator_value(separator);
                let base = env
                    .get(key)
                    .cloned()
                    .or_else(|| app.env().var(key))
                    .unwrap_or_default();
                let resolved = resolve_resource_refs(value, resource_home);
                env.insert(key.clone(), merge_values(&resolved, &base, sep, *dedup));
            }
            EnvOpProfile::Append {
                key,
                value,
                separator,
                dedup,
            } => {
                let sep = separator_value(separator);
                let base = env
                    .get(key)
                    .cloned()
                    .or_else(|| app.env().var(key))
                    .unwrap_or_default();
                let resolved = resolve_resource_refs(value, resource_home);
                env.insert(key.clone(), merge_values(&base, &resolved, sep, *dedup));
            }
            EnvOpProfile::Unset { key } => {
                env.remove(key);
            }
        }
    }
    Ok(())
}

fn separator_value(separator: &Option<String>) -> &str {
    match separator.as_deref() {
        None | Some("os") => {
            if cfg!(windows) {
                ";"
            } else {
                ":"
            }
        }
        Some(custom) => custom,
    }
}

fn merge_values(left: &str, right: &str, separator: &str, dedup: bool) -> String {
    let mut out = Vec::new();
    let left_parts = split_parts(left, separator);
    let right_parts = split_parts(right, separator);

    out.extend(left_parts);
    out.extend(right_parts);

    if dedup {
        let mut deduped = Vec::new();
        for entry in out {
            if !deduped.contains(&entry) {
                deduped.push(entry);
            }
        }
        return deduped.join(separator);
    }
    out.join(separator)
}

fn split_parts(value: &str, separator: &str) -> Vec<String> {
    value
        .split(separator)
        .filter(|part| !part.is_empty())
        .map(ToString::to_string)
        .collect()
}

fn resolve_resource_refs(value: &str, resource_home: &Path) -> String {
    let prefix = "resource://";
    let mut out = String::new();
    let mut rest = value;
    while let Some(idx) = rest.find(prefix) {
        out.push_str(&rest[..idx]);
        let token_start = idx + prefix.len();
        let after = &rest[token_start..];
        let token_end = after
            .char_indices()
            .find(|(_, c)| *c == ':' || *c == ';')
            .map(|(i, _)| i)
            .unwrap_or(after.len());
        let rel = &after[..token_end];
        if rel.is_empty() {
            out.push_str(prefix);
        } else {
            let abs = resource_home.join(rel);
            out.push_str(&abs.to_string_lossy());
        }
        rest = &after[token_end..];
    }
    out.push_str(rest);
    out
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::path::PathBuf;

    use super::*;
    use crate::app::{AppContext, CommandRunner, EnvReader};
    use crate::config::{LogFormat, OutputMode, RuntimeConfig};
    use tracing_subscriber::filter::LevelFilter;

    struct TestEnv {
        vars: BTreeMap<String, String>,
    }

    impl EnvReader for TestEnv {
        fn var(&self, key: &str) -> Option<String> {
            self.vars.get(key).cloned()
        }
    }

    struct TestRunner;

    impl CommandRunner for TestRunner {
        fn output(&self, program: &str, args: &[String]) -> Result<std::process::Output> {
            std::process::Command::new(program)
                .args(args)
                .output()
                .map_err(Into::into)
        }
    }

    struct TestApp {
        cfg: RuntimeConfig,
        env: TestEnv,
        runner: TestRunner,
    }

    impl TestApp {
        fn new(resource_home: &str, vars: BTreeMap<String, String>) -> Self {
            Self {
                cfg: RuntimeConfig {
                    profile_path: PathBuf::from("/tmp/unused.json"),
                    output_mode: OutputMode::Shell,
                    strict: false,
                    log_level: LevelFilter::WARN,
                    log_format: LogFormat::Text,
                    command: None,
                    profile_home: PathBuf::from("/tmp/profile-home"),
                    resource_home: PathBuf::from(resource_home),
                },
                env: TestEnv { vars },
                runner: TestRunner,
            }
        }
    }

    impl AppContext for TestApp {
        fn config(&self) -> &RuntimeConfig {
            &self.cfg
        }

        fn env(&self) -> &dyn EnvReader {
            &self.env
        }

        fn command_runner(&self) -> &dyn CommandRunner {
            &self.runner
        }
    }

    #[test]
    fn rejects_empty_env_key() {
        let mut vars = BTreeMap::new();
        vars.insert("   ".to_string(), "x".to_string());
        let injection = EnvInjection::new(EnvProfile {
            enabled: true,
            vars,
            ops: Vec::new(),
        });
        let err = injection.validate().expect_err("empty key should fail");
        assert!(err.to_string().contains("env var key must not be empty"));
    }

    #[test]
    fn prepend_path_with_dedup() {
        let mut vars = BTreeMap::new();
        vars.insert("PATH".to_string(), "/usr/bin:/bin".to_string());
        let injection = EnvInjection::new(EnvProfile {
            enabled: true,
            vars,
            ops: vec![EnvOpProfile::Prepend {
                key: "PATH".to_string(),
                value: "/custom/bin:/usr/bin".to_string(),
                separator: Some("os".to_string()),
                dedup: true,
            }],
        });
        let app = TestApp::new("/tmp/envlock-res", BTreeMap::new());

        let exports = injection.export(&app).expect("export should pass");
        let path = exports
            .into_iter()
            .find(|(k, _)| k == "PATH")
            .map(|(_, v)| v)
            .expect("PATH should exist");
        assert_eq!(path, "/custom/bin:/usr/bin:/bin");
    }

    #[test]
    fn set_if_absent_uses_current_env() {
        let key = "ENVLOCK_TEST_SET_IF_ABSENT";
        let app = TestApp::new(
            "/tmp/envlock-res",
            BTreeMap::from([(key.to_string(), "present".to_string())]),
        );
        let injection = EnvInjection::new(EnvProfile {
            enabled: true,
            vars: BTreeMap::new(),
            ops: vec![EnvOpProfile::SetIfAbsent {
                key: key.to_string(),
                value: "fallback".to_string(),
            }],
        });
        let exports = injection.export(&app).expect("export should pass");
        assert!(!exports.iter().any(|(k, _)| k == key));
    }

    #[test]
    fn resolves_resource_uri_with_default_home() {
        let resolved = resolve_resource_refs(
            "resource://kubeconfig/xx.yaml",
            std::path::Path::new("/tmp/envlock-res"),
        );
        assert_eq!(resolved, "/tmp/envlock-res/kubeconfig/xx.yaml");
    }

    #[test]
    fn resolves_multiple_resource_uris_in_one_value() {
        let resolved = resolve_resource_refs(
            "resource://kubeconfig/xx.yaml:resource://kubeconfig/yy.yaml",
            std::path::Path::new("/tmp/envlock-res"),
        );
        assert_eq!(
            resolved,
            "/tmp/envlock-res/kubeconfig/xx.yaml:/tmp/envlock-res/kubeconfig/yy.yaml"
        );
    }
}
