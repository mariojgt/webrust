use super::{CacheDriver, CacheError};
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use std::time::{Instant, Duration};

struct CacheItem {
    value: String,
    expires_at: Instant,
}

#[derive(Clone)]
pub struct MemoryCache {
    store: Arc<RwLock<HashMap<String, CacheItem>>>,
}

impl MemoryCache {
    pub fn new() -> Self {
        Self {
            store: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl CacheDriver for MemoryCache {
    async fn get(&self, key: &str) -> Result<Option<String>, CacheError> {
        let store = self.store.read().await;
        if let Some(item) = store.get(key) {
            if item.expires_at > Instant::now() {
                return Ok(Some(item.value.clone()));
            }
        }
        Ok(None)
    }

    async fn put(&self, key: &str, value: &str, seconds: u64) -> Result<(), CacheError> {
        let mut store = self.store.write().await;
        store.insert(
            key.to_string(),
            CacheItem {
                value: value.to_string(),
                expires_at: Instant::now() + Duration::from_secs(seconds),
            },
        );
        Ok(())
    }

    async fn add(&self, key: &str, value: &str, seconds: u64) -> Result<bool, CacheError> {
        let mut store = self.store.write().await;
        if let Some(item) = store.get(key) {
            if item.expires_at > Instant::now() {
                return Ok(false);
            }
        }
        store.insert(
            key.to_string(),
            CacheItem {
                value: value.to_string(),
                expires_at: Instant::now() + Duration::from_secs(seconds),
            },
        );
        Ok(true)
    }

    async fn forget(&self, key: &str) -> Result<(), CacheError> {
        let mut store = self.store.write().await;
        store.remove(key);
        Ok(())
    }

    async fn flush(&self) -> Result<(), CacheError> {
        let mut store = self.store.write().await;
        store.clear();
        Ok(())
    }
}
