# make:package Command

## Overview

The `make:package` command is a CLI tool that generates a complete, production-ready package scaffold for WebRust. It creates all the necessary directories, files, and templates to start building modular packages immediately.

---

## Usage

### Basic Syntax

```bash
cargo run -- make:package <name> [--path=DIRECTORY]
```

### Examples

**Create a blog package in the default `packages/` directory:**
```bash
cargo run -- make:package blog
```

**Create an admin package in a custom path:**
```bash
cargo run -- make:package admin --path=src/modules
```

**Create multiple packages:**
```bash
cargo run -- make:package blog
cargo run -- make:package comments
cargo run -- make:package tags
```

---

## Arguments

### `<name>` (Required)
The name of the package to create.

**Requirements:**
- Must be lowercase
- Can contain letters, numbers, underscores, and hyphens
- Cannot start with a number
- Maximum 64 characters
- Examples: `blog`, `admin_panel`, `user-management`

### `--path=DIRECTORY` (Optional)
The directory where the package will be created.

**Default:** `packages/`

**Examples:**
- `--path=packages` (same as default)
- `--path=src/modules`
- `--path=./app/packages`

---

## What Gets Created

When you run `make:package blog`, the following structure is generated:

```
packages/blog/
├── src/
│   ├── controllers/           # HTTP request handlers
│   │   └── .gitkeep
│   ├── models/                # Data models
│   │   └── .gitkeep
│   ├── providers/             # Service providers
│   │   ├── mod.rs
│   │   └── BlogServiceProvider.rs
│   ├── routes/                # API routes
│   │   ├── mod.rs
│   │   └── web.rs
│   ├── services/              # Business logic
│   │   └── .gitkeep
│   ├── mod.rs
│   └── lib.rs                 # Package entry point
├── config/
│   └── blog.json              # Configuration file
├── resources/
│   └── views/                 # Templates
│       └── .gitkeep
├── database/
│   └── migrations/            # SQL migrations
│       └── .gitkeep
├── tests/                      # Package tests
│   └── .gitkeep
├── package.json               # Package manifest
└── README.md                  # Documentation
```

---

## Generated Files

### `src/lib.rs` - Package Entry Point
The main package file that implements the `Package` trait.

```rust
use async_trait::async_trait;
use crate::services::package_manager::{Package, PackageManifest, ServiceProvider};

pub struct BlogPackage;

#[async_trait]
impl Package for BlogPackage {
    fn manifest(&self) -> PackageManifest {
        PackageManifest::new("blog")
            .with_version("0.1.0")
            .with_description("A WebRust modular package")
    }

    fn providers(&self) -> Vec<Box<dyn ServiceProvider>> {
        vec![]
    }

    // Implement lifecycle hooks...
}
```

### `src/providers/{Name}ServiceProvider.rs` - Service Provider
Initializes services with register/boot lifecycle.

```rust
use async_trait::async_trait;
use crate::services::package_manager::ServiceProvider;

pub struct BlogServiceProvider;

#[async_trait]
impl ServiceProvider for BlogServiceProvider {
    fn name(&self) -> &str {
        "blog"
    }

    async fn register(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("📝 Registering blog services");
        Ok(())
    }

    async fn boot(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🚀 Booting blog services");
        Ok(())
    }
}
```

### `src/routes/web.rs` - Routes
Define your package's API endpoints.

```rust
use axum::{Router, routing::get, extract::State};
use crate::framework::AppState;

pub fn routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/api/posts", get(/* handler */))
        .with_state(state)
}
```

### `src/controllers/` - Controllers
Create HTTP request handlers here.

### `src/models/` - Models
Define your data structures and models.

### `src/services/` - Services
Implement business logic.

### `config/{name}.json` - Configuration
Package-specific configuration.

```json
{
  "package": "blog",
  "debug": true,
  "features": {}
}
```

### `package.json` - Package Manifest
Metadata and configuration for the package.

```json
{
  "name": "blog",
  "version": "0.1.0",
  "description": "A WebRust package",
  "author": "Your Name",
  "providers": ["src/providers/BlogServiceProvider.rs"],
  "routes": ["src/routes/web.rs"],
  "migrations": [],
  "config": {},
  "dependencies": {},
  "enabled": true
}
```

### `README.md` - Documentation
Template with installation and usage instructions.

---

## Step-by-Step Guide

### 1. Create the Package

```bash
cargo run -- make:package blog
```

You'll see:
```
🚀 Creating package: blog
📁 Path: packages/blog

✅ Package 'blog' created successfully!
```

### 2. Implement Your Package

Edit `packages/blog/src/lib.rs`:
- Update the manifest (name, version, description, author)
- Add service providers
- Implement package methods

### 3. Create Your Service Provider

Edit `packages/blog/src/providers/BlogServiceProvider.rs`:
- Implement `register()` to bind services
- Implement `boot()` to initialize services
- Add configuration with `config()` method

### 4. Create Controllers

Add files to `packages/blog/src/controllers/`:
```rust
use axum::{response::IntoResponse, Json};

pub async fn list_posts() -> impl IntoResponse {
    Json(vec![])
}

pub async fn show_post(id: u64) -> impl IntoResponse {
    Json(serde_json::json!({ "id": id }))
}
```

### 5. Create Models

Add files to `packages/blog/src/models/`:
```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Post {
    pub id: u64,
    pub title: String,
    pub content: String,
}
```

