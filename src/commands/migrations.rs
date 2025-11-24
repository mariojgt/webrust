use std::fs;
use std::io;
use std::path::Path;
use chrono::Utc;

/// Generate a migration file with templates
pub fn make_migration(
    name: &str,
    flags: &MigrationFlags,
) -> io::Result<()> {
    let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();
    let migration_name = format!("{}_{}", timestamp, to_snake_case(name));

    let migrations_dir = Path::new("migrations");
    fs::create_dir_all(migrations_dir)?;

    let file_path = migrations_dir.join(format!("{}.sql", migration_name));

    if file_path.exists() {
        println!("⚠️  Migration file already exists: {:?}", file_path);
        return Ok(());
    }

    let contents = if let Some(table) = &flags.create {
        generate_create_table_migration(table)
    } else if let Some(table) = &flags.table {
        if flags.add {
            generate_add_columns_migration(table)
        } else {
            generate_modify_table_migration(table)
        }
    } else {
        generate_blank_migration(name)
    };

    fs::write(&file_path, contents)?;
    println!("✨ Migration created: migrations/{}.sql", migration_name);
    println!("   📝 Edit the file to add your migration logic");
    Ok(())
}

pub struct MigrationFlags {
    pub create: Option<String>,
    pub table: Option<String>,
    pub add: bool,
}

fn generate_create_table_migration(table: &str) -> String {
    format!(
        r#"-- Create table: {}
-- Generated: {}

CREATE TABLE IF NOT EXISTS `{}` (
    `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
    `created_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    `updated_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

-- Add your columns here. Examples:
-- ALTER TABLE `{}` ADD COLUMN `name` VARCHAR(255) NOT NULL;
-- ALTER TABLE `{}` ADD COLUMN `email` VARCHAR(255) UNIQUE NOT NULL;
-- ALTER TABLE `{}` ADD COLUMN `is_active` BOOLEAN DEFAULT TRUE;
-- ALTER TABLE `{}` ADD COLUMN `description` TEXT;
-- ALTER TABLE `{}` ADD INDEX `idx_email` (`email`);

-- To modify this migration, edit the SQL above.
"#,
        table,
        Utc::now().to_rfc3339(),
        table,
        table,
        table,
        table,
        table,
        table,
    )
}

fn generate_add_columns_migration(table: &str) -> String {
    format!(
        r#"-- Add columns to: {}
-- Generated: {}

-- Example: Uncomment and modify as needed

-- ALTER TABLE `{}` ADD COLUMN `column_name` VARCHAR(255) NOT NULL;
-- ALTER TABLE `{}` ADD COLUMN `another_column` INT DEFAULT 0;
-- ALTER TABLE `{}` ADD INDEX `idx_column_name` (`column_name`);

-- Column Type Reference:
-- VARCHAR(255)        - Text field (max 255 chars)
-- TEXT                - Long text field
-- INT                 - Integer
-- BIGINT              - Large integer
-- DECIMAL(10,2)       - Number with decimals
-- BOOLEAN             - True/False
-- TIMESTAMP           - Date and time
-- DATE                - Date only
-- JSON                - JSON data
-- ENUM('a','b','c')   - Choose from options
"#,
        table,
        Utc::now().to_rfc3339(),
        table,
        table,
        table,
    )
}

fn generate_modify_table_migration(table: &str) -> String {
    format!(
        r#"-- Modify table: {}
-- Generated: {}

-- Example modification commands:

-- Change column type:
-- ALTER TABLE `{}` MODIFY COLUMN `column_name` VARCHAR(500);

-- Rename column:
-- ALTER TABLE `{}` CHANGE COLUMN `old_name` `new_name` VARCHAR(255);

-- Drop column:
-- ALTER TABLE `{}` DROP COLUMN `column_name`;

-- Add index:
-- CREATE INDEX `idx_column` ON `{}` (`column_name`);

-- Drop index:
-- DROP INDEX `idx_column` ON `{}`;

-- Add unique constraint:
-- ALTER TABLE `{}` ADD UNIQUE KEY `unique_email` (`email`);

-- Add foreign key:
-- ALTER TABLE `{}` ADD CONSTRAINT `fk_user_id` FOREIGN KEY (`user_id`) REFERENCES `users` (`id`);

-- Uncomment and modify as needed
"#,
        table,
        Utc::now().to_rfc3339(),
        table,
        table,
        table,
        table,
        table,
        table,
        table,
    )
}

fn generate_blank_migration(name: &str) -> String {
    format!(
        r#"-- Migration: {}
-- Generated: {}

-- Write your SQL migration here
-- Example:
-- CREATE TABLE `example_table` (
--     `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
--     `name` VARCHAR(255) NOT NULL,
--     `created_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP
-- );

-- Tip: Always include rollback commands in comments for reference
-- ROLLBACK: DROP TABLE IF EXISTS `example_table`;
"#,
        name,
        Utc::now().to_rfc3339(),
    )
}

pub fn list_migrations() -> io::Result<()> {
    let migrations_dir = Path::new("migrations");

    if !migrations_dir.exists() {
        println!("📁 No migrations directory found. Run migrations with:");
        println!("   cargo run -- rune migrate");
        return Ok(());
    }

    println!("📜 Available Migrations:");
    println!();

    let mut entries: Vec<_> = fs::read_dir(migrations_dir)?
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| ext == "sql")
                .unwrap_or(false)
        })
        .collect();

    entries.sort_by_key(|e| e.path());

    if entries.is_empty() {
        println!("   No migrations found.");
    } else {
        for entry in entries {
            let path = entry.path();
            if let Some(file_name) = path.file_name() {
                if let Some(name) = file_name.to_str() {
                    println!("   • {}", name);
                }
            }
        }
    }

    println!();
    println!("💡 Commands:");
    println!("   cargo run -- rune migrate           - Run migrations");
    println!("   cargo run -- rune migrate:rollback  - Rollback last migration");
    println!("   cargo run -- rune make:migration    - Create new migration");

    Ok(())
}

fn to_snake_case(s: &str) -> String {
    let mut result = String::new();
    for (i, ch) in s.chars().enumerate() {
        if i > 0 && ch.is_uppercase() {
            result.push('_');
        }
        result.push(ch.to_lowercase().next().unwrap_or(ch));
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_snake_case() {
        assert_eq!(to_snake_case("CreateUsersTable"), "create_users_table");
        assert_eq!(to_snake_case("AddEmailToUsers"), "add_email_to_users");
        assert_eq!(to_snake_case("UserProfile"), "user_profile");
    }

    #[test]
    fn test_create_table_migration() {
        let migration = generate_create_table_migration("users");
        assert!(migration.contains("CREATE TABLE"));
        assert!(migration.contains("`users`"));
        assert!(migration.contains("`id`"));
    }
}
