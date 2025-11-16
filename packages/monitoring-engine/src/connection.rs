//! Substrate node connection management

use crate::{Error, Result};
use subxt::{OnlineClient, PolkadotConfig};
use tokio::sync::RwLock;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};

/// Connection manager for Substrate nodes
pub struct ConnectionManager {
    endpoint: String,
    client: Arc<RwLock<Option<OnlineClient<PolkadotConfig>>>>,
    reconnect_attempts: Arc<AtomicU32>,
    should_reconnect: Arc<AtomicBool>,
}

impl ConnectionManager {
    /// Create a new connection manager
    pub fn new(endpoint: String) -> Self {
        Self {
            endpoint,
            client: Arc::new(RwLock::new(None)),
            reconnect_attempts: Arc::new(AtomicU32::new(0)),
            should_reconnect: Arc::new(AtomicBool::new(true)),
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

        // Reset reconnect attempts on successful connection
        self.reconnect_attempts.store(0, Ordering::SeqCst);

        tracing::info!("Successfully connected to Substrate node");
        Ok(())
    }

    /// Connect with automatic retry using exponential backoff
    pub async fn connect_with_retry(&self, max_attempts: u32) -> Result<()> {
        let mut attempt = 0;

        while attempt < max_attempts {
            match self.connect().await {
                Ok(_) => return Ok(()),
                Err(e) => {
                    attempt += 1;
                    self.reconnect_attempts.store(attempt, Ordering::SeqCst);

                    if attempt >= max_attempts {
                        tracing::error!(
                            "Failed to connect after {} attempts: {}",
                            max_attempts,
                            e
                        );
                        return Err(e);
                    }

                    // Exponential backoff: 2^attempt seconds, max 60 seconds
                    let backoff_secs = std::cmp::min(2_u64.pow(attempt), 60);
                    tracing::warn!(
                        "Connection attempt {} failed, retrying in {} seconds: {}",
                        attempt,
                        backoff_secs,
                        e
                    );

                    tokio::time::sleep(std::time::Duration::from_secs(backoff_secs)).await;
                }
            }
        }

        Err(Error::ConnectionError("Max reconnection attempts reached".to_string()))
    }

    /// Get the number of reconnection attempts
    pub fn get_reconnect_attempts(&self) -> u32 {
        self.reconnect_attempts.load(Ordering::SeqCst)
    }

    /// Check if automatic reconnection is enabled
    pub fn should_reconnect(&self) -> bool {
        self.should_reconnect.load(Ordering::SeqCst)
    }

    /// Enable or disable automatic reconnection
    pub fn set_reconnect(&self, enabled: bool) {
        self.should_reconnect.store(enabled, Ordering::SeqCst);
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
        // Disable automatic reconnection when explicitly disconnecting
        self.should_reconnect.store(false, Ordering::SeqCst);

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
        assert!(manager.should_reconnect());
        assert_eq!(manager.get_reconnect_attempts(), 0);
    }

    #[tokio::test]
    async fn test_connection_to_invalid_endpoint() {
        let manager = ConnectionManager::new("ws://127.0.0.1:9999".to_string());
        let result = manager.connect().await;
        assert!(result.is_err());
        assert!(!manager.is_connected().await);
    }

    #[tokio::test]
    async fn test_reconnect_attempts_tracking() {
        let manager = ConnectionManager::new("ws://127.0.0.1:9999".to_string());

        // Try to connect with retry (should fail)
        let result = manager.connect_with_retry(3).await;
        assert!(result.is_err());

        // Should have attempted 3 times
        assert_eq!(manager.get_reconnect_attempts(), 3);
    }

    #[tokio::test]
    async fn test_disable_reconnect_on_disconnect() {
        let manager = ConnectionManager::new("ws://127.0.0.1:9944".to_string());
        assert!(manager.should_reconnect());

        manager.disconnect().await;
        assert!(!manager.should_reconnect());
    }
}
