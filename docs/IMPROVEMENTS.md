# WebRust Framework Improvements – Laravel-Inspired Enhancements

This document covers the latest improvements to the WebRust framework to make it behave more like Laravel with modern Rust patterns.

## ⚡ Quick Wins – Latest Features

These are the fastest, most impactful features we just added:

### 🔧 Tinker REPL Shell
Interactive debugging and testing shell, just like Laravel's `tinker`.

```bash
cargo run -- rune tinker
```

Inside Tinker:
- `db:tables` – List all database tables
- `db:table users` – Show table columns
- `db:count users` – Count rows
- `sql:execute <query>` – Run raw SQL
- `config:app` – View app config
- `route:list` – List all routes
- `info` – Show app info

### 📍 Route:List Command
List all application routes with methods and descriptions.

```bash
cargo run -- rune route:list
```

### 🏭 Factories Pattern
Generate test data using the Factory pattern (like Laravel Factories).

```rust
use crate::services::factory::{UserFactory, PostFactory, Factory};

let user = UserFactory::new()
    .with_email("user@example.com")
    .admin()
    .create()
    .await?;

let posts = PostFactory::new()
    .with_user_id(1)
    .create_many(5)
    .await?;
```

### 📝 Migration Templates
Better migration file generation with SQL templates.

```bash
cargo run -- rune make:migration create_posts_table --create=posts
cargo run -- rune make:migration add_email_to_users --table=users --add
cargo run -- rune migration:list
```

---

## 🎉 Phase 3 Features – Architecture Enhancements

### 🎯 Events/Listener System
Like Laravel Events, enable decoupled architecture with event-driven patterns.

#### Creating Events
```rust
use crate::prelude::*;

#[derive(Clone, Debug)]
pub struct UserCreatedEvent {
    pub user_id: i64,
    pub email: String,
    pub name: String,
}

#[async_trait]
impl Event for UserCreatedEvent {
    fn name(&self) -> &'static str {
        "user.created"
    }

    fn to_json(&self) -> Value {
        json!({
            "user_id": self.user_id,
            "email": self.email,
            "name": self.name,
        })
    }
}
```

#### Creating Listeners
```rust
use crate::prelude::*;

pub struct SendWelcomeEmailListener;

#[async_trait]
impl Listener for SendWelcomeEmailListener {
    async fn handle(&self, event: &dyn Event) -> Result<(), Box<dyn std::error::Error>> {
        if event.name() == "user.created" {
            // Send welcome email
            mail::send_welcome_email(event).await?;
        }
        Ok(())
    }
}
```

#### Using Events
```rust
// In your controller or service
let dispatcher = EventDispatcher::new();
dispatcher.listen("user.created", SendWelcomeEmailListener).await;

// When creating a user
let event = UserCreatedEvent {
    user_id: user.id,
    email: user.email.clone(),
    name: user.name.clone(),
};

dispatcher.emit(&event).await?;
```

**Benefits:**
- Decouples business logic from side effects
- Easy to add/remove listeners without changing core logic
- Clean, testable architecture
- Async event handling

---

### 👁️ Model Observers
Like Laravel Model Observers, automatically trigger code on model lifecycle events.

#### Creating an Observer
```rust
use crate::prelude::*;

pub struct UserObserver;

#[async_trait]
impl Observer for UserObserver {
    async fn created(&self, user: &Value) -> Result<(), Box<dyn std::error::Error>> {
        println!("👤 User created - sending welcome email");
        // Send welcome email
        Ok(())
    }

    async fn updated(&self, user: &Value) -> Result<(), Box<dyn std::error::Error>> {
        println!("📝 User updated - logging changes");
        // Log changes to audit table
        Ok(())
    }

    async fn deleted(&self, user: &Value) -> Result<(), Box<dyn std::error::Error>> {
        println!("🗑️  User deleted - cleaning up related data");
        // Delete user posts, comments, etc.
        Ok(())
    }
}
```

