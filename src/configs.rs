use std::path::PathBuf;

use crate::logger::LogConfig;

#[derive(Clone, serde::Deserialize, Debug)]
pub struct Config {
    pub server: Server,
    pub database: Database,
    pub log: LogConfig,
    pub cache: Cache,
    pub limit: Limit,
    pub secrets: Secrets,
}

#[derive(Clone, serde::Deserialize, Debug)]
pub struct Server {
    pub host: String,
    pub port: u16,
}

#[derive(Clone, serde::Deserialize, Debug)]
pub struct Database {
    pub username: String,
    // KMS encrypted
    pub password: String,
    pub host: String,
    pub port: u16,
    pub dbname: String,
    pub pool_size: Option<usize>,
}

#[derive(Clone, serde::Deserialize, Debug)]
pub struct Cache {
    // time to idle (in secs)
    pub tti: Option<u64>,
    // maximum capacity of the cache
    pub max_capacity: u64,
}

#[derive(Clone, serde::Deserialize, Debug)]
pub struct Limit {
    pub request_count: u64,
    pub duration: u64,
    pub buffer_size: Option<usize>,
}

#[derive(Debug, serde::Deserialize, Clone)]
pub struct Secrets {
    // KMS encrypted
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Env {
    Development,
    Release,
}

impl Env {
    pub const fn current_env() -> Self {
        if cfg!(debug_assertions) {
            Self::Development
        } else {
            Self::Release
        }
    }

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
