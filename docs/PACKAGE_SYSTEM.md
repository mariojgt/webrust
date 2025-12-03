# WebRust Module System

## Overview

WebRust's advanced Module System (Package System) provides a professional-grade foundation for building modular applications, comparable to Laravel's modular architecture. This system enables you to organize large applications into cohesive, reusable packages with their own controllers, models, migrations, configurations, and service providers.

**Key Features:**
- 🎯 Service Provider Pattern (register/boot lifecycle)
- 📦 Package Management with auto-discovery
- 🔗 Dependency Tracking
- 🚀 Package Lifecycle Hooks (install, enable, disable, uninstall)
- 🛣️ Automatic Route Aggregation
- 🗄️ Migration Management
- ⚙️ Package-level Configuration
- 🧪 Built for scalability and maintainability

## Quick Start

### Creating Your First Package

**Step 1: Generate the package scaffold**

```rust
use crate::services::scaffold_package;

scaffold_package("blog", Some("packages"))?;
```

This creates:
```
packages/blog/
├── src/
│   ├── controllers/      # Your HTTP handlers
│   ├── models/           # Your data models
│   ├── providers/        # Service providers
│   ├── routes/           # Package routes
│   ├── services/         # Business logic
│   └── lib.rs            # Package entry point
├── config/               # Configuration files
├── resources/views/      # Package views/templates
├── database/migrations/  # Database migrations
├── tests/                # Package tests
├── package.json          # Package manifest
└── README.md             # Documentation
```

**Step 2: Implement your package**

Edit `packages/blog/src/lib.rs`:

```rust
use async_trait::async_trait;
use crate::services::package_manager::{Package, PackageManifest, ServiceProvider};

pub struct BlogPackage;

#[async_trait]
impl Package for BlogPackage {
    fn manifest(&self) -> PackageManifest {
        PackageManifest::new("blog")
            .with_version("1.0.0")
            .with_description("Blog module for WebRust")
    }

    fn providers(&self) -> Vec<Box<dyn ServiceProvider>> {
        vec![
            Box::new(providers::BlogServiceProvider),
        ]
    }

    fn routes(&self) -> Vec<String> {
        vec![
            "src/routes/web.rs".to_string(),
        ]
    }

    async fn install(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Installing Blog package");
        // Run migrations, seed data, etc.
        Ok(())
    }

    async fn enable(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Blog package enabled");
        Ok(())
    }
}
```

**Step 3: Register in your application**

In `src/main.rs`:

```rust
use crate::services::PackageManager;
use my_app::blog::BlogPackage;

#[tokio::main]
async fn main() {
    let mut package_manager = PackageManager::new("packages");

    // Register packages
    package_manager.register(Box::new(BlogPackage))?;

    // Boot all packages (runs service providers)
    package_manager.boot().await?;

    // Get aggregated routes
    let routes = package_manager.routes();
    let migrations = package_manager.migrations();

    // Continue with app startup...
}
```

## Service Providers

Service providers are the heart of package initialization. They follow a two-phase lifecycle: **register** and **boot**.

### The Two-Phase Lifecycle

```rust
use async_trait::async_trait;
use crate::services::package_manager::ServiceProvider;
use std::collections::HashMap;

pub struct BlogServiceProvider;

#[async_trait]
impl ServiceProvider for BlogServiceProvider {
    fn name(&self) -> &str {
        "blog"
    }

    /// Phase 1: Register services
    /// Called first - register bindings, factories, services
    async fn register(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Register services into container
        // Set up bindings
        // Register event listeners
        println!("📝 Registering Blog services");
        Ok(())
    }

    /// Phase 2: Boot services
    /// Called after all registrations - boot and initialize
    async fn boot(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Boot services
        // Set up routes
        // Initialize configurations
        println!("🚀 Booting Blog services");
        Ok(())
    }

    /// Package configuration
    fn config(&self) -> HashMap<String, String> {
        let mut config = HashMap::new();
        config.insert("posts_per_page".to_string(), "10".to_string());
        config.insert("enable_comments".to_string(), "true".to_string());
        config
    }
}
```

### Why Two Phases?

**Register Phase:**
- Services don't depend on each other being initialized yet
- Safe to register interfaces and bindings
- No side effects or external calls

