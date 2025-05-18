//! dodopayments

/// Application modules
pub mod app;
/// Configuration modules
pub mod configs;
/// Constant values
pub mod consts;
/// Error definitions
pub mod error;
/// Logging setup
pub mod logger;
/// Route definitions
pub mod routes;
/// Storage layer
pub mod storage;
/// Type definitions
pub mod types;
/// Utility functions
pub mod utils;

/// Macro for defining the service name
#[macro_export]
macro_rules! service_name {
    () => {
        env!("CARGO_PKG_NAME")
    };
}
