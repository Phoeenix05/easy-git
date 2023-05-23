mod commands;
mod exec;
mod util;

#[tokio::main]
async fn main() {
    // let output = exec::exec_command("git", vec!["pull", "--help"])
    let output = util::current_branch().await.unwrap();
    println!("{}", output);

    // let _args = commands::CliArgs::parse();
}
