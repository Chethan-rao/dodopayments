use diesel::{ExpressionMethods, QueryDsl, };
use diesel_async::{RunQueryDsl};
use error_stack::ResultExt;

use crate::{
    error::{
        UserDbError,
        container::{ContainerError, ResultContainerExt},
    },
    storage::{Storage, UserInterface,  types},
};

/// Implementation of the UserInterface for the Storage struct.
impl UserInterface for Storage {
    type Error = UserDbError;

    /// Retrieves a user by their user ID.
    async fn get_user_by_user_id(
        &self,
        _user_id: &str,
    ) -> Result<super::types::User, ContainerError<Self::Error>> {
        use diesel::result::Error;
        use crate::storage::schema::users::dsl::*;

        let mut conn = self.get_conn().await.change_context(UserDbError::DBError)?;

        let output: Result<types::User, diesel::result::Error> = users
            .filter(user_id.eq(_user_id))
            .get_result(&mut conn)
            .await;

        let output = match output {
            Err(err) => match err {
                Error::NotFound => {
                    Err(err).change_error(UserDbError::NotFoundError)
                }
                _ => Err(err).change_error(UserDbError::DBFilterError),
            },
            Ok(user) => Ok(user),
        };

        output
    }

    /// Retrieves a user by their email address.
    async fn get_user_by_email(
        &self,
        _email: &str,
    ) -> Result<super::types::User, ContainerError<Self::Error>> {
        use diesel::result::Error;
        use crate::storage::schema::users::dsl::*;

        let mut conn = self.get_conn().await.change_context(UserDbError::DBError)?;

        let output: Result<types::User, diesel::result::Error> = users
            .filter(email.eq(_email))
            .get_result(&mut conn)
            .await;

        let output = match output {
            Err(err) => match err {
                Error::NotFound => {
                    Err(err).change_error(UserDbError::NotFoundError)
                }
                _ => Err(err).change_error(UserDbError::DBFilterError),
            },
            Ok(user) => Ok(user),
        };

        output
    }

    /// Creates a new user in the database.
    async fn create_user(
        &self,
        user: super::types::UserNew,
    ) -> Result<super::types::User, ContainerError<Self::Error>> {
        use crate::storage::schema::users::dsl::*;

        let mut conn = self.get_conn().await.change_context(UserDbError::DBError)?;

        let query = diesel::insert_into(users).values(user);

        Ok(query
            .get_result(&mut conn)
            .await
            .change_error(UserDbError::DBInsertError)?)
    }

    /// Updates an existing user in the database.
    async fn update_user(
        &self,
        _user_id: &str,
        user_update: super::types::UserUpdateInternal,
    ) -> Result<super::types::User, ContainerError<Self::Error>> {
        use crate::storage::schema::users::dsl::*;

        let mut conn = self.get_conn().await.change_context(UserDbError::DBError)?;

        let query = diesel::update(users)
            .filter(user_id.eq(_user_id))
            .set(user_update);

        Ok(query
            .get_result(&mut conn)
            .await
            .change_error(UserDbError::DBUpdateError)?)
    }
}
