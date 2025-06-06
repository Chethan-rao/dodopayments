use crate::{
    consts,
    error::{
        self, ApiError,
        container::{ContainerError, ResultContainerExt},
    },
    routes::{api_models, user::password},
    storage,
    types::{self, Claims},
    utils,
};

impl TryFrom<api_models::SignUpRequest> for storage::types::UserNew {
    type Error = ContainerError<error::ApiError>;

    fn try_from(new_user: api_models::SignUpRequest) -> Result<Self, Self::Error> {
        let user = storage::types::UserNew {
            user_id: utils::generate_uuid(),
            email: new_user.email.0,
            name: new_user.name,
            password: password::generate_password_hash(new_user.password.0)
                .change_error(error::ApiError::UnknownError("Failed to hash password"))?,
            balance_in_rs: 0.0,
        };
        Ok(user)
    }
}

impl From<storage::types::User> for api_models::SignUpResponse {
    fn from(user: storage::types::User) -> Self {
        Self {
            user_id: user.user_id.to_string(),
            name: user.name.to_string(),
            email: types::Email::new(user.email),
            created_at: user.created_at.to_string(),
        }
    }
}

impl TryFrom<&storage::types::User> for Claims {
    type Error = ContainerError<ApiError>;

    fn try_from(value: &storage::types::User) -> Result<Self, Self::Error> {
        let exp_duration = std::time::Duration::from_secs(consts::JWT_TOKEN_TIME_IN_SECS);
        let exp = utils::generate_exp(exp_duration)?.as_secs();
        Ok(Self {
            user_id: value.user_id.clone(),
            email: value.email.clone(),
            name: value.name.clone(),
            exp,
        })
    }
}

impl From<storage::types::User> for api_models::GetUserResponse {
    fn from(value: storage::types::User) -> Self {
        Self {
            user_id: value.user_id,
            email: value.email,
            name: value.name,
            balance_in_rs: value.balance_in_rs,
            created_at: value.created_at.to_string(),
            last_modified_at: value.last_modified_at.to_string(),
        }
    }
}

impl From<storage::types::User> for api_models::UpdateUserResponse {
    fn from(value: storage::types::User) -> Self {
        Self {
            user_id: value.user_id,
            name: value.name,
            balance_in_rs: value.balance_in_rs,
        }
    }
}
