use error_stack::report;
use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::{
    error::{ValidationError, container::ContainerError},
    logger,
};

/// Maximum password length.
pub const MAX_PASSWORD_LENGTH: usize = 70;
/// Minimum password length.
pub const MIN_PASSWORD_LENGTH: usize = 8;

/// Represents an email address.
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Email(pub String);

impl Email {
    /// Creates a new Email.
    pub fn new(email: String) -> Self {
        Self(email)
    }

    /// Validates the email address.
    pub fn validate(&self) -> Result<(), ContainerError<ValidationError>> {
        let email = &self.0;
        static EMAIL_REGEX: Lazy<Option<Regex>> = Lazy::new(|| {
            match Regex::new(
                r"^(?i)[a-z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?(?:\.[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?)+$",
            ) {
                Ok(regex) => Some(regex),
                Err(_error) => {
                    logger::error!(?_error);
                    None
                }
            }
        });
       let email_regex = match EMAIL_REGEX.as_ref() {
            Some(regex) => regex.clone(),
            None => {
                return Err(ValidationError::InvalidValue {
                    message: "Invalid regex expression".into(),
                }
                .into());
            }
        };

        const EMAIL_MAX_LENGTH: usize = 319;
        if email.is_empty() || email.chars().count() > EMAIL_MAX_LENGTH {
            return Err(report!(ValidationError::InvalidValue {
                message: "Email address is either empty or exceeds maximum allowed length".into()
            })
            .into());
        }

        if !email_regex.is_match(email) {
            return Err(report!(ValidationError::InvalidValue {
                message: "Invalid email address format".into()
            })
            .into());
        }

        Ok(())
    }
}

/// Represents a password.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Password(pub String);

impl Password {
    /// Creates a new Password.
    pub fn new(password: String) -> Self {
        Self(password)
    }

    /// Validates the password.
    pub fn validate(&self) -> Result<(), ContainerError<ValidationError>> {
        let password = &self.0;

        let mut has_upper_case = false;
        let mut has_lower_case = false;
        let mut has_numeric_value = false;
        let mut has_special_character = false;
        let mut has_whitespace = false;

        for c in password.chars() {
            has_upper_case = has_upper_case || c.is_uppercase();
            has_lower_case = has_lower_case || c.is_lowercase();
            has_numeric_value = has_numeric_value || c.is_numeric();
            has_special_character = has_special_character || !c.is_alphanumeric();
            has_whitespace = has_whitespace || c.is_whitespace();
        }

        let is_password_format_valid = has_upper_case
            && has_lower_case
            && has_numeric_value
            && has_special_character
            && !has_whitespace;

        let is_too_long = password.chars().count() > MAX_PASSWORD_LENGTH;
        let is_too_short = password.chars().count() < MIN_PASSWORD_LENGTH;

        if is_too_short || is_too_long || !is_password_format_valid {
            return Err(ValidationError::IncorrectValueProvided {
                field_name: "password",
            }
            .into());
        }

        Ok(())
    }
}

/// Define the claims structure, including user information.
#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct Claims {
    pub user_id: String,
    pub email: String,
    pub name: String,
    pub exp: u64,
}
