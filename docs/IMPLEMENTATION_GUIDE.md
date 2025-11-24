# WebRust Laravel-Style Implementation Guide

This guide shows how to build a complete feature using WebRust following Laravel patterns.

## Example: Building a Blog Feature

### Step 1: Generate Resources

```bash
# Generate resource controller
cargo run -- rune make:resource Post

# Generate model
cargo run -- rune make:model Post

# Generate request/validation
cargo run -- rune make:request PostRequest

# Generate migration
cargo run -- rune make:migration create_posts_table
```

### Step 2: Create Database Migration

File: `migrations/XXXXXX_create_posts_table.sql`

```sql
CREATE TABLE posts (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    user_id BIGINT NOT NULL,
    title VARCHAR(255) NOT NULL,
    slug VARCHAR(255) NOT NULL UNIQUE,
    body LONGTEXT NOT NULL,
    published_at TIMESTAMP NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);
```

### Step 3: Define the Model

File: `src/models/post.rs`

```rust
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};
use crate::orbit::Orbit;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Post {
    pub id: i64,
    pub user_id: i64,
    pub title: String,
    pub slug: String,
    pub body: String,
    pub published_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Orbit for Post {
    fn table() -> &'static str {
        "posts"
    }

    fn connection() -> Option<&'static str> {
        None // Uses default connection
    }
}

impl Post {
    /// Find published posts
    pub async fn published(manager: &crate::database::DatabaseManager) -> Result<Vec<Self>, sqlx::Error> {
        Self::query()
            .where_not_null("published_at")
            .latest("published_at")
            .get(manager)
            .await
    }

    /// Find by slug
    pub async fn find_by_slug(manager: &crate::database::DatabaseManager, slug: &str) -> Result<Option<Self>, sqlx::Error> {
        Self::query()
            .where_eq("slug", slug)
            .first(manager)
            .await
    }

    /// Get paginated posts for a user
    pub async fn user_posts_paginated(
        manager: &crate::database::DatabaseManager,
        user_id: i64,
        page: i64,
        per_page: i64,
    ) -> Result<(Vec<Self>, i64), sqlx::Error> {
        Self::query()
            .where_eq("user_id", user_id)
            .latest("created_at")
            .paginate(manager, page, per_page)
            .await
    }
}
```

### Step 4: Create Validation Request

File: `src/requests/post.rs`

```rust
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct PostRequest {
    #[validate(length(min = 3, max = 255))]
    pub title: String,

    #[validate(length(min = 5, max = 255))]
    pub slug: String,

    #[validate(length(min = 10))]
    pub body: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct PublishPostRequest {
    #[validate]
    pub publish: bool,
}
```

### Step 5: Create Repository

File: `src/services/repositories/post_repository.rs`

```rust
use async_trait::async_trait;
use crate::database::DbPool;
use crate::models::post::Post;
use crate::services::repository::{Repository, BaseRepository};

pub struct PostRepository {
    base: BaseRepository<Post>,
}

impl PostRepository {
    pub fn new(pool: DbPool) -> Self {
        Self {
            base: BaseRepository::new(pool),
        }
    }

    pub fn pool(&self) -> &DbPool {
        self.base.pool()
    }

    pub async fn published(&self) -> Result<Vec<Post>, Box<dyn std::error::Error + Send + Sync>> {
        let pool = self.pool();
        Ok(Post::query()
            .where_not_null("published_at")
            .latest("published_at")
            .get(&DatabaseManager::new("default".to_string()))
            .await?)
    }
}

#[async_trait]
impl Repository<Post> for PostRepository {
    async fn all(&self) -> Result<Vec<Post>, Box<dyn std::error::Error + Send + Sync>> {
        Ok(Post::query()
            .get(&DatabaseManager::new("default".to_string()))
            .await?)
    }

    async fn find(&self, id: i64) -> Result<Option<Post>, Box<dyn std::error::Error + Send + Sync>> {
        Ok(Post::find(self.pool(), id).await?)
    }

    async fn paginate(&self, page: i64, per_page: i64) -> Result<(Vec<Post>, i64), Box<dyn std::error::Error + Send + Sync>> {
        Ok(Post::query()
            .paginate(&DatabaseManager::new("default".to_string()), page, per_page)
            .await?)
    }

    async fn create(&self, data: Post) -> Result<Post, Box<dyn std::error::Error + Send + Sync>> {
        // Implementation depends on your create approach
        unimplemented!("Create logic needed")
    }

    async fn update(&self, id: i64, data: Post) -> Result<Post, Box<dyn std::error::Error + Send + Sync>> {
        unimplemented!("Update logic needed")
    }

    async fn delete(&self, id: i64) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        let pool = self.pool();
        sqlx::query("DELETE FROM posts WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(true)
    }

    async fn count(&self) -> Result<i64, Box<dyn std::error::Error + Send + Sync>> {
        let pool = self.pool();
        let (count,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM posts")
            .fetch_one(pool)
            .await?;
        Ok(count)
    }
}
```

