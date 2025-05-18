use jsonwebtoken::{EncodingKey, Header, encode};

use crate::{
    error::{
        ApiError,
        container::{ContainerError, ResultContainerExt},
    },
    routes::user::error::UserError,
};

/// Characters to use for generating NanoID
pub(crate) const ALPHABETS: [char; 62] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
    'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B',
    'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U',
    'V', 'W', 'X', 'Y', 'Z',
];

/// Generates a NanoID of the specified length.
pub fn generate_nano_id(id_length: usize) -> String {
    nanoid::nanoid!(id_length, &ALPHABETS)
}

/// Generates a UUID v4 as strings to be used in storage layer
pub fn generate_uuid() -> String {
    uuid::Uuid::now_v7().to_string()
}

/// Generates an expiration time.
pub fn generate_exp(
    exp_duration: std::time::Duration,
) -> Result<std::time::Duration, ContainerError<ApiError>> {
    std::time::SystemTime::now()
        .checked_add(exp_duration)
        .ok_or(ApiError::UnknownError(
            "Failed to add exp_duration to current system time",
        ))?
        .duration_since(std::time::UNIX_EPOCH)
        .change_error(ApiError::UnknownError("Failed to compute elapsed time"))
}

/// Generates a JWT token.
pub fn generate_jwt<T>(
    claims_data: &T,
    jwt_secret: &String,
) -> Result<String, ContainerError<UserError>>
where
    T: serde::ser::Serialize,
{
    encode(
        &Header::default(),
        claims_data,
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    )
    .change_error(UserError::InternalServerError)
}

/// Datetime utilities.
pub mod datetime {
    use time::{OffsetDateTime, PrimitiveDateTime};

    /// Returns the current datetime.
    pub fn now() -> PrimitiveDateTime {
        let utc_date_time: OffsetDateTime = OffsetDateTime::now_utc();
        PrimitiveDateTime::new(utc_date_time.date(), utc_date_time.time())
    }
}
