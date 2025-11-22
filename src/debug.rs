/// Struct to hold debug information for panic handling
#[derive(Debug)]
pub struct DebugDump {
    pub value: String,
    pub file: &'static str,
    pub line: u32,
}

/// Laravel-style `dd()` function for debugging
/// Dumps the given value and panics with a DebugDump payload
/// This allows the web server to catch it and render a nice HTML page
#[macro_export]
macro_rules! dd {
    ($($val:expr),+ $(,)?) => {{
        let mut output = String::new();
        $(
            output.push_str(&format!("{:#?}\n", $val));
        )+

        let dump = $crate::debug::DebugDump {
            value: output,
            file: file!(),
            line: line!(),
        };
        std::panic::panic_any(dump);
    }};
}

/// Laravel-style `dump()` function for debugging
/// Dumps the given value in a pretty format but continues execution
///
/// # Examples
///
/// ```
/// dump!("Debug message");
/// let x = dump!(calculate_value());
/// ```
#[macro_export]
macro_rules! dump {
    ($val:expr) => {{
        eprintln!("\n🔍 DEBUG:");
        eprintln!("{:#?}", $val);
        eprintln!("📍 at: {}:{}\n", file!(), line!());
        $val
    }};
    ($($val:expr),+ $(,)?) => {{
        eprintln!("\n🔍 DEBUG:");
        $(
            eprintln!("{:#?}", $val);
        )+
        eprintln!("📍 at: {}:{}\n", file!(), line!());
    }};
}

/// Print debug info with a custom label
///
/// # Examples
///
/// ```
/// debug!("user_data", user);
/// debug!("response", response);
/// ```
#[macro_export]
macro_rules! debug {
    ($label:expr, $val:expr) => {{
        eprintln!("\n🔧 [{}]", $label);
        eprintln!("{:#?}", $val);
        eprintln!("📍 at: {}:{}\n", file!(), line!());
    }};
}
