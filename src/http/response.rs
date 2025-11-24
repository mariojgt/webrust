/// Enhanced HTTP response helpers inspired by Laravel
/// Provides convenient methods for common response patterns

use axum::{
    response::{IntoResponse, Response},
    http::{StatusCode, HeaderMap, header},
    body::Body,
};
use serde_json::{json, Value};

/// Success response with data
pub fn success<T: serde::Serialize>(data: T) -> Response {
    (
        StatusCode::OK,
        axum::Json(json!({
            "success": true,
            "data": data
        }))
    ).into_response()
}

/// Success response with message
pub fn success_message(message: &str) -> Response {
    (
        StatusCode::OK,
        axum::Json(json!({
            "success": true,
            "message": message
        }))
    ).into_response()
}

/// Created response (201)
pub fn created<T: serde::Serialize>(data: T) -> Response {
    (
        StatusCode::CREATED,
        axum::Json(json!({
            "success": true,
            "data": data,
            "message": "Resource created successfully"
        }))
    ).into_response()
}

/// Accepted response (202)
pub fn accepted<T: serde::Serialize>(data: T) -> Response {
    (
        StatusCode::ACCEPTED,
        axum::Json(json!({
            "success": true,
            "data": data,
            "message": "Request accepted for processing"
        }))
    ).into_response()
}

/// No content response (204)
pub fn no_content() -> Response {
    StatusCode::NO_CONTENT.into_response()
}

/// Redirect response
pub fn redirect(location: &str) -> Response {
    let mut headers = HeaderMap::new();
    headers.insert(header::LOCATION, location.parse().unwrap());
    (StatusCode::FOUND, headers).into_response()
}

/// Error response with status code
pub fn error(status: StatusCode, message: &str) -> Response {
    (
        status,
        axum::Json(json!({
            "success": false,
            "error": message
        }))
    ).into_response()
}

/// Bad request response (400)
pub fn bad_request(message: &str) -> Response {
    error(StatusCode::BAD_REQUEST, message)
}

/// Unauthorized response (401)
pub fn unauthorized(message: &str) -> Response {
    error(StatusCode::UNAUTHORIZED, message)
}

/// Forbidden response (403)
pub fn forbidden(message: &str) -> Response {
    error(StatusCode::FORBIDDEN, message)
}

/// Not found response (404)
pub fn not_found_response(message: &str) -> Response {
    error(StatusCode::NOT_FOUND, message)
}

/// Unprocessable entity response (422) - for validation errors
pub fn unprocessable_entity(errors: Value) -> Response {
    (
        StatusCode::UNPROCESSABLE_ENTITY,
        axum::Json(json!({
            "success": false,
            "message": "Validation failed",
            "errors": errors
        }))
    ).into_response()
}

/// Too many requests response (429)
pub fn too_many_requests(message: &str) -> Response {
    error(StatusCode::TOO_MANY_REQUESTS, message)
}

/// Server error response (500)
pub fn server_error(message: &str) -> Response {
    error(StatusCode::INTERNAL_SERVER_ERROR, message)
}

/// Paginated response
pub fn paginated<T: serde::Serialize>(
    data: Vec<T>,
    current_page: i64,
    per_page: i64,
    total: i64,
) -> Response {
    let total_pages = (total as f64 / per_page as f64).ceil() as i64;

    (
        StatusCode::OK,
        axum::Json(json!({
            "success": true,
            "data": data,
            "pagination": {
                "current_page": current_page,
                "per_page": per_page,
                "total": total,
                "total_pages": total_pages,
                "has_more": current_page < total_pages
            }
        }))
    ).into_response()
}

/// Response with custom headers
pub fn with_headers(response: Response, headers: HeaderMap) -> Response {
    let (mut parts, body) = response.into_parts();
    parts.headers.extend(headers);
    Response::from_parts(parts, body)
}
