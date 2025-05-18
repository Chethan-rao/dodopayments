pub mod error;
pub mod password;

use std::sync::Arc;

use crate::{
    consts,
    error::{
        ApiError,
        container::{ContainerError, ResultContainerExt},
    },
    routes::api_models,
    storage::{
        UserInterface,
        types::{User, UserNew},
    },
    types::Claims,
    utils,
};
use axum::{
    Json,
    extract::State,
    routing::{get, post, put},
};
use error_stack::ResultExt;

use crate::app::AppState;

pub fn serve(app_state: Arc<AppState>) -> axum::Router<Arc<AppState>> {
    let router = axum::Router::new()
        .route("/signup", post(sign_up))
        .route("/login", post(login));
    // .route("/:user_id", put(update_user_profile));

    router
}

async fn sign_up(
    State(app_state): State<Arc<AppState>>,
    Json(new_user_request): Json<api_models::SignUpRequest>,
) -> Result<Json<api_models::SignUpResponse>, ContainerError<ApiError>> {
    new_user_request
        .validate()
        .change_error(ApiError::ValidationError)?;

    let new_user = UserNew::try_from(new_user_request)?;

    let user = app_state
        .db
        .create_user(new_user)
        .await
        .change_error(ApiError::DatabaseInsertFailed("users"))?;

    Ok(Json(user.into()))
}

async fn login(
    State(app_state): State<Arc<AppState>>,
    Json(login_request): Json<api_models::LoginRequest>,
) -> Result<Json<api_models::LoginResponse>, ContainerError<ApiError>> {
    login_request
        .validate()
        .change_error(ApiError::ValidationError)?;

    let user = app_state
        .db
        .get_user_by_email(&login_request.email.0)
        .await
        .change_error(ApiError::NotFoundError("user"))?;

    if !password::is_correct_password(&login_request.password.0, &user.password).change_error(
        ApiError::UnknownError("Failed while validating user password"),
    )? {
        return Err(ApiError::IncorrectPassword.into());
    }

    let token_payload = Claims::try_from(&user)?;
    let token = utils::generate_jwt(&token_payload, &app_state.config.secrets.jwt_secret)
        .change_error(ApiError::UnknownError(
            "Failed to generate jwt token for user",
        ))?;

    Ok(Json(api_models::LoginResponse {
        token,
        user_id: user.user_id,
    }))
}
