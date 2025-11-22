use std::any::Any;
use axum::{
    response::{Response, IntoResponse, Html},
    http::StatusCode,
};
use crate::debug::DebugDump;

pub fn handle_panic(err: Box<dyn Any + Send + 'static>) -> Response {
    if let Some(dump) = err.downcast_ref::<DebugDump>() {
        let html = format!(r#"
            <!DOCTYPE html>
            <html>
                <head>
                    <title>🐛 Debug Dump</title>
                    <style>
                        body {{ font-family: system-ui, -apple-system, sans-serif; background: #1a202c; color: #e2e8f0; padding: 2rem; margin: 0; }}
                        .container {{ max-width: 1000px; margin: 0 auto; }}
                        .card {{ background: #2d3748; padding: 2rem; border-radius: 0.75rem; box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1); border: 1px solid #4a5568; }}
                        h1 {{ color: #fc8181; margin-top: 0; font-size: 1.5rem; display: flex; align-items: center; gap: 0.5rem; }}
                        pre {{ background: #171923; padding: 1.5rem; border-radius: 0.5rem; overflow-x: auto; font-family: 'Menlo', 'Monaco', 'Courier New', monospace; font-size: 0.9rem; line-height: 1.5; border: 1px solid #2d3748; color: #a3bffa; }}
                        .meta {{ color: #a0aec0; margin-bottom: 1.5rem; font-family: monospace; background: #232936; padding: 0.75rem; border-radius: 0.375rem; display: inline-block; }}
                        .label {{ color: #718096; margin-right: 0.5rem; }}
                    </style>
                </head>
                <body>
                    <div class="container">
                        <div class="card">
                            <h1>🐛 Debug Dump</h1>
                            <div class="meta">
                                <span class="label">📍 Location:</span> {}:{}
                            </div>
                            <pre><code>{}</code></pre>
                        </div>
                    </div>
                </body>
            </html>
        "#, dump.file, dump.line, dump.value);

        return Html(html).into_response();
    }

    // Default panic handling
    let msg = if let Some(s) = err.downcast_ref::<&str>() {
        *s
    } else if let Some(s) = err.downcast_ref::<String>() {
        &**s
    } else {
        "Unknown panic"
    };

    tracing::error!("Panic occurred: {}", msg);

    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Internal Server Error: {}", msg)
    ).into_response()
}
