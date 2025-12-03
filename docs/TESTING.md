# Testing in WebRust

WebRust provides a fluent testing API inspired by Laravel, making it easy to write integration tests for your application.

## Getting Started

To create a test, use the `TestClient` struct from `crate::support::testing`.

```rust
#[cfg(test)]
mod tests {
    use crate::support::testing::TestClient;

    #[tokio::test]
    async fn test_home_page() {
        let client = TestClient::new().await;

        client.get("/")
            .await
            .assert_ok()
            .assert_see("Welcome");
    }
}
```

## Making Requests

The `TestClient` supports common HTTP methods:

```rust
// GET
let response = client.get("/api/users").await;

// POST (with JSON body)
let response = client.post("/api/users", &json!({
    "name": "John Doe",
    "email": "john@example.com"
})).await;

// PUT
let response = client.put("/api/users/1", &data).await;

// DELETE
let response = client.delete("/api/users/1").await;
```

## Assertions

The `TestResponse` object provides several assertion methods to verify the response:

### Status Code

```rust
response.assert_status(201);
response.assert_ok();       // 200
response.assert_not_found(); // 404
```

### Content

```rust
// Check if body contains text
response.assert_see("User created");
response.assert_dont_see("Error");

// Check JSON content
let user: User = response.json();
assert_eq!(user.name, "John Doe");
```

### Redirects

```rust
response.assert_redirect("/login");
```

## Database Testing

By default, `TestClient::new()` initializes the application with the configuration defined in your environment. For testing, you should use a separate test database to avoid wiping your development data.

1. Create a `.env.test` file with your test database URL.
2. Load it in your test setup or ensure your CI environment sets the correct variables.

## Example: Testing an API Endpoint

```rust
#[tokio::test]
async fn test_create_user() {
    let client = TestClient::new().await;

    let response = client.post("/api/users", &json!({
        "username": "testuser",
        "email": "test@example.com"
    })).await;

    response.assert_status(201);

    let user: User = response.json();
    assert_eq!(user.username, "testuser");
}
```
