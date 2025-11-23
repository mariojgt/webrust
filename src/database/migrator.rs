use crate::database::{DatabaseManager, DbPool};
use crate::database::migrations::Migration;
use sqlx::{Row, Executor};

pub struct Migrator {
    manager: DatabaseManager,
}

impl Migrator {
    pub fn new(manager: DatabaseManager) -> Self {
        Self { manager }
    }

    pub async fn ensure_migration_table(&self) -> Result<(), sqlx::Error> {
        let pool = self.manager.default_connection().expect("No default connection");
        
        // Check if table exists
        let exists_query = "SHOW TABLES LIKE 'migrations'";
        let exists = sqlx::query(exists_query).fetch_optional(pool).await?;
        
        if exists.is_none() {
             let sql = "CREATE TABLE migrations (
                id BIGINT AUTO_INCREMENT PRIMARY KEY,
                migration VARCHAR(255) NOT NULL,
                batch INT NOT NULL
            )";
            sqlx::query(sql).execute(pool).await?;
        }
        
        Ok(())
    }

    pub async fn run(&self, migrations: Vec<Box<dyn Migration>>) -> Result<(), sqlx::Error> {
        self.ensure_migration_table().await?;
        let pool = self.manager.default_connection().unwrap();

        // Get ran migrations
        let ran_migrations: Vec<String> = sqlx::query("SELECT migration FROM migrations")
            .map(|row: sqlx::mysql::MySqlRow| row.get(0))
            .fetch_all(pool)
            .await?;

        // Get next batch number
        let last_batch: Option<i32> = sqlx::query("SELECT MAX(batch) FROM migrations")
            .map(|row: sqlx::mysql::MySqlRow| row.get(0))
            .fetch_one(pool)
            .await
            .unwrap_or(None);
        let batch = last_batch.unwrap_or(0) + 1;

        for migration in migrations {
            if !ran_migrations.contains(&migration.name().to_string()) {
                println!("Migrating: {}", migration.name());
                migration.up(&self.manager).await?;
                
                sqlx::query("INSERT INTO migrations (migration, batch) VALUES (?, ?)")
                    .bind(migration.name())
                    .bind(batch)
                    .execute(pool)
                    .await?;
                    
                println!("Migrated:  {}", migration.name());
            }
        }

        Ok(())
    }

    pub async fn rollback(&self, migrations: Vec<Box<dyn Migration>>) -> Result<(), sqlx::Error> {
        self.ensure_migration_table().await?;
        let pool = self.manager.default_connection().unwrap();

        // Get last batch
        let last_batch: Option<i32> = sqlx::query("SELECT MAX(batch) FROM migrations")
            .map(|row: sqlx::mysql::MySqlRow| row.get(0))
            .fetch_one(pool)
            .await?;

        if let Some(batch) = last_batch {
            // Get migrations in this batch
            let migrations_to_rollback: Vec<String> = sqlx::query("SELECT migration FROM migrations WHERE batch = ?")
                .bind(batch)
                .map(|row: sqlx::mysql::MySqlRow| row.get(0))
                .fetch_all(pool)
                .await?;

            // Reverse list of all migrations to find the matching objects
            // We need to run them in reverse order of definition usually, or just match by name.
            // Since we have the list of names to rollback, we find the corresponding objects.
            
            // Create a map for easy lookup
            let mut migration_map = std::collections::HashMap::new();
            for m in &migrations {
                migration_map.insert(m.name(), m);
            }

            for name in migrations_to_rollback {
                if let Some(migration) = migration_map.get(name.as_str()) {
                    println!("Rolling back: {}", name);
                    migration.down(&self.manager).await?;
                    
                    sqlx::query("DELETE FROM migrations WHERE migration = ?")
                        .bind(&name)
                        .execute(pool)
                        .await?;
                        
                    println!("Rolled back:  {}", name);
                } else {
                    println!("⚠️  Migration '{}' found in database but not in code. Skipping.", name);
                }
            }
        } else {
            println!("Nothing to rollback.");
        }

        Ok(())
    }
}
