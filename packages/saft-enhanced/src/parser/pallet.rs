//! Data structures representing FRAME pallet components

use serde::{Deserialize, Serialize};

/// Represents a complete FRAME pallet
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FramePallet {
    /// Name of the pallet module
    pub name: Option<String>,
    /// Configuration trait
    pub config: Option<PalletConfig>,
    /// Storage items
    pub storage_items: Vec<PalletStorage>,
    /// Dispatchable calls
    pub calls: Vec<PalletCall>,
    /// Events
    pub events: Vec<PalletEvent>,
    /// Errors
    pub errors: Vec<PalletError>,
}

/// Pallet configuration trait
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PalletConfig {
    /// Trait bounds (e.g., frame_system::Config)
    pub bounds: Vec<String>,
}

/// A storage item in the pallet
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PalletStorage {
    /// Name of the storage item
    pub name: String,
    /// Type definition as string
    pub ty: String,
}

/// A dispatchable call
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PalletCall {
    /// Name of the call
    pub name: String,
    /// Parameters
    pub params: Vec<String>,
}

/// An event emitted by the pallet
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PalletEvent {
    /// Name of the event
    pub name: String,
}

/// An error that can be returned by the pallet
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PalletError {
    /// Name of the error
    pub name: String,
}

impl FramePallet {
    /// Check if the pallet has a specific storage item
    pub fn has_storage(&self, name: &str) -> bool {
        self.storage_items.iter().any(|s| s.name == name)
    }

    /// Check if the pallet has a specific call
    pub fn has_call(&self, name: &str) -> bool {
        self.calls.iter().any(|c| c.name == name)
    }

    /// Get all storage item names
    pub fn storage_names(&self) -> Vec<&str> {
        self.storage_items.iter().map(|s| s.name.as_str()).collect()
    }

    /// Get all call names
    pub fn call_names(&self) -> Vec<&str> {
        self.calls.iter().map(|c| c.name.as_str()).collect()
    }

    /// Get the number of dispatchable calls
    pub fn call_count(&self) -> usize {
        self.calls.len()
    }

    /// Get the number of storage items
    pub fn storage_count(&self) -> usize {
        self.storage_items.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pallet_has_storage() {
        let pallet = FramePallet {
            name: Some("TestPallet".to_string()),
            config: None,
            storage_items: vec![PalletStorage {
                name: "MyValue".to_string(),
                ty: "u32".to_string(),
            }],
            calls: vec![],
            events: vec![],
            errors: vec![],
        };

        assert!(pallet.has_storage("MyValue"));
        assert!(!pallet.has_storage("NonExistent"));
    }

    #[test]
    fn test_pallet_call_names() {
        let pallet = FramePallet {
            name: Some("TestPallet".to_string()),
            config: None,
            storage_items: vec![],
            calls: vec![
                PalletCall {
                    name: "transfer".to_string(),
                    params: vec![],
                },
                PalletCall {
                    name: "approve".to_string(),
                    params: vec![],
                },
            ],
            events: vec![],
            errors: vec![],
        };

        let names = pallet.call_names();
        assert_eq!(names, vec!["transfer", "approve"]);
        assert_eq!(pallet.call_count(), 2);
    }
}
