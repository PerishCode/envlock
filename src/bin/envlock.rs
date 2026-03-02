use std::path::PathBuf;
use std::process;

use anyhow::{Context, Result};
use clap::{Args, Parser, Subcommand, ValueEnum};
use envlock::app::{App, AppContext};
use envlock::config::{CliInput, LogFormat as RuntimeLogFormat, OutputMode, RawEnv, RuntimeConfig};
use envlock::preview::{PreviewOutputMode, run as run_preview};
use envlock::run;
use envlock::self_update::{SelfUpdateOptions, run as run_self_update};
use tracing_subscriber::{EnvFilter, prelude::*};

#[derive(Debug, Parser)]
#[command(
    name = "envlock",
    version,
    about = "Build environment sessions from JSON profile",
    after_help = "Docs: https://perishcode.github.io/envlock/"
)]
struct Cli {
    #[command(subcommand)]
    subcommand: Option<Commands>,

    #[command(flatten)]
    run_args: RunArgs,
}

#[derive(Debug, Subcommand)]
enum Commands {
    SelfUpdate(SelfUpdateArgs),
    Preview(PreviewArgs),
}

#[derive(Debug, Args)]
struct SelfUpdateArgs {
    #[arg(long = "check")]
    check: bool,

    #[arg(long = "version")]
    version: Option<String>,

    #[arg(long = "yes", short = 'y')]
    yes: bool,
}

#[derive(Debug, Args)]
struct PreviewArgs {
    #[arg(short = 'p', long = "profile")]
    profile: PathBuf,

    #[arg(long = "output", default_value = "text", value_enum)]
    output: PreviewOutputFormat,
}

#[derive(Debug, Args)]
struct RunArgs {
    #[arg(short = 'p', long = "profile")]
    profile: Option<PathBuf>,

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
    if let Some(command) = cli.subcommand {
        return match command {
            Commands::SelfUpdate(args) => run_self_update(SelfUpdateOptions {
                check_only: args.check,
                version: args.version,
                yes: args.yes,
            }),
            Commands::Preview(args) => run_preview(
                &args.profile,
                match args.output {
                    PreviewOutputFormat::Text => PreviewOutputMode::Text,
                    PreviewOutputFormat::Json => PreviewOutputMode::Json,
                },
            ),
        };
    }

    let config = RuntimeConfig::from_cli_and_env(
        CliInput {
            profile: cli.run_args.profile,
            output_mode: match cli.run_args.output {
                OutputFormat::Shell => OutputMode::Shell,
                OutputFormat::Json => OutputMode::Json,
            },
            strict: cli.run_args.strict,
            log_level: cli.run_args.log_level.into(),
            log_format: match cli.run_args.log_format {
                LogFormat::Text => RuntimeLogFormat::Text,
                LogFormat::Json => RuntimeLogFormat::Json,
            },
            command: cli.run_args.command,
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

#[derive(Debug, Clone, Copy, ValueEnum)]
enum PreviewOutputFormat {
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
