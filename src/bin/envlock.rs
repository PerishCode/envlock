use std::path::PathBuf;
use std::process;

use anyhow::{Context, Result};
use clap::{Args, Parser, Subcommand, ValueEnum};
use envlock::commands::alias::{
    AliasAppendOptions, resolve_profile_for_alias, run_append as run_alias_append,
    run_list as run_alias_list,
};
use envlock::commands::preview::{PreviewOutputMode, run as run_preview};
use envlock::commands::profiles::{
    InitProfileType, ProfilesInitOptions, run_init as run_profiles_init,
    run_status as run_profiles_status,
};
use envlock::commands::self_update::{SelfUpdateOptions, run as run_self_update};
use envlock::core::app::{App, AppContext};
use envlock::core::config::{
    CliInput, LogFormat as RuntimeLogFormat, OutputMode, RawEnv, RuntimeConfig,
};
use envlock::run;
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
    Profiles(ProfilesArgs),
    Alias(AliasArgs),
    #[command(external_subcommand)]
    External(Vec<String>),
}

#[derive(Debug, Args)]
struct ProfilesArgs {
    #[command(subcommand)]
    command: ProfilesSubcommand,
}

#[derive(Debug, Subcommand)]
enum ProfilesSubcommand {
    Status,
    Init(ProfilesInitArgs),
}

#[derive(Debug, Args)]
struct ProfilesInitArgs {
    #[arg(long = "type", value_enum, default_value = "minimal")]
    profile_type: ProfileTemplateType,

    #[arg(long = "name")]
    name: Option<String>,

    #[arg(long = "force")]
    force: bool,
}

#[derive(Debug, Args)]
struct AliasArgs {
    #[command(subcommand)]
    command: AliasSubcommand,
}

#[derive(Debug, Subcommand)]
enum AliasSubcommand {
    List,
    Append(AliasAppendArgs),
}

#[derive(Debug, Args)]
struct AliasAppendArgs {
    name: String,

    #[arg(long = "profile")]
    profile: String,
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
            Commands::Profiles(args) => match args.command {
                ProfilesSubcommand::Status => run_profiles_status(),
                ProfilesSubcommand::Init(init) => run_profiles_init(ProfilesInitOptions {
                    profile_type: match init.profile_type {
                        ProfileTemplateType::Minimal => InitProfileType::Minimal,
                        ProfileTemplateType::Sample => InitProfileType::Sample,
                    },
                    name: init.name,
                    force: init.force,
                }),
            },
            Commands::Alias(args) => match args.command {
                AliasSubcommand::List => run_alias_list(),
                AliasSubcommand::Append(append) => run_alias_append(AliasAppendOptions {
                    name: append.name,
                    profile: append.profile,
                }),
            },
            Commands::External(tokens) => run_alias_fallback(&tokens, &cli.run_args),
        };
    }

    if cli.run_args.profile.is_none()
        && !cli.run_args.command.is_empty()
        && resolve_profile_for_alias(&cli.run_args.command[0])?.is_some()
    {
        return run_alias_fallback(&cli.run_args.command, &cli.run_args);
    }

    let config = build_runtime_config(&cli.run_args, None, None)?;
    let app = App::new(config);
    init_logging(app.config().log_level, app.config().log_format)?;
    let result = run(&app)?;
    if let Some(code) = result.exit_code {
        process::exit(code);
    }
    Ok(())
}

fn run_alias_fallback(tokens: &[String], run_args: &RunArgs) -> Result<()> {
    let alias_name = tokens
        .first()
        .context("missing alias token from external command")?;
    let profile = resolve_profile_for_alias(alias_name)?;
    let Some(profile) = profile else {
        anyhow::bail!("unknown command or alias: {}", alias_name);
    };

    let command_override = if tokens.len() > 1 {
        let mut command = tokens[1..].to_vec();
        if command.first().map(String::as_str) == Some("--") {
            command.remove(0);
        }
        Some(command)
    } else {
        Some(Vec::new())
    };
    let config = build_runtime_config(run_args, Some(PathBuf::from(profile)), command_override)?;
    let app = App::new(config);
    init_logging(app.config().log_level, app.config().log_format)?;
    let result = run(&app)?;
    if let Some(code) = result.exit_code {
        process::exit(code);
    }
    Ok(())
}

fn build_runtime_config(
    run_args: &RunArgs,
    profile_override: Option<PathBuf>,
    command_override: Option<Vec<String>>,
) -> Result<RuntimeConfig> {
    RuntimeConfig::from_cli_and_env(
        CliInput {
            profile: profile_override.or_else(|| run_args.profile.clone()),
            output_mode: match run_args.output {
                OutputFormat::Shell => OutputMode::Shell,
                OutputFormat::Json => OutputMode::Json,
            },
            strict: run_args.strict,
            log_level: run_args.log_level.into(),
            log_format: match run_args.log_format {
                LogFormat::Text => RuntimeLogFormat::Text,
                LogFormat::Json => RuntimeLogFormat::Json,
            },
            command: command_override.unwrap_or_else(|| run_args.command.clone()),
        },
        RawEnv::from_process(),
    )
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

#[derive(Debug, Clone, Copy, ValueEnum)]
enum ProfileTemplateType {
    Minimal,
    Sample,
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