#### Using Observers in Models
```rust
use crate::prelude::*;

pub struct User {
    pub id: i64,
    pub name: String,
    pub email: String,
}

#[async_trait]
impl Observable for User {
    fn observers() -> Vec<Box<dyn Observer>> {
        vec![
            Box::new(UserObserver),
            Box::new(AuditObserver),
        ]
    }
}

// Trigger observer events
impl User {
    pub async fn create(data: UserData) -> Result<Self, Box<dyn std::error::Error>> {
        let user = User { /* ... */ };
        user.fire_created().await?; // Triggers all observers
        Ok(user)
    }

    pub async fn update(&mut self, data: UserData) -> Result<(), Box<dyn std::error::Error>> {
        self.fire_updating().await?; // Before update
        // Update logic here
        self.fire_updated().await?; // After update
        Ok(())
    }

    pub async fn delete(self) -> Result<(), Box<dyn std::error::Error>> {
        self.fire_deleting().await?;
        // Delete logic here
        self.fire_deleted().await?;
        Ok(())
    }
}
```

**Observer Events:**
- `creating()` - Before creation
- `created()` - After creation
- `updating()` - Before update
- `updated()` - After update
- `deleting()` - Before deletion
- `deleted()` - After deletion
- `saving()` - Before create or update
- `saved()` - After create or update

---

### 🔐 Authorization Policies
Like Laravel Policies, implement clean authorization patterns.

#### Creating a Policy
```rust
use crate::prelude::*;

pub struct PostPolicy;

#[async_trait]
impl Policy for PostPolicy {
    async fn view(&self, user: &Value, post: &Value) -> PolicyResult {
        // Anyone can view posts
        Ok(true)
    }

    async fn create(&self, user: &Value) -> PolicyResult {
        // Only authenticated users can create posts
        Ok(user.get("id").is_some())
    }

    async fn update(&self, user: &Value, post: &Value) -> PolicyResult {
        // Users can only update their own posts
        let user_id = user.get("id").and_then(|v| v.as_i64());
        let post_user_id = post.get("user_id").and_then(|v| v.as_i64());
        Ok(user_id == post_user_id)
    }

    async fn delete(&self, user: &Value, post: &Value) -> PolicyResult {
        // Users can delete their own, admins can delete any
        let user_id = user.get("id").and_then(|v| v.as_i64());
        let post_user_id = post.get("user_id").and_then(|v| v.as_i64());
        let is_admin = user.get("is_admin").and_then(|v| v.as_bool()).unwrap_or(false);
        Ok(user_id == post_user_id || is_admin)
    }
}
```

#### Using Policies in Controllers
```rust
use crate::prelude::*;

pub async fn update_post(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    user: User,
    Json(payload): Json<UpdatePostPayload>,
) -> impl IntoResponse {
    // Get post
    let post = Post::find(&state.db_manager, id).await.ok();

    // Check authorization
    let policy = PostPolicy;
    match Authorizer::authorize(&policy, &user.to_json(), &post.to_json(), "update").await {
        Ok(true) => {
            // Update the post
            success_message("Post updated successfully")
        }
        Ok(false) => forbidden("You are not authorized to update this post"),
        Err(_) => server_error("Authorization check failed"),
    }
}
```

**Available Methods:**
- `view()` - Can user view this resource?
- `create()` - Can user create a resource?
- `update()` - Can user update this resource?
- `delete()` - Can user delete this resource?
- `restore()` - Can user restore this resource?
- `force_delete()` - Can user permanently delete this resource?

---

## 📦 New Features Overview

### 1. Resource Controllers & RESTful Routing

WebRust now supports full RESTful resource controllers with built-in CRUD scaffolding, just like Laravel's resource controllers.

#### Generating a Resource Controller

```bash
cargo run -- rune make:resource Post
cargo run -- rune make:resource User --api
```

This creates:
- **Controller** with 7 standard methods: `index`, `create`, `store`, `show`, `edit`, `update`, `destroy`
- **Routes** for full CRUD operations
- **Templates** scaffold for all views

#### Resource Controller Structure

