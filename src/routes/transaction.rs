use std::sync::Arc;

use crate::{
    app::AppState,
    error::{
        ApiError,
        container::{ContainerError, ResultContainerExt},
    },
    routes::{
        api_models::{
            CreateTransactionRequest, GetTransactionResponse, ListTransactionsRequest,
            ListTransactionsResponse,
        },
        auth::AuthResolver,
    },
    storage::{
        TransactionInterface,
        types::{self, Transaction},
    },
    utils::{datetime, generate_nano_id},
};
use axum::{
    Json, Router,
    extract::{Path, Query, State},
    response::IntoResponse,
    routing::{get, post},
};
use diesel::BoolExpressionMethods;
use error_stack::ResultExt;

/// Serves transaction routes.
pub fn serve(app_state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(create_transaction))
        .route("/:transaction_id", get(get_transaction))
        .route("/", get(list_transactions))
        .with_state(app_state)
}

/// Creates a new transaction.
async fn create_transaction(
    State(app_state): State<Arc<AppState>>,
    AuthResolver(claims): AuthResolver,
    Json(payload): Json<CreateTransactionRequest>,
) -> Result<impl IntoResponse, ContainerError<ApiError>> {
    payload.validate().change_error(ApiError::ValidationError)?;

    let transaction_id = generate_nano_id(10);
    let created_at = datetime::now();

    let new_transaction = types::NewTransaction {
        transaction_id: transaction_id.clone(),
        sender_id: payload.sender_id,
        recipient_id: payload.receiver_id,
        amount_in_rs: payload.amount,
        created_at,
        description: None,
        status: "PENDING".to_string(),
        updated_at: datetime::now(),
    };

    let transaction = app_state
        .db
        .create_transaction(new_transaction)
        .await
        .change_error(ApiError::TransactionDatabaseError)?;

    let response = GetTransactionResponse {
        transaction_id: transaction.transaction_id,
        sender_id: transaction.sender_id,
        receiver_id: transaction.recipient_id,
        amount: transaction.amount_in_rs,
        created_at: transaction.created_at.to_string(),
    };

    Ok((axum::http::StatusCode::CREATED, Json(response)))
}

/// Gets a transaction by ID.
async fn get_transaction(
    State(app_state): State<Arc<AppState>>,
    Path(transaction_id): Path<String>,
    AuthResolver(claims): AuthResolver,
) -> Result<impl IntoResponse, ContainerError<ApiError>> {
    let transaction = app_state
        .db
        .get_transaction_by_id(&transaction_id)
        .await
        .change_error(ApiError::TransactionDatabaseError)?;

    let response = GetTransactionResponse {
        transaction_id: transaction.transaction_id,
        sender_id: transaction.sender_id,
        receiver_id: transaction.recipient_id,
        amount: transaction.amount_in_rs,
        created_at: transaction.created_at.to_string(),
    };

    Ok((axum::http::StatusCode::OK, Json(response)))
}

/// Lists transactions for a user with pagination.
async fn list_transactions(
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<ListTransactionsRequest>,
    AuthResolver(claims): AuthResolver,
) -> Result<impl IntoResponse, ContainerError<ApiError>> {
    use crate::error::container::ResultContainerExt;

    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(10);
    let user_id = &claims.user_id;

    let (transactions, total_count) =
        get_paginated_transactions(app_state.clone(), user_id, page, page_size)
            .await
            .change_error(ApiError::TransactionDatabaseError)?;

    let response = ListTransactionsResponse {
        transactions,
        total_count,
        page,
        page_size,
    };

    Ok((axum::http::StatusCode::OK, Json(response)))
}

async fn get_paginated_transactions(
    app_state: Arc<AppState>,
    user_id: &str,
    page: u64,
    page_size: u64,
) -> Result<(Vec<GetTransactionResponse>, u64), ContainerError<ApiError>> {
    use crate::error::container::ResultContainerExt;
    use diesel::ExpressionMethods;
    use diesel::QueryDsl;
    use diesel::dsl::count;
    use diesel_async::RunQueryDsl;

    use crate::storage::schema::transactions::dsl::*;

    let mut conn = app_state
        .db
        .get_conn()
        .await
        .change_error(ApiError::DatabaseError)?;

    let offset = (page - 1) * page_size;

    let transaction_list: Vec<Transaction> = transactions
        .filter(sender_id.eq(user_id).or(recipient_id.eq(user_id)))
        .limit(page_size as i64)
        .offset(offset as i64)
        .load(&mut conn) // Use load instead of get_results
        .await
        .change_error(ApiError::TransactionDatabaseError)?;

    let mut transaction_response_list: Vec<GetTransactionResponse> = Vec::new();

    for transaction in transaction_list {
        transaction_response_list.push(GetTransactionResponse {
            transaction_id: transaction.transaction_id,
            sender_id: transaction.sender_id,
            receiver_id: transaction.recipient_id,
            amount: transaction.amount_in_rs,
            created_at: transaction.created_at.to_string(),
        });
    }

    let total_transaction: i64 = transactions
        .filter(sender_id.eq(user_id).or(recipient_id.eq(user_id)))
        .select(count(transaction_id)) // Select count
        .get_result(&mut conn)
        .await
        .change_error(ApiError::DatabaseError)?;

    Ok((transaction_response_list, total_transaction as u64))
}
