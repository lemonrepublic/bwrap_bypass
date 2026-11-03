use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub bypass: BypassConfig,
    pub binds: BindsConfig,
}

#[derive(Deserialize, Debug)]
pub struct BypassConfig {
    pub target_bwrap: String,
    pub enable_sanitization: bool,
}

#[derive(Deserialize, Debug)]
pub struct BindsConfig {
    pub read_only: Vec<String>,
    pub read_write: Vec<String>,
}

impl Config {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }
}
