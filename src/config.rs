use std::path::PathBuf;

use anyhow::{Result, bail};
use tracing_subscriber::filter::LevelFilter;

#[derive(Debug, Clone, Copy)]
pub enum OutputMode {
    Shell,
    Json,
}

#[derive(Debug, Clone, Copy)]
pub enum LogFormat {
    Text,
    Json,
}

#[derive(Debug, Clone)]
pub struct CliInput {
    pub profile: Option<PathBuf>,
    pub use_name: Option<String>,
    pub output_mode: OutputMode,
    pub strict: bool,
    pub log_level: LevelFilter,
    pub log_format: LogFormat,
    pub command: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct RawEnv {
    pub home: Option<PathBuf>,
    pub envlock_profile_home: Option<PathBuf>,
    pub envlock_resource_home: Option<PathBuf>,
}

impl RawEnv {
    pub fn from_process() -> Self {
        Self {
            home: std::env::var_os("HOME").map(PathBuf::from),
            envlock_profile_home: std::env::var_os("ENVLOCK_PROFILE_HOME").map(PathBuf::from),
            envlock_resource_home: std::env::var_os("ENVLOCK_RESOURCE_HOME").map(PathBuf::from),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RuntimeConfig {
    pub profile_path: PathBuf,
    pub output_mode: OutputMode,
    pub strict: bool,
    pub log_level: LevelFilter,
    pub log_format: LogFormat,
    pub command: Option<Vec<String>>,
    pub profile_home: PathBuf,
    pub resource_home: PathBuf,
}

impl RuntimeConfig {
    pub fn from_cli_and_env(cli: CliInput, env: RawEnv) -> Result<Self> {
        let profile_home = env
            .envlock_profile_home
            .unwrap_or_else(|| default_profile_home(env.home.as_ref()));
        let resource_home = env
            .envlock_resource_home
            .unwrap_or_else(|| default_resource_home(env.home.as_ref()));

        let profile_path = if let Some(profile) = cli.profile {
            profile
        } else if let Some(use_name) = cli.use_name {
            if use_name.trim().is_empty() {
                bail!("--use must not be empty");
            }
            profile_home
                .join("profiles")
                .join(format!("{use_name}.json"))
        } else {
            bail!("one of --profile or --use is required")
        };

        Ok(Self {
            profile_path,
            output_mode: cli.output_mode,
            strict: cli.strict,
            log_level: cli.log_level,
            log_format: cli.log_format,
            command: if cli.command.is_empty() {
                None
            } else {
                Some(cli.command)
            },
            profile_home,
            resource_home,
        })
    }
}

fn default_profile_home(home: Option<&PathBuf>) -> PathBuf {
    if let Some(home) = home {
        return home.join(".envlock");
    }
    PathBuf::from("~/.envlock")
}

fn default_resource_home(home: Option<&PathBuf>) -> PathBuf {
    if let Some(home) = home {
        return home.join(".envlock/resources");
    }
    PathBuf::from("~/.envlock/resources")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn base_cli() -> CliInput {
        CliInput {
            profile: None,
            use_name: Some("dev".to_string()),
            output_mode: OutputMode::Shell,
            strict: false,
            log_level: LevelFilter::WARN,
            log_format: LogFormat::Text,
            command: Vec::new(),
        }
    }

    #[test]
    fn use_path_prefers_env_profile_home() {
        let cfg = RuntimeConfig::from_cli_and_env(
            base_cli(),
            RawEnv {
                home: Some(PathBuf::from("/Users/tester")),
                envlock_profile_home: Some(PathBuf::from("/tmp/profile-home")),
                envlock_resource_home: None,
            },
        )
        .expect("config should build");

        assert_eq!(cfg.profile_home, PathBuf::from("/tmp/profile-home"));
        assert_eq!(
            cfg.profile_path,
            PathBuf::from("/tmp/profile-home/profiles/dev.json")
        );
    }

    #[test]
    fn resource_home_defaults_from_home() {
        let cfg = RuntimeConfig::from_cli_and_env(
            base_cli(),
            RawEnv {
                home: Some(PathBuf::from("/Users/tester")),
                envlock_profile_home: None,
                envlock_resource_home: None,
            },
        )
        .expect("config should build");
        assert_eq!(
            cfg.resource_home,
            PathBuf::from("/Users/tester/.envlock/resources")
        );
    }

    #[test]
    fn profile_home_fallback_without_home() {
        let cfg = RuntimeConfig::from_cli_and_env(
            base_cli(),
            RawEnv {
                home: None,
                envlock_profile_home: None,
                envlock_resource_home: None,
            },
        )
        .expect("config should build");
        assert_eq!(cfg.profile_home, PathBuf::from("~/.envlock"));
        assert_eq!(cfg.resource_home, PathBuf::from("~/.envlock/resources"));
    }
}
