# Package Development in WebRust

WebRust supports a modular package system similar to Laravel's package development. This allows you to create reusable functionality that can be shared across projects or simply to organize your large application into smaller, manageable modules.

## Creating a Package

To create a new package, use the `make:package` command via the CLI:

```bash
cargo run -- rune make:package my-awesome-package
```

This will scaffold a new Rust crate in the `packages/my-awesome-package` directory with the following structure:

```
packages/my-awesome-package/
├── Cargo.toml
└── src/
    └── lib.rs
```

## Registering Your Package

After creating a package, you need to tell the main application about it. This involves two steps:

### 1. Update `Cargo.toml` (Root)

You need to add the new package to your workspace members and as a dependency.

Open the root `Cargo.toml` and add:

```toml
[workspace]
members = [
    ".",
    "packages/my-awesome-package", # Add this line
]

[dependencies]
# ... other dependencies
my-awesome-package = { path = "packages/my-awesome-package" } # Add this line
```

### 2. Register in `src/main.rs`

Open `src/main.rs` and register the package in the `main` function. This allows the package to register its own routes.

```rust
// Import your package
use my_awesome_package::MyAwesomePackage;

// ... inside main() ...

// Register the package
let app_state = AppState {
    // ...
};

// When defining routes, you can now include package routes if the package exposes them
// Currently, packages implement the WebRustPackage trait.
```

## The `WebRustPackage` Trait

Your package's `lib.rs` will automatically implement the `WebRustPackage` trait. This trait allows the package to define its own routes.

```rust
// packages/my-awesome-package/src/lib.rs

use axum::Router;
use webrust::framework::WebRustPackage; // Assuming webrust is available as a dependency

pub struct MyAwesomePackage;

impl WebRustPackage for MyAwesomePackage {
    fn register_routes(&self) -> Router {
        Router::new().route("/my-package", axum::routing::get(|| async { "Hello from My Package!" }))
    }
}
```

## Workflow

1.  **Scaffold**: Run `cargo run -- rune make:package <name>`.
2.  **Link**: Add to `[workspace]` and `[dependencies]` in root `Cargo.toml`.
3.  **Develop**: Write your logic in `packages/<name>/src/lib.rs`.
4.  **Register**: Use the package in your main application logic.
