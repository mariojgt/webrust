use axum::{
    routing::{get, post, put, delete, patch},
    Router, handler::Handler,
};

use crate::framework::AppState;

/// RouteGroup provides a fluent interface for defining routes similar to Laravel
pub struct RouteGroup {
    prefix: String,
    routes: Router<AppState>,
}

impl RouteGroup {
    pub fn new(prefix: &str) -> Self {
        Self {
            prefix: prefix.to_string(),
            routes: Router::new(),
        }
    }

    /// Add a GET route
    pub fn get<H, T>(self, path: &str, handler: H) -> Self
    where
        H: Handler<T, AppState> + Clone + Send + Sync + 'static,
        T: 'static,
    {
        Self {
            routes: self.routes.route(
                &format!("{}{}", self.prefix, path),
                get(handler),
            ),
            ..self
        }
    }

    /// Add a POST route
    pub fn post<H, T>(self, path: &str, handler: H) -> Self
    where
        H: Handler<T, AppState> + Clone + Send + Sync + 'static,
        T: 'static,
    {
        Self {
            routes: self.routes.route(
                &format!("{}{}", self.prefix, path),
                post(handler),
            ),
            ..self
        }
    }

    /// Add a PUT route
    #[allow(dead_code)]
    pub fn put<H, T>(self, path: &str, handler: H) -> Self
    where
        H: Handler<T, AppState> + Clone + Send + Sync + 'static,
        T: 'static,
    {
        Self {
            routes: self.routes.route(
                &format!("{}{}", self.prefix, path),
                put(handler),
            ),
            ..self
        }
    }

    /// Add a DELETE route
    #[allow(dead_code)]
    pub fn delete<H, T>(self, path: &str, handler: H) -> Self
    where
        H: Handler<T, AppState> + Clone + Send + Sync + 'static,
        T: 'static,
    {
        Self {
            routes: self.routes.route(
                &format!("{}{}", self.prefix, path),
                delete(handler),
            ),
            ..self
        }
    }

    /// Add a PATCH route
    #[allow(dead_code)]
    pub fn patch<H, T>(self, path: &str, handler: H) -> Self
    where
        H: Handler<T, AppState> + Clone + Send + Sync + 'static,
        T: 'static,
    {
        Self {
            routes: self.routes.route(
                &format!("{}{}", self.prefix, path),
                patch(handler),
            ),
            ..self
        }
    }

    /// Get the built router
    pub fn build(self) -> Router<AppState> {
        self.routes
    }
}

/// Convenience function to create a new route group (Laravel-inspired)
#[allow(dead_code)]
pub fn group(prefix: &str) -> RouteGroup {
    RouteGroup::new(prefix)
}

/// Convenience function for web routes (no prefix by default)
pub fn web() -> RouteGroup {
    RouteGroup::new("")
}

/// Convenience function for API routes (with /api prefix)
pub fn api() -> RouteGroup {
    RouteGroup::new("/api")
}
