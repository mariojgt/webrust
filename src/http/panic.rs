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

    let html = format!(r#"
        <!DOCTYPE html>
        <html>
            <head>
                <title>💥 Server Panic</title>
                <style>
                    body {{ font-family: system-ui, -apple-system, sans-serif; background: #1a202c; color: #e2e8f0; padding: 0; margin: 0; height: 100vh; display: flex; flex-direction: column; }}
                    .header {{ background: #e53e3e; color: white; padding: 2rem; }}
                    .container {{ max-width: 1200px; margin: 0 auto; width: 100%; padding: 2rem; flex: 1; }}
                    .card {{ background: #2d3748; border-radius: 0.75rem; box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1); border: 1px solid #4a5568; overflow: hidden; }}
                    .card-header {{ background: #232936; padding: 1.5rem; border-bottom: 1px solid #4a5568; display: flex; align-items: center; justify-content: space-between; }}
                    .card-body {{ padding: 2rem; }}
                    h1 {{ margin: 0; font-size: 1.8rem; font-weight: 600; }}
                    h2 {{ margin-top: 0; color: #fc8181; font-size: 1.2rem; }}
                    pre {{ background: #171923; padding: 1.5rem; border-radius: 0.5rem; overflow-x: auto; font-family: 'Menlo', 'Monaco', 'Courier New', monospace; font-size: 0.9rem; line-height: 1.6; border: 1px solid #2d3748; color: #a3bffa; margin: 0; }}
                    .stack-trace {{ margin-top: 2rem; }}
                    .badge {{ background: rgba(255, 255, 255, 0.2); padding: 0.25rem 0.75rem; border-radius: 9999px; font-size: 0.875rem; font-weight: 500; }}
                </style>
            </head>
            <body>
                <div class="header">
                    <div style="max-width: 1200px; margin: 0 auto;">
                        <div style="display: flex; align-items: center; gap: 1rem; margin-bottom: 1rem;">
                            <span style="font-size: 2rem;">💥</span>
                            <h1>Server Panic</h1>
                        </div>
                        <div style="font-family: monospace; font-size: 1.1rem; opacity: 0.9;">
                            {}
                        </div>
                    </div>
                </div>
                <div class="container">
                    <div class="card">
                        <div class="card-header">
                            <span style="font-weight: 600; color: #a0aec0;">Panic Details</span>
                            <span class="badge">500 Internal Server Error</span>
                        </div>
                        <div class="card-body">
                            <h2>Panic Message</h2>
                            <pre><code>{}</code></pre>
                            
                            <div style="margin-top: 2rem; color: #a0aec0; font-size: 0.9rem;">
                                Check the server logs for the full stack trace.
                            </div>
                        </div>
                    </div>
                </div>
            </body>
        </html>
    "#, msg, msg);

    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Html(html)
    ).into_response()
}
