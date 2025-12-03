/// Error logging system with file rotation and structured logging
/// Like Laravel's error logging system

use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use chrono::Local;
use serde_json::json;

/// Error logger with file rotation and structured logging
pub struct ErrorLogger {
    log_path: String,
    max_size: u64,  // Max file size in bytes (default: 10MB)
    is_debug: bool,
}

impl ErrorLogger {
    /// Create a new error logger
    pub fn new(log_path: impl Into<String>) -> Self {
        Self {
            log_path: log_path.into(),
            max_size: 10 * 1024 * 1024,  // 10MB default
            is_debug: cfg!(debug_assertions),
        }
    }

    /// Set max file size before rotation
    pub fn with_max_size(mut self, bytes: u64) -> Self {
        self.max_size = bytes;
        self
    }

    /// Log an error with context
    pub fn log_error(&self, title: &str, message: &str, file: &str, line: u32, context: Option<serde_json::Value>) {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S%.3f");

        let log_entry = json!({
            "timestamp": timestamp.to_string(),
            "level": "ERROR",
            "title": title,
            "message": message,
            "file": file,
            "line": line,
            "context": context,
            "is_debug": self.is_debug,
        });

        let formatted = format!(
            "[{}] {} | {}: {} at {}:{}\n",
            timestamp, title, "ERROR", message, file, line
        );

        // Write to stderr
        eprint!("{}", formatted);

        // Write to file if debug mode
        if self.is_debug {
            self.write_to_file(&formatted);
        }
    }

    /// Log a warning
    pub fn log_warning(&self, message: &str, context: Option<serde_json::Value>) {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S%.3f");

        let formatted = format!(
            "[{}] WARNING: {}\n",
            timestamp, message
        );

        eprintln!("{}", formatted);

        if self.is_debug {
            self.write_to_file(&formatted);
        }
    }

    /// Log debug information
    pub fn log_debug(&self, title: &str, data: &str) {
        if !self.is_debug {
            return;
        }

        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S%.3f");

        let formatted = format!(
            "[{}] DEBUG [{}]: {}\n",
            timestamp, title, data
        );

        eprintln!("{}", formatted);
        self.write_to_file(&formatted);
    }

    /// Write to log file with rotation
    fn write_to_file(&self, content: &str) {
        let path = Path::new(&self.log_path);

        // Create parent directories if they don't exist
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }

        // Check if file needs rotation
        if let Ok(metadata) = std::fs::metadata(&self.log_path) {
            if metadata.len() > self.max_size {
                self.rotate_log_file();
            }
        }

        // Append to file
        if let Ok(mut file) = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.log_path)
        {
            let _ = file.write_all(content.as_bytes());
        }
    }

    /// Rotate log file (archive old file with timestamp)
    fn rotate_log_file(&self) {
        let path = Path::new(&self.log_path);
        if let Some(extension) = path.extension() {
            let stem = path.file_stem().unwrap().to_str().unwrap();
            let parent = path.parent().unwrap();

            let timestamp = Local::now().format("%Y%m%d_%H%M%S");
            let archived_name = format!(
                "{}_{}.{}",
                stem,
                timestamp,
                extension.to_str().unwrap()
            );
            let archived_path = parent.join(archived_name);

            // Archive the current log
            let _ = std::fs::rename(&self.log_path, archived_path);
        }
    }

    /// Get recent log entries (for dashboard/monitoring)
    pub fn get_recent_errors(&self, limit: usize) -> Vec<String> {
        let path = Path::new(&self.log_path);
        if !path.exists() {
            return vec![];
        }

        std::fs::read_to_string(path)
            .unwrap_or_default()
            .lines()
            .filter(|line| line.contains("ERROR"))
            .rev()
            .take(limit)
            .map(|s| s.to_string())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_logger_creation() {
        let logger = ErrorLogger::new("test.log");
        assert_eq!(logger.log_path, "test.log");
        assert_eq!(logger.max_size, 10 * 1024 * 1024);
    }

    #[test]
    fn test_error_logger_with_max_size() {
        let logger = ErrorLogger::new("test.log")
            .with_max_size(5 * 1024 * 1024);
        assert_eq!(logger.max_size, 5 * 1024 * 1024);
    }
}
