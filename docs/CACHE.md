# Caching in WebRust

WebRust provides a unified API for various caching backends, similar to Laravel. The cache configuration is located in your `.env` file.

## Configuration

Specify the cache driver in your `.env` file:

```dotenv
CACHE_DRIVER=file
```

Supported drivers: `file`, `redis`, `array` (memory).

### Redis Configuration

If using `redis`, ensure you have the `REDIS_URL` set:

```dotenv
CACHE_DRIVER=redis
REDIS_URL=redis://127.0.0.1:6379/
```

## Usage

The cache instance is available in the `AppState`. You can access it in your controllers.

```rust
use axum::{extract::State, Json};
use crate::framework::AppState;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
struct UserStats {
    visits: u32,
    last_seen: String,
}

pub async fn index(State(state): State<AppState>) -> Json<UserStats> {
    // Retrieve from cache or compute
    let stats = state.cache.remember("user_stats", 60, || async {
        // Expensive operation...
        UserStats {
            visits: 100,
            last_seen: "now".to_string(),
        }
    }).await.unwrap();

    Json(stats)
}
```

### Available Methods

The `Cache` trait provides the following methods:

- `get(key: &str) -> Result<Option<String>, CacheError>`
- `put(key: &str, value: &str, seconds: u64) -> Result<(), CacheError>`
- `has(key: &str) -> Result<bool, CacheError>`
- `forget(key: &str) -> Result<(), CacheError>`
- `flush() -> Result<(), CacheError>`

### Helper Methods (Typed)

- `get_json<T>(key: &str) -> Result<Option<T>, CacheError>`
- `put_json<T>(key: &str, value: &T, seconds: u64) -> Result<(), CacheError>`
- `remember<T>(key: &str, seconds: u64, callback: F) -> Result<T, CacheError>`

## Drivers

### File
Stores cached items in `storage/cache`. Keys are hashed to generate filenames.

### Redis
Uses a Redis server. Supports high-performance caching.

### Array (Memory)
Stores items in memory. Useful for testing or ephemeral data. Data is lost when the application restarts.