### 6. Create Routes

Edit `packages/blog/src/routes/web.rs`:
```rust
use axum::{Router, routing::get};
use crate::blog::controllers::*;
use crate::framework::AppState;

pub fn routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/api/posts", get(list_posts))
        .route("/api/posts/:id", get(show_post))
        .with_state(state)
}
```

### 7. Register in main.rs

Add to `src/main.rs`:
```rust
use crate::services::PackageManager;
use my_app::blog::BlogPackage;

#[tokio::main]
async fn main() {
    let mut manager = PackageManager::new("packages");
    manager.register(Box::new(BlogPackage))?;
    manager.boot().await?;

    // Continue with app startup...
}
```

### 8. Document Your Package

Edit `packages/blog/README.md`:
- Add description
- Installation instructions
- Usage examples
- API documentation

---

## Command Output

After running the command, you'll see detailed next steps:

```
📋 Next steps:

1. Implement your package:
   Edit: packages/blog/src/lib.rs
   • Update package manifest
   • Implement the Package trait
   • Add service providers

2. Create service provider:
   Edit: packages/blog/src/providers/BlogServiceProvider.rs
   • Implement register() method
   • Implement boot() method
   • Add configuration

3. Create controllers:
   Add files to: packages/blog/src/controllers/
   • Create your HTTP handlers

...and more

📚 For more help:
   • Read: docs/PACKAGE_SYSTEM.md
   • Quick ref: docs/PACKAGE_SYSTEM_QUICK_REF.md
```

---

## Validation

The command validates the package name before creation:

### Valid Names ✅
- `blog`
- `admin_panel`
- `user_management`
- `auth2`
- `my-package`

### Invalid Names ❌
- `Blog` (uppercase)
- `2blog` (starts with number)
- `blog package` (contains space)
- `blog@package` (special characters)
- Empty string

If you use an invalid name, you'll see:
```
Invalid package name: 'BlogPackage'. Package names must be
lowercase alphanumeric with underscores.
```

---

## Common Use Cases

### Create a Blog System with Multiple Packages

```bash
# Create individual packages
cargo run -- make:package blog
cargo run -- make:package comments
cargo run -- make:package tags
cargo run -- make:package ratings

# Then register all in main.rs
let mut manager = PackageManager::new("packages");
manager.register(Box::new(BlogPackage))?;
manager.register(Box::new(CommentsPackage))?;
manager.register(Box::new(TagsPackage))?;
manager.register(Box::new(RatingsPackage))?;
manager.boot().await?;
```

### Create in a Custom Directory

```bash
cargo run -- make:package payment --path=src/modules
```

This creates: `src/modules/payment/`

### Create Multiple Packages for E-Commerce

```bash
cargo run -- make:package products
cargo run -- make:package orders
cargo run -- make:package payments
cargo run -- make:package shipping
cargo run -- make:package notifications
cargo run -- make:package analytics
```

---

## Troubleshooting

### Package Directory Already Exists

If the package directory already exists, you'll see an error. Remove it first:

```bash
rm -rf packages/blog
cargo run -- make:package blog
```

### Permission Denied

If you get permission errors, ensure the `packages/` directory exists and is writable:

```bash
mkdir -p packages
chmod 755 packages
cargo run -- make:package blog
```

### Invalid Package Name Error

Make sure your package name:
- Is lowercase
- Doesn't start with a number
- Doesn't contain spaces or special characters

```bash
❌ cargo run -- make:package MyBlog     # Uppercase
❌ cargo run -- make:package 2blog      # Starts with number
❌ cargo run -- make:package my blog    # Space

✅ cargo run -- make:package my_blog
```

---

## Integration with Other Commands

The `make:package` command integrates with:
- **Package Manager** - Package registration and lifecycle
- **Package Scaffold** - File and structure generation
- **Service Providers** - Registration system

---

## File Naming Convention

The command automatically converts package names:
- Package: `blog` → Class name: `Blog`
- Package: `user_panel` → Class name: `UserPanel`
- Package: `admin_management` → Class name: `AdminManagement`

This is used in:
- Service provider file names
- Class/struct names
- Import statements

---

## Next Steps After Creation

1. ✅ Run the command to create package scaffold
2. ⭕ Edit `src/lib.rs` to implement Package trait
3. ⭕ Create service provider with register/boot methods
4. ⭕ Add controllers, models, and services
5. ⭕ Define routes in `src/routes/web.rs`
6. ⭕ Register package in `src/main.rs`
7. ⭕ Document in `README.md`
8. ⭕ Test the package

---

## Related Documentation

- **Package System Guide:** `docs/PACKAGE_SYSTEM.md`
- **Quick Reference:** `docs/PACKAGE_SYSTEM_QUICK_REF.md`
- **Scaffolding:** `src/services/package_scaffold.rs`
- **Package Manager:** `src/services/package_manager.rs`

---

## Example: Complete Blog Package Setup

```bash
# 1. Create the package
cargo run -- make:package blog

# 2. The command creates all files and shows next steps

# 3. Edit src/lib.rs (implement Package trait)

# 4. Edit src/providers/BlogServiceProvider.rs

# 5. Create controllers in src/controllers/

# 6. Create models in src/models/

# 7. Edit src/routes/web.rs with your routes

# 8. Register in src/main.rs

# 9. Test your package
cargo run
```

That's it! Your modular blog package is ready to use. 🚀
