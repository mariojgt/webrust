# 🪐 Orbit ORM

Orbit is WebRust's built-in ORM, designed to feel like Laravel Eloquent but with the type safety and performance of Rust.

## 1. Defining a Model

To make a struct an Orbit model, it must implement `sqlx::FromRow`, `serde::Serialize`, and the `Orbit` trait.

```rust
use crate::orbit::Orbit;
use sqlx::FromRow;
use serde::Serialize;

#[derive(Debug, Serialize, FromRow)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub email: String,
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
}
```

---

## 2. Retrieving Data

### Basic Methods

```rust
// Get all records
let users = User::all(&pool).await?;

// Find by Primary Key
let user = User::find(&pool, 1).await?;
```

### Query Builder

Orbit provides a fluent query builder for more complex queries.

```rust
let users = User::query()
    .select(&["id", "name"])      // Optional: Select specific columns
    .where_eq("active", true)     // WHERE active = ?
    .r#where("age", ">", 18)      // WHERE age > ?
    .order_by("created_at", "DESC")
    .limit(10)
    .get(&pool)
    .await?;
```

To get a single record:

```rust
let user = User::query()
    .where_eq("email", "admin@example.com")
    .first(&pool)
    .await?;
```

---

## 3. Creating Records

To create a record, pass a struct (or reference) that implements `Serialize`. This allows you to use partial structs for insertion.

```rust
#[derive(Serialize)]
struct NewUser {
    name: String,
    email: String,
}

let new_id = User::create(&pool, NewUser {
    name: "Mario".to_string(),
    email: "mario@example.com".to_string(),
}).await?;
```

---

## 4. Updating Records

You can update a record instance directly. Like creation, pass a struct with *only* the fields you want to update.

```rust
#[derive(Serialize)]
struct UpdateName {
    name: String,
}

if let Some(user) = User::find(&pool, 1).await? {
    user.update(&pool, UpdateName {
        name: "Super Mario".to_string()
    }).await?;
}
```

---

## 5. Deleting Records

```rust
if let Some(user) = User::find(&pool, 1).await? {
    user.delete(&pool).await?;
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

        // Future: Register global scopes or observers here
    }
}

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

// Usage
let posts = user.posts().get(&pool).await?;
```

### Belongs To

If a `Post` belongs to a `User`:

```rust
impl Post {
    pub async fn user(&self, pool: &MySqlPool) -> Result<Option<User>, sqlx::Error> {
        User::belongs_to(pool, self.user_id).await
    }
}

// Usage
let user = post.user(&pool).await?;
```

---

## 8. Custom Methods (Scopes / Accessors)
```

---

## 7. Custom Methods (Scopes / Accessors)

Since Rust structs are static, we don't have "dynamic attributes", but we can just add methods to the struct.

```rust
impl User {
    // Like an Eloquent Accessor: $user->full_name
    pub fn full_name(&self) -> String {
        format!("{} ({})", self.name, self.email)
    }

    // Custom Query Scope
    pub async fn active(pool: &MySqlPool) -> Result<Vec<Self>, sqlx::Error> {
        Self::query()
            .where_eq("is_active", true)
            .get(pool)
            .await
    }
}
```
