# 🚨 Enhanced Error Logging & Debugging System

WebRust now includes a **beautiful error display system** inspired by Laravel's Ignition, plus enhanced debugging macros for better development experience.

## Features

### 1. **Beautiful Error Pages (Ignition-style)**
Automatic error pages with:
- 🎨 Beautiful, dark-themed UI
- 📍 Stack traces with file/line information
- 🔍 Context capture (variables, request data)
- 💡 Solution hints
- 🔄 Debug vs Production modes
- 📊 Error context display

### 2. **Enhanced Debugging Macros**
Better than Laravel's `dd()`:
- **Color output** for different log levels
- **Performance timing** with `timer!()` macro
- **Benchmarking** with `benchmark!()` macro
- **Conditional debug** with `debug_if!()`
- **Better assertions** with `assert_debug!()`
- **Memory snapshots** with `memory_snapshot!()`

### 3. **Error Logging Service**
File-based error logging with:
- 📁 Automatic file rotation
- ⏰ Timestamps
- 🔍 Structured logging
- 📊 Recent error retrieval

---

## 1. Enhanced Debugging Macros

### `dd!()` - Dump and Die (Improved)

```rust
use crate::prelude::*;

let user = User { id: 1, name: "John", email: "john@example.com" };

// Beautiful dump with border and emoji
dd!(user);
// Output:
// ╔════════════════════════════════════════════════════════════╗
// ║                      🔴 DEBUG DUMP                        ║
// ╚════════════════════════════════════════════════════════════╝
// User { id: 1, name: "John", email: "john@example.com" }
//
// 📍 Location: src/users.rs:42

// Multiple values
dd!(user, post, comment);
```

---

### `dump!()` - Dump and Continue (Improved)

```rust
use crate::prelude::*;

// Chainable - returns the value
let user = dump!(fetch_user(1).await?);

// Multi-value dump
dump!(user, config, settings);

// Output with borders:
// ┌──────────────────────────────────────────────┐
// │ 🔍 DEBUG INFO                                │
// ├──────────────────────────────────────────────┤
// │ User { id: 1, name: "John" ... }
// ├──────────────────────────────────────────────┤
// │ 📍 src/users.rs:10
// └──────────────────────────────────────────────┘
```

---

### `debug!()` - Labeled Debug (Improved)

```rust
use crate::prelude::*;

let user = get_user(1);

// With label and context
debug!("user_data", user);

// Output:
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// 🧠 [user_data]
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// User { id: 1, name: "John", email: "john@example.com" }
// 📍 at src/users.rs:15
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

---

### `debug_if!()` - Conditional Debug (NEW)

Only dump if condition is true (useful for debugging specific scenarios):

```rust
use crate::prelude::*;

let user = get_user(1);

// Only debug if user is admin
debug_if!(user.is_admin, "admin_user", user);

// Only debug if expensive operation happens
debug_if!(should_debug, "expensive_result", result);
```

---

### `timer!()` - Performance Timer (NEW)

Measure execution time of a code block:

```rust
use crate::prelude::*;

// Time a database query
let users = timer!("fetch_users", {
    User::query()
        .where_eq("active", true)
        .limit(100)
        .get(&state.db_manager)
        .await?
});

// Output:
// ⏱️  [fetch_users] took 42.5ms

// Time an API call
let response = timer!("external_api", {
    http::get("https://api.example.com/data").await?
});
```

---

### `benchmark!()` - Run Benchmarks (NEW)

Time multiple iterations of code:

```rust
use crate::prelude::*;

// Benchmark 1000 iterations
benchmark!("hash_password", 1000, {
    hash::make("password123").ok();
});

// Output:
// 📊 [hash_password] 1000 iterations: 5.234s (avg: 5.234ms)
```

---

### `log_success!()` / `log_error!()` / `log_warning!()` / `log_info!()` (NEW)

Quick colored log output:

```rust
use crate::prelude::*;

