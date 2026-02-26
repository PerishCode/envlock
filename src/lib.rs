pub mod injections;
pub mod profile;

use std::{collections::BTreeMap, path::Path, process::Command};

use anyhow::{Context, Result, bail};
use tracing::{debug, info};

pub struct RunOptions {
    pub json: bool,
    pub strict: bool,
    pub command: Option<Vec<String>>,
}

pub struct RunResult {
    pub exit_code: Option<i32>,
}

pub fn run(profile_path: &Path, options: &RunOptions) -> Result<RunResult> {
    info!(
        profile_path = %profile_path.display(),
        json = options.json,
        strict = options.strict,
        has_command = options.command.is_some(),
        "envlock run started"
    );
    let profile = profile::load(profile_path).context("unable to load envlock profile")?;
    let run_result = injections::with_registered_exports(profile.injections, |exports| {
        info!(
            export_count = exports.len(),
            "injections lifecycle completed"
        );
        if let Some(command) = &options.command {
            let code = run_command(command, exports)?;
            return Ok(RunResult {
                exit_code: Some(code),
            });
        }
        print_outputs(exports.to_vec(), options.json, options.strict)?;
        Ok(RunResult { exit_code: None })
    })?;
    info!("envlock run completed");
    Ok(run_result)
}

fn print_outputs(exports: Vec<(String, String)>, as_json: bool, strict: bool) -> Result<()> {
    let env = to_env_map(exports, strict)?;
    debug!(
        output_mode = if as_json { "json" } else { "shell" },
        "rendering output"
    );
    if as_json {
        println!("{}", serde_json::to_string_pretty(&env)?);
    } else {
        for (key, value) in env {
            println!("export {}='{}'", key, shell_single_quote_escape(&value));
        }
    }
    Ok(())
}

fn to_env_map(exports: Vec<(String, String)>, strict: bool) -> Result<BTreeMap<String, String>> {
    let mut env = BTreeMap::new();
    for (key, value) in exports {
        if strict && env.contains_key(&key) {
            bail!("duplicate exported key detected in strict mode: {}", key);
        }
        env.insert(key, value);
    }
    Ok(env)
}

fn shell_single_quote_escape(input: &str) -> String {
    input.replace('\'', "'\"'\"'")
}

fn run_command(command: &[String], exports: &[(String, String)]) -> Result<i32> {
    if command.is_empty() {
        bail!("command mode requires at least one command token");
    }

    let mut child = Command::new(&command[0]);
    if command.len() > 1 {
        child.args(&command[1..]);
    }
    child.envs(exports.iter().map(|(k, v)| (k.as_str(), v.as_str())));

    let status = child.status().context("failed to execute child command")?;
    if let Some(code) = status.code() {
        return Ok(code);
    }

    #[cfg(unix)]
    {
        use std::os::unix::process::ExitStatusExt;
        if let Some(signal) = status.signal() {
            return Ok(128 + signal);
        }
    }

    Ok(1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escape_single_quotes_for_shell() {
        assert_eq!(shell_single_quote_escape("a'b"), "a'\"'\"'b");
    }

    #[test]
    fn env_map_keeps_last_value_for_duplicate_keys() {
        let map = to_env_map(
            vec![
                ("A".to_string(), "1".to_string()),
                ("B".to_string(), "2".to_string()),
                ("A".to_string(), "3".to_string()),
            ],
            false,
        )
        .expect("non-strict mode should allow duplicate keys");
        assert_eq!(map.get("A"), Some(&"3".to_string()));
        assert_eq!(map.get("B"), Some(&"2".to_string()));
    }

    #[test]
    fn env_map_rejects_duplicate_keys_in_strict_mode() {
        let err = to_env_map(
            vec![
                ("A".to_string(), "1".to_string()),
                ("A".to_string(), "2".to_string()),
            ],
            true,
        )
        .expect_err("strict mode should reject duplicate keys");
        assert!(err.to_string().contains("duplicate exported key"));
    }
}
