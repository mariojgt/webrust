# WebRust Quick Reference – Laravel Conventions

Quick lookup for WebRust commands and patterns compared to Laravel.

## CLI Commands

```bash
# Setup
cargo run -- rune setup

# Development
cargo run -- rune dev                          # Hot reload server
cargo run -- rune serve --host 0.0.0.0 --port 3000
cargo run -- rune tinker                       # Interactive REPL shell
cargo run -- rune route:list                   # List all routes
cargo run -- rune migration:list               # List all migrations

# Generation
cargo run -- rune make:controller Post         # Simple controller
cargo run -- rune make:resource Post           # Full CRUD controller
cargo run -- rune make:model Post              # Eloquent-style model
cargo run -- rune make:middleware Auth         # Middleware
cargo run -- rune make:request PostRequest     # Form request
cargo run -- rune make:migration create_posts_table
cargo run -- rune make:auth                    # Full auth scaffold
cargo run -- rune make:package blog            # Package

# Database
cargo run -- rune migrate
cargo run -- rune migrate:rollback

# Queues & Scheduling
cargo run -- rune queue:work --queue default
cargo run -- rune schedule:run

# Custom Commands
cargo run -- rune make:command SendEmails
```

---

## Response Helpers

| Purpose | Laravel | WebRust |
|---------|---------|---------|
| Success | `response()->json($data)` | `success(data)` |
| Message | `response()->json(['message' => '...'])` | `success_message("text")` |
| Created | `response()->json($data, 201)` | `created(data)` |
| Accepted | `response()->json($data, 202)` | `accepted(data)` |
| No Content | `response()->noContent()` | `no_content()` |
| Redirect | `redirect('/path')` | `redirect("/path")` |
| Bad Request | `response()->json([...], 400)` | `bad_request("error")` |
| Unauthorized | `abort(401)` | `unauthorized("error")` |
| Forbidden | `abort(403)` | `forbidden("error")` |
| Not Found | `abort(404)` | `not_found_response("error")` |
| Validation | `response()->json([...], 422)` | `unprocessable_entity(errors)` |
| Rate Limited | `abort(429)` | `too_many_requests("error")` |
| Server Error | `abort(500)` | `server_error("error")` |
| Paginated | custom | `paginated(items, page, per_page, total)` |

---

## Query Builder Comparison

| Operation | Laravel | WebRust |
|-----------|---------|---------|
| All | `User::all()` | `User::query().get(&manager)` |
| Find | `User::find(1)` | `User::find(&pool, 1)` |
| Where | `->where('status', 'active')` | `.where_eq("status", "active")` |
| Where Op | `->where('age', '>', 18)` | `.where("age", ">", 18)` |
| Where In | `->whereIn('id', [...])` | `.where_in("id", vec![...])` |
| Where Not In | `->whereNotIn('id', [...])` | `.where_not_in("id", vec![...])` |
| Where Null | `->whereNull('deleted_at')` | `.where_null("deleted_at")` |
| Where Not Null | `->whereNotNull('email')` | `.where_not_null("email")` |
| Where Between | `->whereBetween('age', [18, 65])` | `.where_between("age", 18, 65)` |
| Or Where | `->orWhere('role', 'admin')` | `.or_where("role", "=", "admin")` |
| Order By | `->orderBy('name')` | `.order_by("name", "ASC")` |
| Latest | `->latest('created_at')` | `.latest("created_at")` |
| Oldest | `->oldest('created_at')` | `.oldest("created_at")` |
| Limit | `->limit(10)` | `.limit(10)` |
| Offset | `->offset(20)` | `.offset(20)` |
| Paginate | `->paginate(15)` | `.paginate(&manager, 1, 15)` |
| First | `->first()` | `.first(&manager)` |
| Get | `->get()` | `.get(&manager)` |
| Distinct | `->distinct()` | `.distinct()` |
| Group By | `->groupBy('status')` | `.group_by(&["status"])` |
| Having | `->having('COUNT(*)', '>', 5)` | `.having("COUNT(*)", ">", 5)` |
| Dump | `->dump()` | `.dump()` |
| Dump & Die | `->dd()` | `.dd()` |

---

## Model Definition

