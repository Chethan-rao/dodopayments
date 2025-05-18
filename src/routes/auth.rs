use std::sync::Arc;

use axum::{extract::FromRequestParts, http::request::Parts};
use hyper::HeaderMap;
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode};

use crate::{
    app::AppState,
    error::{
        ApiError,
        container::{ContainerError, ResultContainerExt},
    },
    logger,
    types::Claims,
};

#[derive(Clone)]
pub struct AuthResolver(pub Claims);

#[async_trait::async_trait]
impl FromRequestParts<Arc<AppState>> for AuthResolver {
    type Rejection = ContainerError<ApiError>;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &Arc<AppState>,
    ) -> Result<Self, Self::Rejection> {
        let headers = &parts.headers;

        authenticate_jwt_key(state, headers)
            .await
            .change_error(ApiError::UnAuthenticated)
    }
}

async fn authenticate_jwt_key(
    state: &Arc<AppState>,
    headers: &HeaderMap,
) -> Result<AuthResolver, ContainerError<ApiError>> {
    let jwt_key = match headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .and_then(|h| {
            if h.is_empty() {
                None
            } else {
                h.strip_prefix("Bearer ").map(|token| token.to_string())
            }
        }) {
        Some(key) => key,
        None => {
            logger::error!("JWT header not found or empty");
            return Err(ApiError::HeadersError("JWT key").into());
        }
    };

    match decode_jwt(&jwt_key, state).await {
        Ok(jwt_claims) => Ok(AuthResolver(jwt_claims)),
        Err(e) => {
            logger::error!("JWT authentication failed: {:?}", e);
            Err(e)
        }
    }
}

async fn decode_jwt(
    token: &str,
    state: &Arc<AppState>,
) -> Result<Claims, ContainerError<ApiError>> {
    let secret = state.config.secrets.jwt_secret.as_bytes();
    let key = DecodingKey::from_secret(secret);
    Ok(
        decode::<Claims>(token, &key, &Validation::new(Algorithm::HS256))
            .map(|token_data| token_data.claims)
            .change_error(ApiError::UnAuthenticated)?,
    )
}
