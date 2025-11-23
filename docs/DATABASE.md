# Database Configuration

WebRust supports multiple database connections, similar to Laravel. You can configure them in your `.env` file and use them in your models.

## Configuration

By default, WebRust looks for a `DATABASE_URL` environment variable. This is used for the default connection (usually `mysql`).

You can define additional connections or override the default one in `src/config/database.rs` (though currently it's hardcoded to read from env).

## Using Multiple Connections

### In Models

To specify which connection a model should use, override the `connection()` method in your model's `Orbit` implementation.

```rust
use crate::orbit::Orbit;

impl Orbit for User {
    fn table_name() -> &'static str {
        "users"
    }

    fn id(&self) -> i64 {
        self.id
    }

    // Optional: Specify a connection name
    // If not provided, it defaults to None (the default connection)
    fn connection() -> Option<&'static str> {
        Some("sqlite") 
    }
}
```

### In Controllers

When using Orbit methods like `all`, `find`, `create`, etc., you now pass the `DatabaseManager` instead of a specific pool. The model itself will decide which pool to use.

```rust
use crate::framework::AppState;
use crate::models::user::User;
use crate::orbit::Orbit;

pub async fn index(State(state): State<AppState>) -> Html<String> {
    // Pass the manager, not the pool
    let users = User::all(&state.db_manager).await.unwrap();
    
    // ...
}
```

## Connection Management

The `DatabaseManager` is initialized at startup and attempts to connect to all defined connections. If a connection fails, it logs a warning but continues starting the server.

You can access the manager in your `AppState`:

```rust
pub struct AppState {
    pub db_manager: DatabaseManager,
    // ...
}
```

To get a raw pool manually:

```rust
let pool = state.db_manager.connection(Some("mysql")); // Returns Option<&DbPool>
let default_pool = state.db_manager.default_connection();
```
