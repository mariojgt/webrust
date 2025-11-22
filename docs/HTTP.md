# HTTP Client

WebRust provides an expressive, fluent API for making outgoing HTTP requests, inspired by Laravel's `Http` facade. It is built on top of the robust `reqwest` crate but offers a simplified developer experience.

## Basic Usage

The `Http` client is available globally via the prelude. You can use it to make `GET`, `POST`, `PUT`, `PATCH`, and `DELETE` requests.

```rust
use crate::prelude::*;

// Simple GET request
let response = Http::get("https://api.example.com/users")
    .send()
    .await?;

// POST request with JSON
let response = Http::post("https://api.example.com/users")
    .json(&serde_json::json!({
        "name": "Mario",
        "role": "Developer"
    }))
    .send()
    .await?;
```

## Request Data

### Sending JSON

Use the `.json()` method (from the underlying `reqwest` builder) to send JSON data.

```rust
let response = Http::as_json()
    .post("https://api.example.com/users")
    .json(&my_struct)
    .send()
    .await?;
```

### Sending Form Data

Use the `.form()` method to send `application/x-www-form-urlencoded` data.

```rust
let params = [("key", "value"), ("foo", "bar")];

let response = Http::as_form()
    .post("https://api.example.com/submit")
    .form(&params)
    .send()
    .await?;
```

## Headers and Authentication

### Custom Headers

You can add headers using `with_headers`.

```rust
use std::collections::HashMap;

let mut headers = HashMap::new();
headers.insert("X-Custom-Header".to_string(), "Value".to_string());

let response = Http::with_headers(headers)
    .get("https://api.example.com/data")
    .send()
    .await?;
```

### Bearer Token Authentication

The `with_token` method makes it easy to authenticate with APIs.

```rust
let response = Http::with_token("your-secret-token")
    .get("https://api.example.com/user")
    .send()
    .await?;
```

## Content Negotiation

WebRust provides convenience methods to set common `Accept` and `Content-Type` headers.

### Expecting JSON

Use `accept_json()` to set the `Accept: application/json` header.

```rust
let response = Http::accept_json()
    .get("https://api.example.com/users")
    .send()
    .await?;
```

### Sending JSON

Use `as_json()` to set the `Content-Type: application/json` header.

```rust
let response = Http::as_json()
    .post("https://api.example.com/users")
    .body(json_string)
    .send()
    .await?;
```

## Timeouts

You can specify a timeout for the request using the `timeout` method (in seconds).

```rust
let response = Http::timeout(5) // 5 seconds
    .get("https://slow-api.example.com/data")
    .send()
    .await?;
```

## Error Handling

Since the `Http` client returns a standard `reqwest::RequestBuilder`, you handle responses just like you would in `reqwest`.

```rust
let response = Http::get("https://api.example.com/users").send().await?;

if response.status().is_success() {
    let users: Vec<User> = response.json().await?;
    println!("Users: {:?}", users);
} else {
    eprintln!("Request failed: {}", response.status());
}
```
