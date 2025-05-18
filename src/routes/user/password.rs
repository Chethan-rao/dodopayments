use argon2::{
    Argon2, PasswordHash, PasswordVerifier,
    password_hash::{Error as argon2Err, PasswordHasher, SaltString, rand_core::OsRng},
};
use error_stack::ResultExt;

use crate::{
    error::container::{ContainerError, ResultContainerExt},
    routes::user::error::UserError,
};

/// Generates a password hash using Argon2.
pub fn generate_password_hash(password: String) -> Result<String, ContainerError<UserError>> {
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .change_context(UserError::InternalServerError)?;
    Ok(password_hash.to_string())
}

/// Verifies if the candidate password is correct.
pub fn is_correct_password(
    candidate: &String,
    password: &String,
) -> Result<bool, ContainerError<UserError>> {
    let parsed_hash =
        PasswordHash::new(&password).change_context(UserError::InternalServerError)?;
    let result = Argon2::default().verify_password(candidate.as_bytes(), &parsed_hash);
    match result {
        Ok(_) => Ok(true),
        Err(argon2Err::Password) => Ok(false),
        Err(e) => Err(e),
    }
    .change_error(UserError::InternalServerError)
}
