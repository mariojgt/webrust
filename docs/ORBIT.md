# 🪐 Orbit ORM

Orbit is WebRust's built-in ORM, designed to feel like Laravel Eloquent but with the type safety and performance of Rust.

## 1. Defining a Model

To make a struct an Orbit model, it must implement `sqlx::FromRow`, `serde::Serialize`, and the `Orbit` trait.

```rust
use crate::orbit::Orbit;
use sqlx::FromRow;
use serde::Serialize;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, FromRow)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    #[serde(skip)]
    pub password_hash: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl Orbit for User {
    // Required: The table name in the database
    fn table_name() -> &'static str {
        "users"
    }

    // Required: How to get the ID from an instance
    fn id(&self) -> i64 {
        self.id
    }

    // Optional: Override if your primary key is not "id"
    // fn primary_key() -> &'static str { "user_id" }

    // Optional: Specify a specific database connection (defaults to default)
    // fn connection() -> Option<&'static str> { Some("sqlite") }
}
```

---

## 2. Retrieving Data

Orbit methods generally take a reference to `DatabaseManager` (available in `state.db_manager`), which automatically handles connection selection.

### Basic Methods

```rust
// Get all records
let users = User::all(&state.db_manager).await?;

// Find by Primary Key
let user = User::find(&state.db_manager, 1).await?;
```

### Query Builder

Orbit provides a fluent query builder for more complex queries. Note that the builder's final execution methods (`get`, `first`) require a `&DbPool`, not the manager.

```rust
// Get the pool (either default or specific)
let pool = state.db_manager.default_connection().unwrap();

let users = User::query()
    .select(&["id", "username"])  // Optional: Select specific columns
    .where_eq("active", true)     // WHERE active = ?
    .r#where("age", ">", 18)      // WHERE age > ?
    .order_by("created_at", "DESC")
    .limit(10)
    .get(pool)
    .await?;
```

To get a single record:

```rust
let user = User::query()
    .where_eq("email", "admin@example.com")
    .first(pool)
    .await?;
```

---

## 3. Creating Records

To create a record, pass a struct (or reference) that implements `Serialize`. This allows you to use partial structs for insertion.

```rust
#[derive(Serialize)]
struct NewUser {
    username: String,
    email: String,
    password_hash: String,
}

let new_id = User::create(&state.db_manager, NewUser {
    username: "Mario".to_string(),
    email: "mario@example.com".to_string(),
    password_hash: "hashed_secret".to_string(),
}).await?;
```

---

## 4. Updating Records

You can update a record instance directly. Like creation, pass a struct with *only* the fields you want to update.

```rust
#[derive(Serialize)]
struct UpdateName {
    username: String,
}

if let Some(user) = User::find(&state.db_manager, 1).await? {
    user.update(&state.db_manager, UpdateName {
        username: "Super Mario".to_string()
    }).await?;
}
```

---

## 5. Deleting Records

```rust
if let Some(user) = User::find(&state.db_manager, 1).await? {
    user.delete(&state.db_manager).await?;
}
```

---

## 6. Lifecycle Hooks (Boot)

You can override the `boot` method to hook into model events. This is useful for setting global scopes or default behavior.

```rust
impl Orbit for User {
    fn table_name() -> &'static str { "users" }
    fn id(&self) -> i64 { self.id }

    fn boot() {
        // Example: Log when the model is booted
        println!("User model booted!");
    }
}
```

---

## 7. Relationships

Orbit provides `has_many` and `belongs_to` helpers.

### Has Many

If a `User` has many `Post`s:

```rust
impl User {
    pub fn posts(&self) -> builder::Builder<Post> {
        self.has_many("user_id")
    }
}

// Usage (requires pool)
let pool = state.db_manager.default_connection().unwrap();
let posts = user.posts().get(pool).await?;
```

### Belongs To

If a `Post` belongs to a `User`:

```rust
use crate::database::DatabaseManager;

impl Post {
    pub async fn user(&self, manager: &DatabaseManager) -> Result<Option<User>, sqlx::Error> {
        User::belongs_to(manager, self.user_id).await
    }
}

// Usage
let user = post.user(&state.db_manager).await?;
```

---

## 8. Custom Methods (Scopes / Accessors)

Since Rust structs are static, we don't have "dynamic attributes", but we can just add methods to the struct.

```rust
use crate::database::DbPool;

impl User {
    // Like an Eloquent Accessor: $user->full_name
    pub fn display_name(&self) -> String {
        format!("{} <{}>", self.username, self.email)
    }

    // Custom Query Scope
    pub async fn active(pool: &DbPool) -> Result<Vec<Self>, sqlx::Error> {
        Self::query()
            .where_eq("is_active", true)
            .get(pool)
            .await
    }
}
```

---

## 9. Database Abstraction

WebRust is designed to be database-agnostic (mostly). By default, it is configured for **MySQL**, but you can switch the underlying driver by changing the type aliases in `src/database.rs`.

### Switching Databases

WebRust uses Cargo features to select the database driver. By default, it uses **MySQL**.

To switch to **PostgreSQL**:

1.  Open `Cargo.toml`.
2.  Change the default feature:

```toml
[features]
default = ["postgres"]
# default = ["mysql"]
```

3.  Update your `.env` file with the PostgreSQL connection string.

```dotenv
DATABASE_URL=postgres://user:password@localhost:5432/webrust_app
```

4.  Recompile your project.

```bash
cargo run
```

The framework will automatically recompile with the PostgreSQL driver and types.