```rust
// Laravel
class Post extends Model {
    protected $table = 'posts';
    protected $fillable = ['title', 'body'];
}

// WebRust
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Post {
    pub id: i64,
    pub title: String,
    pub body: String,
    pub created_at: DateTime<Utc>,
}

impl Orbit for Post {
    fn table() -> &'static str { "posts" }
}
```

---

## Controller Structure

```rust
// Laravel Resource Controller
class PostController extends Controller {
    public function index() {}    // GET /posts
    public function create() {}   // GET /posts/create
    public function store() {}    // POST /posts
    public function show() {}     // GET /posts/{id}
    public function edit() {}     // GET /posts/{id}/edit
    public function update() {}   // PUT /posts/{id}
    public function destroy() {}  // DELETE /posts/{id}
}

// WebRust Resource Controller
pub async fn index() {}           // GET /posts
pub async fn create() {}          // GET /posts/create
pub async fn store() {}           // POST /posts
pub async fn show(Path(id)) {}    // GET /posts/:id
pub async fn edit(Path(id)) {}    // GET /posts/:id/edit
pub async fn update(Path(id)) {}  // PUT /posts/:id
pub async fn destroy(Path(id)) {} // DELETE /posts/:id
```

---

## Route Definition

```rust
// Laravel
Route::resource('posts', PostController::class);

// WebRust (auto-generated from make:resource)
router()
    .route("/posts", get(post::index).post(post::store))
    .route("/posts/create", get(post::create))
    .route("/posts/:id", get(post::show).put(post::update).delete(post::destroy))
    .route("/posts/:id/edit", get(post::edit))
```

---

## Form Validation

```rust
// Laravel
class StorePostRequest extends FormRequest {
    public function rules() {
        return [
            'title' => 'required|min:3|max:255',
            'body' => 'required|min:10',
        ];
    }
}

// WebRust
#[derive(Deserialize, Validate)]
pub struct StorePostRequest {
    #[validate(length(min = 3, max = 255))]
    pub title: String,

    #[validate(length(min = 10))]
    pub body: String,
}

// In controller
if let Err(errors) = payload.validate() {
    return unprocessable_entity(json!(errors));
}
```

---

## Service Layer

```rust
// Laravel
class PostService {
    public function getPublished() { ... }
}

// WebRust
#[async_trait]
impl BusinessService<Post> for PostService {
    async fn get_all(&self) { ... }
}
```

---

## Repository Pattern

```rust
// Laravel (optional)
class PostRepository {
    public function all() { ... }
    public function find($id) { ... }
}

// WebRust
#[async_trait]
impl Repository<Post> for PostRepository {
    async fn all(&self) { ... }
    async fn find(&self, id: i64) { ... }
}
```

---

## Template Tags (Tera)

| Function | Tera Syntax | Example |
|----------|------------|---------|
| Variable | `{{ var }}` | `{{ user.name }}` |
| If | `{% if condition %}` | `{% if user %}...{% endif %}` |
| For | `{% for item in items %}` | `{% for post in posts %}...{% endfor %}` |
| Extends | `{% extends "file" %}` | `{% extends "layout.html" %}` |
| Block | `{% block name %}` | `{% block content %}...{% endblock %}` |
| Include | `{% include "file" %}` | `{% include "nav.html" %}` |
| Set | `{% set var = value %}` | `{% set count = 5 %}` |
| Filter | `{{ var \| filter }}` | `{{ date \| date(format="%Y-%m-%d") }}` |
| Length | `{{ array \| length }}` | `{{ posts \| length }}` |

---

## Common Patterns

### Fetch & Display

```rust
// Laravel
$posts = Post::where('published', true)->get();
return view('posts.index', compact('posts'));

// WebRust
let posts = Post::query()
    .where_eq("published", true)
    .get(&state.db_manager)
    .await?;
let mut ctx = Context::new();
ctx.insert("posts", &posts);
Html(state.templates.render("posts/index.rune.html", &ctx)?)
```

### Create with Validation

```rust
// Laravel
$validated = $request->validated();
$post = Post::create($validated);

// WebRust
if let Err(_) = payload.validate() {
    return unprocessable_entity(errors);
}
sqlx::query("INSERT INTO posts ...")
    .bind(&payload.title)
    .execute(&state.db)?;
```

