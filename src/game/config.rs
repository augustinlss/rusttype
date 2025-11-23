use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub word_count: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self { word_count: 10 }
    }
}

impl Config {
    fn get_config_path() -> &'static str {
        "config.json"
    }

    pub fn load() -> Self {
        let path = Self::get_config_path();
        if Path::new(path).exists() {
            if let Ok(content) = fs::read_to_string(path) {
                if let Ok(config) = serde_json::from_str(&content) {
                    return config;
                }
            }
        }
        Self::default()
    }

    pub fn save(&self) {
        let path = Self::get_config_path();
        if let Ok(content) = serde_json::to_string_pretty(self) {
            let _ = fs::write(path, content);
        }
    }
}
