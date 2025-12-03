/// Advanced Modular Package System (Laravel-inspired)
/// Allows true modular organization with service providers, configurations, and auto-discovery

use std::collections::HashMap;
use std::path::PathBuf;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// Package manifest configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageManifest {
    /// Package name (e.g., "blog")
    pub name: String,
    /// Package version
    pub version: String,
    /// Package description
    pub description: String,
    /// Package author
    pub author: Option<String>,
    /// List of service providers to register
    pub providers: Vec<String>,
    /// List of routes to register
    pub routes: Vec<String>,
    /// List of migrations to run
    pub migrations: Vec<String>,
    /// Package configuration files
    pub config: HashMap<String, String>,
    /// Package dependencies
    pub dependencies: HashMap<String, String>,
    /// Enable/disable the package
    pub enabled: bool,
}

impl PackageManifest {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            version: "0.1.0".to_string(),
            description: "A WebRust package".to_string(),
            author: None,
            providers: vec![],
            routes: vec![],
            migrations: vec![],
            config: HashMap::new(),
            dependencies: HashMap::new(),
            enabled: true,
        }
    }

    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        self.version = version.into();
        self
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    pub fn with_provider(mut self, provider: impl Into<String>) -> Self {
        self.providers.push(provider.into());
        self
    }

    pub fn with_route(mut self, route: impl Into<String>) -> Self {
        self.routes.push(route.into());
        self
    }

    pub fn with_migration(mut self, migration: impl Into<String>) -> Self {
        self.migrations.push(migration.into());
        self
    }

    pub fn with_config(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.config.insert(key.into(), value.into());
        self
    }

    pub fn with_dependency(mut self, name: impl Into<String>, version: impl Into<String>) -> Self {
        self.dependencies.insert(name.into(), version.into());
        self
    }
}

/// Package service provider trait - Initialize package services
#[async_trait]
pub trait ServiceProvider: Send + Sync {
    /// Provider name
    fn name(&self) -> &str;

