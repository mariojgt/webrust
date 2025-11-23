# Custom Console Commands

WebRust includes a powerful console component inspired by Laravel Artisan, allowing you to build custom CLI commands for your application. These commands are useful for cron jobs, administrative tasks, data migrations, or any background processing.

## Generating a Command

To create a new command, use the `make:command` utility:

```bash
cargo run -- rune make:command SendEmails
```

This will generate a new file at `src/commands/send_emails.rs`.

## Anatomy of a Command

A command is a Rust struct that implements the `Command` trait. Here is what a generated command looks like:

```rust
use async_trait::async_trait;
use crate::services::console::Command;

pub struct SendEmails;

#[async_trait]
impl Command for SendEmails {
    /// The signature of the command (how you call it from the CLI)
    fn name(&self) -> &str {
        "email:send"
    }

    /// A brief description shown in help menus
    fn description(&self) -> &str {
        "Send queued emails to users"
    }

    /// The logic to execute
    async fn handle(&self, args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
        println!("Sending emails...");
        
        // You can access arguments passed to the command
        if let Some(user_id) = args.get(0) {
            println!("Sending to user ID: {}", user_id);
        }

        Ok(())
    }
}
```

## Registering Commands

When you run `make:command`, WebRust **automatically registers** your new command in `src/commands/mod.rs`.

It adds the module declaration:
```rust
pub mod send_emails;
```

And injects the registration into the `kernel` function:
```rust
commands.insert("email:send".to_string(), Box::new(send_emails::SendEmails));
```

If you need to manually register a command (e.g. if you created the file manually), you can edit `src/commands/mod.rs` yourself.

## Running Commands

Once registered, you can run your command using the `rune` CLI:

```bash
# Run the command
cargo run -- rune email:send

# Pass arguments
cargo run -- rune email:send 123
```

## Accessing Application Services

Since commands are just Rust code, you can import and use any part of your application, such as models, database pools, or services.

```rust
use crate::framework;
use crate::models::user::User;

async fn handle(&self, _args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    // Establish a DB connection if needed
    let pool = framework::build_pool().await?;
    
    let users = User::all(&pool).await?;
    println!("Found {} users.", users.len());
    
    Ok(())
}
```

## Running Commands in Docker

When running your application in Docker, you don't have access to `cargo`. Instead, you execute the compiled binary directly.

### 1. Enter the Container

First, access the shell of your running container:

```bash
# Using the Makefile helper
make shell

# OR using docker-compose directly
docker-compose -f docker/docker-compose.yml exec app /bin/bash
```

### 2. Execute Commands

Inside the container, use the `./webrust` binary followed by `rune` and your command:

```bash
# Run migrations
./webrust rune migrate

# Run the queue worker
./webrust rune queue:work

# Run scheduled tasks
./webrust rune schedule:run
```

### ⚠️ Important Note on Scaffolding

Commands that generate code (like `make:controller`, `make:model`, etc.) should **NOT** be run inside the Docker container.

*   **Why?** The Docker container runs a compiled binary and does not mount your source code (`src/`) for editing. Any files generated inside the container would be lost when the container stops.
*   **Solution:** Run scaffolding commands on your **host machine** using `cargo run -- rune make:...`, then rebuild your container if necessary.
