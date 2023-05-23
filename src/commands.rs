use anyhow::Result;

pub use clap::{Args, Parser, Subcommand};
use inquire::{Confirm, Text};
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
    /// Commits local changes
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
        let current_branch = current_branch().await.unwrap();
        let branch = self.branch.clone().unwrap_or(current_branch);
        exec_command("git", ["pull", "origin", branch.as_str()], true).await.unwrap();

        Ok(())
    }
}

#[derive(Debug, Args)]
pub struct Commit {
    /// Message that is shown with the commit
    pub message: Option<String>,
}

#[async_trait::async_trait]
impl Command for Commit {
    async fn run(&self) -> Result<()> {
        exec_command("git", ["add", "."], false).await.unwrap();

        let message = self.message.clone().unwrap_or("commit".to_owned());
        exec_command("git", ["commit", "-m", message.as_str()], true).await.unwrap();

        let current_branch = current_branch().await.unwrap();
        let push = Confirm::new("Do you want to push changes to remote origin?").prompt().unwrap();
        if push {
            let branch = Text::new("Please enter the branch name you want to push to.")
                    .with_initial_value(current_branch.as_str())
                    .prompt()
                    .unwrap();

            exec_command("git", ["push", "origin", branch.as_str()], true).await.unwrap();
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
        let current_branch = current_branch().await.unwrap();
        let branch = self.branch.clone().unwrap_or(current_branch);

        exec_command("git", ["push", "origin", branch.as_str()], true).await.unwrap();

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
        exec_command("git", ["reset", "--soft", "HEAD^"], false).await.unwrap();

        Ok(())
    }
}

#[derive(Debug, Args)]
pub struct OverrideGit {}

#[async_trait::async_trait]
impl Command for OverrideGit {
    async fn run(&self) -> Result<()> {
        // exec_command(cmd, args, out)
        Ok(())
    }
}