use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
use diesel_async::{AsyncConnection, RunQueryDsl};
use error_stack::ResultExt;

use crate::{
    error::{
        UserDbError,
        container::{ContainerError, ResultContainerExt},
    },
    routes::user::error,
    storage::{Storage, UserInterface, schema, types},
};

impl UserInterface for Storage {
    type Error = UserDbError;

    async fn get_user_by_user_id(
        &self,
        user_id: &str,
    ) -> Result<super::types::User, ContainerError<Self::Error>> {
        let mut conn = self.get_conn().await.change_context(UserDbError::DBError)?;

        let output: Result<types::User, diesel::result::Error> = types::User::table()
            .filter(schema::users::user_id.eq(user_id))
            .get_result(&mut conn)
            .await;

        let output = match output {
            Err(err) => match err {
                diesel::result::Error::NotFound => {
                    Err(err).change_error(UserDbError::NotFoundError)
                }
                _ => Err(err).change_error(UserDbError::DBFilterError),
            },
            Ok(user) => Ok(user),
        };

        output
    }

    async fn get_user_by_email(
        &self,
        email: &str,
    ) -> Result<super::types::User, ContainerError<Self::Error>> {
        let mut conn = self.get_conn().await.change_context(UserDbError::DBError)?;

        let output: Result<types::User, diesel::result::Error> = types::User::table()
            .filter(schema::users::email.eq(email))
            .get_result(&mut conn)
            .await;

        let output = match output {
            Err(err) => match err {
                diesel::result::Error::NotFound => {
                    Err(err).change_error(UserDbError::NotFoundError)
                }
                _ => Err(err).change_error(UserDbError::DBFilterError),
            },
            Ok(user) => Ok(user),
        };

        output
    }

    async fn create_user(
        &self,
        user: super::types::UserNew,
    ) -> Result<super::types::User, ContainerError<Self::Error>> {
        let mut conn = self.get_conn().await.change_context(UserDbError::DBError)?;

        let query = diesel::insert_into(types::User::table()).values(user);

        Ok(query
            .get_result(&mut conn)
            .await
            .change_error(UserDbError::DBInsertError)?)
    }

    async fn update_user(
        &self,
        user_id: &str,
        user_update: super::types::UserUpdateInternal,
    ) -> Result<super::types::User, ContainerError<Self::Error>> {
        let mut conn = self.get_conn().await.change_context(UserDbError::DBError)?;

        let query = diesel::update(types::User::table())
            .filter(schema::users::user_id.eq(user_id))
            .set(user_update);

        Ok(query
            .get_result(&mut conn)
            .await
            .change_error(UserDbError::DBUpdateError)?)
    }
}
