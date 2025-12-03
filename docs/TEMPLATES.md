# Templates

WebRust uses the **Tera** template engine, which is inspired by Jinja2 and Django templates. Template files are stored in the `templates/` directory and typically use the `.rune.html` extension.

## Basic Syntax

- **Variables**: `{{ variable_name }}`
- **Tags**: `{% if user %}` ... `{% endif %}`
- **Comments**: `{# This is a comment #}`

## Layouts and Inheritance

WebRust supports template inheritance, allowing you to define a base layout and extend it in your pages.

### Base Layout (`templates/layout.rune.html`)

```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>{% block title %}WebRust{% endblock %}</title>
</head>
<body>
    <nav>
        <!-- Navigation -->
    </nav>

    <main>
        {% block content %}{% endblock %}
    </main>

    <footer>
        <!-- Footer -->
    </footer>
</body>
</html>
```

### Child Page (`templates/home.rune.html`)

```html
{% extends "layout.rune.html" %}

{% block title %}Home Page{% endblock %}

{% block content %}
    <h1>Welcome to WebRust</h1>
    <p>This is the home page.</p>
{% endblock %}
```

## Control Structures

### Loops

```html
<ul>
{% for user in users %}
    <li>{{ user.name }}</li>
{% endfor %}
</ul>
```

### Conditionals

```html
{% if user.is_admin %}
    <button>Delete</button>
{% else %}
    <p>You do not have permission.</p>
{% endif %}
```

## Passing Data to Templates

You pass data to templates using a `Context` object in your controller.

```rust
use tera::Context;

pub async fn index(State(state): State<AppState>) -> Html<String> {
    let mut ctx = Context::new();
    ctx.insert("name", "Mario");
    ctx.insert("is_admin", &true);

    let body = state.templates.render("index.rune.html", &ctx).unwrap();
    Html(body)
}
```

## Global Variables

By default, Tera templates do not have access to global variables like `auth` or `csrf_token` unless you explicitly pass them from your controller.

To make variables available globally, you would typically create a helper function or middleware to merge them into the context, or simply add them in every controller method.

### Example: Passing CSRF Token

```rust
use tower_sessions::Session;

pub async fn form(session: Session, State(state): State<AppState>) -> Html<String> {
    let mut ctx = Context::new();
    
    // Retrieve CSRF token from session
    if let Some(token) = session.get::<String>("_csrf_token").await.unwrap() {
        ctx.insert("csrf_token", &token);
    }

    let body = state.templates.render("form.rune.html", &ctx).unwrap();
    Html(body)
}
```
