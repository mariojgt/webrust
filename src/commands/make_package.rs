/// Make:Package Command
/// Generates a new package scaffold
/// Usage: cargo run -- make:package blog
/// Usage: cargo run -- make:package admin --path=packages

use crate::services::console::Command;
use crate::services::scaffold_package;
use async_trait::async_trait;
use std::error::Error;

pub struct MakePackageCommand;

#[async_trait]
impl Command for MakePackageCommand {
    fn name(&self) -> &str {
        "make:package"
    }

    fn description(&self) -> &str {
        "Generate a new package scaffold"
    }

    async fn handle(&self, args: Vec<String>) -> Result<(), Box<dyn Error>> {
        // Print help if no arguments
        if args.is_empty() {
            self.print_help();
            return Err("Package name is required".into());
        }

        let package_name = &args[0];
        let mut package_path = "packages".to_string();

        // Parse options
        for arg in &args[1..] {
            if arg.starts_with("--path=") {
                package_path = arg.strip_prefix("--path=").unwrap().to_string();
            }
        }

        // Validate package name
        if !is_valid_package_name(package_name) {
            return Err(format!(
                "Invalid package name: '{}'. Package names must be lowercase alphanumeric with underscores.",
                package_name
            ).into());
        }

        println!("🚀 Creating package: {}", package_name);
        println!("📁 Path: {}/{}", package_path, package_name);

        // Generate package
        scaffold_package(package_name, Some(&package_path))?;

        println!("\n✅ Package '{}' created successfully!", package_name);
        self.print_next_steps(package_name, &package_path);

        Ok(())
    }
}

impl MakePackageCommand {
    fn print_help(&self) {
        println!(
            r#"Usage:
  make:package <name> [options]

Arguments:
  name                    Package name (e.g., blog, admin, payment)

Options:
  --path=DIRECTORY        Directory to create package in (default: packages)

Examples:
  cargo run -- make:package blog
  cargo run -- make:package admin --path=packages
  cargo run -- make:package payment

This command creates a complete package structure with:
  ✓ src/ directory with all modules
  ✓ Controllers, Models, Providers, Routes, Services
  ✓ Configuration files
  ✓ Database migrations folder
  ✓ Tests directory
  ✓ package.json manifest
  ✓ README.md documentation
  ✓ All necessary mod.rs files
"#
        );
    }

    fn print_next_steps(&self, name: &str, path: &str) {
        let pascal_name = to_pascal_case(name);

        println!("\n📋 Next steps:");
        println!("\n1. Implement your package:");
        println!("   Edit: {}/{}/src/lib.rs", path, name);
        println!("   • Update package manifest (name, version, description)");
        println!("   • Implement the Package trait");
        println!("   • Add service providers");

        println!("\n2. Create service provider:");
        println!("   Edit: {}/{}/src/providers/{}ServiceProvider.rs", path, name, pascal_name);
        println!("   • Implement register() method");
        println!("   • Implement boot() method");
        println!("   • Add configuration");

        println!("\n3. Create controllers:");
        println!("   Add files to: {}/{}/src/controllers/", path, name);
        println!("   • Create your HTTP handlers");

        println!("\n4. Create models:");
        println!("   Add files to: {}/{}/src/models/", path, name);
        println!("   • Define your data structures");

        println!("\n5. Create routes:");
        println!("   Edit: {}/{}/src/routes/web.rs", path, name);
        println!("   • Define your API endpoints");

        println!("\n6. Register package in main.rs:");
        println!("   Add to src/main.rs:");
        println!("   ```rust");
        println!("   use my_app::{}::{}Package;", name, pascal_name);
        println!("   ");
        println!("   let mut manager = PackageManager::new(\"{}\");", path);
        println!("   manager.register(Box::new({}Package))?;", pascal_name);
        println!("   manager.boot().await?;");
        println!("   ```");

        println!("\n7. Documentation:");
        println!("   Edit: {}/{}/README.md", path, name);
        println!("   • Document your package");

        println!("\n📚 For more help:");
        println!("   • Read: docs/PACKAGE_SYSTEM.md");
        println!("   • Quick ref: docs/PACKAGE_SYSTEM_QUICK_REF.md");
    }
}

/// Validate package name (lowercase alphanumeric + underscores)
fn is_valid_package_name(name: &str) -> bool {
    if name.is_empty() || name.len() > 64 {
        return false;
    }

    name.chars().all(|c| c.is_lowercase() || c.is_numeric() || c == '_' || c == '-')
        && !name.starts_with(|c: char| c.is_numeric())
}

/// Convert snake_case to PascalCase
fn to_pascal_case(s: &str) -> String {
    s.split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_package_names() {
        assert!(is_valid_package_name("blog"));
        assert!(is_valid_package_name("admin_panel"));
        assert!(is_valid_package_name("user_management"));
        assert!(is_valid_package_name("auth2"));
        assert!(is_valid_package_name("my-package"));
    }

    #[test]
    fn test_invalid_package_names() {
        assert!(!is_valid_package_name(""));
        assert!(!is_valid_package_name("Blog")); // uppercase
        assert!(!is_valid_package_name("2blog")); // starts with number
        assert!(!is_valid_package_name("blog package")); // spaces
        assert!(!is_valid_package_name("blog@package")); // special chars
    }

    #[test]
    fn test_to_pascal_case() {
        assert_eq!(to_pascal_case("blog"), "Blog");
        assert_eq!(to_pascal_case("user_panel"), "UserPanel");
        assert_eq!(to_pascal_case("admin_management"), "AdminManagement");
    }

    #[test]
    fn test_command_name() {
        let cmd = MakePackageCommand;
        assert_eq!(cmd.name(), "make:package");
    }

    #[test]
    fn test_command_description() {
        let cmd = MakePackageCommand;
        assert!(cmd.description().len() > 0);
    }
}
