use std::path::PathBuf;

use anyhow::{Context, Result, bail};
use clap::{Parser, ValueEnum};
use envlock::{RunOptions, run};
use tracing_subscriber::{EnvFilter, prelude::*};

#[derive(Debug, Parser)]
#[command(
    name = "envlock",
    version,
    about = "Build environment sessions from JSON config"
)]
struct Cli {
    #[arg(short = 'c', long = "config")]
    config: Option<PathBuf>,

    #[arg(long = "use")]
    use_name: Option<String>,

    #[arg(long = "output", default_value = "shell", value_enum)]
    output: OutputFormat,

    #[arg(long = "strict")]
    strict: bool,

    #[arg(long = "log-level", default_value = "warn", value_enum)]
    log_level: LogLevel,

    #[arg(long = "log-format", default_value = "text", value_enum)]
    log_format: LogFormat,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    init_logging(cli.log_level, cli.log_format)?;
    let config_path = resolve_config_path(&cli)?;
    run(
        &config_path,
        &RunOptions {
            json: matches!(cli.output, OutputFormat::Json),
            strict: cli.strict,
        },
    )
}

fn resolve_config_path(cli: &Cli) -> Result<PathBuf> {
    if let Some(config) = &cli.config {
        return Ok(config.clone());
    }

    if let Some(use_name) = &cli.use_name {
        if use_name.trim().is_empty() {
            bail!("--use must not be empty");
        }
        let config_home = std::env::var("ENVLOCK_CONFIG_HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|_| default_config_home());
        return Ok(config_home.join("configs").join(format!("{use_name}.json")));
    }

    bail!("one of --config or --use is required")
}

fn default_config_home() -> PathBuf {
    if let Ok(home) = std::env::var("HOME") {
        return PathBuf::from(home).join(".envlock");
    }
    PathBuf::from("~/.envlock")
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum OutputFormat {
    Shell,
    Json,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl From<LogLevel> for tracing_subscriber::filter::LevelFilter {
    fn from(value: LogLevel) -> Self {
        match value {
            LogLevel::Error => Self::ERROR,
            LogLevel::Warn => Self::WARN,
            LogLevel::Info => Self::INFO,
            LogLevel::Debug => Self::DEBUG,
            LogLevel::Trace => Self::TRACE,
        }
    }
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum LogFormat {
    Text,
    Json,
}

fn init_logging(level: LogLevel, format: LogFormat) -> Result<()> {
    let default_level: tracing_subscriber::filter::LevelFilter = level.into();
    let env_filter = EnvFilter::builder()
        .with_default_directive(default_level.into())
        .from_env_lossy();

    match format {
        LogFormat::Text => tracing_subscriber::registry()
            .with(env_filter)
            .with(tracing_subscriber::fmt::layer().with_writer(std::io::stderr))
            .try_init()
            .context("failed to initialize text logger")?,
        LogFormat::Json => tracing_subscriber::registry()
            .with(env_filter)
            .with(
                tracing_subscriber::fmt::layer()
                    .json()
                    .with_writer(std::io::stderr),
            )
            .try_init()
            .context("failed to initialize JSON logger")?,
    }

    Ok(())
}