    /// Register services (before boot)
    async fn register(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    /// Boot services (after all registrations)
    async fn boot(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    /// Return configuration for this provider
    fn config(&self) -> HashMap<String, String> {
        HashMap::new()
    }
}

/// Package trait - Define a package
#[async_trait]
pub trait Package: Send + Sync {
    /// Get package manifest
    fn manifest(&self) -> PackageManifest;

    /// Get service providers
    fn providers(&self) -> Vec<Box<dyn ServiceProvider>> {
        vec![]
    }

    /// Get package routes
    fn routes(&self) -> Vec<String> {
        vec![]
    }

    /// Get package migrations
    fn migrations(&self) -> Vec<String> {
        vec![]
    }

    /// Get package assets (views, etc.)
    fn assets(&self) -> Vec<PathBuf> {
        vec![]
    }

    /// Hook called when package is installed
    async fn install(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    /// Hook called when package is uninstalled
    async fn uninstall(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    /// Hook called when package is enabled
    async fn enable(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    /// Hook called when package is disabled
    async fn disable(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}

/// Package manager - Handles package discovery, registration, and lifecycle
pub struct PackageManager {
    packages: HashMap<String, Box<dyn Package>>,
    manifest_cache: HashMap<String, PackageManifest>,
    base_path: PathBuf,
}

impl PackageManager {
    pub fn new(base_path: impl Into<PathBuf>) -> Self {
        Self {
            packages: HashMap::new(),
            manifest_cache: HashMap::new(),
            base_path: base_path.into(),
        }
    }

    /// Register a package
    pub fn register(&mut self, package: Box<dyn Package>) -> Result<(), Box<dyn std::error::Error>> {
        let manifest = package.manifest();

        if !manifest.enabled {
            println!("⏭️  Package {} is disabled, skipping registration", manifest.name);
            return Ok(());
        }

        let name = manifest.name.clone();
        self.manifest_cache.insert(name.clone(), manifest);
        self.packages.insert(name, package);

        Ok(())
    }

    /// Register multiple packages
    pub fn register_many(&mut self, packages: Vec<Box<dyn Package>>) -> Result<(), Box<dyn std::error::Error>> {
        for package in packages {
            self.register(package)?;
        }
        Ok(())
    }

    /// Get registered packages
    pub fn packages(&self) -> &HashMap<String, Box<dyn Package>> {
        &self.packages
    }

    /// Get package by name
    pub fn get(&self, name: &str) -> Option<&Box<dyn Package>> {
        self.packages.get(name)
    }

    /// Get package manifest
    pub fn manifest(&self, name: &str) -> Option<&PackageManifest> {
        self.manifest_cache.get(name)
    }

    /// Boot all packages
    pub async fn boot(&self) -> Result<(), Box<dyn std::error::Error>> {
        for (name, package) in &self.packages {
            println!("🚀 Booting package: {}", name);

            for provider in package.providers() {
                provider.register().await?;
            }
        }

        for (name, package) in &self.packages {
            for provider in package.providers() {
                provider.boot().await?;
            }
        }

        Ok(())
    }

    /// Get all package routes
    pub fn routes(&self) -> Vec<String> {
        let mut routes = vec![];
        for package in self.packages.values() {
            routes.extend(package.routes());
        }
        routes
    }

    /// Get all package migrations
    pub fn migrations(&self) -> Vec<String> {
        let mut migrations = vec![];
        for package in self.packages.values() {
            migrations.extend(package.migrations());
        }
        migrations
    }

    /// Get all package assets
    pub fn assets(&self) -> Vec<PathBuf> {
        let mut assets = vec![];
        for package in self.packages.values() {
            assets.extend(package.assets());
        }
        assets
    }

    /// Install a package
    pub async fn install(&self, name: &str) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(package) = self.packages.get(name) {
            package.install().await?;
            println!("✅ Package {} installed", name);
        } else {
            println!("❌ Package {} not found", name);
        }
        Ok(())
    }

    /// Enable a package
    pub async fn enable(&self, name: &str) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(package) = self.packages.get(name) {
            package.enable().await?;
            println!("✅ Package {} enabled", name);
        }
        Ok(())
    }

    /// Disable a package
    pub async fn disable(&self, name: &str) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(package) = self.packages.get(name) {
            package.disable().await?;
            println!("✅ Package {} disabled", name);
        }
        Ok(())
    }

    /// List all packages
    pub fn list(&self) -> Vec<(&str, &PackageManifest)> {
        self.manifest_cache
            .iter()
            .map(|(name, manifest)| (name.as_str(), manifest))
            .collect()
    }

    /// Get package dependencies
    pub fn dependencies(&self, name: &str) -> Option<&HashMap<String, String>> {
        self.manifest_cache
            .get(name)
            .map(|m| &m.dependencies)
    }

    /// Check if package is enabled
    pub fn is_enabled(&self, name: &str) -> bool {
        self.manifest_cache
            .get(name)
            .map(|m| m.enabled)
            .unwrap_or(false)
    }

    /// Get package configuration
    pub fn config(&self, name: &str) -> Option<&HashMap<String, String>> {
        self.manifest_cache
            .get(name)
            .map(|m| &m.config)
    }

    /// Search packages by partial name
    pub fn search(&self, query: &str) -> Vec<(&str, &PackageManifest)> {
        self.manifest_cache
            .iter()
            .filter(|(name, _)| name.to_lowercase().contains(&query.to_lowercase()))
            .map(|(name, manifest)| (name.as_str(), manifest))
            .collect()
    }

    /// Get package info
    pub fn info(&self, name: &str) -> Option<String> {
        self.manifest_cache.get(name).map(|m| {
            format!(
                "📦 {} v{}\n{}\n👤 {}\n🔌 Providers: {}\n📍 Routes: {}\n💾 Migrations: {}",
                m.name,
                m.version,
                m.description,
                m.author.as_ref().unwrap_or(&"Unknown".to_string()),
                m.providers.len(),
                m.routes.len(),
                m.migrations.len()
            )
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestPackage;

    #[async_trait]
    impl Package for TestPackage {
        fn manifest(&self) -> PackageManifest {
            PackageManifest::new("test")
                .with_version("1.0.0")
                .with_description("Test package")
        }

        fn routes(&self) -> Vec<String> {
            vec!["GET /test".to_string()]
        }
    }

    #[test]
    fn test_package_manager_creation() {
        let manager = PackageManager::new(".");
        assert_eq!(manager.packages().len(), 0);
    }

    #[tokio::test]
    async fn test_package_registration() {
        let mut manager = PackageManager::new(".");
        let package = Box::new(TestPackage);
        manager.register(package).ok();
        assert_eq!(manager.packages().len(), 1);
    }

    #[test]
    fn test_manifest_builder() {
        let manifest = PackageManifest::new("blog")
            .with_version("2.0.0")
            .with_description("Blog system")
            .with_route("GET /posts")
            .with_migration("create_posts_table");

        assert_eq!(manifest.name, "blog");
        assert_eq!(manifest.version, "2.0.0");
        assert_eq!(manifest.routes.len(), 1);
        assert_eq!(manifest.migrations.len(), 1);
    }

    #[test]
    fn test_package_listing() {
        let mut manager = PackageManager::new(".");
        let package = Box::new(TestPackage);
        manager.register(package).ok();

        let packages = manager.list();
        assert_eq!(packages.len(), 1);
        assert_eq!(packages[0].1.name, "test");
    }

    #[test]
    fn test_package_search() {
        let mut manager = PackageManager::new(".");
        let package1 = Box::new(TestPackage);
        manager.register(package1).ok();

        let results = manager.search("test");
        assert_eq!(results.len(), 1);
    }
}
