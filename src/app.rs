use std::sync::Arc;

use axum::{error_handling::HandleErrorLayer, response::IntoResponse};
use error_stack::ResultExt;
use tower_http::trace as tower_trace;

use crate::{
    configs::Config,
    error, logger, routes,
    storage::{self, caching::Caching},
};

const BUFFER_LIMIT: usize = 1024;

async fn ratelimit_err_handler(_: axum::BoxError) -> impl IntoResponse {
    (hyper::StatusCode::TOO_MANY_REQUESTS, "Rate Limit Applied")
}

type Storage = Caching<storage::Storage>;

pub struct AppState {
    pub db: Storage,
    pub config: Config,
}

impl AppState {
    pub async fn new(config: Config) -> error_stack::Result<Self, error::ConfigurationError> {
        #[allow(clippy::map_identity)]
        let db = storage::Storage::new(&config.database)
            .await
            .map(Caching::implement_cache(&config.cache))
            .change_context(error::ConfigurationError::DatabaseError)?;

        Ok(Self { db, config })
    }
}

pub async fn server_builder(app_state: Arc<AppState>) -> Result<(), error::ConfigurationError>
where
{
    let socket_addr = std::net::SocketAddr::new(
        app_state.config.server.host.parse()?,
        app_state.config.server.port,
    );

    let ratelimit_middleware = tower::ServiceBuilder::new()
        .layer(HandleErrorLayer::new(ratelimit_err_handler))
        .buffer(app_state.config.limit.buffer_size.unwrap_or(BUFFER_LIMIT))
        .load_shed()
        .rate_limit(
            app_state.config.limit.request_count,
            std::time::Duration::from_secs(app_state.config.limit.duration),
        )
        .into_inner();

    let router = axum::Router::new()
        .nest("/user", routes::user::serve(app_state.clone()))
        .nest(
            "/transaction",
            routes::transaction::serve(app_state.clone()),
        )
        .layer(ratelimit_middleware);

    let router = router.layer(
        tower_trace::TraceLayer::new_for_http()
            .on_request(tower_trace::DefaultOnRequest::new().level(tracing::Level::INFO))
            .on_response(
                tower_trace::DefaultOnResponse::new()
                    .level(tracing::Level::INFO)
                    .latency_unit(tower_http::LatencyUnit::Micros),
            )
            .on_failure(
                tower_trace::DefaultOnFailure::new()
                    .latency_unit(tower_http::LatencyUnit::Micros)
                    .level(tracing::Level::ERROR),
            ),
    );

    let router = router
        .nest("/health", routes::health::serve())
        .with_state(app_state.clone());

    logger::info!(startup_config=?app_state.config, "App started");

    let tcp_listener = tokio::net::TcpListener::bind(socket_addr).await?;

    axum::serve(tcp_listener, router.into_make_service()).await?;

    Ok(())
}
