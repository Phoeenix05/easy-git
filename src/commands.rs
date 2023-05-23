use anyhow::Result;

pub use clap::{Args, Parser, Subcommand};

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
}

pub trait Command {
    fn run(&self) -> Result<()>;
}

#[derive(Debug, Args)]
pub struct Pull {
    /// Branch to pull changes from. Default value is the current active branch
    pub branch: Option<String>,
}

impl Command for Pull {
    fn run(&self) -> Result<()> {
        todo!()
    }
}

#[derive(Debug, Args)]
pub struct Commit {
    /// Message that is shown with the commit
    pub message: Option<String>,
}

impl Command for Commit {
    fn run(&self) -> Result<()> {
        todo!()
    }
}

#[derive(Debug, Args)]
pub struct Push {
    /// Branch name local commits are going to be pushed to. Default value is the current active branch
    #[arg(short, long)]
    pub branch: Option<String>,
}

impl Command for Push {
    fn run(&self) -> Result<()> {
        todo!()
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

impl Command for Tag {
    fn run(&self) -> Result<()> {
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

impl Command for DeleteTag {
    fn run(&self) -> Result<()> {
        todo!()
    }
}

#[derive(Debug, Args)]
pub struct UpdateTag {}

impl Command for UpdateTag {
    fn run(&self) -> Result<()> {
        todo!()
    }
}

#[derive(Debug, Args)]
pub struct Undo {}

impl Command for Undo {
    fn run(&self) -> Result<()> {
        todo!()
    }
}
