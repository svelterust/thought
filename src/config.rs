use color_eyre::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub title: String,
    pub folder: PathBuf,
}

impl Config {
    pub fn config_path() -> PathBuf {
        let config_dir = dirs::config_dir()
            .expect("Could not find config directory")
            .join("thought");
        std::fs::create_dir_all(&config_dir).expect("Could not create config directory");
        config_dir.join("config.toml")
    }

    pub fn load() -> Option<Self> {
        let config_path = Self::config_path();
        if config_path.exists() {
            let content = std::fs::read_to_string(&config_path).ok()?;
            toml::from_str(&content).ok()
        } else {
            None
        }
    }

    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_path();
        let content = toml::to_string(self)?;
        Ok(std::fs::write(config_path, content)?)
    }

    pub fn ensure_folder_exists(&self) -> Result<()> {
        Ok(std::fs::create_dir_all(&self.folder)?)
    }
}