### Update

```rust
// Laravel
$post->update($validated);

// WebRust
sqlx::query("UPDATE posts SET ... WHERE id = ?")
    .bind(&payload.title)
    .bind(id)
    .execute(&state.db)?;
```

### Delete

```rust
// Laravel
$post->delete();

// WebRust
sqlx::query("DELETE FROM posts WHERE id = ?")
    .bind(id)
    .execute(&state.db)?;
```

### Pagination

```rust
// Laravel
$posts = Post::paginate(15);

// WebRust
let (posts, total) = Post::query()
    .paginate(&manager, page, 15)
    .await?;
paginated(posts, page, 15, total)
```

---

## File Organization

```
src/
├── main.rs                  # Entry point
├── cli.rs                   # CLI commands
├── framework.rs             # App setup
├── prelude.rs              # Common imports
├── controllers/             # HTTP handlers
│   ├── mod.rs
│   ├── post.rs
│   └── home.rs
├── services/
│   ├── mod.rs
│   ├── repository.rs        # Repository trait
│   ├── service_layer.rs     # Service traits
│   ├── post_service.rs      # Business logic
│   └── auth.rs
├── models/
│   ├── mod.rs
│   ├── post.rs
│   └── user.rs
├── routes/
│   ├── mod.rs
│   ├── web.rs
│   ├── api.rs
│   └── post.rs
├── http/
│   ├── mod.rs
│   ├── response.rs          # Response helpers
│   ├── middleware_helpers.rs
│   ├── resource_controller.rs
│   └── middleware/
└── requests/
    ├── mod.rs
    └── post.rs

templates/
├── layout.rune.html
├── post/
│   ├── index.rune.html
│   ├── create.rune.html
│   ├── show.rune.html
│   └── edit.rune.html
└── ...

migrations/
├── XXXXXX_create_posts_table.sql
└── ...
```

---

## Debugging

```rust
// Print variable
dump!(user);

// Print and continue
debug!("User", user);

// Print and stop (panic)
dd!(user);

// In queries
User::query()
    .where_eq("status", "active")
    .dump()      // Print SQL
    .dd()        // Print SQL and stop
```

---

## Environment Variables

```env
# Database
DATABASE_URL=mysql://user:pass@localhost/db
DB_CONNECTION=mysql

# Cache
CACHE_DRIVER=redis
REDIS_URL=redis://127.0.0.1:6379

# Session
SESSION_DRIVER=database
SESSION_LIFETIME=120

# Mail
MAIL_HOST=smtp.mailtrap.io
MAIL_PORT=587
MAIL_USERNAME=user
MAIL_PASSWORD=pass

# App
APP_KEY=your-secret-key
APP_ENV=local
```

---

## Tinker REPL (Interactive Shell)

Access the interactive shell for debugging and testing:

```bash
cargo run -- rune tinker

# Inside Tinker, available commands:
>> help                    # Show all commands
>> db:tables               # List all database tables
>> db:table users          # Show columns in 'users' table
>> db:count users          # Count rows in 'users' table
>> sql:execute <query>     # Execute raw SQL
>> config:app              # Show app configuration
>> config:db               # Show database config
>> route:list              # List all routes
>> info                    # Show app info
>> clear                   # Clear screen
>> exit                    # Exit Tinker
```

---

## Factories Pattern (Test Data)

Generate test/dummy data using factories:

```rust
use crate::services::factory::{UserFactory, PostFactory, Factory};

// Generate without persisting
let user = UserFactory::new()
    .with_name("John Doe")
    .with_email("john@example.com")
    .admin()
    .make()
    .await;

// Create (persist to DB)
let user = UserFactory::new()
    .with_email("jane@example.com")
    .create()
    .await?;

// Create multiple
let users = UserFactory::new()
    .create_many(10)
    .await?;

// Post factory
let post = PostFactory::new()
    .with_title("My Post")
    .with_user_id(1)
    .create()
    .await?;

// Comment factory
let comment = CommentFactory::new()
    .with_post_id(1)
    .with_user_id(2)
    .create()
    .await?;
```

