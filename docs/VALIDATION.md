# Validation

WebRust provides a robust validation system using the `validator` crate, integrated with `FormRequest` structs and automatic error flashing.

## Creating a Form Request

You can generate a form request using the CLI:

```bash
cargo run -- rune make:request LoginRequest
```

This creates a struct in `src/requests/login_request.rs`.

## Defining Rules

Add validation attributes to your struct fields.

```rust
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, Debug)]
pub struct LoginRequest {
    #[validate(email(message = "Invalid email address"))]
    pub email: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
}
```

## Using in Controllers

In your controller, use `Form<LoginRequest>` to extract the data. You can then call `.validate()` manually, or rely on the framework's helpers if you implement them.

### Manual Validation with Flash Messages

```rust
use axum::{extract::State, response::{IntoResponse, Redirect}, Form};
use tower_sessions::Session;
use validator::Validate;
use crate::requests::login_request::LoginRequest;
use crate::services::flash::Flash;
use crate::services::validation::ValidationErrors;

pub async fn login(
    session: Session,
    Form(payload): Form<LoginRequest>,
) -> impl IntoResponse {
    // 1. Validate
    if let Err(e) = payload.validate() {
        // 2. Convert errors to a HashMap
        let errors = e.field_errors().iter().map(|(k, v)| {
            (k.to_string(), v.iter().map(|e| e.message.clone().unwrap_or_default().into_owned()).collect())
        }).collect();

        // 3. Flash errors to session
        ValidationErrors::flash(&session, errors).await;

        // 4. Redirect back
        return Redirect::to("/login").into_response();
    }

    // ... proceed with login
}
```

## Displaying Errors in Views

### Blade/Tera Templates

In your controller that renders the form, retrieve the errors from the session and pass them to the view.

```rust
pub async fn login_form(session: Session, State(state): State<AppState>) -> impl IntoResponse {
    let mut ctx = Context::new();

    // Retrieve errors
    let errors = ValidationErrors::get(&session).await;
    ctx.insert("errors", &errors);

    // ... render view
}
```

In your template:

```html
<input type="email" name="email">
{% if errors.email %}
    <div class="text-red-500">{{ errors.email[0] }}</div>
{% endif %}
```

### Inertia.js

If you are using Inertia, the `share_inertia_data` middleware automatically injects validation errors into the `errors` prop.

```vue
<script setup>
defineProps({ errors: Object })
</script>

<template>
    <input v-model="form.email">
    <div v-if="errors.email">{{ errors.email }}</div>
</template>
```
