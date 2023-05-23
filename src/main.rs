mod commands;
mod exec;
mod util;
use commands::*;

#[tokio::main]
async fn main() {
    let args = CliArgs::parse();

    match args.command {
        Commands::Pull(cmd) => cmd.run().await.unwrap(),
        Commands::Commit(cmd) => cmd.run().await.unwrap(),
        Commands::Push(cmd) => cmd.run().await.unwrap(),
        Commands::Tag(cmd) => cmd.run().await.unwrap(),
        Commands::Undo(cmd) => cmd.run().await.unwrap(),
    }
}
   