//! Substrate node connection management

use crate::{Error, Result};
use subxt::{OnlineClient, PolkadotConfig};
use tokio::sync::RwLock;
use std::sync::Arc;

/// Connection manager for Substrate nodes
pub struct ConnectionManager {
    endpoint: String,
    client: Arc<RwLock<Option<OnlineClient<PolkadotConfig>>>>,
}

impl ConnectionManager {
    /// Create a new connection manager
    pub fn new(endpoint: String) -> Self {
        Self {
            endpoint,
            client: Arc::new(RwLock::new(None)),
        }
    }

    /// Connect to the Substrate node
    pub async fn connect(&self) -> Result<()> {
        tracing::info!("Connecting to Substrate node at {}", self.endpoint);

        // Attempt to connect with a timeout
        let client = tokio::time::timeout(
            std::time::Duration::from_secs(10),
            OnlineClient::<PolkadotConfig>::from_url(&self.endpoint)
        )
        .await
        .map_err(|_| Error::ConnectionError(format!("Connection timeout to {}", self.endpoint)))?
        .map_err(|e| Error::ConnectionError(format!("Failed to connect: {}", e)))?;

        let mut client_lock = self.client.write().await;
        *client_lock = Some(client);

        tracing::info!("Successfully connected to Substrate node");
        Ok(())
    }

    /// Check if connected
    pub async fn is_connected(&self) -> bool {
        self.client.read().await.is_some()
    }

    /// Get the client (if connected)
    pub async fn get_client(&self) -> Option<OnlineClient<PolkadotConfig>> {
        self.client.read().await.clone()
    }

    /// Disconnect from the node
    pub async fn disconnect(&self) {
        let mut client_lock = self.client.write().await;
        *client_lock = None;
        tracing::info!("Disconnected from Substrate node");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_connection_manager_creation() {
        let manager = ConnectionManager::new("ws://127.0.0.1:9944".to_string());
        assert!(!manager.is_connected().await);
    }

    #[tokio::test]
    async fn test_connection_to_invalid_endpoint() {
        let manager = ConnectionManager::new("ws://127.0.0.1:9999".to_string());
        let result = manager.connect().await;
        assert!(result.is_err());
        assert!(!manager.is_connected().await);
    }
}
