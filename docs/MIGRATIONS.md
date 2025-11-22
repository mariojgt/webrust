# 📦 Migrations

WebRust uses `sqlx` for migrations, but provides a Laravel-like CLI wrapper to make it feel familiar.

## 1. Creating a Migration

To create a new migration file, use the `make:migration` command:

```bash
cargo run -- rune make:migration create_posts_table
```

This will create a file in `migrations/` with a timestamp, e.g.:
`migrations/20231122120000_create_posts_table.sql`

## 2. Writing Migrations

Open the generated `.sql` file and write your SQL.

```sql
-- migrations/20231122120000_create_posts_table.sql

CREATE TABLE posts (
    id BIGINT PRIMARY KEY AUTO_INCREMENT,
    title VARCHAR(255) NOT NULL,
    content TEXT NOT NULL,
    user_id BIGINT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);
```

## 3. Running Migrations

To apply pending migrations:

```bash
cargo run -- rune migrate
```

This runs `sqlx migrate run` under the hood.

## 4. Rolling Back

To revert the last batch of migrations:

```bash
cargo run -- rune migrate:rollback
```

This runs `sqlx migrate revert`.

---

## 💡 Schema Builder (Experimental)

If you need to generate SQL dynamically in your Rust code (e.g. for a plugin system or setup script), you can use the `Orbit::Schema` builder.

```rust
use crate::orbit::schema::Schema;

let sql = Schema::create("posts", |table| {
    table.id();
    table.string("title");
    table.text("content");
    table.timestamps();
});

println!("{}", sql);
```

This will output the standard MySQL `CREATE TABLE` statement.
