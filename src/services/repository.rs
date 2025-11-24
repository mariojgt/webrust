/// Repository Pattern for abstracted data access (Laravel-inspired)
/// Provides a clean separation between business logic and data access

use crate::database::DbPool;
use async_trait::async_trait;
use serde::Serialize;
use std::fmt::Debug;

/// Generic Repository trait for CRUD operations
#[async_trait]
pub trait Repository<T: Send + Sync + Debug + Serialize> {
    /// Get all records
    async fn all(&self) -> Result<Vec<T>, Box<dyn std::error::Error + Send + Sync>>;

    /// Get a record by ID
    async fn find(&self, id: i64) -> Result<Option<T>, Box<dyn std::error::Error + Send + Sync>>;

    /// Get records with pagination
    async fn paginate(&self, page: i64, per_page: i64) -> Result<(Vec<T>, i64), Box<dyn std::error::Error + Send + Sync>>;

    /// Create a new record
    async fn create(&self, data: T) -> Result<T, Box<dyn std::error::Error + Send + Sync>>;

    /// Update a record
    async fn update(&self, id: i64, data: T) -> Result<T, Box<dyn std::error::Error + Send + Sync>>;

    /// Delete a record
    async fn delete(&self, id: i64) -> Result<bool, Box<dyn std::error::Error + Send + Sync>>;

    /// Count total records
    async fn count(&self) -> Result<i64, Box<dyn std::error::Error + Send + Sync>>;
}

/// Base repository implementation with common utilities
pub struct BaseRepository<T> {
    pool: DbPool,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: Send + Sync + Debug + Serialize> BaseRepository<T> {
    pub fn new(pool: DbPool) -> Self {
        Self {
            pool,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn pool(&self) -> &DbPool {
        &self.pool
    }
}
