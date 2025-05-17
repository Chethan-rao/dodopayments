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
pub mod schema;
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

pub trait Cacheable<Table> {
    type Key: std::hash::Hash + Eq + PartialEq + Send + Sync + 'static;
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
