/// Package scaffolding and file generation
/// Helpers for creating well-structured packages

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

/// Generate a new package structure
pub fn scaffold_package(name: &str, path: Option<&str>) -> std::io::Result<()> {
    let base_path = path.unwrap_or("packages");
    let package_dir = PathBuf::from(base_path).join(name);

    // Create directory structure
    let directories = vec![
        package_dir.join("src"),
        package_dir.join("src/controllers"),
        package_dir.join("src/models"),
        package_dir.join("src/providers"),
        package_dir.join("src/routes"),
        package_dir.join("src/services"),
        package_dir.join("config"),
        package_dir.join("resources/views"),
        package_dir.join("database/migrations"),
        package_dir.join("tests"),
    ];

    for dir in directories {
        fs::create_dir_all(&dir)?;
    }

    // Create package.json manifest
    create_package_manifest(&package_dir, name)?;

    // Create lib.rs
    create_lib_rs(&package_dir, name)?;

    // Create mod.rs files
    create_mod_files(&package_dir)?;

    // Create service provider template
    create_service_provider(&package_dir, name)?;

    // Create routes template
    create_routes_template(&package_dir, name)?;

    // Create config template
    create_config_template(&package_dir, name)?;

    // Create README
    create_readme(&package_dir, name)?;

    // Create .gitkeep files
    create_gitkeep(&package_dir)?;

    println!("✅ Package scaffold created: {}", package_dir.display());
    println!("\n📦 Package structure:");
    println!("{}/", name);
    println!("├── src/");
    println!("│   ├── controllers/      # Package controllers");
    println!("│   ├── models/           # Package models");
    println!("│   ├── providers/        # Service providers");
    println!("│   ├── routes/           # Package routes");
    println!("│   ├── services/         # Business logic");
    println!("│   └── lib.rs            # Package entry point");
    println!("├── config/               # Configuration files");
    println!("├── resources/views/      # Package views");
    println!("├── database/migrations/  # Package migrations");
    println!("├── tests/                # Package tests");
    println!("├── package.json          # Package manifest");
    println!("└── README.md             # Documentation");

    Ok(())
}

fn create_package_manifest(package_dir: &Path, name: &str) -> std::io::Result<()> {
    let manifest = format!(
        r#"{{
  "name": "{}",
  "version": "0.1.0",
  "description": "A WebRust package",
  "author": "Your Name",
  "providers": [
    "src/providers/{}ServiceProvider.rs"
  ],
  "routes": [
    "src/routes/web.rs"
  ],
  "migrations": [],
  "config": {{}},
  "dependencies": {{}},
  "enabled": true
}}
"#,
        name, to_pascal_case(name)
    );

    fs::write(package_dir.join("package.json"), manifest)?;
    Ok(())
}

fn create_lib_rs(package_dir: &Path, name: &str) -> std::io::Result<()> {
    let pascal_name = to_pascal_case(name);
    let lib_content = format!(
        r#"use async_trait::async_trait;
use crate::services::package_manager::{{Package, PackageManifest, ServiceProvider}};
use std::collections::HashMap;

pub mod controllers;
pub mod models;
pub mod providers;
pub mod routes;
pub mod services;

/// {0} Package
pub struct {0}Package;

#[async_trait]
impl Package for {0}Package {{
    fn manifest(&self) -> PackageManifest {{
        PackageManifest::new("{1}")
            .with_version("0.1.0")
            .with_description("A WebRust modular package")
    }}

    fn providers(&self) -> Vec<Box<dyn ServiceProvider>> {{
        vec![
            // Box::new(providers::{0}ServiceProvider),
        ]
    }}

    fn routes(&self) -> Vec<String> {{
        vec![
            // "src/routes/web.rs".to_string(),
        ]
    }}

    async fn install(&self) -> Result<(), Box<dyn std::error::Error>> {{
        println!("Installing {1} package...", self.manifest().name);
        // Run migrations, create tables, etc.
        Ok(())
    }}

    async fn enable(&self) -> Result<(), Box<dyn std::error::Error>> {{
        println!("Enabling {1} package...", self.manifest().name);
        Ok(())
    }}

    async fn disable(&self) -> Result<(), Box<dyn std::error::Error>> {{
        println!("Disabling {1} package...", self.manifest().name);
        Ok(())
    }}
}}
"#,
        pascal_name, name
    );

    fs::write(package_dir.join("src/lib.rs"), lib_content)?;
    Ok(())
}

fn create_mod_files(package_dir: &Path) -> std::io::Result<()> {
    let modules = vec![
        ("src", "pub mod controllers;\npub mod models;\npub mod providers;\npub mod routes;\npub mod services;"),
        ("src/controllers", ""),
        ("src/models", ""),
        ("src/providers", ""),
        ("src/routes", ""),
        ("src/services", ""),
    ];

    for (dir, content) in modules {
        let mod_path = package_dir.join(dir).join("mod.rs");
        if !mod_path.exists() || content.is_empty() {
            fs::write(&mod_path, content)?;
        }
    }

    Ok(())
}

