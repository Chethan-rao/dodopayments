use std::sync::Arc;

use crate::storage::types;

pub(super) type Cache<T, U> =
    moka::future::Cache<<T as super::Cacheable<U>>::Key, Arc<<T as super::Cacheable<U>>::Value>>;

#[derive(Clone)]
pub struct Caching<T>
where
    T: super::Cacheable<types::User> + super::Cacheable<types::Transaction>,
{
    inner: T,
    user_cache: Cache<T, types::User>,
    txn_cache: Cache<T, types::Transaction>,
}

impl<T> std::ops::Deref for Caching<T>
where
    T: super::Cacheable<types::User> + super::Cacheable<types::Transaction>,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

fn new_cache<T, U>(config: &crate::configs::Cache, name: &str) -> Cache<T, U>
where
    T: super::Cacheable<U>,
{
    let cache = moka::future::CacheBuilder::new(config.max_capacity).name(name);
    let cache = match config.tti {
        Some(value) => cache.time_to_idle(std::time::Duration::from_secs(value)),
        None => cache,
    };

    cache.build()
}

pub trait GetCache<T, U>
where
    T: super::Cacheable<U>,
{
    fn get_cache(&self) -> &Cache<T, U>;
}

impl<T> GetCache<T, types::User> for Caching<T>
where
    T: super::Cacheable<types::User> + super::Cacheable<types::Transaction>,
{
    fn get_cache(&self) -> &Cache<T, types::User> {
        &self.user_cache
    }
}

impl<T> GetCache<T, types::Transaction> for Caching<T>
where
    T: super::Cacheable<types::User> + super::Cacheable<types::Transaction>,
{
    fn get_cache(&self) -> &Cache<T, types::Transaction> {
        &self.txn_cache
    }
}

impl<T> Caching<T>
where
    T: super::Cacheable<types::User> + super::Cacheable<types::Transaction>,
{
    #[inline(always)]
    pub async fn lookup<U>(
        &self,
        key: <T as super::Cacheable<U>>::Key,
    ) -> Option<<T as super::Cacheable<U>>::Value>
    where
        T: super::Cacheable<U>,
        Self: GetCache<T, U>,
    {
        self.get_cache()
            .get(&key)
            .await
            .map(|value: Arc<<T as super::Cacheable<U>>::Value>| {
                let data = value.as_ref();
                data.clone()
            })
    }

    #[inline(always)]
    pub async fn cache_data<U>(
        &self,
        key: <T as super::Cacheable<U>>::Key,
        value: <T as super::Cacheable<U>>::Value,
    ) where
        T: super::Cacheable<U>,
        Self: GetCache<T, U>,
    {
        self.get_cache().insert(key, value.into()).await;
    }

    pub fn implement_cache(config: &'_ crate::configs::Cache) -> impl Fn(T) -> Self + '_ {
        move |inner: T| {
            let user_cache = new_cache::<T, types::User>(config, "user");
            let txn_cache = new_cache::<T, types::Transaction>(config, "transaction");

            Self {
                inner,
                user_cache,
                txn_cache,
            }
        }
    }
}

// pub mod user;
// pub mod transaction;