**Boot Phase:**
- All services have been registered
- Safe to resolve dependencies
- Can set up more complex initialization
- Can start listening to events

## Package Lifecycle Hooks

Packages support four lifecycle hooks for managing their state:

### 1. Install Hook

Called when the package is first installed:

```rust
async fn install(&self) -> Result<(), Box<dyn std::error::Error>> {
    // Run migrations
    run_migrations().await?;

    // Seed initial data
    seed_data().await?;

    // Create necessary files/directories
    fs::create_dir_all("storage/blog")?;

    println!("✅ Blog package installed");
    Ok(())
}
```

### 2. Enable Hook

Called when the package is enabled (can be multiple times):

```rust
async fn enable(&self) -> Result<(), Box<dyn std::error::Error>> {
    // Activate features
    // Register routes
    // Subscribe to events

    println!("✅ Blog package enabled");
    Ok(())
}
```

### 3. Disable Hook

Called when the package is disabled:

```rust
async fn disable(&self) -> Result<(), Box<dyn std::error::Error>> {
    // Clean up resources
    // Unregister event listeners
    // Stop background jobs

    println!("✅ Blog package disabled");
    Ok(())
}
```

### 4. Uninstall Hook

Called when the package is being removed:

```rust
async fn uninstall(&self) -> Result<(), Box<dyn std::error::Error>> {
    // Rollback migrations
    rollback_migrations().await?;

    // Remove stored data
    remove_data().await?;

    // Clean up files
    fs::remove_dir_all("storage/blog")?;

    println!("✅ Blog package uninstalled");
    Ok(())
}
```

## Package Manifest

The `package.json` file defines package metadata and configuration:

```json
{
  "name": "blog",
  "version": "1.0.0",
  "description": "Blog module for WebRust",
  "author": "Your Name",
  "providers": [
    "src/providers/BlogServiceProvider.rs"
  ],
  "routes": [
    "src/routes/web.rs"
  ],
  "migrations": [
    "database/migrations/2024_01_15_create_posts_table.sql"
  ],
  "config": {
    "posts_per_page": "10",
    "enable_comments": "true"
  },
  "dependencies": {
    "auth": "1.0.0",
    "database": "2.0.0"
  },
  "enabled": true
}
```

**Key Fields:**
- `name` - Unique package identifier
- `version` - Semantic version
- `description` - Package purpose
- `author` - Package author(s)
- `providers` - Service providers to register
- `routes` - Route files to include
- `migrations` - Database migrations
- `config` - Default configuration
- `dependencies` - Required packages
- `enabled` - Enable/disable the package

## Package Manager API

The `PackageManager` provides comprehensive package management:

### Registration

```rust
// Register a single package
manager.register(Box::new(BlogPackage))?;

// Register multiple packages at once
manager.register_many(vec![
    Box::new(BlogPackage),
    Box::new(AuthPackage),
    Box::new(AdminPackage),
])?;
```

### Booting

```rust
// Boot all packages (runs service provider lifecycle)
manager.boot().await?;
```

This:
1. Calls `register()` on all providers
2. Calls `boot()` on all providers
3. Aggregates routes, migrations, assets

### Querying Packages

```rust
// List all packages
let packages = manager.list();
for (name, manifest) in packages {
    println!("{}: v{}", name, manifest.version);
}

// Search packages
let results = manager.search("blog");

// Get package info
if let Some(info) = manager.info("blog") {
    println!("{}", info);
}

// Check if enabled
if manager.is_enabled("blog") {
    println!("Blog is enabled");
}

// Get package config
if let Some(config) = manager.config("blog") {
    println!("Posts per page: {}", config.get("posts_per_page").unwrap_or(&"10".to_string()));
}

// Get dependencies
if let Some(deps) = manager.dependencies("blog") {
    for (name, version) in deps {
        println!("{}: {}", name, version);
    }
}
```

### Lifecycle Management

```rust
// Install package
manager.install("blog").await?;

// Enable package
manager.enable("blog").await?;

// Disable package
manager.disable("blog").await?;

// Uninstall package
manager.uninstall("blog").await?;
```

### Aggregation

