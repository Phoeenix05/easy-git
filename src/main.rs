use tokio::process::Command;

mod commands;
use commands::*;

#[tokio::main]
async fn main() {
    let _args = CliArgs::parse();

    let output = Command::new("ls")
        .arg("-a")
        .output()
        .await
        .expect("Failed to execute command");

    println!("{}", String::from_utf8_lossy(&output.stdout));
}
