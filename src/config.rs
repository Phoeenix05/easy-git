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

    pub fn get_config_files() -> anyhow::Result<Vec<String>> {
        let config_files = ["Cargo.toml", "package.json"];

        let current_dir = std::env::current_dir()?;
        let files = std::fs::read_dir(current_dir)?
            .filter_map(|entry| {
                let file_name = entry.expect("Failed to read entry").file_name();
                file_name.to_string_lossy().to_string().into()
            })
            .filter(|file_name| config_files.contains(&file_name.as_str()))
            .collect::<Vec<String>>();

        Ok(files)
    }
}