```rust
// src/controllers/post.rs
use crate::prelude::*;

pub async fn index(State(state): State<AppState>) -> impl IntoResponse {
    // List all posts
}

pub async fn create(State(state): State<AppState>) -> impl IntoResponse {
    // Show creation form
}

pub async fn store(State(state): State<AppState>) -> impl IntoResponse {
    created(serde_json::json!({ "id": 1, "title": "My Post" }))
}

pub async fn show(State(state): State<AppState>, Path(id): Path<i64>) -> impl IntoResponse {
    // Show single post
}

pub async fn edit(State(state): State<AppState>, Path(id): Path<i64>) -> impl IntoResponse {
    // Show edit form
}

pub async fn update(State(state): State<AppState>, Path(id): Path<i64>) -> impl IntoResponse {
    success_message("Post updated successfully")
}

pub async fn destroy(State(state): State<AppState>, Path(id): Path<i64>) -> impl IntoResponse {
    no_content()
}
```

#### Routes Registration

```rust
// src/routes/post.rs
use axum::Router;
use crate::framework::AppState;
use crate::controllers::post;

pub fn routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/posts", get(post::index).post(post::store))
        .route("/posts/create", get(post::create))
        .route("/posts/:id", get(post::show).put(post::update).delete(post::destroy))
        .route("/posts/:id/edit", get(post::edit))
        .with_state(state)
}
```

---

### 2. Enhanced HTTP Response Helpers

WebRust now provides Laravel-like response helpers for consistent API responses.

#### Available Response Functions

```rust
use crate::prelude::*;

// Success responses
success(data)                           // 200 OK with data
success_message("Message")              // 200 OK with message
created(data)                           // 201 Created
accepted(data)                          // 202 Accepted
no_content()                            // 204 No Content

// Redirect
redirect("/dashboard")                  // 302 redirect

// Error responses
error(StatusCode, "message")            // Generic error
bad_request("validation error")         // 400
unauthorized("Not authenticated")       // 401
forbidden("Not authorized")             // 403
not_found_response("Resource not found") // 404
unprocessable_entity(errors_map)        // 422 validation
too_many_requests("Rate limit exceeded") // 429
server_error("Something went wrong")    // 500

// Pagination
paginated(items, page, per_page, total)  // 200 with pagination meta
```

#### Example Usage

```rust
pub async fn store(State(state): State<AppState>, Form(data): Form<PostData>) -> Response {
    // Validate
    if let Err(errors) = data.validate() {
        return unprocessable_entity(json!(errors));
    }

    // Create
    match create_post(&state.db, data).await {
        Ok(post) => created(post),
        Err(e) => server_error(&e.to_string()),
    }
}
```

---

### 3. Repository Pattern

Abstract data access with the Repository Pattern for cleaner business logic separation.

#### Creating a Repository

```rust
use crate::services::repository::{Repository, BaseRepository};
use crate::models::Post;
use async_trait::async_trait;

pub struct PostRepository {
    base: BaseRepository<Post>,
}

impl PostRepository {
    pub fn new(pool: DbPool) -> Self {
        Self {
            base: BaseRepository::new(pool),
        }
    }
}

#[async_trait]
impl Repository<Post> for PostRepository {
    async fn all(&self) -> Result<Vec<Post>, Box<dyn std::error::Error + Send + Sync>> {
        // Implement your query logic
    }

    async fn find(&self, id: i64) -> Result<Option<Post>, Box<dyn std::error::Error + Send + Sync>> {
        // Implement your query logic
    }

    // Implement other trait methods...
}
```

#### Using Repositories in Controllers

```rust
pub async fn index(State(state): State<AppState>) -> impl IntoResponse {
    let repo = PostRepository::new(state.db.clone());

    match repo.all().await {
        Ok(posts) => success(posts),
        Err(_) => server_error("Failed to fetch posts"),
    }
}
```

---

### 4. Service Layer

Organize business logic with reusable service classes (Laravel-style).

#### Creating a Business Service

