mod commands;
mod exec;
use commands::*;

#[tokio::main]
async fn main() {
    // let output = exec::exec_command("git", vec!["pull", "--help"])
    //     .await
    //     .unwrap();
    // println!("{}", output);

    let _args = CliArgs::parse();
}