```rust
// Get all routes from all packages
let routes = manager.routes();

// Get all migrations from all packages
let migrations = manager.migrations();

// Get all assets from all packages
let assets = manager.assets();
```

## Real-World Example: Blog Package

Here's a complete example of a blog package:

### Structure
```
packages/blog/
├── src/
│   ├── controllers/
│   │   ├── mod.rs
│   │   └── post_controller.rs
│   ├── models/
│   │   ├── mod.rs
│   │   └── post.rs
│   ├── providers/
│   │   ├── mod.rs
│   │   └── BlogServiceProvider.rs
│   ├── routes/
│   │   ├── mod.rs
│   │   └── web.rs
│   ├── services/
│   │   ├── mod.rs
│   │   └── post_service.rs
│   └── lib.rs
├── config/
│   └── blog.json
├── database/migrations/
│   └── 2024_01_15_create_posts_table.sql
├── resources/views/
│   ├── posts/index.html
│   ├── posts/show.html
│   └── posts/create.html
├── tests/
│   └── posts_test.rs
├── package.json
└── README.md
```

### Package Library (`src/lib.rs`)

```rust
use async_trait::async_trait;
use crate::services::package_manager::{Package, PackageManifest, ServiceProvider};

pub mod controllers;
pub mod models;
pub mod providers;
pub mod routes;
pub mod services;

pub struct BlogPackage;

#[async_trait]
impl Package for BlogPackage {
    fn manifest(&self) -> PackageManifest {
        PackageManifest::new("blog")
            .with_version("1.0.0")
            .with_description("Comprehensive blog module")
            .with_provider("src/providers/BlogServiceProvider.rs")
            .with_route("src/routes/web.rs")
    }

    fn providers(&self) -> Vec<Box<dyn ServiceProvider>> {
        vec![
            Box::new(providers::BlogServiceProvider),
        ]
    }

    fn routes(&self) -> Vec<String> {
        vec![
            "src/routes/web.rs".to_string(),
        ]
    }

    fn migrations(&self) -> Vec<String> {
        vec![
            "database/migrations/2024_01_15_create_posts_table.sql".to_string(),
        ]
    }

    async fn install(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("📦 Installing Blog package...");
        // Run migrations
        // Seed data
        Ok(())
    }

    async fn enable(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("✅ Blog package enabled");
        Ok(())
    }

    async fn disable(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("⏸️ Blog package disabled");
        Ok(())
    }
}
```

### Service Provider (`src/providers/BlogServiceProvider.rs`)

```rust
use async_trait::async_trait;
use crate::services::package_manager::ServiceProvider;
use std::collections::HashMap;

pub struct BlogServiceProvider;

#[async_trait]
impl ServiceProvider for BlogServiceProvider {
    fn name(&self) -> &str {
        "blog"
    }

    async fn register(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Register services, bindings, factories
        println!("📝 Registering Blog services");
        Ok(())
    }

    async fn boot(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Boot and initialize
        println!("🚀 Booting Blog services");
        Ok(())
    }

    fn config(&self) -> HashMap<String, String> {
        let mut config = HashMap::new();
        config.insert("posts_per_page".to_string(), "10".to_string());
        config.insert("enable_comments".to_string(), "true".to_string());
        config.insert("enable_tags".to_string(), "true".to_string());
        config
    }
}
```

### Model (`src/models/post.rs`)

```rust
use sqlx::{FromRow, Row};
use serde::{Serialize, Deserialize};

#[derive(FromRow, Serialize, Deserialize, Clone, Debug)]
pub struct Post {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub author_id: i64,
    pub published: bool,
    pub created_at: String,
    pub updated_at: String,
}

impl Post {
    pub fn new(title: String, content: String, author_id: i64) -> Self {
        Self {
            id: 0,
            title,
            content,
            author_id,
            published: false,
            created_at: chrono::Local::now().to_rfc3339(),
            updated_at: chrono::Local::now().to_rfc3339(),
        }
    }
}
```

### Controller (`src/controllers/post_controller.rs`)

```rust
use axum::{extract::Path, response::IntoResponse, Json};
use crate::http::response::success;
use super::Post;

pub async fn index() -> impl IntoResponse {
    let posts = vec![];
    success(Json(posts))
}

pub async fn show(Path(id): Path<i64>) -> impl IntoResponse {
    // Fetch post
    success(Json(serde_json::json!({})))
}

pub async fn store(Json(post): Json<Post>) -> impl IntoResponse {
    // Save post
    success(Json(post))
}
```

