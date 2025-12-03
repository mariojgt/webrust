# 🚨 Error Logging & Debugging - Quick Reference

## Quick Start

```rust
use crate::prelude::*;

// Beautiful error page (like Laravel Ignition)
let error = ErrorContext::new("My Error", "Something went wrong")
    .with_solution("Try restarting the server")
    .at("src/main.rs", 42);
error.into_response()  // Returns HTML page

// File-based error logging
let logger = ErrorLogger::new("storage/logs/errors.log");
logger.log_error("Title", "Message", file!(), line!(), None);

// Enhanced debugging
dd!(value);           // Dump and die with formatting
dump!(value);         // Dump and continue (chainable)
debug!("label", value);  // Labeled debug
debug_if!(condition, "label", value);  // Conditional

// Performance
timer!("operation", { /* code */ });
benchmark!("test", 1000, { /* code */ });

// Logging
log_success!("Done");
log_error!("Failed");
log_warning!("Warning");
log_info!("Info");

// Assertions
assert_debug!(condition, "message");
```

---

## Debugging Macros

| Macro | Purpose | Output | Example |
|-------|---------|--------|---------|
| `dd!()` | Stop & inspect | Formatted dump | `dd!(user)` |
| `dump!()` | Log & continue | Formatted dump | `let u = dump!(fetch())` |
| `debug!()` | Named breakpoint | Labeled section | `debug!("user", user)` |
| `debug_if!()` | Conditional log | Only if true | `debug_if!(admin, "admin", user)` |
| `timer!()` | Measure time | `⏱️ [label] took Xms` | `timer!("query", { query() })` |
| `benchmark!()` | Benchmark code | `📊 iterations: time` | `benchmark!("hash", 100, { hash() })` |
| `log_success!()` | Success log | `✅ message` | `log_success!("Done")` |
| `log_error!()` | Error log | `❌ message` | `log_error!("Failed")` |
| `log_warning!()` | Warning log | `⚠️ message` | `log_warning!("Warn")` |
| `log_info!()` | Info log | `ℹ️ message` | `log_info!("Info")` |
| `assert_debug!()` | Assert w/ msg | Error & panic | `assert_debug!(x == 5, "x is {}", x)` |
| `memory_snapshot!()` | Memory snapshot | Comment | `memory_snapshot!("before")` |

---

## Error Logger API

```rust
let logger = ErrorLogger::new("storage/logs/app.log");

// Log with context
logger.log_error(
    "Title",
    "Message",
    file!(),
    line!(),
    Some(json!({"key": "value"}))
);

// Log warning
logger.log_warning("Message", None);

// Log debug (debug mode only)
logger.log_debug("Label", "Data");

// Get recent errors
let errors = logger.get_recent_errors(10);

// Configure size
let logger = ErrorLogger::new("path")
    .with_max_size(50 * 1024 * 1024);  // 50MB
```

---

## ErrorContext API

```rust
let error = ErrorContext::new("Error Title", "Error message")
    .with_solution("How to fix it")
    .at("src/file.rs", 42)
    .with_frame("function", "file.rs", 42)
    .with_context("key", "value");

// Into response (HTML page)
error.into_response()

// To HTML string
let html = error.to_html();
```

---

## Error Page Output

```
╔════════════════════════════════════════════════════════════╗
║              💥 Database Connection Failed                 ║
║     MySQL server is not accessible at 127.0.0.1:3306      ║
╚════════════════════════════════════════════════════════════╝

Status: 500 Internal Server Error
File: src/framework.rs | Line: 142 | DEBUG MODE

📋 Error Message
─────────────────────────────────────────────────────────
Could not connect to MySQL at 127.0.0.1:3306

📍 Stack Trace
─────────────────────────────────────────────────────────
① connect (src/database.rs:150)
② init_pool (src/framework.rs:142)
③ main (src/main.rs:85)

🔍 Context
─────────────────────────────────────────────────────────
database: webrust_app
host: 127.0.0.1:3306
port: 3306

💡 Solution
─────────────────────────────────────────────────────────
How to fix this:
✓ Ensure MySQL server is running
✓ Check DATABASE_URL in .env
✓ Verify network connectivity to database host
```

---

## Real-World Examples

### Controller with Error Handling
```rust
#[post("/users")]
pub async fn store(
    State(state): State<AppState>,
    Form(data): Form<CreateUserRequest>
) -> impl IntoResponse {
    let logger = ErrorLogger::new("storage/logs/users.log");

    match User::create(&state.db_manager, data).await {
        Ok(user) => {
            log_success!("User created");
            created(user)
        }
        Err(e) => {
            logger.log_error(
                "User Creation Failed",
                &e.to_string(),
                file!(),
                line!(),
                Some(json!({"email": &data.email}))
            );
            log_error!("Failed to create user");
            server_error("Could not create user")
        }
    }
}
```

### Service with Timing
```rust
pub async fn batch_process(items: Vec<Item>) -> Result<Vec<Processed>> {
    let results = timer!("batch_process", {
        let mut results = Vec::new();
        for item in items {
            debug_if!(item.priority > 5, "high_priority", item);
            results.push(process_item(item).await?);
        }
        results
    });

    log_success!("Batch processing complete");
    Ok(results)
}
```

### Database Query Profiling
```rust
let users = timer!("fetch_active_users", {
    User::query()
        .where_eq("status", "active")
        .where_gt("created_at", yesterday)
        .latest("updated_at")
        .limit(100)
        .get(&state.db_manager)
        .await?
});

debug!("query_results", format!("Found {} users", users.len()));
```

### Performance Benchmarking
```rust
benchmark!("password_hash", 1000, {
    hash::make("secretpassword123").ok();
});

benchmark!("json_parse", 1000, {
    serde_json::from_str::<User>(&json_string).ok();
});

benchmark!("uuid_generation", 1000, {
    uuid::Uuid::new_v4();
});
```

---

## Integration with Existing Code

All debugging macros are **opt-in** and don't affect existing code:

```rust
// Old code still works
let user = get_user(1);
println!("{:?}", user);

// Just add macros where needed
let user = dump!(get_user(1));  // Same, but logged
```

---

## Files

- 📄 `docs/ERROR_LOGGING.md` - Full documentation
- 🔧 `src/http/ignition.rs` - Error pages (~563 lines)
- 🔧 `src/services/error_logger.rs` - Logging service (~100 lines)
- ✨ `src/debug.rs` - Enhanced macros (~190 lines)

---

## Testing

Enable error logging in tests:

```rust
#[test]
fn test_user_creation() {
    let logger = ErrorLogger::new("storage/logs/tests.log");

    timer!("test_user_creation", {
        // Test code
    });

    log_success!("Test passed");
}
```

---

## Troubleshooting

| Issue | Solution |
|-------|----------|
| Macros not found | Add `use crate::prelude::*;` |
| dd!() panicking | That's expected - it's `dump and die` |
| No log output | Check file path and permissions |
| Logs not rotating | Check `with_max_size()` setting |
| Error page not showing | Use `error.into_response()` or integrate with middleware |

---

## Best Practices

✅ Use `dd!()` when you need to stop and inspect
✅ Use `dump!()` in chains for logging intermediate values
✅ Use `timer!()` for performance-critical sections
✅ Use `log_*!()` for status messages
✅ Use `debug_if!()` for conditional breakpoints
✅ Use `ErrorLogger` for persistent error tracking
✅ Use `ErrorContext` for custom error pages

---

See `docs/ERROR_LOGGING.md` for complete documentation.
