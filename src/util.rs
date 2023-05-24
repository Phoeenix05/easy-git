use crate::exec::exec_command;

/// Gets the current active branch
pub async fn current_branch() -> anyhow::Result<String> {
    let branch = exec_command("git", ["branch", "--show-current"])
        .await?
        .stdout();
    Ok(branch.trim().to_owned())
}
