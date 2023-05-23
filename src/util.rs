use crate::exec::exec_command;
use anyhow::Result;

/// Gets the current active branch
pub async fn current_branch() -> Result<String> {
    let branch = exec_command("git", ["branch", "--show-current"]).await?;
    Ok(branch.trim().to_owned())
}
