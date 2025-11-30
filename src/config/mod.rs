use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub update_interval: u64,
}

pub async fn load_config(path: &str) -> anyhow::Result<Config> {
    // Try to read the config file
    match fs::read_to_string(path) {
        Ok(content) => {
            let config: Config = toml::from_str(&content)?;
            println!("📄 Loaded config from: {}", path);
            Ok(config)
        }
        Err(_) => {
            println!("⚠️  Config file not found, using defaults");
            // Return default config if file doesn't exist
            Ok(Config { update_interval: 1 })
        }
    }
}
