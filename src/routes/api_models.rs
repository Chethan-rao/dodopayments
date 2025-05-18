use serde::{Deserialize, Serialize};

use crate::{
    error::{ValidationError, container::ContainerError},
    types::{Email, Password},
};

/// Represents the sign-up request body.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SignUpRequest {
    pub name: String,
    pub email: Email,
    pub password: Password,
}

impl SignUpRequest {
    /// Validates the sign-up request.
    pub fn validate(&self) -> Result<(), ContainerError<ValidationError>> {
        self.email.validate()?;
        self.password.validate()?;
        if self.name.is_empty() {
            return Err(ValidationError::InvalidValue {
                message: "Name cannot be empty".into(),
            }
            .into());
        }

        Ok(())
    }
}

/// Represents the sign-up response body.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SignUpResponse {
    pub user_id: String,
    pub name: String,
    pub email: Email,
    pub created_at: String,
}

/// Represents the login request body.
#[derive(Serialize, Deserialize, Debug)]
pub struct LoginRequest {
    pub email: Email,
    pub password: Password,
}

impl LoginRequest {
    /// Validates the login request.
    pub fn validate(&self) -> Result<(), ContainerError<ValidationError>> {
        self.email.validate()?;
        self.password.validate()?;

        Ok(())
    }
}

/// Represents the login response body.
#[derive(Serialize, Default, Deserialize, Debug)]
pub struct LoginResponse {
    pub token: String,
    pub user_id: String,
}

/// Represents the get user response body.
#[derive(Serialize, Default, Deserialize, Debug)]
pub struct GetUserResponse {
    pub user_id: String,
    pub email: String,
    pub name: String,
    pub balance_in_rs: f64,
    pub created_at: String,
    pub last_modified_at: String,
}

/// Represents the update user request body.
#[derive(Serialize, Default, Deserialize, Debug)]
pub struct UpdateUserRequest {
    pub name: Option<String>,
    pub amount: Option<f64>,
}

impl UpdateUserRequest {
    /// Validates the update user request.
    pub fn validate(&self) -> Result<(), ContainerError<ValidationError>> {
        if self.name.is_none() && self.amount.is_none() {
            return Err(ValidationError::InvalidValue {
                message: "At least one of name or amount must be provided for update".into(),
            }
            .into());
        }
        if let Some(amount) = self.amount {
            if amount < 0.0 {
                return Err(ValidationError::InvalidValue {
                    message: "Amount cannot be negative".into(),
                }
                .into());
            }
        }

        if let Some(name) = &self.name {
            if name.is_empty() {
                return Err(ValidationError::InvalidValue {
                    message: "Name cannot be empty".into(),
                }
                .into());
            }
        }

        Ok(())
    }
}

/// Represents the update user response body.
#[derive(Serialize, Default, Deserialize, Debug)]
pub struct UpdateUserResponse {
    pub user_id: String,
    pub name: String,
    pub balance_in_rs: f64,
}
