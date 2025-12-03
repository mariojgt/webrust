use axum::{
    body::Body,
    http::{Request, StatusCode, header},
    response::Response,
    Router,
};
use tower::util::ServiceExt; // for `oneshot`
use bytes::Bytes;
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use crate::framework::{AppState, build_tera, build_database_manager};
use crate::cache::{Cache, MemoryCache};
use crate::routes::router;

pub struct TestClient {
    app: Router,
}

impl TestClient {
    /// Create a new test client with a default application state
    pub async fn new() -> Self {
        // Setup minimal state for testing
        // In a real scenario, you might want to use a test database
        let db_manager = build_database_manager().await;
        let tera = build_tera().unwrap_or_else(|_| tera::Tera::default());
        let cache = Cache::Memory(MemoryCache::new());

        let state = AppState::new(db_manager, tera, cache);
        let app = router(state).await;

        Self { app }
    }

    /// Create a new test client with a custom application state
    pub async fn with_state(state: AppState) -> Self {
        let app = router(state).await;
        Self { app }
    }

    /// Send a GET request
    pub async fn get(&self, uri: &str) -> TestResponse {
        let request = Request::builder()
            .uri(uri)
            .method("GET")
            .body(Body::empty())
            .unwrap();

        self.execute(request).await
    }

    /// Send a POST request
    pub async fn post<T: serde::Serialize>(&self, uri: &str, body: &T) -> TestResponse {
        let json = serde_json::to_vec(body).expect("Failed to serialize body");
        let request = Request::builder()
            .uri(uri)
            .method("POST")
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(json))
            .unwrap();

        self.execute(request).await
    }

    /// Send a PUT request
    pub async fn put<T: serde::Serialize>(&self, uri: &str, body: &T) -> TestResponse {
        let json = serde_json::to_vec(body).expect("Failed to serialize body");
        let request = Request::builder()
            .uri(uri)
            .method("PUT")
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(json))
            .unwrap();

        self.execute(request).await
    }

    /// Send a DELETE request
    pub async fn delete(&self, uri: &str) -> TestResponse {
        let request = Request::builder()
            .uri(uri)
            .method("DELETE")
            .body(Body::empty())
            .unwrap();

        self.execute(request).await
    }

    async fn execute(&self, request: Request<Body>) -> TestResponse {
        // We clone the router for each request because `oneshot` consumes it
        // This is cheap because Router is just an Arc wrapper
        let response = self.app.clone().oneshot(request).await.expect("Failed to execute request");

        let (parts, body) = response.into_parts();
        let bytes = axum::body::to_bytes(body, usize::MAX).await.expect("Failed to read response body");

        TestResponse {
            status: parts.status,
            headers: parts.headers,
            body: bytes,
        }
    }
}

pub struct TestResponse {
    pub status: StatusCode,
    pub headers: axum::http::HeaderMap,
    pub body: Bytes,
}

impl TestResponse {
    /// Assert that the response has a specific status code
    pub fn assert_status(&self, status: u16) -> &Self {
        assert_eq!(
            self.status.as_u16(),
            status,
            "Expected status {}, got {}",
            status,
            self.status.as_u16()
        );
        self
    }

    /// Assert that the response is OK (200)
    pub fn assert_ok(&self) -> &Self {
        self.assert_status(200)
    }

    /// Assert that the response is Not Found (404)
    pub fn assert_not_found(&self) -> &Self {
        self.assert_status(404)
    }

    /// Assert that the response contains the given text
    pub fn assert_see(&self, text: &str) -> &Self {
        let body_str = std::str::from_utf8(&self.body).expect("Response body is not valid UTF-8");
        assert!(
            body_str.contains(text),
            "Expected response to contain '{}', but it didn't.\nBody: {}",
            text,
            body_str
        );
        self
    }

    /// Assert that the response does not contain the given text
    pub fn assert_dont_see(&self, text: &str) -> &Self {
        let body_str = std::str::from_utf8(&self.body).expect("Response body is not valid UTF-8");
        assert!(
            !body_str.contains(text),
            "Expected response NOT to contain '{}', but it did.\nBody: {}",
            text,
            body_str
        );
        self
    }

    /// Assert that the response is a redirect
    pub fn assert_redirect(&self, location: &str) -> &Self {
        assert!(
            self.status.is_redirection(),
            "Expected redirect status (3xx), got {}",
            self.status
        );

        let location_header = self.headers.get(header::LOCATION)
            .expect("Response is a redirect but missing Location header")
            .to_str()
            .expect("Location header is not valid UTF-8");

        assert_eq!(
            location_header,
            location,
            "Expected redirect to '{}', got '{}'",
            location,
            location_header
        );
        self
    }

    /// Deserialize the JSON response body
    pub fn json<T: DeserializeOwned>(&self) -> T {
        serde_json::from_slice(&self.body).expect("Failed to deserialize JSON response")
    }
}
