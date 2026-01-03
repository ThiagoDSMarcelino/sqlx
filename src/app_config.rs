use crate::connection::Connection;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct AppConfig {
    connections: HashMap<String, Connection>,
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

    pub fn add_connection(
        &mut self,
        name: &str,
        dns: &str,
        driver: &str,
    ) -> Result<(), anyhow::Error> {
        if self.connections.contains_key(name) {
            return Err(anyhow::anyhow!(
                "Connection name '{}' already exists.",
                name
            ));
        }

        self.connections
            .insert(name.to_string(), Connection::new(dns, driver));
        Ok(())
    }

    pub fn remove_connection(&mut self, name: &str) -> Result<(), anyhow::Error> {
        if !self.connections.contains_key(name) {
            return Err(anyhow::anyhow!(
                "Connection name '{}' does not exist.",
                name
            ));
        }

        self.connections.remove(name);
        Ok(())
    }

    pub fn get_connection(&self, name: &str) -> Option<&Connection> {
        self.connections.get(name)
    }

    pub fn get_connections(&self) -> &HashMap<String, Connection> {
        &self.connections
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::get_config_path()?;

        let content = serde_json::to_string_pretty(self)?;

        fs::write(path, content).context("Failed to write configuration file")?;

        Ok(())
    }
}
