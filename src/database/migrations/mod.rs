use async_trait::async_trait;
use crate::database::DatabaseManager;

#[async_trait]
pub trait Migration: Sync + Send {
    fn name(&self) -> &str;
    async fn up(&self, manager: &DatabaseManager) -> Result<(), sqlx::Error>;
    async fn down(&self, manager: &DatabaseManager) -> Result<(), sqlx::Error>;
}

pub fn get_migrations() -> Vec<Box<dyn Migration>> {
    let mut migrations: Vec<Box<dyn Migration>> = Vec::new();
    
    // Auto-registered migrations will be added here
    
    migrations
}