### Step 6: Create Business Service

File: `src/services/post_service.rs`

```rust
use async_trait::async_trait;
use crate::database::DatabaseManager;
use crate::models::post::Post;
use crate::services::service_layer::BusinessService;
use crate::services::repository::Repository;
use std::sync::Arc;
use crate::services::repositories::post_repository::PostRepository;

pub struct PostService {
    db_manager: Arc<DatabaseManager>,
    repository: PostRepository,
}

impl PostService {
    pub fn new(db_manager: Arc<DatabaseManager>, repository: PostRepository) -> Self {
        Self { db_manager, repository }
    }

    /// Get all published posts
    pub async fn get_published(&self) -> Result<Vec<Post>, Box<dyn std::error::Error + Send + Sync>> {
        self.repository.published().await
    }

    /// Publish a post
    pub async fn publish(&self, post_id: i64) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        let pool = self.repository.pool();
        sqlx::query("UPDATE posts SET published_at = NOW() WHERE id = ? AND published_at IS NULL")
            .bind(post_id)
            .execute(pool)
            .await?;
        Ok(true)
    }

    /// Unpublish a post
    pub async fn unpublish(&self, post_id: i64) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        let pool = self.repository.pool();
        sqlx::query("UPDATE posts SET published_at = NULL WHERE id = ?")
            .bind(post_id)
            .execute(pool)
            .await?;
        Ok(true)
    }
}

impl crate::services::service_layer::Service for PostService {
    fn service_name(&self) -> &str {
        "PostService"
    }
}

#[async_trait]
impl BusinessService<Post> for PostService {
    async fn get_all(&self) -> Result<Vec<Post>, Box<dyn std::error::Error + Send + Sync>> {
        self.repository.all().await
    }

    async fn get_by_id(&self, id: i64) -> Result<Option<Post>, Box<dyn std::error::Error + Send + Sync>> {
        self.repository.find(id).await
    }

    async fn create(&self, data: Post) -> Result<Post, Box<dyn std::error::Error + Send + Sync>> {
        // Implement creation logic
        unimplemented!("Create logic needed")
    }

    async fn update(&self, id: i64, data: Post) -> Result<Post, Box<dyn std::error::Error + Send + Sync>> {
        // Implement update logic
        unimplemented!("Update logic needed")
    }

    async fn delete(&self, id: i64) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        self.repository.delete(id).await
    }
}
```

### Step 7: Implement Controller

File: `src/controllers/post.rs` (generated by `make:resource`)

