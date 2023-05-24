pub use clap::{Args, Parser, Subcommand};

use crate::config::Config;
use crate::exec::exec_command;
use crate::util::current_branch;
use crate::version::*;

#[derive(Parser, Debug)]
pub struct CliArgs {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Pulls the latest changes from remote origin
    Pull(Pull),
    /// Commits changes
    Commit(Commit),
    /// Pushes all unpushed local commits to remote origin
    Push(Push),
    /// Creates a new tag
    Tag(Tag),
    /// Undos the last commit
    Undo(Undo),
    /// Overrides the actual git command by setting an alias
    /// and changes the actual git command with an alias to <ggit>
    OverrideGit(OverrideGit),
    /// Project version
    Version(Version),
}

#[async_trait::async_trait]
pub trait Command {
    async fn run(&self, config: Config) -> anyhow::Result<()>;
}

#[derive(Debug, Args)]
pub struct Pull {
    /// Branch to pull changes from. Default value is the current active branch
    pub branch: Option<String>,
}

#[async_trait::async_trait]
impl Command for Pull {
    async fn run(&self, _: Config) -> anyhow::Result<()> {
        let current_branch = current_branch().await?;
        let branch = self.branch.clone().unwrap_or(current_branch);
        exec_command("git", ["pull", "origin", branch.as_str()]).await?;

        Ok(())
    }
}

#[derive(Debug, Args)]
pub struct Commit {
    /// Message that is shown with the commit
    pub message: String,
}

#[async_trait::async_trait]
impl Command for Commit {
    async fn run(&self, config: Config) -> anyhow::Result<()> {
        exec_command("git", ["add", "."]).await?;
        exec_command("git", ["commit", "-m", self.message.clone().as_str()]).await?;

        if config.commit.auto_push {
            let current_branch = current_branch().await?;
            exec_command("git", ["push", "origin", current_branch.as_str()]).await?;
        }

        Ok(())
    }
}

#[derive(Debug, Args)]
pub struct Push {
    /// Branch name local commits are going to be pushed to. Default value is the current active branch
    #[arg(short, long)]
    pub branch: Option<String>,
}

#[async_trait::async_trait]
impl Command for Push {
    async fn run(&self, _: Config) -> anyhow::Result<()> {
        let current_branch = current_branch().await?;
        let branch = self.branch.clone().unwrap_or(current_branch);

        exec_command("git", ["push", "origin", branch.as_str()]).await?;

        Ok(())
    }
}

#[derive(Debug, Args)]
pub struct Tag {
    /// Name for the new tag
    #[arg(short, long)]
    pub tag_name: String,
}

#[async_trait::async_trait]
impl Command for Tag {
    async fn run(&self, config: Config) -> anyhow::Result<()> {
        if !config.version_tag.match_project {
            return Ok(())
        }

        // exec_command("git", ["tag", "-a", "<version>", "-m", "'version <version>'"]).await?;

        Ok(())
    }
}

#[derive(Debug, Args)]
pub struct Undo {}

#[async_trait::async_trait]
impl Command for Undo {
    async fn run(&self, _: Config) -> anyhow::Result<()> {
        exec_command("git", ["reset", "--soft", "HEAD^"]).await?;

        Ok(())
    }
}

#[derive(Debug, Args)]
pub struct OverrideGit {}

#[async_trait::async_trait]
impl Command for OverrideGit {
    async fn run(&self, _: Config) -> anyhow::Result<()> {
        todo!()
    }
}
