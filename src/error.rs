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

    #[error("Error while encoding data")]
    EncodingError,

    #[error("Failed while decoding data")]
    DecodingError,

    #[error("Failed while inserting data into {0}")]
    DatabaseInsertFailed(&'static str),

    #[error("failed while deleting data from {0}")]
    DatabaseDeleteFailed(&'static str),

    #[error("Something went wrong: {0}")]
    UnknownError(&'static str),

    #[error("Failed while connecting to database")]
    DatabaseError,

    #[error("Failed while validating request")]
    ValidationError,

    #[error("Incorrect password")]
    IncorrectPassword,

    #[error("Requested resource not found: {0}")]
    NotFoundError(&'static str),

    #[error("The request source is un-authenticated")]
    UnAuthenticated,

    #[error("Failed to parse headers: {0}")]
    HeadersError(&'static str),
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
            data @ Self::IncorrectPassword | data @ Self::UnAuthenticated => (
                hyper::StatusCode::UNAUTHORIZED,
                axum::Json(ApiErrorResponse::new(
                    error_codes::TE_00,
                    format!("{}", data),
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

            data @ Self::UnknownError(_) => (
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
            data @ Self::DecodingError
            | data @ Self::ValidationError
            | data @ Self::HeadersError(_) => (
                hyper::StatusCode::BAD_REQUEST,
                axum::Json(ApiErrorResponse::new(
                    error_codes::TE_03,
                    format!("{}", data),
                    None,
                )),
            )
                .into_response(),
            data @ Self::NotFoundError(_) => (
                hyper::StatusCode::NOT_FOUND,
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

#[derive(Debug, thiserror::Error, Clone, PartialEq)]
pub enum UserDbError {
    #[error("Error while connecting to database")]
    DBError,
    #[error("Error while finding user record in the database")]
    DBFilterError,
    #[error("Error while inserting user record in the database")]
    DBInsertError,
    #[error("Error while updating user record in the database")]
    DBUpdateError,
    #[error("Unpredictable error occurred")]
    UnknownError,
    #[error("Element not found in storage")]
    NotFoundError,
}

#[derive(Debug, thiserror::Error, Clone, PartialEq)]
pub enum TransactionDbError {
    #[error("Error while connecting to database")]
    DBError,
    #[error("Error while finding transaction record in the database")]
    DBFilterError,
    #[error("Error while inserting transaction record in the database")]
    DBInsertError,
    #[error("Error while updating transaction record in the database")]
    DBUpdateError,
    #[error("Unpredictable error occurred")]
    UnknownError,
    #[error("Element not found in storage")]
    NotFoundError,
}
