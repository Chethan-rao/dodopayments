use crate::error::container::ContainerError;

pub mod container;

#[derive(Debug, thiserror::Error)]
pub enum ConfigurationError {
    #[error("error while creating the webserver")]
    ServerError(#[from] hyper::Error),
    #[error("invalid host for socket")]
    AddressError(#[from] std::net::AddrParseError),
    #[error("invalid host for socket")]
    IOError(#[from] std::io::Error),
    #[error("Error while connecting/creating database pool")]
    DatabaseError,
    #[error("Invalid configuration value provided: {0}")]
    InvalidConfigurationValueError(String),
}

#[derive(Debug, thiserror::Error, Clone, PartialEq)]
pub enum ValidationError {
    #[error("Missing required field: {field_name}")]
    MissingRequiredField { field_name: String },

    #[error("Incorrect value provided for field: {field_name}")]
    IncorrectValueProvided { field_name: &'static str },

    #[error("{message}")]
    InvalidValue { message: String },
}

#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("failed to construct database pool")]
    DBPoolError,
    #[error("failed to construct database pool")]
    PoolClientFailure,
    #[error("Error while finding element in database")]
    FindError,
    #[error("Error while inserting data in database")]
    InsertError,
    #[error("Error while deleting data in database")]
    DeleteError,
    #[error("Element not found in storage")]
    NotFoundError,
}

#[derive(Debug, Copy, Clone, thiserror::Error)]
pub enum ApiError {
    #[error("failed while retrieving stored data")]
    RetrieveDataFailed(&'static str),

    #[error("failed to decrypt two custodian keys: {0}")]
    DecryptingKeysFailed(&'static str),

    #[error("failed in request middleware: {0}")]
    RequestMiddlewareError(&'static str),

    #[error("failed in response middleware: {0}")]
    ResponseMiddlewareError(&'static str),

    #[error("Error while encoding data")]
    EncodingError,

    #[error("Failed while decoding data")]
    DecodingError,

    #[error("Failed while inserting data into {0}")]
    DatabaseInsertFailed(&'static str),

    #[error("failed while deleting data from {0}")]
    DatabaseDeleteFailed(&'static str),

    #[error("Failed while getting merchant from DB")]
    MerchantError,

    #[error("Something went wrong")]
    UnknownError,

    #[error("Error while encrypting with merchant key")]
    MerchantKeyError,

    #[error("Failed while connecting to database")]
    DatabaseError,

    #[error("Failed while validating request")]
    ValidationError,

    #[error("Requested resource not found")]
    NotFoundError,

    #[error("TTL is invalid")]
    InvalidTtl,

    #[error("Custodian is locked")]
    CustodianLocked,

    #[error("Custodian is already unlocked")]
    CustodianUnlocked,

    #[error("Tenant error: {0}")]
    TenantError(&'static str),

    #[error("Key manager error: {0}")]
    KeyManagerError(&'static str),
}

/// Error code constants.
mod error_codes {
    /// Processing error: Indicates an error that occurred during processing of a task or operation.
    pub const TE_00: &str = "TE_00";

    /// Database error: Denotes an error related to database operations or connectivity.
    pub const TE_01: &str = "TE_01";

    /// Resource not found: Signifies that the requested resource could not be located.
    pub const TE_02: &str = "TE_02";

    /// Validation error: Represents an error occurring during data validation or integrity checks.
    pub const TE_03: &str = "TE_03";
}

impl axum::response::IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::CustodianLocked => (
                hyper::StatusCode::UNAUTHORIZED,
                axum::Json(ApiErrorResponse::new(
                    error_codes::TE_00,
                    "Custodian is locked".into(),
                    None,
                )),
            )
                .into_response(),
            Self::DecryptingKeysFailed(err) => (
                hyper::StatusCode::UNAUTHORIZED,
                axum::Json(ApiErrorResponse::new(
                    error_codes::TE_00,
                    format!("Failed while decrypting two custodian keys: {err}"),
                    None,
                )),
            )
                .into_response(),
            data @ Self::EncodingError => (
                hyper::StatusCode::INTERNAL_SERVER_ERROR,
                axum::Json(ApiErrorResponse::new(
                    error_codes::TE_00,
                    format!("{}", data),
                    None,
                )),
            )
                .into_response(),
            data @ Self::ResponseMiddlewareError(_)
            | data @ Self::UnknownError
            | data @ Self::MerchantKeyError
            | data @ Self::KeyManagerError(_) => (
                hyper::StatusCode::INTERNAL_SERVER_ERROR,
                axum::Json(ApiErrorResponse::new(
                    error_codes::TE_00,
                    format!("{}", data),
                    None,
                )),
            )
                .into_response(),

            data @ Self::DatabaseInsertFailed(_)
            | data @ Self::DatabaseError
            | data @ Self::DatabaseDeleteFailed(_)
            | data @ Self::RetrieveDataFailed(_) => (
                hyper::StatusCode::INTERNAL_SERVER_ERROR,
                axum::Json(ApiErrorResponse::new(
                    error_codes::TE_01,
                    format!("{}", data),
                    None,
                )),
            )
                .into_response(),
            data @ Self::RequestMiddlewareError(_)
            | data @ Self::DecodingError
            | data @ Self::ValidationError
            | data @ Self::InvalidTtl
            | data @ Self::CustodianUnlocked
            | data @ Self::TenantError(_) => (
                hyper::StatusCode::BAD_REQUEST,
                axum::Json(ApiErrorResponse::new(
                    error_codes::TE_03,
                    format!("{}", data),
                    None,
                )),
            )
                .into_response(),
            data @ Self::NotFoundError => (
                hyper::StatusCode::NOT_FOUND,
                axum::Json(ApiErrorResponse::new(
                    error_codes::TE_02,
                    format!("{}", data),
                    None,
                )),
            )
                .into_response(),
            data @ Self::MerchantError => (
                hyper::StatusCode::INTERNAL_SERVER_ERROR,
                axum::Json(ApiErrorResponse::new(
                    error_codes::TE_02,
                    format!("{}", data),
                    None,
                )),
            )
                .into_response(),
        }
    }
}

impl<T: axum::response::IntoResponse + error_stack::Context + Copy> axum::response::IntoResponse
    for ContainerError<T>
{
    fn into_response(self) -> axum::response::Response {
        crate::logger::error!(error=?self.error);
        (*self.error.current_context()).into_response()
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ApiErrorResponse {
    code: &'static str,
    message: String,
    data: Option<serde_json::Value>,
}

impl ApiErrorResponse {
    fn new(code: &'static str, message: String, data: Option<serde_json::Value>) -> Self {
        Self {
            code,
            message,
            data,
        }
    }
}
