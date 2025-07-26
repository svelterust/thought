use color_eyre::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub title: String,
    pub username: String,
    pub folder: PathBuf,
}

impl Config {
    pub fn config_path() -> PathBuf {
        dirs::config_dir()
            .expect("Could not find config directory")
            .join("thought/config.toml")
    }

    pub fn load() -> Option<Self> {
        std::fs::read_to_string(Self::config_path())
            .ok()
            .and_then(|content| toml::from_str(&content).ok())
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::config_path();
        std::fs::create_dir_all(path.parent().unwrap())?;
        Ok(std::fs::write(path, toml::to_string(self)?)?)
    }

    pub fn ensure_folder_exists(&self) -> Result<()> {
        Ok(std::fs::create_dir_all(&self.folder)?)
    }
}
