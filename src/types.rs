use error_stack::report;
use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::{
    error::{ValidationError, container::ContainerError},
    logger,
};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Email(pub String);

impl Email {
    pub fn new(email: String) -> Self {
        Self(email)
    }

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
            Some(regex) => Ok::<Regex, ValidationError>(regex.clone()),
            None => Err(ValidationError::InvalidValue {
                message: "Invalid regex expression".into(),
            }
            .into()),
        }?;

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
