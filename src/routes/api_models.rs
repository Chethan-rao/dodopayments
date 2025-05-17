use serde::{Deserialize, Serialize};

use crate::{
    error::{ValidationError, container::ContainerError},
    types::Email,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SignUpRequest {
    pub name: String,
    pub email: Email,
    pub password: String,
}

impl SignUpRequest {
    pub fn validate(&self) -> Result<(), ContainerError<ValidationError>> {
        self.email.validate()?;
        if self.name.is_empty() {
            return Err(ValidationError::InvalidValue {
                message: "Name cannot be empty".into(),
            }
            .into());
        }
        if self.password.is_empty() {
            return Err(ValidationError::InvalidValue {
                message: "Password cannot be empty".into(),
            }
            .into());
        }
        if self.password.len() < 8 {
            return Err(ValidationError::InvalidValue {
                message: "Password must be at least 8 characters long".into(),
            }
            .into());
        }
        if !self.password.chars().any(|c| c.is_digit(10)) {
            return Err(ValidationError::InvalidValue {
                message: "Password must contain at least one digit".into(),
            }
            .into());
        }
        if !self.password.chars().any(|c| c.is_alphabetic()) {
            return Err(ValidationError::InvalidValue {
                message: "Password must contain at least one letter".into(),
            }
            .into());
        }
        if !self.password.chars().any(|c| c.is_ascii_punctuation()) {
            return Err(ValidationError::InvalidValue {
                message: "Password must contain at least one special character".into(),
            }
            .into());
        }
        if self.password.chars().any(|c| c.is_whitespace()) {
            return Err(ValidationError::InvalidValue {
                message: "Password cannot contain whitespace".into(),
            }
            .into());
        }
        if self.password.chars().count() > 20 {
            return Err(ValidationError::InvalidValue {
                message: "Password exceeds maximum allowed length".into(),
            }
            .into());
        }
        if self.password.chars().count() < 8 {
            return Err(ValidationError::InvalidValue {
                message: "Password must be at least 8 characters long".into(),
            }
            .into());
        }
        if self.password.chars().any(|c| c.is_whitespace()) {
            return Err(ValidationError::InvalidValue {
                message: "Password cannot contain whitespace".into(),
            }
            .into());
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SignUpResponse {
    pub user_id: String,
    pub name: String,
    pub email: Email,
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginRequest {
    pub email: Email,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginResponse {
    pub token: String,
    pub user_id: String,
}
