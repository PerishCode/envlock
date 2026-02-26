use std::path::PathBuf;
use std::process;

use anyhow::{Context, Result};
use clap::{Parser, ValueEnum};
use envlock::app::{App, AppContext};
use envlock::config::{CliInput, LogFormat as RuntimeLogFormat, OutputMode, RawEnv, RuntimeConfig};
use envlock::run;
use tracing_subscriber::{EnvFilter, prelude::*};

#[derive(Debug, Parser)]
#[command(
    name = "envlock",
    version,
    about = "Build environment sessions from JSON profile"
)]
struct Cli {
    #[arg(short = 'p', long = "profile")]
    profile: Option<PathBuf>,

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

    #[arg(trailing_var_arg = true)]
    command: Vec<String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = RuntimeConfig::from_cli_and_env(
        CliInput {
            profile: cli.profile,
            use_name: cli.use_name,
            output_mode: match cli.output {
                OutputFormat::Shell => OutputMode::Shell,
                OutputFormat::Json => OutputMode::Json,
            },
            strict: cli.strict,
            log_level: cli.log_level.into(),
            log_format: match cli.log_format {
                LogFormat::Text => RuntimeLogFormat::Text,
                LogFormat::Json => RuntimeLogFormat::Json,
            },
            command: cli.command,
        },
        RawEnv::from_process(),
    )?;
    let app = App::new(config);
    init_logging(app.config().log_level, app.config().log_format)?;
    let result = run(&app)?;
    if let Some(code) = result.exit_code {
        process::exit(code);
    }
    Ok(())
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

fn init_logging(
    level: tracing_subscriber::filter::LevelFilter,
    format: RuntimeLogFormat,
) -> Result<()> {
    let env_filter = EnvFilter::builder()
        .with_default_directive(level.into())
        .from_env_lossy();

    match format {
        RuntimeLogFormat::Text => tracing_subscriber::registry()
            .with(env_filter)
            .with(tracing_subscriber::fmt::layer().with_writer(std::io::stderr))
            .try_init()
            .context("failed to initialize text logger")?,
        RuntimeLogFormat::Json => tracing_subscriber::registry()
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
