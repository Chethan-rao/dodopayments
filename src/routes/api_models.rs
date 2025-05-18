use serde::{Deserialize, Serialize};

use crate::{
    error::{ValidationError, container::ContainerError},
    types::{Email, Password},
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SignUpRequest {
    pub name: String,
    pub email: Email,
    pub password: Password,
}

impl SignUpRequest {
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
    pub password: Password,
}

impl LoginRequest {
    pub fn validate(&self) -> Result<(), ContainerError<ValidationError>> {
        self.email.validate()?;
        self.password.validate()?;

        Ok(())
    }
}

#[derive(Serialize, Default, Deserialize, Debug)]
pub struct LoginResponse {
    pub token: String,
    pub user_id: String,
}

#[derive(Serialize, Default, Deserialize, Debug)]
pub struct GetUserResponse {
    pub user_id: String,
    pub email: String,
    pub name: String,
    pub balance_in_rs: f64,
    pub created_at: String,
    pub last_modified_at: String,
}
