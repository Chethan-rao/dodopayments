use serde::Deserialize;

use tracing::level_filters::LevelFilter;
pub use tracing::{debug, error, info, trace, warn};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{EnvFilter, Layer, fmt, layer::SubscriberExt, util::SubscriberInitExt};

/// Represents the logging configuration.
#[derive(Deserialize, Debug, Clone)]
pub struct LogConfig {
    /// The log level.
    pub log_level: LogLevel,
    /// The log format.
    pub log_format: LogFormat,
}

/// Represents the log level.
#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    /// Debug log level.
    Debug,
    /// Info log level.
    Info,
    /// Warn log level.
    Warn,
    /// Error log level.
    Error,
    /// Off log level.
    Off,
}

/// Represents the log format.
#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum LogFormat {
    /// Console log format.
    Console,
    /// JSON log format.
    Json,
}

impl From<LogLevel> for LevelFilter {
    fn from(value: LogLevel) -> Self {
        match value {
            LogLevel::Debug => Self::DEBUG,
            LogLevel::Info => Self::INFO,
            LogLevel::Warn => Self::WARN,
            LogLevel::Error => Self::ERROR,
            LogLevel::Off => Self::OFF,
        }
    }
}

/// Struct to hold the log guard.
pub struct LogGuard {
    _log_guard: WorkerGuard,
}

/// Sets up the logging pipeline.
pub fn setup_logging_pipeline(
    log_config: &LogConfig,
    crates_to_filter: impl AsRef<[&'static str]>,
) -> LogGuard {
    let subscriber = tracing_subscriber::registry();

    let console_filter = get_envfilter(LogLevel::Warn, log_config.log_level, crates_to_filter);

    let (non_blocking, guard) = tracing_appender::non_blocking(std::io::stdout());

    match log_config.log_format {
        LogFormat::Console => {
            let logging_layer = fmt::layer()
                .with_timer(fmt::time::time())
                .pretty()
                .with_writer(non_blocking)
                .with_filter(console_filter);

            subscriber.with(logging_layer).init();
        }
        LogFormat::Json => {
            let logging_layer = fmt::layer()
                .json()
                .with_timer(fmt::time::time())
                .with_writer(non_blocking)
                .with_filter(console_filter);

            subscriber.with(logging_layer).init();
        }
    }

    LogGuard { _log_guard: guard }
}

#[macro_export]
macro_rules! workspace_members {
    () => {
        std::env!("CARGO_WORKSPACE_MEMBERS")
            .split(",")
            .collect::<std::collections::HashSet<&'static str>>()
    };
}

fn get_envfilter<'a>(
    default_log_level: impl Into<LevelFilter> + Copy,
    filter_log_level: impl Into<LevelFilter> + Copy,
    crates_to_filter: impl AsRef<[&'a str]>,
) -> EnvFilter {
    let mut workspace_members = workspace_members!();
    workspace_members.extend(crates_to_filter.as_ref());

    workspace_members
        .drain()
        .zip(std::iter::repeat(filter_log_level.into()))
        .fold(
            EnvFilter::default().add_directive(default_log_level.into().into()),
            |env_filter, (target, level)| {
                // Safety: This is a hardcoded basic filtering directive. If even the basic
                // filter is wrong, it's better to panic.
                #[allow(clippy::expect_used)]
                env_filter.add_directive(
                    format!("{target}={level}")
                        .parse()
                        .expect("Invalid EnvFilter directive format"),
                )
            },
        )
}
