//! Configuration persistence module
//!
//! Handles saving and loading chain configuration from disk

use crate::{MonitorConfig, Result, Error};
use std::path::PathBuf;
use std::fs;
use serde::{Deserialize, Serialize};

/// Saved configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedConfig {
    pub chain_name: String,
    pub timestamp: u64,
}

impl SavedConfig {
    pub fn new(chain_name: String) -> Self {
        Self {
            chain_name,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
}

/// Get the config file path
pub fn get_config_path() -> PathBuf {
    // Use current directory for simplicity
    // In production, this could be ~/.config/monitoring-engine/config.json
    PathBuf::from("chain_config.json")
}

/// Save chain configuration to disk
pub fn save_chain_config(chain_name: &str) -> Result<()> {
    let config = SavedConfig::new(chain_name.to_string());
    let config_path = get_config_path();

    let json = serde_json::to_string_pretty(&config)
        .map_err(|e| Error::ConfigError(format!("Failed to serialize config: {}", e)))?;

    fs::write(&config_path, json)
        .map_err(|e| Error::IoError(e))?;

    tracing::info!("Saved chain configuration: {} to {:?}", chain_name, config_path);
    Ok(())
}

/// Load chain configuration from disk
pub fn load_chain_config() -> Result<Option<SavedConfig>> {
    let config_path = get_config_path();

    if !config_path.exists() {
        return Ok(None);
    }

    let contents = fs::read_to_string(&config_path)
        .map_err(|e| Error::IoError(e))?;

    let config: SavedConfig = serde_json::from_str(&contents)
        .map_err(|e| Error::ConfigError(format!("Failed to parse config: {}", e)))?;

    tracing::info!("Loaded chain configuration: {}", config.chain_name);
    Ok(Some(config))
}

/// Load MonitorConfig based on saved configuration, or use default
pub fn load_monitor_config() -> MonitorConfig {
    match load_chain_config() {
        Ok(Some(saved)) => {
            tracing::info!("Loading saved chain configuration: {}", saved.chain_name);
            match MonitorConfig::from_chain_name(&saved.chain_name) {
                Some(config) => {
                    tracing::info!("Successfully loaded {} configuration", saved.chain_name);
                    config
                }
                None => {
                    tracing::warn!(
                        "Unknown chain '{}' in saved config, using default (westend)",
                        saved.chain_name
                    );
                    MonitorConfig::default()
                }
            }
        }
        Ok(None) => {
            tracing::info!("No saved configuration found, using default (westend)");
            MonitorConfig::default()
        }
        Err(e) => {
            tracing::error!("Error loading saved configuration: {}, using default", e);
            MonitorConfig::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_saved_config_creation() {
        let config = SavedConfig::new("westend".to_string());
        assert_eq!(config.chain_name, "westend");
        assert!(config.timestamp > 0);
    }

    #[test]
    fn test_config_serialization() {
        let config = SavedConfig::new("polkadot".to_string());
        let json = serde_json::to_string(&config).unwrap();
        let deserialized: SavedConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(config.chain_name, deserialized.chain_name);
    }
}
