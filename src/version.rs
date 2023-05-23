pub use clap::Args;
use anyhow::Result;
use crate::commands::Command;

#[derive(Debug, Args)]
pub struct Version {}

#[async_trait::async_trait]
impl Command for Version {
    async fn run(&self) -> Result<()> {
        Ok(())
    }    
}