log_success!("User created successfully");  // ✅ User created successfully
log_error!("Database connection failed");   // ❌ Database connection failed
log_warning!("Cache miss on key 'users'");  // ⚠️  Cache miss on key 'users'
log_info!("Processing batch #42");          // ℹ️  Processing batch #42
```

---

### `assert_debug!()` - Better Assertions (NEW)

Better error messages than standard `assert!`:

```rust
use crate::prelude::*;

let user = get_user(1);

// Assertwith message
assert_debug!(user.id == 1, "User ID should be 1, got: {:?}", user.id);

// Output on failure:
// ❌ ASSERTION FAILED
//    User ID should be 1, got: 2
//    at: src/users.rs:25
```

---

### `memory_snapshot!()` - Memory Profiling (NEW)

Capture memory state (for debugging memory leaks):

```rust
use crate::prelude::*;

memory_snapshot!("before_batch_processing");

for item in items {
    process_item(item).await?;
}

memory_snapshot!("after_batch_processing");

// Output:
// 💾 [before_batch_processing] Memory snapshot at src/main.rs:42
//    (Run with Instruments for detailed memory analysis)
```

---

## 2. Error Logging Service

### Basic Usage

```rust
use crate::prelude::*;

// Create error logger
let logger = ErrorLogger::new("storage/logs/errors.log");

// Log an error with context
logger.log_error(
    "Database Connection",
    "Failed to connect to database",
    "src/database.rs",
    42,
    Some(json!({
        "host": "localhost",
        "port": 3306,
        "database": "webrust_app"
    }))
);

// Log warning
logger.log_warning("Cache disabled", None);

// Log debug info (only in debug mode)
logger.log_debug("Performance", "Query took 125ms");

// Get recent errors
let recent = logger.get_recent_errors(10);
for error in recent {
    println!("{}", error);
}
```

### Configuration

```rust
use crate::prelude::*;

// Custom log path and size
let logger = ErrorLogger::new("storage/logs/app_errors.log")
    .with_max_size(50 * 1024 * 1024);  // 50MB before rotation
```

### Log File Rotation

Logs are automatically rotated when they exceed max size:

```
storage/logs/
├── errors.log           (current, <10MB)
├── errors_20251124_143022.log  (archived)
├── errors_20251124_120515.log  (archived)
└── errors_20251123_095430.log  (archived)
```

---

## 3. Error Context (Ignition-style Pages)

### Manual Error Context

```rust
use crate::prelude::*;

// Create error context
let error = ErrorContext::new("Database Error", "Connection timeout")
    .with_solution("Check your database connection string and ensure MySQL is running")
    .at("src/database.rs", 150)
    .with_frame("connect".to_string(), "src/database.rs".to_string(), 150)
    .with_frame("init_pool".to_string(), "src/framework.rs".to_string(), 42)
    .with_context("host", "localhost:3306")
    .with_context("database", "webrust_app")
    .with_context("timeout", "5000ms");

// Use as response (returns HTML error page)
error.into_response()
```

### Error Page Features

When an error occurs, the page displays:

1. **Header** with error title and status
2. **Metadata** (file, line, debug mode indicator)
3. **Error Message** in highlighted box
4. **Stack Trace** (if debug mode) with numbered frames
5. **Context** (if debug mode) showing variables and request data
6. **Solution** hints for fixing the issue

**In Production:** Detailed information is hidden for security, only showing "Check logs"

**In Debug:** Full details including stack traces and context are shown

---

## 4. Best Practices

### ✅ When to Use Each

```rust
use crate::prelude::*;

// Use dd!() - Stop execution to inspect
if let Err(e) = validate_user(&user) {
    dd!(e);  // Stop and inspect the error
}

// Use dump!() - Continue but log
let result = dump!(expensive_calculation());  // Log and continue
let filtered: Vec<_> = items
    .iter()
    .filter(|x| dump!(x.check_valid()).unwrap_or(false))
    .collect();

