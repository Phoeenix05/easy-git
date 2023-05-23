use anyhow::Result;
use tokio::process::Command;

pub async fn exec_command<'a, I>(cmd: &str, args: I) -> Result<String>
where
    I: IntoIterator<Item = &'a str>,
{
    let output = Command::new(cmd).args(args).output().await?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}
