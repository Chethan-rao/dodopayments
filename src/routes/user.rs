use std::sync::Arc;

use crate::{
    error::{self, container::ContainerError},
    routes::api_models,
    storage::types::User,
};
use axum::{
    Json,
    extract::State,
    routing::{get, post, put},
};
use error_stack::ResultExt;

use crate::app::AppState;

pub fn serve(app_state: Arc<AppState>) -> axum::Router<Arc<AppState>> {
    let router = axum::Router::new().route("/signup", post(sign_up));
    // .route("/login", post(login))
    // .route("/:user_id", get(get_user_profile))
    // .route("/:user_id", put(update_user_profile));

    router
}

#[axum::debug_handler]
async fn sign_up(
    State(app_state): State<Arc<AppState>>,
    Json(new_user_request): Json<api_models::SignUpRequest>,
) -> Result<Json<api_models::SignUpResponse>, ContainerError<error::ApiError>> {
    new_user_request
        .validate()
        .change_context(error::ApiError::ValidationError)?;

    let mut conn = app_state
        .db
        .get_conn()
        .await
        .change_context(error::ApiError::DatabaseError)?;

    // let user_service = UserServiceImpl::new();
    // let new_user = NewUser {
    //     user_id: &Uuid::new_v4().to_string(), // Generate user ID here
    //     email: &new_user_request.email,
    //     name: &new_user_request.name,
    //     password: &new_user_request.password, // Store HASHED password
    //     balance_in_rs: 0,
    //     created_at: Utc::now().naive_utc(),
    //     last_modified_at: Utc::now().naive_utc(),
    // };

    // let created_user = user_service.create_user(new_user, &mut conn).await?;

    Ok(Json(api_models::SignUpResponse::default()))
}
