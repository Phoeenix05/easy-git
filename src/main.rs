mod commands;
mod exec;
mod util;
mod version;

use commands::*;

#[tokio::main]
async fn main() {
    let args = CliArgs::parse();

    let _ = match args.command {
        Commands::Pull(cmd) => cmd.run().await,
        Commands::Commit(cmd) => cmd.run().await,
        Commands::Push(cmd) => cmd.run().await,
        Commands::Tag(cmd) => cmd.run().await,
        Commands::Undo(cmd) => cmd.run().await,
        Commands::OverrideGit(cmd) => cmd.run().await,
        Commands::Version(cmd) => cmd.run().await,
    };
}
   