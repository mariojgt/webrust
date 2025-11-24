/// Enhanced error page handler inspired by Laravel's Ignition
/// Provides beautiful, informative error displays with stack traces and context

use axum::{
    response::{IntoResponse, Response, Html},
    http::StatusCode,
};
use serde_json::json;
use std::collections::HashMap;

/// Error context that captures useful debugging information
#[derive(Debug, Clone)]
pub struct ErrorContext {
    pub title: String,
    pub message: String,
    pub stack_trace: Vec<StackFrame>,
    pub context: HashMap<String, String>,
    pub solution: String,
    pub file: String,
    pub line: u32,
    pub is_debug: bool,
}

/// Individual frame in a stack trace
#[derive(Debug, Clone)]
pub struct StackFrame {
    pub function: String,
    pub file: String,
    pub line: u32,
    pub code: Option<String>,
}

impl ErrorContext {
    /// Create a new error context
    pub fn new(title: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            message: message.into(),
            stack_trace: vec![],
            context: HashMap::new(),
            solution: "Check application logs for more details.".to_string(),
            file: "unknown".to_string(),
            line: 0,
            is_debug: cfg!(debug_assertions),
        }
    }

    /// Add a stack frame
    pub fn with_frame(mut self, function: String, file: String, line: u32) -> Self {
        self.stack_trace.push(StackFrame {
            function,
            file,
            line,
            code: None,
        });
        self
    }

    /// Add context information (e.g., request data, variables)
    pub fn with_context(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.context.insert(key.into(), value.into());
        self
    }

    /// Add solution hint
    pub fn with_solution(mut self, solution: impl Into<String>) -> Self {
        self.solution = solution.into();
        self
    }

    /// Set file and line information
    pub fn at(mut self, file: impl Into<String>, line: u32) -> Self {
        self.file = file.into();
        self.line = line;
        self
    }

    /// Generate HTML error page
    pub fn to_html(&self) -> String {
        let status_color = self.get_status_color();
        let status_code = 500;
        
        let stack_html = if self.is_debug {
            self.render_stack_trace()
        } else {
            String::new()
        };

        let context_html = if self.is_debug && !self.context.is_empty() {
            self.render_context()
        } else {
            String::new()
        };

        format!(
            r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>🚨 {}</title>
    <style>
        * {{ margin: 0; padding: 0; box-sizing: border-box; }}
        body {{
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
            background: #0f172a;
            color: #e2e8f0;
            line-height: 1.6;
        }}
        
        .error-header {{
            background: linear-gradient(135deg, {} 0%, {} 100%);
            padding: 3rem 2rem;
            border-bottom: 1px solid rgba(255, 255, 255, 0.1);
        }}

        .error-header-content {{
            max-width: 1200px;
            margin: 0 auto;
        }}

        .error-icon {{
            font-size: 3rem;
            margin-bottom: 1rem;
        }}

        .error-header h1 {{
            font-size: 2rem;
            font-weight: 700;
            margin-bottom: 0.5rem;
        }}

        .error-header p {{
            font-size: 1.1rem;
            opacity: 0.9;
            margin-bottom: 1rem;
        }}

        .error-meta {{
            display: flex;
            gap: 1rem;
            flex-wrap: wrap;
        }}

        .meta-badge {{
            background: rgba(0, 0, 0, 0.3);
            padding: 0.5rem 1rem;
            border-radius: 0.5rem;
            font-family: 'Monaco', 'Menlo', monospace;
            font-size: 0.9rem;
        }}

        .error-container {{
            max-width: 1200px;
            margin: 0 auto;
            padding: 2rem;
        }}

        .error-section {{
            background: #1e293b;
            border: 1px solid #334155;
            border-radius: 0.75rem;
            margin-bottom: 2rem;
            overflow: hidden;
        }}

        .error-section-title {{
            background: #0f172a;
            padding: 1.5rem;
            border-bottom: 1px solid #334155;
            font-weight: 600;
            font-size: 1.1rem;
            display: flex;
            align-items: center;
            gap: 0.75rem;
        }}

        .error-section-content {{
            padding: 1.5rem;
        }}

        .message-box {{
            background: #374151;
            padding: 1.5rem;
            border-radius: 0.5rem;
            border-left: 4px solid {};
            font-family: 'Monaco', 'Menlo', monospace;
            font-size: 0.95rem;
            line-height: 1.6;
            word-break: break-word;
        }}

        .stack-trace {{
            font-family: 'Monaco', 'Menlo', monospace;
            font-size: 0.85rem;
            line-height: 1.8;
        }}

        .stack-frame {{
            padding: 1rem;
            background: #0f172a;
            border-bottom: 1px solid #334155;
            display: grid;
            grid-template-columns: auto 1fr;
            gap: 1rem;
            align-items: start;
        }}

        .stack-frame:last-child {{
            border-bottom: none;
        }}

        .frame-number {{
            background: {};
            color: white;
            width: 2.5rem;
            height: 2.5rem;
            border-radius: 50%;
            display: flex;
            align-items: center;
            justify-content: center;
            font-weight: 600;
            flex-shrink: 0;
        }}

        .frame-info {{
            flex: 1;
        }}

        .frame-function {{
            color: #7dd3fc;
            font-weight: 600;
            margin-bottom: 0.25rem;
        }}

        .frame-location {{
            color: #cbd5e1;
            font-size: 0.85rem;
            opacity: 0.8;
        }}

        .context-grid {{
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
            gap: 1.5rem;
        }}

        .context-item {{
            background: #0f172a;
            padding: 1rem;
            border-radius: 0.5rem;
            border: 1px solid #334155;
        }}

        .context-key {{
            color: #7dd3fc;
            font-weight: 600;
            font-size: 0.9rem;
            margin-bottom: 0.5rem;
        }}

        .context-value {{
            background: #1e293b;
            padding: 0.75rem;
            border-radius: 0.375rem;
            font-family: 'Monaco', 'Menlo', monospace;
            font-size: 0.85rem;
            word-break: break-all;
            color: #a3e635;
        }}

        .solution-box {{
            background: linear-gradient(135deg, #065f46 0%, #047857 100%);
            border-left: 4px solid #10b981;
            padding: 1.5rem;
            border-radius: 0.5rem;
            margin-top: 1.5rem;
        }}

        .solution-box h3 {{
            display: flex;
            align-items: center;
            gap: 0.5rem;
            margin-bottom: 0.5rem;
            font-size: 1rem;
        }}

        .solution-box p {{
            opacity: 0.95;
            line-height: 1.6;
        }}

        .debug-badge {{
            background: #d97706;
            color: white;
            padding: 0.375rem 0.75rem;
            border-radius: 9999px;
            font-size: 0.75rem;
            font-weight: 600;
        }}

        .production-notice {{
            background: #1e3a8a;
            border: 1px solid #1e40af;
            color: #93c5fd;
            padding: 1.5rem;
            border-radius: 0.5rem;
            margin-bottom: 2rem;
            display: flex;
            gap: 1rem;
        }}

        .production-notice-icon {{
            font-size: 1.5rem;
            flex-shrink: 0;
        }}

        @media (max-width: 768px) {{
            .error-header {{
                padding: 2rem 1rem;
            }}

            .error-header h1 {{
                font-size: 1.5rem;
            }}

            .error-container {{
                padding: 1rem;
            }}

            .context-grid {{
                grid-template-columns: 1fr;
            }}
        }}
    </style>
</head>
<body>
    <div class="error-header">
        <div class="error-header-content">
            <div class="error-icon">💥</div>
            <h1>{}</h1>
            <p>{}</p>
            <div class="error-meta">
                <div class="meta-badge">Status: {}</div>
                <div class="meta-badge">File: {}</div>
                <div class="meta-badge">Line: {}</div>
                {}
            </div>
        </div>
    </div>

    <div class="error-container">
        {}

        <div class="error-section">
            <div class="error-section-title">
                📋 Error Message
            </div>
            <div class="error-section-content">
                <div class="message-box">{}</div>
            </div>
        </div>

        {}

        {}

        <div class="error-section">
            <div class="error-section-title">
                💡 Solution
            </div>
            <div class="error-section-content">
                <div class="solution-box">
                    <h3>💚 How to fix this</h3>
                    <p>{}</p>
                </div>
            </div>
        </div>
    </div>
</body>
</html>
            "#,
            self.title,
            status_color.0,
            status_color.1,
            status_color.0,
            status_color.0,
            self.title,
            self.message,
            500,
            self.file,
            self.line,
            if self.is_debug {
                r#"<div class="meta-badge" style="background: #d97706;"><span class="debug-badge">DEBUG MODE</span></div>"#
            } else {
                ""
            },
            if !self.is_debug {
                r#"<div class="production-notice">
                    <div class="production-notice-icon">ℹ️</div>
                    <div>
                        <strong>Production Mode</strong>
                        <p>Detailed error information is hidden for security. Check your server logs for more details.</p>
                    </div>
                </div>"#
            } else {
                ""
            },
            self.message,
            stack_html,
            context_html,
            self.solution,
        )
    }

    fn render_stack_trace(&self) -> String {
        if self.stack_trace.is_empty() {
            return String::new();
        }

        let mut html = String::from(
            r#"
        <div class="error-section">
            <div class="error-section-title">
                📍 Stack Trace
            </div>
            <div class="error-section-content">
                <div class="stack-trace">
            "#
        );

        for (index, frame) in self.stack_trace.iter().enumerate() {
            html.push_str(&format!(
                r#"
                <div class="stack-frame">
                    <div class="frame-number">{}</div>
                    <div class="frame-info">
                        <div class="frame-function">{}</div>
                        <div class="frame-location">{}:{}</div>
                    </div>
                </div>
                "#,
                index + 1,
                frame.function,
                frame.file,
                frame.line
            ));
        }

        html.push_str(
            r#"
                </div>
            </div>
        </div>
            "#
        );

        html
    }

    fn render_context(&self) -> String {
        if self.context.is_empty() {
            return String::new();
        }

        let mut html = String::from(
            r#"
        <div class="error-section">
            <div class="error-section-title">
                🔍 Context
            </div>
            <div class="error-section-content">
                <div class="context-grid">
            "#
        );

        for (key, value) in &self.context {
            html.push_str(&format!(
                r#"
                <div class="context-item">
                    <div class="context-key">{}</div>
                    <div class="context-value">{}</div>
                </div>
                "#,
                html_escape(key),
                html_escape(value)
            ));
        }

        html.push_str(
            r#"
                </div>
            </div>
        </div>
            "#
        );

        html
    }

    fn get_status_color(&self) -> (&'static str, &'static str) {
        // Red gradient for errors
        ("#dc2626", "#991b1b")
    }
}

