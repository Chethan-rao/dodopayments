/// User specific errors
#[derive(Debug, thiserror::Error, Clone, PartialEq)]
pub enum UserError {
    /// User InternalServerError
    #[error("User InternalServerError")]
    InternalServerError,
    /// User not found
    #[error("User not found")]
    UserNotFound,
    /// User already exists
    #[error("User already exists")]
    UserAlreadyExists,
    /// Invalid credentials
    #[error("Invalid credentials")]
    InvalidCredentials,
    /// Invalid token
    #[error("Invalid token")]
    InvalidToken,
    /// Token expired
    #[error("Token expired")]
    TokenExpired,
}
