use tokio::process::Command;

pub struct Output(String);

impl Output {
    pub fn stdout(&self) -> String {
        self.0.clone()
    }
}

pub async fn exec_command<'a, I>(cmd: &str, args: I) -> anyhow::Result<Output>
where
    I: IntoIterator<Item = &'a str>,
{
    let process = Command::new(cmd)
        .args(args)
        .stdout(std::process::Stdio::piped())
        .spawn()
        .unwrap();
    let output = process.wait_with_output().await.unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();

    Ok(Output(stdout))
}
