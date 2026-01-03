use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct AppConfig {
    connections: HashMap<String, String>,
}

use anyhow::{Context, Result};
use directories::ProjectDirs;
use std::fs;
use std::path::PathBuf;

impl AppConfig {
    fn get_config_path() -> Result<PathBuf> {
        let proj_dirs = ProjectDirs::from("com", "ThiagoDSMarcelino", "sqlx")
            .context("Failed to locate home directory")?;

        let config_dir = proj_dirs.config_dir();

        if !config_dir.exists() {
            fs::create_dir_all(config_dir)?;
        }

        Ok(config_dir.join("config.json"))
    }

    pub fn load() -> Result<Self> {
        let path = Self::get_config_path()?;

        if !path.exists() {
            return Ok(AppConfig::default());
        }

        let content = fs::read_to_string(path)?;
        let config: AppConfig =
            serde_json::from_str(&content).context("Failed to parse configuration file")?;

        Ok(config)
    }

    pub fn add_connection(&mut self, name: String, dns: String) -> Result<(), anyhow::Error> {
        if self.connections.contains_key(&name) {
            return Err(anyhow::anyhow!(
                "Connection name '{}' already exists.",
                name
            ));
        }

        self.connections.insert(name, dns);
        Ok(())
    }

    pub fn get_connections(&self) -> &HashMap<String, String> {
        &self.connections
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::get_config_path()?;

        let content = serde_json::to_string_pretty(self)?;

        fs::write(path, content).context("Failed to write configuration file")?;

        Ok(())
    }
}
