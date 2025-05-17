use std::sync::Arc;

use axum::{Json, routing::get};

use crate::app::AppState;

///
/// Function for registering routes that is specifically handling the health apis
///
pub fn serve() -> axum::Router<Arc<AppState>> {
    axum::Router::new().route("/", get(health))
}

/// '/health` API handler`
pub async fn health() -> Json<HealthRespPayload> {
    crate::logger::debug!("Health was called");
    Json(HealthRespPayload {
        message: "Health is good".into(),
    })
}

#[derive(serde::Serialize, Debug)]
pub struct HealthRespPayload {
    pub message: String,
}
