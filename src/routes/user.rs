pub mod error;
pub mod password;

use std::sync::Arc;

use crate::{
    error::{
        ApiError,
        container::{ContainerError, ResultContainerExt},
    },
    logger,
    routes::{api_models, auth::AuthResolver},
    storage::{
        UserInterface,
        types::{UserNew, UserUpdateInternal},
    },
    types::Claims,
    utils,
};
use axum::{
    Json,
    extract::State,
    routing::{get, post, put},
};



use crate::app::AppState;

pub fn serve(_app_state: Arc<AppState>) -> axum::Router<Arc<AppState>> {
    let router = axum::Router::new()
        .route("/signup", post(sign_up))
        .route("/login", post(login))
        .route("/", get(get_user_profile))
        .route("/", put(update_user));

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

    logger::info!("User created with user_id: {}", user.user_id);

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

    logger::info!("User logged in with user_id: {}", user.user_id);

    Ok(Json(api_models::LoginResponse {
        token,
        user_id: user.user_id,
    }))
}

async fn get_user_profile(
    State(app_state): State<Arc<AppState>>,
    AuthResolver(user_info): AuthResolver,
) -> Result<Json<api_models::GetUserResponse>, ContainerError<ApiError>> {
    let user = app_state
        .db
        .get_user_by_user_id(&user_info.user_id)
        .await
        .change_error(ApiError::NotFoundError("user"))?;

    logger::info!("User profile fetched with user_id: {}", user.user_id);

    Ok(Json(user.into()))
}

async fn update_user(
    State(app_state): State<Arc<AppState>>,
    AuthResolver(user_info): AuthResolver,
    Json(update_request): Json<api_models::UpdateUserRequest>,
) -> Result<Json<api_models::UpdateUserResponse>, ContainerError<ApiError>> {
    update_request
        .validate()
        .change_error(ApiError::ValidationError)?;

    let user = app_state
        .db
        .get_user_by_user_id(&user_info.user_id)
        .await
        .change_error(ApiError::NotFoundError("user"))?;

    let updated_user = app_state
        .db
        .update_user(
            &user.user_id,
            UserUpdateInternal::new(update_request.name, update_request.amount),
        )
        .await
        .change_error(ApiError::DatabaseUpdationFailed("users"))?;

    logger::info!("User updated with user_id: {}", user.user_id);

    Ok(Json(updated_user.into()))
}
