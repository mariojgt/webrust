use axum::{
    response::{IntoResponse, Response, Html},
    http::StatusCode,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Template error: {0}")]
    Template(#[from] tera::Error),

    #[error("Internal error: {0}")]
    Any(#[from] anyhow::Error),

    #[error("{0}")]
    Message(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            AppError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            AppError::Template(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            AppError::Any(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            AppError::Message(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
        };

        let html = format!(r#"
            <!DOCTYPE html>
            <html>
                <head>
                    <title>🚨 Application Error</title>
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
                        .solution {{ margin-top: 2rem; background: #2f855a; color: white; padding: 1.5rem; border-radius: 0.5rem; }}
                        .solution h3 {{ margin-top: 0; display: flex; align-items: center; gap: 0.5rem; }}
                    </style>
                </head>
                <body>
                    <div class="header">
                        <div style="max-width: 1200px; margin: 0 auto;">
                            <div style="display: flex; align-items: center; gap: 1rem; margin-bottom: 1rem;">
                                <span style="font-size: 2rem;">💣</span>
                                <h1>Application Error</h1>
                            </div>
                            <div style="font-family: monospace; font-size: 1.1rem; opacity: 0.9;">
                                {}
                            </div>
                        </div>
                    </div>
                    <div class="container">
                        <div class="card">
                            <div class="card-header">
                                <span style="font-weight: 600; color: #a0aec0;">Error Details</span>
                                <span class="badge">500 Internal Server Error</span>
                            </div>
                            <div class="card-body">
                                <h2>{}</h2>
                                <pre><code>{:?}</code></pre>

                                <div class="solution">
                                    <h3>💡 Possible Solution</h3>
                                    <p>Check your application logs for more details. Ensure your database connection is correct and all migrations have been run.</p>
                                </div>
                            </div>
                        </div>
                    </div>
                </body>
            </html>
        "#, message, message, self);

        (status, Html(html)).into_response()
    }
}
