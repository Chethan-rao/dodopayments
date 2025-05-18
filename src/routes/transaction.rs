use std::sync::Arc;

use axum::routing::post;

use crate::app::AppState;

pub fn serve(_app_state: Arc<AppState>) -> axum::Router<Arc<AppState>> {
    let router = axum::Router::new().route("/create", post(create_transaction));

    router
}

async fn create_transaction() -> &'static str {
    "Txn Created"
}
