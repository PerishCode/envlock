use anyhow::{Context, Result, bail};

use crate::app::{AppContext, EnvReader};
use crate::profile::CommandProfile;

pub(crate) struct CommandInjection {
    cfg: CommandProfile,
}

impl CommandInjection {
    pub(crate) fn new(cfg: CommandProfile) -> Self {
        Self { cfg }
    }

    pub(crate) fn name(&self) -> &'static str {
        "command"
    }

    pub(crate) fn validate(&self) -> Result<()> {
        if self.cfg.program.trim().is_empty() {
            bail!("program must not be empty");
        }
        Ok(())
    }

    pub(crate) fn register(&mut self) -> Result<()> {
        Ok(())
    }

    pub(crate) fn export(&self, app: &dyn AppContext) -> Result<Vec<(String, String)>> {
        let output = app
            .command_runner()
            .output(&self.cfg.program, &self.cfg.args)
            .with_context(|| format!("failed to run command: {}", self.cfg.program))?;

        if !output.status.success() {
            bail!(
                "command exited with non-zero status: {}",
                output
                    .status
                    .code()
                    .map_or_else(|| "unknown".to_string(), |code| code.to_string())
            );
        }

        let stdout =
            String::from_utf8(output.stdout).context("command stdout is not valid UTF-8")?;
        Ok(parse_exports(&stdout, app.env()))
    }

    pub(crate) fn shutdown(&mut self) -> Result<()> {
        Ok(())
    }
}

fn parse_exports(stdout: &str, env: &dyn EnvReader) -> Vec<(String, String)> {
    let mut out = Vec::new();
    for line in stdout.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let assignment = trimmed.strip_prefix("export ").unwrap_or(trimmed);
        let Some((key_raw, value_raw)) = assignment.split_once('=') else {
            continue;
        };
        let key = key_raw.trim();
        if key.is_empty() {
            continue;
        }
        let value = normalize_value(value_raw.trim(), env);
        out.push((key.to_string(), value));
    }
    out
}

fn normalize_value(raw: &str, env: &dyn EnvReader) -> String {
    let unquoted = raw.replace('"', "").replace('\'', "");
    expand_vars(&unquoted, env)
}

fn expand_vars(input: &str, env: &dyn EnvReader) -> String {
    let mut out = String::new();
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0usize;
    while i < chars.len() {
        if chars[i] != '$' {
            out.push(chars[i]);
            i += 1;
            continue;
        }

        if i + 1 < chars.len() && chars[i + 1] == '{' {
            let mut j = i + 2;
            while j < chars.len() && chars[j] != '}' {
                j += 1;
            }
            if j < chars.len() {
                let key: String = chars[i + 2..j].iter().collect();
                if !key.is_empty() {
                    out.push_str(&env.var(&key).unwrap_or_default());
                }
                i = j + 1;
                continue;
            }
        }

        let mut j = i + 1;
        while j < chars.len() && (chars[j].is_ascii_alphanumeric() || chars[j] == '_') {
            j += 1;
        }
        if j > i + 1 {
            let key: String = chars[i + 1..j].iter().collect();
            out.push_str(&env.var(&key).unwrap_or_default());
            i = j;
            continue;
        }

        out.push('$');
        i += 1;
    }
    out
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;

    struct MockEnv {
        vars: BTreeMap<String, String>,
    }

    impl EnvReader for MockEnv {
        fn var(&self, key: &str) -> Option<String> {
            self.vars.get(key).cloned()
        }
    }

    #[test]
    fn parse_export_and_plain_assignment() {
        let env = MockEnv {
            vars: BTreeMap::new(),
        };
        let vars = parse_exports("export A='1'\nB=2\nignored line\n", &env);
        assert_eq!(
            vars,
            vec![
                ("A".to_string(), "1".to_string()),
                ("B".to_string(), "2".to_string())
            ]
        );
    }

    #[test]
    fn parse_fnm_style_path_value() {
        let env = MockEnv {
            vars: BTreeMap::from([("ENVLOCK_TEST_PATH".to_string(), "/usr/bin:/bin".to_string())]),
        };
        let vars = parse_exports(
            "export PATH=\"/tmp/fnm/bin\":\"$ENVLOCK_TEST_PATH\"\n",
            &env,
        );
        assert_eq!(
            vars,
            vec![("PATH".to_string(), "/tmp/fnm/bin:/usr/bin:/bin".to_string())]
        );
    }
}