// Use debug!() - Named breakpoints
debug!("processing_step_1", state);
debug!("processing_step_2", state);

// Use timer!() - Performance profiling
let data = timer!("database_query", {
    User::query().where_eq("active", true).get(&pool).await?
});

// Use benchmark!() - Compare implementations
benchmark!("hash_comparison", 1000, {
    hash::make("password").ok();
});

// Use log_* - Status messages
log_success!("Database migration complete");
log_warning!("Cache is not configured");
```

---

## 5. Environment Variables

Control debugging behavior with environment variables:

```bash
# Enable detailed logging
export RUST_LOG=debug

# Show all logs including trace level
export RUST_LOG=trace

# Filter by module
export RUST_LOG=webrust::controllers=debug

# Disable debugging features (production)
export APP_DEBUG=false
```

---

## 6. Error Page Example

When an error occurs, users see a beautiful error page with:

```
┌─────────────────────────────────────────────┐
│                  💥                         │
│         Database Connection Failed          │
│    MySQL server is not accessible at        │
│         127.0.0.1:3306                      │
└─────────────────────────────────────────────┘

Status: 500 Internal Server Error
File: src/framework.rs
Line: 142
DEBUG MODE

📋 Error Message
─────────────────────────────────────────────
Could not connect to MySQL at 127.0.0.1:3306
Connection timeout after 5000ms

📍 Stack Trace
─────────────────────────────────────────────
① connect (src/database.rs:150)
② init_pool (src/framework.rs:142)
③ build_app (src/main.rs:85)
④ main (src/main.rs:42)

🔍 Context
─────────────────────────────────────────────
database: webrust_app
host: 127.0.0.1:3306
timeout: 5000ms

💡 Solution
─────────────────────────────────────────────
How to fix this:
✓ Ensure MySQL server is running
✓ Check DATABASE_URL in .env
✓ Verify network connectivity to database host
✓ Check MySQL port is 3306
```

---

## 7. Comparison with Laravel

| Feature | Laravel | WebRust |
|---------|---------|---------|
| Error Pages | Ignition | ✅ Ignition-style |
| dd() macro | Basic | ✅ Enhanced with borders |
| dump() macro | Basic | ✅ Enhanced chainable |
| Timer | Manual | ✅ Built-in `timer!()` |
| Benchmarking | Manual | ✅ Built-in `benchmark!()` |
| Log Rotation | Built-in | ✅ Automatic |
| Colored Output | Built-in | ✅ Emoji-enhanced |
| Assertions | Built-in | ✅ `assert_debug!()` |
| Memory Profiling | XDebug | ✅ `memory_snapshot!()` |

---

## 8. Integration with Existing Code

The error logging system works seamlessly with existing code:

```rust
// In your controller
#[post("/users")]
pub async fn store(State(state): State<AppState>, Form(data): Form<CreateUserRequest>) -> impl IntoResponse {
    let logger = ErrorLogger::new("storage/logs/users.log");
    
    match validate_user_data(&data) {
        Ok(user) => {
            match save_to_database(&user, &state.db_manager).await {
                Ok(created_user) => {
                    log_success!("User created successfully");
                    created(created_user)
                }
                Err(e) => {
                    logger.log_error("User Creation", &e.to_string(), file!(), line!(), None);
                    log_error!("Failed to create user");
                    server_error("Could not create user")
                }
            }
        }
        Err(e) => {
            logger.log_warning(&format!("Validation failed: {:?}", e), None);
            log_warning!("Validation failed");
            unprocessable_entity(e)
        }
    }
}
```

---

## Next Steps

- ✅ Use enhanced macros in development
- ✅ Configure error logging in production
- ✅ Customize error solutions in ErrorContext
- ✅ Monitor error logs with `logger.get_recent_errors()`
- ✅ Integrate with error tracking service (Sentry, etc.)
