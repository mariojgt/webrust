use crate::services::console::Command;
use std::collections::HashMap;

pub mod tinker;
pub mod routes;
pub mod migrations;
pub mod make_package;

// Register your custom commands here
pub fn kernel() -> HashMap<String, Box<dyn Command>> {
    let mut commands: HashMap<String, Box<dyn Command>> = HashMap::new();

    // Package management
    commands.insert("make:package".to_string(), Box::new(make_package::MakePackageCommand));

    // Example:
    // commands.insert("example:command".to_string(), Box::new(ExampleCommand));

    commands
}
