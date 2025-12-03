use async_trait::async_trait;
use crate::database::DatabaseManager;

#[async_trait]
pub trait Seeder: Send + Sync {
    async fn run(&self, db: &DatabaseManager) -> Result<(), Box<dyn std::error::Error>>;
}

pub struct DatabaseSeeder;

impl DatabaseSeeder {
    pub async fn run(db: &DatabaseManager) -> Result<(), Box<dyn std::error::Error>> {
        // Register your seeders here
        // self.call(UserSeeder, db).await?;
        Ok(())
    }

    pub async fn call<S: Seeder>(seeder: S, db: &DatabaseManager) -> Result<(), Box<dyn std::error::Error>> {
        println!("🌱 Seeding: {}", std::any::type_name::<S>());
        seeder.run(db).await?;
        Ok(())
    }
}