```rust
use axum::{
    extract::{State, Path},
    response::{Html, IntoResponse, Response},
    Form, Json,
};
use tera::Context;
use validator::Validate;

use crate::framework::AppState;
use crate::prelude::*;
use crate::models::post::Post;
use crate::requests::post::PostRequest;

pub async fn index(State(state): State<AppState>) -> impl IntoResponse {
    let mut ctx = Context::new();
    ctx.insert("title", "Blog Posts");

    // Fetch posts from database
    match Post::query()
        .latest("created_at")
        .limit(20)
        .get(&state.db_manager)
        .await
    {
        Ok(posts) => {
            ctx.insert("posts", &posts);
            let body = state
                .templates
                .render("post/index.rune.html", &ctx)
                .unwrap();
            Html(body).into_response()
        }
        Err(_) => {
            Html("<h1>Error loading posts</h1>").into_response()
        }
    }
}

pub async fn create(State(state): State<AppState>) -> impl IntoResponse {
    let mut ctx = Context::new();
    ctx.insert("title", "Create Post");

    let body = state
        .templates
        .render("post/create.rune.html", &ctx)
        .unwrap();
    Html(body)
}

pub async fn store(
    State(state): State<AppState>,
    Form(payload): Form<PostRequest>,
) -> Response {
    // Validate
    if let Err(_) = payload.validate() {
        return unprocessable_entity(json!({
            "message": "Validation failed",
            "errors": {
                "title": ["Title must be between 3 and 255 characters"],
                "slug": ["Slug must be between 5 and 255 characters"],
                "body": ["Body must be at least 10 characters"]
            }
        }));
    }

    // Create post
    if let Some(pool) = &state.db {
        match sqlx::query(
            "INSERT INTO posts (user_id, title, slug, body, created_at, updated_at)
             VALUES (?, ?, ?, ?, NOW(), NOW())"
        )
        .bind(1) // Get from auth
        .bind(&payload.title)
        .bind(&payload.slug)
        .bind(&payload.body)
        .execute(pool)
        .await
        {
            Ok(result) => {
                created(json!({
                    "id": result.last_insert_id(),
                    "title": payload.title,
                    "slug": payload.slug,
                }))
            }
            Err(_) => server_error("Failed to create post"),
        }
    } else {
        server_error("Database connection failed")
    }
}

pub async fn show(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    match Post::find(&state.db.clone().unwrap(), id).await {
        Ok(Some(post)) => {
            let mut ctx = Context::new();
            ctx.insert("post", &post);
            let body = state
                .templates
                .render("post/show.rune.html", &ctx)
                .unwrap();
            Html(body).into_response()
        }
        _ => not_found_response("Post not found").into_response(),
    }
}

pub async fn edit(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    match Post::find(&state.db.clone().unwrap(), id).await {
        Ok(Some(post)) => {
            let mut ctx = Context::new();
            ctx.insert("post", &post);
            let body = state
                .templates
                .render("post/edit.rune.html", &ctx)
                .unwrap();
            Html(body)
        }
        _ => Html("<h1>Post not found</h1>"),
    }
}

pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Form(payload): Form<PostRequest>,
) -> Response {
    if let Err(_) = payload.validate() {
        return unprocessable_entity(json!({
            "message": "Validation failed"
        }));
    }

    if let Some(pool) = &state.db {
        match sqlx::query(
            "UPDATE posts SET title = ?, slug = ?, body = ?, updated_at = NOW() WHERE id = ?"
        )
        .bind(&payload.title)
        .bind(&payload.slug)
        .bind(&payload.body)
        .bind(id)
        .execute(pool)
        .await
        {
            Ok(_) => success_message("Post updated successfully"),
            Err(_) => server_error("Failed to update post"),
        }
    } else {
        server_error("Database connection failed")
    }
}

pub async fn destroy(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Response {
    if let Some(pool) = &state.db {
        match sqlx::query("DELETE FROM posts WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await
        {
            Ok(_) => no_content(),
            Err(_) => server_error("Failed to delete post"),
        }
    } else {
        server_error("Database connection failed")
    }
}
```

### Step 8: Register Routes

Update `src/routes/mod.rs`:

```rust
pub mod web;
pub mod api;
pub mod post;

// In the routes function:
pub async fn router(state: AppState) -> Router {
    let web_routes = web::web(state.clone())
        .merge(post::routes(state.clone()));

    // ... rest of setup
}
```

### Step 9: Create Templates

File: `templates/post/index.rune.html`

```html
{% extends "layout.rune.html" %}

{% block content %}
<div class="container">
    <h1>{{ title }}</h1>
    <a href="/posts/create" class="btn btn-primary">Create Post</a>

    <table class="table">
        <thead>
            <tr>
                <th>Title</th>
                <th>Slug</th>
                <th>Published</th>
                <th>Actions</th>
            </tr>
        </thead>
        <tbody>
            {% for post in posts %}
            <tr>
                <td><a href="/posts/{{ post.id }}">{{ post.title }}</a></td>
                <td>{{ post.slug }}</td>
                <td>{{ post.published_at | date(format="%Y-%m-%d") }}</td>
                <td>
                    <a href="/posts/{{ post.id }}/edit">Edit</a>
                    <form action="/posts/{{ post.id }}" method="POST" style="display:inline;">
                        <input type="hidden" name="_method" value="DELETE">
                        <button type="submit" onclick="return confirm('Are you sure?')">Delete</button>
                    </form>
                </td>
            </tr>
            {% endfor %}
        </tbody>
    </table>
</div>
{% endblock %}
```

---

## Key Learnings

1. **Clean Separation**: Controllers handle HTTP, Services handle business logic, Repositories handle data
2. **Validation First**: Always validate before processing
3. **Consistent Responses**: Use the response helpers for consistency
4. **Query Builder**: Leverage the fluent interface for readable queries
5. **Reusability**: Services and Repositories are reusable across different controllers

This pattern mirrors Laravel's structure and makes your Rust code maintainable and scalable!
