use crate::services::console::Command;
use std::collections::HashMap;

// Register your custom commands here
pub fn kernel() -> HashMap<String, Box<dyn Command>> {
    let mut commands: HashMap<String, Box<dyn Command>> = HashMap::new();

    // Example:
    // commands.insert("example:command".to_string(), Box::new(ExampleCommand));

    commands
}
