use std::sync::Arc;

use diesel_async::{
    AsyncPgConnection,
    pooled_connection::{
        self,
        deadpool::{Object, Pool},
    },
};
use error_stack::ResultExt;

use crate::{
    configs::Database,
    error::{self, container::ContainerError},
};

pub mod caching;
pub mod db;
pub mod schema;
pub mod transformers;
pub mod types;

/// Storage State that is to be passed though the application
#[derive(Clone)]
pub struct Storage {
    pg_pool: Arc<Pool<AsyncPgConnection>>,
}

type DeadPoolConnType = Object<AsyncPgConnection>;

impl Storage {
    /// Create a new storage interface from configuration
    pub async fn new(database: &Database) -> error_stack::Result<Self, error::StorageError> {
        let database_url = format!(
            "postgres://{}:{}@{}:{}/{}",
            database.username, database.password, database.host, database.port, database.dbname,
        );

        let config =
            pooled_connection::AsyncDieselConnectionManager::<AsyncPgConnection>::new(database_url);
        let pool = Pool::builder(config);

        let pool = match database.pool_size {
            Some(value) => pool.max_size(value),
            None => pool,
        };

        let pool = pool
            .build()
            .change_context(error::StorageError::DBPoolError)?;
        Ok(Self {
            pg_pool: Arc::new(pool),
        })
    }

    /// Get connection from database pool for accessing data
    pub async fn get_conn(&self) -> Result<DeadPoolConnType, ContainerError<error::StorageError>> {
        Ok(self
            .pg_pool
            .get()
            .await
            .change_context(error::StorageError::PoolClientFailure)?)
    }
}

/// Trait for cacheable tables
pub trait Cacheable<Table> {
    /// The key type for the cache
    type Key: std::hash::Hash + Eq + PartialEq + Send + Sync + 'static;
    /// The value type for the cache
    type Value: Clone + Send + Sync + 'static;
}

impl Cacheable<types::User> for Storage {
    type Key = String;
    type Value = types::User;
}

impl Cacheable<types::Transaction> for Storage {
    type Key = String;
    type Value = types::Transaction;
}

/// User Interface
pub trait UserInterface {
    /// Error type
    type Error;

    /// Get user by user id
    async fn get_user_by_user_id(
        &self,
        user_id: &str,
    ) -> Result<types::User, ContainerError<Self::Error>>;
    /// Get user by email
    async fn get_user_by_email(
        &self,
        email: &str,
    ) -> Result<types::User, ContainerError<Self::Error>>;
    /// Create user
    async fn create_user(
        &self,
        user: types::UserNew,
    ) -> Result<types::User, ContainerError<Self::Error>>;
    /// Update user
    async fn update_user(
        &self,
        user_id: &str,
        user: types::UserUpdateInternal,
    ) -> Result<types::User, ContainerError<Self::Error>>;
}

/// Transaction Interface
pub trait TransactionInterface {
    /// Error type
    type Error;

    /// Get transaction by id
    async fn get_transaction_by_id(
        &self,
        transaction_id: &str,
    ) -> Result<types::Transaction, ContainerError<Self::Error>>;
    /// Create transaction
    async fn create_transaction(
        &self,
        transaction: types::NewTransaction,
    ) -> Result<types::Transaction, ContainerError<Self::Error>>;
}
