use std::process::Command;

use anyhow::{Context, Result, bail};

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

    pub(crate) fn export(&self) -> Result<Vec<(String, String)>> {
        let output = Command::new(&self.cfg.program)
            .args(&self.cfg.args)
            .output()
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
        Ok(parse_exports(&stdout))
    }

    pub(crate) fn shutdown(&mut self) -> Result<()> {
        Ok(())
    }
}

fn parse_exports(stdout: &str) -> Vec<(String, String)> {
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
        let value = normalize_value(value_raw.trim());
        out.push((key.to_string(), value));
    }
    out
}

fn normalize_value(raw: &str) -> String {
    let unquoted = raw.replace('"', "").replace('\'', "");
    expand_vars(&unquoted)
}

fn expand_vars(input: &str) -> String {
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
                    out.push_str(&std::env::var(&key).unwrap_or_default());
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
            out.push_str(&std::env::var(&key).unwrap_or_default());
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
    use super::*;

    #[test]
    fn parse_export_and_plain_assignment() {
        let vars = parse_exports("export A='1'\nB=2\nignored line\n");
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
        let key = "ENVLOCK_TEST_PATH";
        // SAFETY: test-scoped environment mutation to verify expansion semantics.
        unsafe { std::env::set_var(key, "/usr/bin:/bin") };
        let vars = parse_exports("export PATH=\"/tmp/fnm/bin\":\"$ENVLOCK_TEST_PATH\"\n");
        assert_eq!(
            vars,
            vec![("PATH".to_string(), "/tmp/fnm/bin:/usr/bin:/bin".to_string())]
        );
        // SAFETY: restore environment after assertion.
        unsafe { std::env::remove_var(key) };
    }
}
