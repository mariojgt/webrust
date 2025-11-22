// COMPLETE EXAMPLE: Using debug macros in a WebRust controller

use axum::{extract::State, response::Json};
use serde_json::json;
use crate::framework::AppState;

// Example struct
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CreatePostRequest {
    pub title: String,
    pub content: String,
    pub author_id: i64,
}

/// Example: Create a new post with debugging
pub async fn create_post(
    State(state): State<AppState>,
    Json(payload): Json<CreatePostRequest>,
) -> Result<Json<serde_json::Value>, String> {

    // 1. Debug incoming request
    debug!("create_post_request", &payload);

    // 2. Validate data
    if payload.title.is_empty() {
        dd!(payload);  // Stop here if invalid
    }

    // 3. Check database connection
    let db = match &state.db {
        Some(pool) => pool,
        None => dd!("Database not available"),  // Stop if no DB
    };

    // 4. Insert post
    let query = format!(
        "INSERT INTO posts (title, content, author_id) VALUES ('{}', '{}', {})",
        &payload.title, &payload.content, payload.author_id
    );

    debug!("insert_query", &query);

    // Simulate insertion (in real code, would use SQLx)
    let post_id = 42i64;
    dump!(post_id);  // Inspect ID but continue

    // 5. Return response
    let response = json!({
        "id": post_id,
        "title": &payload.title,
        "content": &payload.content,
        "author_id": payload.author_id,
    });

    debug!("create_post_response", &response);
    Ok(Json(response))
}

/// Example: List posts with filtering and debugging
pub async fn list_posts(
    State(state): State<AppState>,
    axum::Query(params): axum::Query<std::collections::HashMap<String, String>>,
) -> Result<Json<serde_json::Value>, String> {

    debug!("list_posts_params", &params);

    // Parse query parameters
    let limit = params
        .get("limit")
        .and_then(|l| l.parse::<i32>().ok())
        .unwrap_or(10);

    let offset = params
        .get("offset")
        .and_then(|o| o.parse::<i32>().ok())
        .unwrap_or(0);

    debug!("pagination", format!("limit={}, offset={}", limit, offset));

    // Validate pagination
    if limit < 1 || limit > 100 {
        dd!(limit);  // Stop if invalid limit
    }

    // Simulate fetching posts
    let posts = vec![
        json!({ "id": 1, "title": "First Post", "author_id": 1 }),
        json!({ "id": 2, "title": "Second Post", "author_id": 2 }),
    ];

    dump!(posts);  // Check posts before returning

    let response = json!({
        "total": posts.len(),
        "posts": posts,
    });

    debug!("list_posts_response", &response);
    Ok(Json(response))
}

/// Example: Update post with state inspection
pub async fn update_post(
    State(state): State<AppState>,
    axum::Path(id): axum::Path<i64>,
    Json(payload): Json<CreatePostRequest>,
) -> Result<Json<serde_json::Value>, String> {

    debug!("update_post_id", id);
    debug!("update_post_payload", &payload);

    // Check if DB available
    let _db = state.db.as_ref().ok_or("No database")?;

    // Simulate update
    let updated_post = json!({
        "id": id,
        "title": &payload.title,
        "content": &payload.content,
        "updated_at": chrono::Utc::now().to_rfc3339(),
    });

    dump!(updated_post);  // Inspect before return

    Ok(Json(updated_post))
}

/// Example: Delete post with conditional debugging
pub async fn delete_post(
    State(_state): State<AppState>,
    axum::Path(id): axum::Path<i64>,
) -> Result<Json<serde_json::Value>, String> {

    debug!("delete_post_id", id);

    // Validate ID
    if id <= 0 {
        dd!(id);  // Stop for invalid ID
    }

    // Check if exists (simulated)
    let exists = id % 2 == 0;  // Fake check

    if !exists {
        dd!("Post not found");
    }

    // Simulate deletion
    let result = json!({
        "deleted": true,
        "id": id,
    });

    dump!(result);
    Ok(Json(result))
}

/*

EXAMPLE OUTPUT:

When calling POST /posts with { title: "My Post", ... }:

🔧 [create_post_request]
CreatePostRequest { title: "My Post", content: "Hello", author_id: 1 }
📍 at: controllers/posts.rs:18

🔧 [insert_query]
"INSERT INTO posts (title, content, author_id) VALUES ('My Post', 'Hello', 1)"
📍 at: controllers/posts.rs:30

🔍 DEBUG:
42
📍 at: controllers/posts.rs:32

🔧 [create_post_response]
{
  "id": 42,
  "title": "My Post",
  "content": "Hello",
  "author_id": 1
}
📍 at: controllers/posts.rs:37

*/
