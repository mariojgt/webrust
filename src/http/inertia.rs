use axum::{
    async_trait,
    extract::{FromRequestParts, State},
    http::{header::HeaderValue, request::Parts, StatusCode},
    response::{Html, IntoResponse, Response, Json},
};
use serde::Serialize;
use serde_json::{json, Value};
use crate::framework::AppState;

#[derive(Clone)]
pub struct Inertia {
    is_inertia: bool,
    version: Option<String>,
    url: String,
    state: AppState,
    shared_props: Value,
}

#[derive(Serialize)]
struct Page {
    component: String,
    props: Value,
    url: String,
    version: Option<String>,
}

// Wrapper for shared props in extensions
#[derive(Clone)]
pub struct SharedInertiaProps(pub Value);

#[async_trait]
impl FromRequestParts<AppState> for Inertia {
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        let is_inertia = parts.headers.contains_key("x-inertia");
        let version = parts.headers.get("x-inertia-version")
            .and_then(|v| v.to_str().ok())
            .map(|v| v.to_string());
        let url = parts.uri.to_string();

        let shared_props = parts.extensions.get::<SharedInertiaProps>()
            .map(|p| p.0.clone())
            .unwrap_or(json!({}));

        Ok(Inertia {
            is_inertia,
            version,
            url,
            state: state.clone(),
            shared_props,
        })
    }
}

impl Inertia {
    /// Render an Inertia component
    pub fn render(self, component: &str, props: Value) -> Response {
        let mut final_props = self.shared_props.clone();

        // Merge props (simple merge for top-level keys)
        if let (Some(shared), Some(new)) = (final_props.as_object_mut(), props.as_object()) {
            for (k, v) in new {
                shared.insert(k.clone(), v.clone());
            }
        } else if props.is_object() {
             // If shared was empty/null but props is object, use props
             final_props = props;
        }

        let page = Page {
            component: component.to_string(),
            props: final_props,
            url: self.url.clone(),
            version: self.version.clone(),
        };

        if self.is_inertia {
            let mut response = Json(page).into_response();
            response.headers_mut().insert("X-Inertia", HeaderValue::from_static("true"));
            response.headers_mut().insert("Vary", HeaderValue::from_static("Accept"));
            response
        } else {
            // Render root template
            let page_json = serde_json::to_string(&page).unwrap();

            let mut ctx = tera::Context::new();

            // Construct the Inertia app div
            // We use single quotes for the attribute to avoid conflict with JSON double quotes
            // and escape any single quotes in the JSON itself.
            let escaped_json = page_json.replace("'", "&#39;");
            let inertia_body = format!(r#"<div id="app" data-page='{}'></div>"#, escaped_json);

            ctx.insert("inertia_body", &inertia_body);
            ctx.insert("inertia_head", ""); // Placeholder for future head injection

            let body = self.state.templates.render("root.rune.html", &ctx)
                .unwrap_or_else(|e| format!(
                    "Inertia Error: Could not render 'root.rune.html'. \
                    Make sure it exists in templates/. Error: {}", e
                ));

            Html(body).into_response()
        }
    }

    /// Create an external redirect (for Inertia to handle)
    pub fn location(url: &str) -> Response {
        let mut response = axum::response::Redirect::to(url).into_response();
        response.status_mut().clone_from(&StatusCode::CONFLICT);
        response.headers_mut().insert("X-Inertia-Location", HeaderValue::from_str(url).unwrap());
        response
    }
}