```rust
use crate::services::service_layer::{Service, BusinessService};
use async_trait::async_trait;

pub struct UserService {
    db_manager: Arc<DatabaseManager>,
}

impl Service for UserService {
    fn service_name(&self) -> &str {
        "UserService"
    }
}

#[async_trait]
impl BusinessService<User> for UserService {
    async fn get_all(&self) -> Result<Vec<User>, Box<dyn std::error::Error + Send + Sync>> {
        // Fetch all users
    }

    async fn get_by_id(&self, id: i64) -> Result<Option<User>, Box<dyn std::error::Error + Send + Sync>> {
        // Fetch user by ID
    }

    async fn create(&self, data: User) -> Result<User, Box<dyn std::error::Error + Send + Sync>> {
        // Create user
    }

    async fn update(&self, id: i64, data: User) -> Result<User, Box<dyn std::error::Error + Send + Sync>> {
        // Update user
    }

    async fn delete(&self, id: i64) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        // Delete user
    }
}
```

#### Using Services in Controllers

```rust
pub async fn index(State(state): State<AppState>) -> impl IntoResponse {
    let service = UserService::new(state.db_manager.clone());

    match service.get_all().await {
        Ok(users) => success(users),
        Err(_) => server_error("Failed to fetch users"),
    }
}
```

---

### 5. Enhanced Query Builder (Orbit ORM)

The Orbit ORM now includes many more fluent query building methods like Laravel's Eloquent.

#### New Query Builder Methods

```rust
use crate::orbit::Orbit;

// Pagination
let (users, total) = User::query()
    .paginate(&db_manager, 1, 15)
    .await?;

// Distinct results
User::query()
    .distinct()
    .get(&db_manager)
    .await?;

// OR WHERE clause
User::query()
    .where_eq("status", "active")
    .or_where("role", "=", "admin")
    .get(&db_manager)
    .await?;

// WHERE IN clause
User::query()
    .where_in("status", vec!["active", "pending"])
    .get(&db_manager)
    .await?;

// WHERE NOT IN clause
User::query()
    .where_not_in("id", vec![1, 2, 3])
    .get(&db_manager)
    .await?;

// WHERE NULL / NOT NULL
User::query()
    .where_null("deleted_at")
    .get(&db_manager)
    .await?;

User::query()
    .where_not_null("verified_at")
    .get(&db_manager)
    .await?;

// WHERE BETWEEN
User::query()
    .where_between("age", 18, 65)
    .get(&db_manager)
    .await?;

// Latest/Oldest (shortcuts for ORDER BY)
User::query()
    .latest("created_at")  // DESC
    .get(&db_manager)
    .await?;

User::query()
    .oldest("created_at")  // ASC
    .get(&db_manager)
    .await?;

// GROUP BY with HAVING
User::query()
    .group_by(&["status"])
    .having("COUNT(*)", ">", 5)
    .get(&db_manager)
    .await?;

// Debugging
User::query()
    .where_eq("status", "active")
    .dump()  // Print SQL without executing
    .get(&db_manager)
    .await?;

User::query()
    .where_eq("status", "active")
    .dd()  // Print SQL and exit
```

#### Example: Complex Queries

```rust
let active_users = User::query()
    .where_eq("status", "active")
    .where_not_null("email_verified_at")
    .latest("created_at")
    .limit(10)
    .get(&db_manager)
    .await?;

let (paginated_users, total) = User::query()
    .where_between("age", 18, 65)
    .order_by("name", "ASC")
    .paginate(&db_manager, page, 20)
    .await?;
```

---

### 6. Middleware Helpers

Simplified middleware creation with helper traits and macros.

#### Available Middleware Helpers

```rust
use crate::http::middleware_helpers::*;

// Middleware marker traits
pub struct RateLimitMiddleware { /* ... */ }
pub struct CorsMiddleware { /* ... */ }
pub struct AuthMiddleware;
pub struct GuestMiddleware;
pub struct ThrottleMiddleware { /* ... */ }

// Creating CORS middleware
let cors = CorsMiddleware::permissive();
// or
let cors = CorsMiddleware::new(
    vec!["https://example.com".to_string()],
    vec!["GET", "POST", "PUT"],
    vec!["Content-Type", "Authorization"],
);

// Rate limiting
let rate_limit = RateLimitMiddleware::new(60, 60); // 60 requests per minute

// Throttling
let throttle = ThrottleMiddleware::new(100, 1); // 100 requests per minute
```

#### Creating Custom Middleware

```rust
use axum::{middleware::Next, http::Request, body::Body, response::Response};

pub async fn custom_auth(req: Request<Body>, next: Next) -> Response {
    // Logic before request
    let response = next.run(req).await;
    // Logic after request
    response
}
```