---

## Events & Listeners

Decoupled event-driven architecture:

```rust
use crate::prelude::*;

// Dispatch event
let dispatcher = EventDispatcher::new();
dispatcher.listen("user.created", SendWelcomeEmailListener).await;

let event = UserCreatedEvent {
    user_id: 1,
    email: "user@example.com".to_string(),
    name: "John".to_string(),
};
dispatcher.emit(&event).await?;

// Create event
#[derive(Clone)]
pub struct UserCreatedEvent { pub user_id: i64, ... }

#[async_trait]
impl Event for UserCreatedEvent {
    fn name(&self) -> &'static str { "user.created" }
}

// Create listener
pub struct SendWelcomeEmailListener;

#[async_trait]
impl Listener for SendWelcomeEmailListener {
    async fn handle(&self, event: &dyn Event) -> Result<(), Box<dyn std::error::Error>> {
        if event.name() == "user.created" {
            mail::send_welcome_email(event).await?;
        }
        Ok(())
    }
}
```

---

## Model Observers

Automatic lifecycle hooks:

```rust
use crate::prelude::*;

// Create observer
pub struct UserObserver;

#[async_trait]
impl Observer for UserObserver {
    async fn created(&self, user: &Value) -> Result<(), Box<dyn std::error::Error>> {
        println!("User created!");
        Ok(())
    }

    async fn updated(&self, user: &Value) -> Result<(), Box<dyn std::error::Error>> {
        println!("User updated!");
        Ok(())
    }

    async fn deleted(&self, user: &Value) -> Result<(), Box<dyn std::error::Error>> {
        println!("User deleted!");
        Ok(())
    }
}

// Use observer in model
#[async_trait]
impl Observable for User {
    fn observers() -> Vec<Box<dyn Observer>> {
        vec![Box::new(UserObserver)]
    }
}

// Trigger events
user.fire_created().await?;  // Calls all observers
user.fire_updated().await?;
user.fire_deleted().await?;
```

**Available Events:** creating, created, updating, updated, deleting, deleted, saving, saved

---

## Authorization Policies

Clean permission patterns:

```rust
use crate::prelude::*;

// Create policy
pub struct PostPolicy;

#[async_trait]
impl Policy for PostPolicy {
    async fn view(&self, user: &Value, post: &Value) -> PolicyResult {
        Ok(true) // Anyone can view
    }

    async fn create(&self, user: &Value) -> PolicyResult {
        Ok(user.get("id").is_some()) // Auth users only
    }

    async fn update(&self, user: &Value, post: &Value) -> PolicyResult {
        let uid = user.get("id").and_then(|v| v.as_i64());
        let pid = post.get("user_id").and_then(|v| v.as_i64());
        Ok(uid == pid) // Own posts only
    }

    async fn delete(&self, user: &Value, post: &Value) -> PolicyResult {
        let uid = user.get("id").and_then(|v| v.as_i64());
        let pid = post.get("user_id").and_then(|v| v.as_i64());
        let admin = user.get("is_admin").and_then(|v| v.as_bool()).unwrap_or(false);
        Ok(uid == pid || admin) // Own or admin
    }
}

// Use in controller
let policy = PostPolicy;
Authorizer::authorize_or_fail(&policy, &user, &post, "update").await?;
```

**Available Methods:** view, create, update, delete, restore, force_delete

---

## Key Differences

| Aspect | Laravel | WebRust |
|--------|---------|---------|
| Language | PHP | Rust |
| Async | Built-in | Native via tokio |
| Type Safety | Dynamic | Static |
| Performance | Moderate | Very High |
| Learning Curve | Gentle | Moderate |
| DB Queries | Eloquent ORM | Orbit ORM + SQLx |
| HTTP | Laravel framework | Axum framework |
| Templating | Blade | Tera |
| Validation | Laravel Validator | validator crate |
| Auth | Sanctum/Passport | Custom + JWT |
| Events | Event Dispatcher | Event/Listener traits |
| Observers | Model Observers | Observer trait |
| Policies | Authorization | Policy trait |

Remember: WebRust gives you **Laravel-like patterns** with **Rust's performance and safety**! 🚀