fn create_service_provider(package_dir: &Path, name: &str) -> std::io::Result<()> {
    let pascal_name = to_pascal_case(name);
    let provider_content = format!(
        r#"use async_trait::async_trait;
use crate::services::package_manager::ServiceProvider;
use std::collections::HashMap;

/// Service Provider for {0} package
pub struct {1}ServiceProvider;

#[async_trait]
impl ServiceProvider for {1}ServiceProvider {{
    fn name(&self) -> &str {{
        "{0}"
    }}

    async fn register(&self) -> Result<(), Box<dyn std::error::Error>> {{
        // Register services, bindings, etc.
        println!("Registering {{}} services...", self.name());
        Ok(())
    }}

    async fn boot(&self) -> Result<(), Box<dyn std::error::Error>> {{
        // Boot services after all registrations
        println!("Booting {{}} services...", self.name());
        Ok(())
    }}

    fn config(&self) -> HashMap<String, String> {{
        let mut config = HashMap::new();
        // config.insert("key".to_string(), "value".to_string());
        config
    }}
}}
"#,
        name, pascal_name
    );

    let provider_path = package_dir
        .join("src/providers")
        .join(format!("{}ServiceProvider.rs", pascal_name));
    fs::write(provider_path, provider_content)?;

    Ok(())
}

fn create_routes_template(package_dir: &Path, _name: &str) -> std::io::Result<()> {
    let routes_content = r#"use axum::Router;
use crate::framework::AppState;

/// Package routes
pub fn routes(_state: AppState) -> Router<AppState> {
    Router::new()
    // Add your routes here
}
"#;

    fs::write(
        package_dir.join("src/routes/web.rs"),
        routes_content,
    )?;

    Ok(())
}

fn create_config_template(package_dir: &Path, name: &str) -> std::io::Result<()> {
    let config_content = format!(
        r#"{{
  "package": "{}",
  "debug": true,
  "features": {{
  }}
}}
"#,
        name
    );

    fs::write(package_dir.join("config/package.json"), config_content)?;
    Ok(())
}

fn create_readme(package_dir: &Path, name: &str) -> std::io::Result<()> {
    let pascal_name = to_pascal_case(name);
    let snake_name = to_snake_case(&pascal_name);
    let readme = format!(
        r#"# {} Package

A modular WebRust package for {} functionality.

## Features

- Clean modular architecture
- Service provider pattern
- Organized controllers and models
- Database migrations support

## Installation

1. Update your `src/main.rs` to register this package:

```rust
use {}::{{}};

let mut package_manager = PackageManager::new("packages");
package_manager.register(Box::new({}Package))?;
```

2. Run migrations:

```bash
cargo run -- migrate
```

## Usage

### Routes

Package routes are automatically registered. Access them via `/api/{}/*`

### Models

Use models from this package:

```rust
use {}::models::*;
```

### Services

Use package services:

```rust
use {}::services::*;
```

## Structure

```
{}
├── src/
│   ├── controllers/    # HTTP handlers
│   ├── models/         # Data models
│   ├── providers/      # Service providers
│   ├── routes/         # Route definitions
│   ├── services/       # Business logic
│   └── lib.rs
├── config/             # Configuration
├── resources/          # Views
├── database/           # Migrations
└── tests/
```

## Configuration

Configure this package in `config/{}.json`

## Testing

```bash
cargo test --package {}
```

## License

MIT
"#,
        pascal_name, name, pascal_name, pascal_name, snake_name, pascal_name, pascal_name, name, snake_name, name
    );

    fs::write(package_dir.join("README.md"), readme)?;
    Ok(())
}

fn create_gitkeep(package_dir: &Path) -> std::io::Result<()> {
    let dirs = vec![
        package_dir.join("src/controllers"),
        package_dir.join("src/models"),
        package_dir.join("src/services"),
        package_dir.join("resources/views"),
        package_dir.join("database/migrations"),
        package_dir.join("tests"),
    ];

    for dir in dirs {
        fs::write(dir.join(".gitkeep"), "")?;
    }

    Ok(())
}

/// Convert string to PascalCase
fn to_pascal_case(s: &str) -> String {
    s.split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect()
}

/// Convert string to snake_case
fn to_snake_case(s: &str) -> String {
    s.chars()
        .map(|c| if c.is_uppercase() { format!("_{}", c.to_lowercase()) } else { c.to_string() })
        .collect::<String>()
        .trim_start_matches('_')
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_pascal_case() {
        assert_eq!(to_pascal_case("hello_world"), "HelloWorld");
        assert_eq!(to_pascal_case("blog"), "Blog");
        assert_eq!(to_pascal_case("admin_panel"), "AdminPanel");
    }
}
