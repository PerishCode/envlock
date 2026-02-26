mod config;
mod injection;

use std::{collections::BTreeMap, path::PathBuf};

use anyhow::{Context, Result, bail};
use clap::Parser;

#[derive(Debug, Parser)]
#[command(
    name = "envlock",
    version,
    about = "Build environment sessions from JSON config"
)]
struct Cli {
    #[arg(short = 'c', long = "config")]
    config: PathBuf,

    #[arg(long = "json")]
    json: bool,

    #[arg(long = "strict")]
    strict: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let cfg = config::load(&cli.config).context("unable to load envlock config")?;
    let exports = injection::execute_lifecycle(cfg.injections)?;
    print_outputs(exports, cli.json, cli.strict)?;
    Ok(())
}

fn print_outputs(exports: Vec<(String, String)>, as_json: bool, strict: bool) -> Result<()> {
    let env = to_env_map(exports, strict)?;
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
