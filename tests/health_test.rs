#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        http::{Request, StatusCode},
        routing::get,
        Router,
    };
    use tower::ServiceExt;

    /// Handler for the health check route.
    async fn health_check() -> StatusCode {
        StatusCode::OK
    }

    /// Creates the application router.
    fn app() -> Router {
        Router::new().route("/health", get(health_check))
    }

    /// Tests the health check route.
    #[tokio::test]
    async fn health_check_route() {
        let app = app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
