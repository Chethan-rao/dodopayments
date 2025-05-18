use std::sync::Arc;

use axum::routing::post;

use crate::app::AppState;

/// Serves the transaction routes.
pub fn serve(_app_state: Arc<AppState>) -> axum::Router<Arc<AppState>> {
    let router = axum::Router::new().route("/create", post(create_transaction));

    router
}

/// Handles the create transaction request.
async fn create_transaction() -> &'static str {
    "Txn Created"
}
