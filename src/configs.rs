use std::path::PathBuf;

use crate::logger::LogConfig;

/// Represents the application configuration.
#[derive(Clone, serde::Deserialize, Debug)]
pub struct Config {
    /// Server configuration.
    pub server: Server,
    /// Database configuration.
    pub database: Database,
    /// Logging configuration.
    pub log: LogConfig,
    /// Cache configuration.
    pub cache: Cache,
    /// Rate limiting configuration.
    pub limit: Limit,
    /// Secrets configuration.
    pub secrets: Secrets,
}

/// Represents the server configuration.
#[derive(Clone, serde::Deserialize, Debug)]
pub struct Server {
    /// The host address.
    pub host: String,
    /// The port number.
    pub port: u16,
}

/// Represents the database configuration.
#[derive(Clone, serde::Deserialize, Debug)]
pub struct Database {
    /// The database username.
    pub username: String,
    /// KMS encrypted password.
    pub password: String,
    /// The database host address.
    pub host: String,
    /// The database port number.
    pub port: u16,
    /// The database name.
    pub dbname: String,
    /// The database pool size.
    pub pool_size: Option<usize>,
}

/// Represents the cache configuration.
#[derive(Clone, serde::Deserialize, Debug)]
pub struct Cache {
    /// Time to idle (in seconds).
    pub tti: Option<u64>,
    /// Maximum capacity of the cache.
    pub max_capacity: u64,
}

/// Represents the rate limiting configuration.
#[derive(Clone, serde::Deserialize, Debug)]
pub struct Limit {
    /// The number of requests allowed.
    pub request_count: u64,
    /// The duration (in seconds) for the rate limit.
    pub duration: u64,
    /// The buffer size.
    pub buffer_size: Option<usize>,
}

/// Represents the secrets configuration.
#[derive(Debug, serde::Deserialize, Clone)]
pub struct Secrets {
    /// KMS encrypted JWT secret.
    pub jwt_secret: String,
}

/// Get the origin directory of the project
pub fn workspace_path() -> PathBuf {
    if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        PathBuf::from(manifest_dir)
    } else {
        PathBuf::from(".")
    }
}

impl Config {
    /// Function to build the configuration by picking it from default locations
    pub fn new() -> Result<Self, config::ConfigError> {
        Self::new_with_config_path(None)
    }

    /// Function to build the configuration by picking it from default locations
    pub fn new_with_config_path(
        explicit_config_path: Option<PathBuf>,
    ) -> Result<Self, config::ConfigError> {
        let env = Env::current_env();
        let config_path = Self::config_path(&env, explicit_config_path);

        let config = Self::builder(&env)?
            .add_source(config::File::from(config_path).required(false))
            .add_source(config::Environment::with_prefix("DODOPAYMENTS").separator("__"))
            .build()?;

        serde_path_to_error::deserialize(config).map_err(|error| {
            eprintln!("Unable to deserialize application configuration: {error}");
            error.into_inner()
        })
    }

    /// Creates a new configuration builder.
    pub fn builder(
        environment: &Env,
    ) -> Result<config::ConfigBuilder<config::builder::DefaultState>, config::ConfigError> {
        config::Config::builder()
            // Here, it should be `set_override()` not `set_default()`.
            // "env" can't be altered by config field.
            // Should be single source of truth.
            .set_override("env", environment.to_string())
    }

    /// Config path.
    pub fn config_path(environment: &Env, explicit_config_path: Option<PathBuf>) -> PathBuf {
        let mut config_path = PathBuf::new();
        if let Some(explicit_config_path_val) = explicit_config_path {
            config_path.push(explicit_config_path_val);
        } else {
            let config_directory: String = "config".into();
            let config_file_name = environment.config_path();

            config_path.push(workspace_path());
            config_path.push(config_directory);
            config_path.push(config_file_name);
        }
        config_path
    }
}

/// Represents the environment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Env {
    /// Development environment.
    Development,
    /// Release environment.
    Release,
}

impl Env {
    /// Gets the current environment.
    pub const fn current_env() -> Self {
        if cfg!(debug_assertions) {
            Self::Development
        } else {
            Self::Release
        }
    }

    /// Gets the configuration path for the environment.
    pub const fn config_path(self) -> &'static str {
        match self {
            Self::Development => "development.toml",
            Self::Release => "production.toml",
        }
    }
}

impl std::fmt::Display for Env {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Development => write!(f, "development"),
            Self::Release => write!(f, "release"),
        }
    }
}
