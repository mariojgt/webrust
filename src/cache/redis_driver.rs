use super::{CacheDriver, CacheError};
use async_trait::async_trait;
use redis::{aio::MultiplexedConnection, AsyncCommands, Client};
use tokio::sync::Mutex;
use std::sync::Arc;

#[derive(Clone)]
pub struct RedisCache {
    client: Client,
    connection: Arc<Mutex<MultiplexedConnection>>,
}

impl RedisCache {
    pub async fn new(connection_string: &str) -> Result<Self, CacheError> {
        let client = Client::open(connection_string)?;
        let connection = client.get_multiplexed_async_connection().await?;
        Ok(Self {
            client,
            connection: Arc::new(Mutex::new(connection)),
        })
    }
}

#[async_trait]
impl CacheDriver for RedisCache {
    async fn get(&self, key: &str) -> Result<Option<String>, CacheError> {
        let mut conn = self.connection.lock().await;
        let val: Option<String> = conn.get(key).await?;
        Ok(val)
    }

    async fn put(&self, key: &str, value: &str, seconds: u64) -> Result<(), CacheError> {
        let mut conn = self.connection.lock().await;
        let _: () = conn.set_ex(key, value, seconds).await?;
        Ok(())
    }

    async fn add(&self, key: &str, value: &str, seconds: u64) -> Result<bool, CacheError> {
        let mut conn = self.connection.lock().await;
        // SET key value NX EX seconds
        let result: Option<String> = redis::cmd("SET")
            .arg(key)
            .arg(value)
            .arg("NX")
            .arg("EX")
            .arg(seconds)
            .query_async(&mut *conn)
            .await?;
        Ok(result.is_some())
    }

    async fn forget(&self, key: &str) -> Result<(), CacheError> {
        let mut conn = self.connection.lock().await;
        let _: () = conn.del(key).await?;
        Ok(())
    }

    async fn flush(&self) -> Result<(), CacheError> {
        let mut conn = self.connection.lock().await;
        let _: () = redis::cmd("FLUSHDB").query_async(&mut *conn).await?;
        Ok(())
    }
}
