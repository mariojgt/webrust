# Logging

WebRust provides a robust logging system powered by `tracing`, with a Laravel-inspired facade for ease of use.

## Configuration

Logging configuration is handled in your `.env` file.

```dotenv
# Options: stack, single, stdout
LOG_CHANNEL=stack

# Log Level: debug, info, warn, error, trace
LOG_LEVEL=debug

# Log Directory (default: storage/logs)
LOG_DIR=storage/logs

# Log File (default: webrust.log)
LOG_FILE=webrust.log
```

### Channels

- **stack**: Logs to both the console (stdout) and the daily log file.
- **single**: Logs only to the daily log file.
- **stdout**: Logs only to the console.

## Usage

You can use the `Log` facade to write logs. It is available globally via the prelude.

```rust
use crate::prelude::*;

// Simple logging
Log::info("User logged in", None::<&String>);
Log::error("Something went wrong", None::<&String>);
Log::warning("Disk space low", None::<&String>);
Log::debug("Debugging variable", None::<&String>);

// Contextual logging
let context = serde_json::json!({
    "user_id": 1,
    "ip": "127.0.0.1"
});

Log::info("User login attempt", Some(&context));
```

## Contextual Information

You can pass any serializable struct or JSON value as context.

```rust
#[derive(serde::Serialize)]
struct UserContext {
    id: i32,
    role: String,
}

let ctx = UserContext { id: 1, role: "admin".to_string() };

Log::info("Admin action", Some(&ctx));
```

## Under the Hood

WebRust uses `tracing` and `tracing-appender` for high-performance, non-blocking logging.
- **Daily Rotation**: Log files are automatically rotated daily (e.g., `webrust.log.2023-10-27`).
- **Non-Blocking**: File writing happens in a separate thread to avoid blocking your application.
