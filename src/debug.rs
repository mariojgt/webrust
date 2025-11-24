/// Struct to hold debug information for panic handling
#[derive(Debug)]
pub struct DebugDump {
    pub value: String,
    pub file: &'static str,
    pub line: u32,
}

/// Struct to hold enhanced debug dump information (alias for compatibility)
pub type EnhancedDebugDump = DebugDump;

/// Enhanced `dd!()` - Dump and Die with beautiful formatting
/// 
/// # Examples
/// ```
/// let user = User { id: 1, name: "John" };
/// dd!(user);  // Pretty prints and panics
/// 
/// // Multiple values
/// dd!(user, post, comment);
/// ```
#[macro_export]
macro_rules! dd {
    ($($val:expr),+ $(,)?) => {{
        let mut output = String::new();
        output.push_str("\n");
        output.push_str("╔════════════════════════════════════════════════════════════╗\n");
        output.push_str("║                      🔴 DEBUG DUMP                        ║\n");
        output.push_str("╚════════════════════════════════════════════════════════════╝\n");
        $(
            output.push_str(&format!("{:#?}\n", $val));
        )+
        output.push_str(&format!("\n📍 Location: {}:{}\n", file!(), line!()));
        output.push_str("╔════════════════════════════════════════════════════════════╗\n\n");

        let dump = $crate::debug::DebugDump {
            value: output,
            file: file!(),
            line: line!(),
        };
        std::panic::panic_any(dump);
    }};
}

/// Enhanced `dump!()` - Dump and Continue with pretty output
/// Returns the value for chaining
///
/// # Examples
/// ```
/// let result = dump!(calculate_value());
/// let filtered = dump!(items.iter().filter(|x| x.id > 5).collect::<Vec<_>>());
/// ```
#[macro_export]
macro_rules! dump {
    ($val:expr) => {{
        eprintln!("\n┌──────────────────────────────────────────────┐");
        eprintln!("│ 🔍 DEBUG INFO                                │");
        eprintln!("├──────────────────────────────────────────────┤");
        eprintln!("│ {:#?}", $val);
        eprintln!("├──────────────────────────────────────────────┤");
        eprintln!("│ 📍 {}:{}", file!(), line!());
        eprintln!("└──────────────────────────────────────────────┘\n");
        $val
    }};
    ($($val:expr),+ $(,)?) => {{
        eprintln!("\n┌──────────────────────────────────────────────┐");
        eprintln!("│ 🔍 MULTI-VALUE DEBUG                         │");
        eprintln!("├──────────────────────────────────────────────┤");
        $(
            eprintln!("│ {:#?}", $val);
        )+
        eprintln!("├──────────────────────────────────────────────┤");
        eprintln!("│ 📍 {}:{}", file!(), line!());
        eprintln!("└──────────────────────────────────────────────┘\n");
    }};
}

/// Enhanced `debug!()` - Labeled debug with context
/// Better for breakpoint-style debugging in complex flows
///
/// # Examples
/// ```
/// debug!("user_data", user);
/// debug!("processing", "Stage 1 complete");
/// debug_if!(should_debug, "complex_value", obj);
/// ```
#[macro_export]
macro_rules! debug {
    ($label:expr, $val:expr) => {{
        eprintln!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        eprintln!("🧠 [{}]", $label);
        eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        eprintln!("{:#?}", $val);
        eprintln!("📍 at {}:{}", file!(), line!());
        eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    }};
}

/// Conditional debug - only dump if condition is true
#[macro_export]
macro_rules! debug_if {
    ($condition:expr, $label:expr, $val:expr) => {{
        if $condition {
            debug!($label, $val);
        }
    }};
}

/// Performance timer - measure execution time of a block
/// 
/// # Examples
/// ```
/// timer!("database_query", {
///     let results = db.query().await?;
///     results
/// });
/// ```
#[macro_export]
macro_rules! timer {
    ($label:expr, $block:block) => {{
        let start = std::time::Instant::now();
        let result = $block;
        let elapsed = start.elapsed();
        eprintln!("⏱️  [{}] took {:?}", $label, elapsed);
        result
    }};
}

/// Quick benchmark - time multiple iterations
#[macro_export]
macro_rules! benchmark {
    ($label:expr, $iterations:expr, $block:block) => {{
        let start = std::time::Instant::now();
        for _ in 0..$iterations {
            $block;
        }
        let elapsed = start.elapsed();
        let avg = elapsed / $iterations as u32;
        eprintln!("📊 [{}] {} iterations: {:?} (avg: {:?})", $label, $iterations, elapsed, avg);
    }};
}

/// Colored output for different log levels
#[macro_export]
macro_rules! log_success {
    ($msg:expr) => {{
        eprintln!("✅ {}", $msg);
    }};
}

#[macro_export]
macro_rules! log_error {
    ($msg:expr) => {{
        eprintln!("❌ {}", $msg);
    }};
}

#[macro_export]
macro_rules! log_warning {
    ($msg:expr) => {{
        eprintln!("⚠️  {}", $msg);
    }};
}

#[macro_export]
macro_rules! log_info {
    ($msg:expr) => {{
        eprintln!("ℹ️  {}", $msg);
    }};
}

/// Assert with better error messages
/// 
/// # Examples
/// ```
/// assert_debug!(user.id == 5, "User ID should be 5, got: {:?}", user.id);
/// ```
#[macro_export]
macro_rules! assert_debug {
    ($condition:expr, $($msg:tt)*) => {{
        if !$condition {
            eprintln!("❌ ASSERTION FAILED");
            eprintln!("   {}", format!($($msg)*));
            eprintln!("   at: {}:{}", file!(), line!());
            panic!("Assertion failed");
        }
    }};
}

/// Memory usage snapshot
#[macro_export]
macro_rules! memory_snapshot {
    ($label:expr) => {{
        #[cfg(target_os = "macos")]
        {
            eprintln!("💾 [{}] Memory snapshot at {}:{}", $label, file!(), line!());
            eprintln!("   (Run with Instruments for detailed memory analysis)");
        }
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_debug_dump_creation() {
        let dump = DebugDump {
            value: "test value".to_string(),
            file: "test.rs",
            line: 42,
        };
        
        assert_eq!(dump.file, "test.rs");
        assert_eq!(dump.line, 42);
    }
}
