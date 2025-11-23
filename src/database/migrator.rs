use crate::database::{DatabaseManager, DbPool};
use sqlx::{Row, Executor};
use std::path::Path;
use std::fs;

pub struct MigrationFile {
    pub name: String,
    pub up_sql: String,
    pub down_sql: String,
}

pub struct Migrator {
    manager: DatabaseManager,
}

impl Migrator {
    pub fn new(manager: DatabaseManager) -> Self {
        Self { manager }
    }

    pub async fn ensure_migration_table(&self) -> Result<(), sqlx::Error> {
        let pool = self.manager.default_connection().expect("No default connection");

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

    fn load_migrations(&self, path: &Path) -> Vec<MigrationFile> {
        let mut migrations = Vec::new();

        if let Ok(entries) = fs::read_dir(path) {
            let mut paths: Vec<_> = entries
                .filter_map(|e| e.ok())
                .map(|e| e.path())
                .filter(|p| p.extension().map_or(false, |ext| ext == "sql"))
                .collect();

            // Sort by filename to ensure order
            paths.sort();

            for path in paths {
                if let Ok(content) = fs::read_to_string(&path) {
                    let file_name = path.file_stem().unwrap().to_string_lossy().to_string();
                    let parts: Vec<&str> = content.split("-- --- DOWN ---").collect();

                    let up_sql = parts.get(0).unwrap_or(&"").replace("-- --- UP ---", "").trim().to_string();
                    let down_sql = parts.get(1).unwrap_or(&"").trim().to_string();

                    migrations.push(MigrationFile {
                        name: file_name,
                        up_sql,
                        down_sql,
                    });
                }
            }
        }
        migrations
    }

    pub async fn run(&self, migrations_path: &Path) -> Result<(), sqlx::Error> {
        self.ensure_migration_table().await?;
        let pool = self.manager.default_connection().unwrap();
        let migrations = self.load_migrations(migrations_path);

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
            if !ran_migrations.contains(&migration.name) {
                println!("Migrating: {}", migration.name);

                if !migration.up_sql.is_empty() {
                    pool.execute(migration.up_sql.as_str()).await?;
                }

                sqlx::query("INSERT INTO migrations (migration, batch) VALUES (?, ?)")
                    .bind(&migration.name)
                    .bind(batch)
                    .execute(pool)
                    .await?;

                println!("Migrated:  {}", migration.name);
            }
        }

        Ok(())
    }

    pub async fn rollback(&self, migrations_path: &Path) -> Result<(), sqlx::Error> {
        self.ensure_migration_table().await?;
        let pool = self.manager.default_connection().unwrap();
        let migrations = self.load_migrations(migrations_path);

        // Get last batch
        let last_batch: Option<i32> = sqlx::query("SELECT MAX(batch) FROM migrations")
            .map(|row: sqlx::mysql::MySqlRow| row.get(0))
            .fetch_one(pool)
            .await?;

        if let Some(batch) = last_batch {
            let migrations_to_rollback: Vec<String> = sqlx::query("SELECT migration FROM migrations WHERE batch = ?")
                .bind(batch)
                .map(|row: sqlx::mysql::MySqlRow| row.get(0))
                .fetch_all(pool)
                .await?;

            // Create a map for easy lookup
            let mut migration_map = std::collections::HashMap::new();
            for m in &migrations {
                migration_map.insert(m.name.clone(), m);
            }

            // Rollback in reverse order of execution (usually)
            for name in migrations_to_rollback.iter().rev() {
                if let Some(migration) = migration_map.get(name) {
                    println!("Rolling back: {}", name);

                    if !migration.down_sql.is_empty() {
                        pool.execute(migration.down_sql.as_str()).await?;
                    }

                    sqlx::query("DELETE FROM migrations WHERE migration = ?")
                        .bind(name)
                        .execute(pool)
                        .await?;

                    println!("Rolled back:  {}", name);
                } else {
                    println!("⚠️  Migration '{}' found in database but file is missing. Skipping.", name);
                }
            }
        } else {
            println!("Nothing to rollback.");
        }

        Ok(())
    }
}
