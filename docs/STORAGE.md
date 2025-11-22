# Storage

WebRust provides a simple file storage abstraction similar to Laravel's Storage facade.

## Configuration

By default, files are stored in `storage/app`.

## Usage

The `Storage` service is available in the prelude.

```rust
use crate::prelude::*;

// Store a file
let content = b"Hello World";
Storage::put("uploads/hello.txt", content).expect("Failed to save file");

// Check existence
if Storage::exists("uploads/hello.txt") {
    println!("File exists!");
}

// Retrieve content
let data = Storage::get("uploads/hello.txt").expect("Failed to read file");

// Delete file
Storage::delete("uploads/hello.txt").expect("Failed to delete file");
```

## Public Files

To make files accessible via the web, you should store them in `storage/app/public` and ensure your web server (or a route) serves this directory.

Currently, you may need to manually configure a static file route in `src/routes.rs` to serve files from `storage/app/public`.
