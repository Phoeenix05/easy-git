use anyhow::Result;
use tokio::process::Command;

pub async fn exec_command<'a, I>(cmd: &str, args: I, out: bool) -> Result<String>
where
    I: IntoIterator<Item = &'a str>,
{
    let process = Command::new(cmd).args(args).stdout(std::process::Stdio::piped()).spawn().unwrap();
    let output = process.wait_with_output().await.unwrap();
    let output = String::from_utf8_lossy(&output.stdout).to_string();

    if out {
        println!("{}", output);
    }

    Ok(output)
}
