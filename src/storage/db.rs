use diesel::{ExpressionMethods, QueryDsl};
use diesel_async::RunQueryDsl;
use error_stack::ResultExt;

use crate::{
    error::TransactionDbError,
    error::{
        UserDbError,
        container::{ContainerError, ResultContainerExt},
    },
    storage::schema::transactions::dsl::transactions,
    storage::{
        Storage, TransactionInterface, UserInterface,
        types::{NewTransaction, Transaction, User},
    },
};
use diesel::dsl::exists;
use diesel::{Connection, result::Error, select};
use time::PrimitiveDateTime;

/// Implementation of the UserInterface for the Storage struct.
impl UserInterface for Storage {
    type Error = UserDbError;

    /// Retrieves a user by their user ID.
    async fn get_user_by_user_id(
        &self,
        _user_id: &str,
    ) -> Result<super::types::User, ContainerError<Self::Error>> {
        use crate::storage::schema::users::dsl::*;
        use diesel::result::Error;

        let mut conn = self.get_conn().await.change_error(UserDbError::DBError)?;

        let output: Result<super::types::User, diesel::result::Error> = users
            .filter(user_id.eq(_user_id))
            .get_result(&mut conn)
            .await;

        let output = match output {
            Err(err) => match err {
                Error::NotFound => Err(err).change_error(UserDbError::NotFoundError),
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
        use crate::storage::schema::users::dsl::*;
        use diesel::result::Error;

        let mut conn = self.get_conn().await.change_error(UserDbError::DBError)?;

        let output: Result<super::types::User, diesel::result::Error> =
            users.filter(email.eq(_email)).get_result(&mut conn).await;

        let output = match output {
            Err(err) => match err {
                Error::NotFound => Err(err).change_error(UserDbError::NotFoundError),
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

        let mut conn = self.get_conn().await.change_error(UserDbError::DBError)?;

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

        let mut conn = self.get_conn().await.change_error(UserDbError::DBError)?;

        let query = diesel::update(users)
            .filter(user_id.eq(_user_id))
            .set(user_update);

        Ok(query
            .get_result(&mut conn)
            .await
            .change_error(UserDbError::DBUpdateError)?)
    }
}

/// Implementation of the TransactionInterface for the Storage struct.
impl TransactionInterface for Storage {
    type Error = TransactionDbError;

    /// Retrieves a transaction by its ID.
    async fn get_transaction_by_id(
        &self,
        _transaction_id: &str,
    ) -> Result<super::types::Transaction, ContainerError<Self::Error>> {
        use crate::storage::schema::transactions::dsl::*;
        use diesel::result::Error;

        let mut conn = self
            .get_conn()
            .await
            .change_error(TransactionDbError::DBError)?;

        let output: Result<Transaction, diesel::result::Error> = transactions
            .filter(transaction_id.eq(_transaction_id))
            .get_result(&mut conn)
            .await;

        let output = match output {
            Err(err) => match err {
                Error::NotFound => Err(err).change_error(TransactionDbError::NotFoundError),
                _ => Err(err).change_error(TransactionDbError::DBFilterError),
            },
            Ok(transaction) => Ok(transaction),
        };

        output
    }

    /// Creates a new transaction in the database.
    async fn create_transaction(
        &self,
        transaction: super::types::NewTransaction,
    ) -> Result<super::types::Transaction, ContainerError<Self::Error>> {
        use crate::error::container::ResultContainerExt;
        use crate::storage::schema::transactions::dsl::*;
        use crate::storage::schema::users::dsl::*;
        use diesel::dsl::exists;
        use diesel::{Connection, result::Error, select};

        let mut conn = self
            .get_conn()
            .await
            .change_error(TransactionDbError::DBError)?;

        let result = conn
            .build_transaction()
            .run::<_, Error, _>(|mut conn| {
                Box::pin(async move {
                    // Check if sender and receiver exists
                    let sender_exists = select(exists(
                        users.filter(user_id.eq(transaction.sender_id.clone())),
                    ))
                    .get_result::<bool>(conn)
                    .await?;

                    let receiver_exists = select(exists(
                        users.filter(user_id.eq(transaction.recipient_id.clone())),
                    ))
                    .get_result::<bool>(conn)
                    .await?;

                    if !sender_exists || !receiver_exists {
                        return Err(Error::NotFound);
                    }

                    // Check if sender has enough balance
                    use diesel::dsl::sum;
                    use diesel::select;

                    let sender_balance = users
                        .filter(user_id.eq(transaction.sender_id.clone()))
                        .select(balance_in_rs)
                        .first::<f64>(conn)
                        .await?;

                    if sender_balance < transaction.amount_in_rs {
                        return Err(Error::NotFound);
                    }

                    // Debit sender
                    diesel::update(users)
                        .filter(user_id.eq(transaction.sender_id.clone()))
                        .set(balance_in_rs.eq(balance_in_rs - transaction.amount_in_rs))
                        .execute(conn)
                        .await?;

                    // Credit receiver
                    diesel::update(users)
                        .filter(user_id.eq(transaction.recipient_id.clone()))
                        .set(balance_in_rs.eq(balance_in_rs + transaction.amount_in_rs))
                        .execute(conn)
                        .await?;

                    // Create transaction
                    let query = diesel::insert_into(transactions).values(transaction);

                    let inserted_transaction: Transaction = query.get_result(&mut conn).await?;
                    Ok(inserted_transaction)
                })
            })
            .await;

        match result {
            Ok(transaction) => {
                // Retrieve the created transaction
                self.get_transaction_by_id(&transaction.transaction_id)
                    .await
            }
            Err(err) => Err(err).change_error(TransactionDbError::DBInsertError)?,
        }
    }
}