/// Implement IntoResponse for ErrorContext
impl IntoResponse for ErrorContext {
    fn into_response(self) -> Response {
        let html = self.to_html();
        (StatusCode::INTERNAL_SERVER_ERROR, Html(html)).into_response()
    }
}

/// Simple HTML escape for context values
fn html_escape(s: &str) -> String {
    s.replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
        .replace("\"", "&quot;")
        .replace("'", "&#39;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_context_creation() {
        let ctx = ErrorContext::new("Database Error", "Connection failed")
            .with_solution("Check your database configuration")
            .at("src/database.rs", 42);

        assert_eq!(ctx.title, "Database Error");
        assert_eq!(ctx.message, "Connection failed");
        assert_eq!(ctx.line, 42);
    }

    #[test]
    fn test_error_context_with_stack_trace() {
        let ctx = ErrorContext::new("Runtime Error", "Invalid value")
            .with_frame("main".to_string(), "src/main.rs".to_string(), 10)
            .with_frame("process".to_string(), "src/lib.rs".to_string(), 25);

        assert_eq!(ctx.stack_trace.len(), 2);
        assert_eq!(ctx.stack_trace[0].function, "main");
    }

    #[test]
    fn test_error_context_with_context() {
        let ctx = ErrorContext::new("Validation Error", "Invalid input")
            .with_context("user_id", "123")
            .with_context("email", "invalid@email");

        assert_eq!(ctx.context.len(), 2);
        assert_eq!(ctx.context.get("user_id").unwrap(), "123");
    }

    #[test]
    fn test_html_escape() {
        assert_eq!(html_escape("<script>alert('xss')</script>"), "&lt;script&gt;alert(&#39;xss&#39;)&lt;/script&gt;");
    }
}
