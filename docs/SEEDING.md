# Database Seeding

WebRust includes a simple method of seeding your database with test data using seed classes. All seed classes are stored in the `src/database/seeders` directory.

## Writing Seeders

To generate a seeder, execute the `make:seeder` command:

```bash
cargo run -- rune make:seeder UserSeeder
```

This will create a new file in `src/database/seeders/user_seeder.rs`.

A seeder class contains a `run` method. Within this method, you may insert data into your database however you wish. You may use the query builder to manually insert data or you may use model factories.

```rust
use async_trait::async_trait;
use crate::database::DatabaseManager;
use crate::database::seeder::Seeder;
use crate::services::factory::UserFactory;

pub struct UserSeeder;

#[async_trait]
impl Seeder for UserSeeder {
    async fn run(&self, db: &DatabaseManager) -> Result<(), Box<dyn std::error::Error>> {
        // Create 10 users using the factory
        UserFactory::new().create_many(10).await?;
        Ok(())
    }
}
```

## Calling Additional Seeders

Within the `DatabaseSeeder` class, you may use the `call` method to execute additional seed classes. This allows you to break up your database seeding into multiple files so that no single seeder class becomes too large.

```rust
// src/database/seeders/database_seeder.rs

use async_trait::async_trait;
use crate::database::DatabaseManager;
use crate::database::seeder::{Seeder, DatabaseSeeder as SeederRunner};
use crate::database::seeders::user_seeder::UserSeeder;

pub struct DatabaseSeeder;

#[async_trait]
impl Seeder for DatabaseSeeder {
    async fn run(&self, db: &DatabaseManager) -> Result<(), Box<dyn std::error::Error>> {
        SeederRunner::call(UserSeeder, db).await?;
        Ok(())
    }
}
```

## Running Seeders

You may execute the `db:seed` command to seed your database. By default, the `db:seed` command runs the `DatabaseSeeder` class, which may in turn invoke other seed classes.

```bash
cargo run -- rune db:seed
```
