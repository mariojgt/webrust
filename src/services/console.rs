use async_trait::async_trait;

#[async_trait]
pub trait Command: Send + Sync {
    /// The signature of the command (e.g., "email:send")
    fn name(&self) -> &str;

    /// Description of what the command does
    fn description(&self) -> &str;

    /// The handler for the command
    async fn handle(&self, args: Vec<String>) -> Result<(), Box<dyn std::error::Error>>;
}
