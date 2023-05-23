mod commands;
mod exec;
mod util;
use commands::*;

#[tokio::main]
async fn main() {
    let args = CliArgs::parse();

    match args.command {
        Commands::Pull(cmd) => {
            let current_branch = util::current_branch().await.unwrap();
            let branch = cmd.branch.unwrap_or(current_branch);

            let output = exec::exec_command("git", ["pull", "origin", branch.as_str()]).await.unwrap();
            println!("{}", output);
        },
        Commands::Commit(cmd) => {
            let output = exec::exec_command("git", ["add", "."]).await.unwrap();
            println!("{}", output);
            
            let message = cmd.message.unwrap_or("commit".to_owned());
            let output = exec::exec_command("git", ["commit", "-m", message.as_str()]).await.unwrap();
            println!("{}", output);
        },
        Commands::Push(cmd) => {
            let current_branch = util::current_branch().await.unwrap();
            let branch = cmd.branch.unwrap_or(current_branch);

            let output = exec::exec_command("git", ["push", "origin", branch.as_str()]).await.unwrap();
            println!("{}", output);
        },
        Commands::Tag(_) => (),
        Commands::Undo(_) => {
            let output = exec::exec_command("git", ["reset", "--soft", "HEAD^"]).await.unwrap();
            println!("{}", output);
        },
    }
}
   