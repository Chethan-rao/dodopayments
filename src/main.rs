use std::sync::Arc;

use dodopayments::{app, configs, logger};

/// Main function of the application.
#[allow(clippy::expect_used)]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configurations from environment variables and files.
    let configs = configs::Config::new().expect("Failed while parsing config");

    // Set up the logging pipeline.
    let _log_guard =
        logger::setup_logging_pipeline(&configs.log, [dodopayments::service_name!(), "tower_http"]);

    // Create the application state.
    let app_state = app::AppState::new(configs)
        .await
        .expect("Failed while creating the app state");

    // Build and start the server.
    app::server_builder(Arc::new(app_state))
        .await
        .expect("Failed while building the server");

    Ok(())
}
