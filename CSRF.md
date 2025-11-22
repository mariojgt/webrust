# 🛡️ CSRF Protection

WebRust includes built-in CSRF protection middleware to prevent Cross-Site Request Forgery attacks.

## How it works

1.  A `_csrf_token` is automatically generated and stored in the session.
2.  For "unsafe" methods (`POST`, `PUT`, `PATCH`, `DELETE`), the middleware checks for a matching token in the `X-CSRF-TOKEN` header.

## Usage

### 1. In Controllers

To get the token to pass to your view (so JavaScript can read it):

```rust
use crate::http::middleware::csrf::CsrfToken;

pub async fn index(CsrfToken(token): CsrfToken) -> Html<String> {
    let mut ctx = Context::new();
    ctx.insert("csrf_token", &token);
    // render view...
}
```

### 2. In Views (Meta Tag)

Add this to your `layout.rune.html` head:

```html
<meta name="csrf-token" content="{{ csrf_token }}">
```

### 3. In JavaScript (AJAX)

Configure your AJAX library (like Axios or fetch) to send the header:

```javascript
const token = document.querySelector('meta[name="csrf-token"]').getAttribute('content');

fetch('/api/endpoint', {
    method: 'POST',
    headers: {
        'X-CSRF-TOKEN': token,
        'Content-Type': 'application/json'
    },
    body: JSON.stringify({ ... })
});
```

## Excluding Routes

You can exclude specific URIs from CSRF verification by adding them to the `EXCLUDED_PATHS` constant in `src/http/middleware/csrf.rs`.

```rust
pub const EXCLUDED_PATHS: &[&str] = &[
    "/webhooks/stripe",
    "/payment/callback",
];
```

This works similarly to the `$except` array in Laravel's `VerifyCsrfToken` middleware.
