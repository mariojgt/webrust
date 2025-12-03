use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};
use thiserror::Error;

pub mod redis_driver;
pub mod file_driver;
pub mod memory_driver;

pub use redis_driver::RedisCache;
pub use file_driver::FileCache;
pub use memory_driver::MemoryCache;

#[derive(Error, Debug)]
pub enum CacheError {
    #[error("Redis error: {0}")]
    Redis(#[from] redis::RedisError),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("Other error: {0}")]
    Other(String),
}

#[async_trait]
pub trait CacheDriver: Send + Sync {
    async fn get(&self, key: &str) -> Result<Option<String>, CacheError>;
    async fn put(&self, key: &str, value: &str, seconds: u64) -> Result<(), CacheError>;
    async fn add(&self, key: &str, value: &str, seconds: u64) -> Result<bool, CacheError>;
    async fn forget(&self, key: &str) -> Result<(), CacheError>;
    async fn flush(&self) -> Result<(), CacheError>;
}

#[derive(Clone)]
pub enum Cache {
    Redis(RedisCache),
    File(FileCache),
    Memory(MemoryCache),
}

impl Cache {
    pub async fn get(&self, key: &str) -> Result<Option<String>, CacheError> {
        match self {
            Cache::Redis(c) => c.get(key).await,
            Cache::File(c) => c.get(key).await,
            Cache::Memory(c) => c.get(key).await,
        }
    }

    pub async fn put(&self, key: &str, value: &str, seconds: u64) -> Result<(), CacheError> {
        match self {
            Cache::Redis(c) => c.put(key, value, seconds).await,
            Cache::File(c) => c.put(key, value, seconds).await,
            Cache::Memory(c) => c.put(key, value, seconds).await,
        }
    }

    pub async fn add(&self, key: &str, value: &str, seconds: u64) -> Result<bool, CacheError> {
        match self {
            Cache::Redis(c) => c.add(key, value, seconds).await,
            Cache::File(c) => c.add(key, value, seconds).await,
            Cache::Memory(c) => c.add(key, value, seconds).await,
        }
    }

    pub async fn has(&self, key: &str) -> Result<bool, CacheError> {
        match self.get(key).await? {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }

    pub async fn forget(&self, key: &str) -> Result<(), CacheError> {
        match self {
            Cache::Redis(c) => c.forget(key).await,
            Cache::File(c) => c.forget(key).await,
            Cache::Memory(c) => c.forget(key).await,
        }
    }

    pub async fn flush(&self) -> Result<(), CacheError> {
        match self {
            Cache::Redis(c) => c.flush().await,
            Cache::File(c) => c.flush().await,
            Cache::Memory(c) => c.flush().await,
        }
    }

    // --- Helper methods with Serde ---

    pub async fn get_json<T: DeserializeOwned>(&self, key: &str) -> Result<Option<T>, CacheError> {
        match self.get(key).await? {
            Some(val) => {
                let parsed: T = serde_json::from_str(&val)?;
                Ok(Some(parsed))
            }
            None => Ok(None),
        }
    }

    pub async fn put_json<T: Serialize + Send + Sync>(&self, key: &str, value: &T, seconds: u64) -> Result<(), CacheError> {
        let serialized = serde_json::to_string(value)?;
        self.put(key, &serialized, seconds).await
    }

    pub async fn remember<T, F, Fut>(&self, key: &str, seconds: u64, callback: F) -> Result<T, CacheError>
    where
        T: Serialize + DeserializeOwned + Send + Sync + Clone,
        F: FnOnce() -> Fut + Send,
        Fut: std::future::Future<Output = T> + Send,
    {
        if let Some(val) = self.get_json::<T>(key).await? {
            return Ok(val);
        }

        let value = callback().await;
        self.put_json(key, &value, seconds).await?;
        Ok(value)
    }
}
