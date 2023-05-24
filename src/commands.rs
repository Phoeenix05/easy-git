use anyhow::Result;

pub use clap::{Args, Parser, Subcommand};
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
    async fn run(&self) -> Result<()>;
}

#[derive(Debug, Args)]
pub struct Pull {
    /// Branch to pull changes from. Default value is the current active branch
    pub branch: Option<String>,
}

#[async_trait::async_trait]
impl Command for Pull {
    async fn run(&self) -> Result<()> {
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
    /// Push commit to remote origin/<current_branch>
    #[arg(short, long)]
    pub push: bool,
}

#[async_trait::async_trait]
impl Command for Commit {
    async fn run(&self) -> Result<()> {
        dbg!(self);
        exec_command("git", ["add", "."]).await?;
        exec_command("git", ["commit", "-m", self.message.clone().as_str()]).await?;

        if self.push {
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
    async fn run(&self) -> Result<()> {
        let current_branch = current_branch().await?;
        let branch = self.branch.clone().unwrap_or(current_branch);

        exec_command("git", ["push", "origin", branch.as_str()]).await?;

        Ok(())
    }
}

#[derive(Debug, Args)]
pub struct Tag {
    #[command(subcommand)]
    pub command: Option<TagCommands>,

    /// Name for the new tag
    #[arg(short, long)]
    pub tag_name: String,
}

#[async_trait::async_trait]
impl Command for Tag {
    async fn run(&self) -> Result<()> {
        todo!()
    }
}

#[derive(Debug, Subcommand)]
pub enum TagCommands {
    Delete(DeleteTag),
    Update(UpdateTag),
}

#[derive(Debug, Args)]
pub struct DeleteTag {}

#[async_trait::async_trait]
impl Command for DeleteTag {
    async fn run(&self) -> Result<()> {
        todo!()
    }
}

#[derive(Debug, Args)]
pub struct UpdateTag {}

#[async_trait::async_trait]
impl Command for UpdateTag {
    async fn run(&self) -> Result<()> {
        todo!()
    }
}

#[derive(Debug, Args)]
pub struct Undo {}

#[async_trait::async_trait]
impl Command for Undo {
    async fn run(&self) -> Result<()> {
        exec_command("git", ["reset", "--soft", "HEAD^"]).await?;

        Ok(())
    }
}

#[derive(Debug, Args)]
pub struct OverrideGit {}

#[async_trait::async_trait]
impl Command for OverrideGit {
    async fn run(&self) -> Result<()> {
        todo!()
    }
}