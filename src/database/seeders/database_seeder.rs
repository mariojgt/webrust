use async_trait::async_trait;
use crate::database::DatabaseManager;
use crate::database::seeder::Seeder;

pub struct DatabaseSeeder;

#[async_trait]
impl Seeder for DatabaseSeeder {
    async fn run(&self, db: &DatabaseManager) -> Result<(), Box<dyn std::error::Error>> {
        // Call other seeders here
        // crate::database::seeder::DatabaseSeeder::call(UserSeeder, db).await?;
        Ok(())
    }
}
