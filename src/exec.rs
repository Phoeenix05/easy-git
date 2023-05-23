use anyhow::Result;
use tokio::process::Command;

pub async fn exec_command(cmd: &str, args: Vec<&str>) -> Result<String> {
    let output = Command::new(cmd).args(&args).output().await?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}
