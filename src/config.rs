use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    pub version_tag: VersionConfig,
    pub commit: CommitConfig,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct VersionConfig {
    pub match_project: bool,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CommitConfig {
    pub auto_push: bool,
}

impl Config {
    pub fn from_file(path: &'static str) -> anyhow::Result<Self> {
        let file_contents = std::fs::read_to_string(path)?;
        let config = toml::from_str(&file_contents)?;
        Ok(config)
    }
}
