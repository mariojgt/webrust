use super::{CacheDriver, CacheError};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::fs;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize)]
struct FileCacheItem {
    expires_at: u64,
    value: String,
}

#[derive(Clone)]
pub struct FileCache {
    directory: PathBuf,
}

impl FileCache {
    pub fn new(path: &str) -> Self {
        let directory = PathBuf::from(path);
        if !directory.exists() {
            std::fs::create_dir_all(&directory).unwrap_or_default();
        }
        Self { directory }
    }

    fn get_path(&self, key: &str) -> PathBuf {
        // Hash the key to avoid filesystem issues with special characters
        let hash = md5::compute(key);
        self.directory.join(format!("{:x}", hash))
    }

    fn now() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}

#[async_trait]
impl CacheDriver for FileCache {
    async fn get(&self, key: &str) -> Result<Option<String>, CacheError> {
        let path = self.get_path(key);
        if !path.exists() {
            return Ok(None);
        }

        let content = fs::read_to_string(&path).await.map_err(CacheError::Io)?;
        let item: FileCacheItem = serde_json::from_str(&content).map_err(CacheError::Serialization)?;

        if item.expires_at < Self::now() {
            let _ = fs::remove_file(path).await;
            return Ok(None);
        }

        Ok(Some(item.value))
    }

    async fn put(&self, key: &str, value: &str, seconds: u64) -> Result<(), CacheError> {
        let path = self.get_path(key);
        let item = FileCacheItem {
            expires_at: Self::now() + seconds,
            value: value.to_string(),
        };
        let content = serde_json::to_string(&item).map_err(CacheError::Serialization)?;
        fs::write(path, content).await.map_err(CacheError::Io)?;
        Ok(())
    }

    async fn add(&self, key: &str, value: &str, seconds: u64) -> Result<bool, CacheError> {
        if self.get(key).await?.is_some() {
            return Ok(false);
        }
        self.put(key, value, seconds).await?;
        Ok(true)
    }

    async fn forget(&self, key: &str) -> Result<(), CacheError> {
        let path = self.get_path(key);
        if path.exists() {
            fs::remove_file(path).await.map_err(CacheError::Io)?;
        }
        Ok(())
    }

    async fn flush(&self) -> Result<(), CacheError> {
        let mut entries = fs::read_dir(&self.directory).await.map_err(CacheError::Io)?;
        while let Some(entry) = entries.next_entry().await.map_err(CacheError::Io)? {
            let _ = fs::remove_file(entry.path()).await;
        }
        Ok(())
    }
}
