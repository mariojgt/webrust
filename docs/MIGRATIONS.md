# 📦 Migrations

WebRust uses a custom SQL-based migration system that is designed to be simple, fast, and familiar to Laravel developers.

Unlike previous versions, migrations are now **runtime SQL files**, meaning you do **not** need to recompile your application to run new migrations.

## 1. Creating a Migration

You can create a new migration using the CLI. This works both on your host machine and inside Docker.

```bash
# On host
cargo run -- rune make:migration create_posts_table

# Inside Docker
./webrust rune make:migration create_posts_table
```

This will create a new `.sql` file in the `migrations/` directory with a timestamp prefix, for example: `migrations/20240520120000_create_posts_table.sql`.

## 2. Migration File Format

Migration files are plain SQL files that contain both the "up" (apply) and "down" (revert) logic, separated by a special comment.

```sql
-- Migration: create_posts_table
-- --- UP ---
CREATE TABLE posts (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    content TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

-- --- DOWN ---
DROP TABLE IF EXISTS posts;
```

The `Migrator` parses this file at runtime, splitting it by the `-- --- DOWN ---` marker.

## 3. Running Migrations

To apply pending migrations:

```bash
# On host
cargo run -- rune migrate

# Inside Docker
./webrust rune migrate
```

This will:
1.  Create the `migrations` table if it doesn't exist.
2.  Read all `.sql` files from the `migrations/` directory.
3.  Check which ones have already been run.
4.  Execute the `UP` section of any new migrations.

## 4. Rolling Back

To revert the last batch of migrations:

```bash
# On host
cargo run -- rune migrate:rollback

# Inside Docker
./webrust rune migrate:rollback
```

This will execute the `DOWN` section of the migrations in the last batch and remove them from the `migrations` table.

## 5. Docker Workflow

Because the `migrations/` directory is mounted as a volume in Docker, you can create migration files on your host machine, and they will be immediately available inside the container.

1.  **Create:** `cargo run -- rune make:migration add_users` (on host)
2.  **Edit:** Open the new SQL file in your editor and write your SQL.
3.  **Run:** `make shell` -> `./webrust rune migrate` (inside container)

No `docker-compose build` or restart is required!