### Routes (`src/routes/web.rs`)

```rust
use axum::{Router, routing::{get, post}, extract::State};
use crate::framework::AppState;
use crate::blog::controllers::*;

pub fn routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/api/posts", get(post_controller::index))
        .route("/api/posts/:id", get(post_controller::show))
        .route("/api/posts", post(post_controller::store))
        .with_state(state)
}
```

## Best Practices

### 1. Single Responsibility
Each package should have a clear, focused purpose. Don't create monolithic "utils" packages.

```rust
❌ Bad
📦 utils_package/  # Too vague

✅ Good
📦 blog_package/   # Clear purpose
📦 auth_package/   # Clear purpose
📦 admin_package/  # Clear purpose
```

### 2. Package Independence
Minimize coupling between packages. Use events or interfaces instead of direct dependencies.

```rust
❌ Bad
// blog package directly uses auth
use crate::auth::AuthService;

✅ Good
// blog package uses an event
pub struct UserCreatedEvent { user_id: i64 }
// auth package dispatches it
dispatcher.dispatch(UserCreatedEvent { user_id });
```

### 3. Configuration Over Code
Use configuration for package behavior instead of hardcoding.

```rust
❌ Bad
const POSTS_PER_PAGE: i64 = 10;

✅ Good
let config = manager.config("blog")?;
let posts_per_page = config
    .get("posts_per_page")
    .and_then(|s| s.parse().ok())
    .unwrap_or(10);
```

### 4. Version Your Packages
Use semantic versioning and track dependencies.

```json
{
  "name": "blog",
  "version": "2.1.0",
  "dependencies": {
    "auth": "^1.0.0",
    "database": "^2.0.0"
  }
}
```

### 5. Comprehensive Documentation
Every package should have clear README and examples.

```markdown
# Blog Package

## Installation
## Usage
## Configuration
## API Reference
## Examples
## Testing
```

### 6. Test Your Packages
Write tests for package-specific functionality.

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_provider_registration() {
        let provider = BlogServiceProvider;
        assert!(provider.register().await.is_ok());
    }
}
```

## Common Patterns

### Pattern 1: Event Broadcasting
Packages can communicate through events:

```rust
// blog package: emit event when post created
dispatcher.dispatch(PostCreatedEvent { post_id, author_id });

// notification package: listen for event
#[async_trait]
pub struct NotifyAuthorListener;

#[async_trait]
impl Listener for NotifyAuthorListener {
    async fn handle(&self, event: PostCreatedEvent) -> Result<()> {
        // Send notification
        Ok(())
    }
}
```

### Pattern 2: Service Injection
Use configuration for service injection:

```rust
pub struct BlogService {
    notification_service: Arc<NotificationService>,
}

impl BlogService {
    pub fn new(config: HashMap<String, String>) -> Self {
        Self {
            notification_service: Arc::new(
                NotificationService::new(config)
            ),
        }
    }
}
```

### Pattern 3: Package Composition
Build complex features from multiple packages:

```
Blog System
├── Blog Package (posts, categories)
├── Comment Package (comments, moderation)
├── Rating Package (likes, ratings)
├── Tag Package (tagging, taxonomy)
└── Analytics Package (views, statistics)
```

## Troubleshooting

### Package Not Loading
1. Verify package is registered with `PackageManager`
2. Check `package.json` is valid JSON
3. Ensure service providers implement the trait correctly
4. Check boot order dependencies

### Migrations Not Running
1. Ensure migrations are listed in `package.json`
2. Check file paths are correct
3. Verify migration SQL syntax
4. Check database permissions

### Routes Not Registered
1. Verify routes are included in package manifest
2. Check route handlers exist
3. Ensure `boot()` is called before using routes
4. Check route path conflicts

## Next Steps

- Create your first package using `scaffold_package()`
- Read the `PACKAGE_SYSTEM_QUICK_REF.md` for code examples
- Review example packages in `packages/examples/`
- Check out the test suite for patterns and examples

Happy packaging! 🎉
