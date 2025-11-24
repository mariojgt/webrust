/// Service Layer utilities for business logic organization
/// Provides base traits and helpers for implementing business services

use std::sync::Arc;
use crate::database::DatabaseManager;
use crate::cache::Cache;

/// Base service trait for dependency injection
pub trait Service: Send + Sync {
    fn service_name(&self) -> &str;
}

/// Application services container (Laravel-style service provider)
pub struct Services {
    pub db_manager: Arc<DatabaseManager>,
    pub cache: Arc<Cache>,
}

impl Services {
    pub fn new(db_manager: DatabaseManager, cache: Cache) -> Self {
        Self {
            db_manager: Arc::new(db_manager),
            cache: Arc::new(cache),
        }
    }
}

/// Example business service (e.g., UserService)
pub trait BusinessService<T>: Service {
    /// Get all items
    async fn get_all(&self) -> Result<Vec<T>, Box<dyn std::error::Error + Send + Sync>>;

    /// Get item by ID
    async fn get_by_id(&self, id: i64) -> Result<Option<T>, Box<dyn std::error::Error + Send + Sync>>;

    /// Create item
    async fn create(&self, data: T) -> Result<T, Box<dyn std::error::Error + Send + Sync>>;

    /// Update item
    async fn update(&self, id: i64, data: T) -> Result<T, Box<dyn std::error::Error + Send + Sync>>;

    /// Delete item
    async fn delete(&self, id: i64) -> Result<bool, Box<dyn std::error::Error + Send + Sync>>;
}

/// Query service for complex queries
pub trait QueryService<T>: Service {
    /// Run a complex query
    async fn query(&self) -> Result<Vec<T>, Box<dyn std::error::Error + Send + Sync>>;
}

/// Action/Command service for business actions
pub trait ActionService: Service {
    /// Execute an action
    async fn execute(&self) -> Result<String, Box<dyn std::error::Error + Send + Sync>>;
}
