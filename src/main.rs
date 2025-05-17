use std::sync::Arc;

use dodopayments::{app, configs, logger};

#[allow(clippy::expect_used)]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let configs = configs::Config::new().expect("Failed while parsing config");

    let _log_guard = logger::setup_logging_pipeline(&configs.log);

    let app_state = app::AppState::new(configs)
        .await
        .expect("Failed while creating the app state");

    app::server_builder(Arc::new(app_state))
        .await
        .expect("Failed while building the server");

    Ok(())
}
