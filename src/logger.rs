use serde::Deserialize;

use tracing::level_filters::LevelFilter;
pub use tracing::{debug, error, info, trace, warn};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{EnvFilter, Layer, fmt, layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Deserialize, Debug, Clone)]
pub struct LogConfig {
    pub log_level: LogLevel,
    pub log_format: LogFormat,
}

#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
    Off,
}

#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum LogFormat {
    Console,
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

pub struct LogGuard {
    _log_guard: WorkerGuard,
}

pub fn setup_logging_pipeline(log_config: &LogConfig) -> LogGuard {
    let subscriber = tracing_subscriber::registry();

    let console_filter = get_envfilter(LogLevel::Warn, log_config.log_level);

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
) -> EnvFilter {
    let mut workspace_members = workspace_members!();

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
