use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;

use std::io::Read;

/// Configuration structure for storing application settings
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Config {
    default_folder: Option<String>,
}

impl Config {
    /// Load configuration from a file
    pub fn load() -> Self {
        let config_path = "config.json";
        if let Ok(mut file) = File::open(config_path) {
            let mut content = String::new();
            if file.read_to_string(&mut content).is_ok() {
                if let Ok(config) = serde_json::from_str(&content) {
                    return config;
                }
            }
        }
        Config::default()
    }

    /// Save configuration to a file
    pub fn save(&self) {
        let config_path = "config.json";
        if let Ok(mut file) = File::create(config_path) {
            if let Ok(content) = serde_json::to_string_pretty(self) {
                let _ = file.write_all(content.as_bytes());
            }
        }
    }

    /// Set the default folder
    pub fn set_default_folder(&mut self, folder_path: String) {
        self.default_folder = Some(folder_path);
        self.save();
        println!("Default folder set to: {}", self.default_folder.as_ref().unwrap());
    }

    /// Get the default folder
    pub fn get_default_folder(&self) -> Option<String> {
        self.default_folder.clone()
    }
}