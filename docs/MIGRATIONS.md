# 📦 Migrations

WebRust uses `sqlx` for migrations, but provides a Laravel-like CLI wrapper to make it feel familiar.

## 1. Creating a Migration

**⚠️ Important:** Because WebRust migrations are compiled Rust code, you must run this command on your **host machine**, not inside Docker.

```bash
cargo run -- rune make:migration create_posts_table
```

This will:
1.  Create a new Rust file in `src/database/migrations/`.
2.  Auto-register it in `src/database/migrations/mod.rs`.

## 2. Rebuilding

Since migrations are code, you must recompile your application to include them.

```bash
# If using Docker
make build
make up
```

## 3. Running Migrations

Once rebuilt, you can run the migrations (inside Docker or on host).

```bash
# Inside Docker
make shell
./webrust rune migrate
```

## 4. Rolling Back

To revert the last migration (executes `.down.sql` files):

```bash
cargo run -- rune migrate:rollback
```

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
