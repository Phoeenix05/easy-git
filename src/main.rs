mod commands;
mod config;
mod exec;
mod util;
mod version;

use commands::*;
use config::Config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = match Config::from_file("./easy-git.config.toml") {
        Ok(config) => config,
        Err(err) => {
            eprintln!("{}", err);
            Config::default()
        },
    };

    let args = CliArgs::parse();

    let _ = match args.command {
        Commands::Pull(cmd) => cmd.run(config).await?,
        Commands::Commit(cmd) => cmd.run(config).await?,
        Commands::Push(cmd) => cmd.run(config).await?,
        // Commands::Tag(cmd) => cmd.run(config).await?,
        Commands::Undo(cmd) => cmd.run(config).await?,
        Commands::OverrideGit(cmd) => cmd.run(config).await?,
        Commands::Version(cmd) => cmd.run(config).await?,
        _ => (),
    };

    Ok(())
}
