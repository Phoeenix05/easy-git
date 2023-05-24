use crate::{commands::Command, config::Config};

use anyhow::Result;
pub use clap::Args;

#[derive(Debug, Args)]
pub struct Version;

#[async_trait::async_trait]
impl Command for Version {
    async fn run(&self, _: Config) -> Result<()> {
        Ok(())
    }
}