---

### 7. Updated Prelude

The prelude has been updated to include all new response helpers for easy access.

```rust
use crate::prelude::*;

// All these are now directly available:
// - success, success_message, created, accepted, no_content
// - redirect
// - error, bad_request, unauthorized, forbidden, not_found_response, unprocessable_entity
// - too_many_requests, server_error
// - paginated
// - Auth, Flash, Storage, Mail, ValidationErrors, Http, Log
// - Orbit, Str, Arr
// - Inertia, Session
// - json!, Context, State, Html, IntoResponse, Json
```

---

## 🚀 Quick Start Guide

### 1. Generate a Complete Resource

```bash
# Create a resource controller for Posts
cargo run -- rune make:resource Post

# Create an API resource controller for Comments
cargo run -- rune make:resource Comment --api

# Create a model for your resource
cargo run -- rune make:model Post

# Create a migration for your database table
cargo run -- rune make:migration create_posts_table
```

### 2. Register Routes

Update `src/routes/mod.rs`:

```rust
mod web;
mod api;
mod post;  // Import your resource routes

pub async fn router(state: AppState) -> Router {
    let web_routes = web::web(state.clone())
        .merge(post::routes(state.clone()));  // Add here

    // ... rest of router setup
}
```

### 3. Implement Controller Logic

```rust
pub async fn index(State(state): State<AppState>) -> impl IntoResponse {
    let repo = PostRepository::new(state.db.clone().unwrap());

    match repo.all().await {
        Ok(posts) => success(posts),
        Err(_) => server_error("Failed to fetch posts"),
    }
}

pub async fn store(
    State(state): State<AppState>,
    Form(data): Form<PostData>,
) -> impl IntoResponse {
    if let Err(errors) = data.validate() {
        return unprocessable_entity(json!(errors));
    }

    created(serde_json::json!({
        "id": 1,
        "title": data.title,
        "body": data.body,
    }))
}
```

---

## 📚 Pattern Recommendations

### Clean Architecture Layers

```
Controller (HTTP Layer)
    ↓
Service (Business Logic)
    ↓
Repository (Data Access)
    ↓
Database (Models)
```

### Typical File Structure

```
src/
├── controllers/           # HTTP handlers
│   └── post.rs           # POST resource controller
├── services/
│   ├── post_service.rs   # Business logic
│   ├── repository.rs     # Repository pattern
│   └── service_layer.rs  # Service base traits
├── models/
│   └── post.rs           # Database model
├── requests/
│   └── post.rs           # Validation
├── routes/
│   ├── mod.rs
│   ├── web.rs
│   ├── api.rs
│   └── post.rs           # POST routes
└── http/
    ├── response.rs       # Response helpers
    ├── middleware_helpers.rs
    └── resource_controller.rs
```

---

## 🔄 Comparison with Laravel

| Feature | Laravel | WebRust |
|---------|---------|---------|
| Resource Controller | `artisan make:controller PostController --resource` | `rune make:resource Post` |
| Response Helpers | `response()->json($data)` | `success(data)` |
| Query Builder | `User::where()->paginate()` | `User::query().where_eq().paginate()` |
| Repositories | Manual | `Repository<T>` trait |
| Services | Manual | `BusinessService<T>` trait |
| Middleware | `Middleware` class | `async fn middleware(req, next)` |
| Validation | `FormRequest` | `#[derive(Validate)]` |

---

## ✨ Best Practices

1. **Use Repositories** for data access abstraction
2. **Create Services** for complex business logic
3. **Leverage Response Helpers** for consistent API responses
4. **Organize Routes** by resource/domain
5. **Use the Query Builder** for complex queries
6. **Validate Input** with validator traits
7. **Implement Error Handling** with proper HTTP status codes
8. **Cache Appropriately** using the Cache service

---

## 📖 Further Reading

- See `docs/ORBIT.md` for advanced query builder usage
- See `docs/BASICS.md` for controller and view creation
- See `docs/DATABASE.md` for database configuration
- See `docs/VALIDATION.md` for form validation patterns
