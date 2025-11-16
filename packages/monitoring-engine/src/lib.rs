//! Monitoring Engine
//!
//! Real-time security monitoring for Polkadot parachains.
//! Detects suspicious patterns, potential attacks, and anomalies in blockchain activity.

pub mod detectors;
pub mod mempool;
pub mod alerts;
pub mod types;
pub mod connection;
pub mod api;
pub mod transaction;

use futures::StreamExt;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::RwLock;

pub use types::{Alert, AlertSeverity, AttackPattern, ChainEvent, DetectionResult, Transaction, ParsedTransaction, TransactionContext};

/// Main error type for the monitoring engine
#[derive(Error, Debug)]
pub enum Error {
    #[error("Connection error: {0}")]
    ConnectionError(String),

    #[error("Subscription error: {0}")]
    SubscriptionError(String),

    #[error("Parsing error: {0}")]
    ParseError(String),

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Alert error: {0}")]
    AlertError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

/// Result type alias
pub type Result<T> = std::result::Result<T, Error>;

/// Configuration for the monitoring engine
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitorConfig {
    /// WebSocket endpoint for the parachain
    pub ws_endpoint: String,
    /// Chain name identifier
    pub chain_name: String,
    /// Enable mempool monitoring
    pub enable_mempool: bool,
    /// Enable block monitoring
    pub enable_blocks: bool,
    /// Enable event monitoring
    pub enable_events: bool,
    /// Alert webhook URL (optional)
    pub alert_webhook: Option<String>,
    /// Minimum alert severity to trigger notifications
    pub min_alert_severity: AlertSeverity,
    /// Buffer size for event processing
    pub buffer_size: usize,
    /// Maximum reconnection attempts (0 = no retry, use connect_with_retry)
    #[serde(default = "default_max_reconnect_attempts")]
    pub max_reconnect_attempts: u32,
}

fn default_max_reconnect_attempts() -> u32 {
    5
}

impl Default for MonitorConfig {
    fn default() -> Self {
        Self {
            ws_endpoint: "ws://localhost:9944".to_string(),
            chain_name: "local".to_string(),
            enable_mempool: true,
            enable_blocks: true,
            enable_events: true,
            alert_webhook: None,
            min_alert_severity: AlertSeverity::Medium,
            buffer_size: 1000,
            max_reconnect_attempts: 5,
        }
    }
}

/// Main monitoring engine
pub struct MonitoringEngine {
    pub config: MonitorConfig,
    state: Arc<RwLock<EngineState>>,
    pub alert_manager: Arc<alerts::AlertManager>,
    pub connection: Arc<connection::ConnectionManager>,
}

/// Internal engine state
#[derive(Debug, Default)]
struct EngineState {
    is_running: bool,
    blocks_processed: u64,
    transactions_analyzed: u64,
    alerts_triggered: u64,
}

impl MonitoringEngine {
    /// Create a new monitoring engine with the given configuration
    pub fn new(config: MonitorConfig) -> Self {
        let alert_manager = Arc::new(alerts::AlertManager::new(
            config.min_alert_severity,
            config.alert_webhook.clone(),
        ));

        let connection = Arc::new(connection::ConnectionManager::new(
            config.ws_endpoint.clone(),
        ));

        Self {
            config,
            state: Arc::new(RwLock::new(EngineState::default())),
            alert_manager,
            connection,
        }
    }

    /// Start monitoring the configured chain
    pub async fn start(&self) -> Result<()> {
        tracing::info!("Starting monitoring engine for {}", self.config.chain_name);

        let mut state = self.state.write().await;
        if state.is_running {
            return Err(Error::ConfigError("Engine already running".to_string()));
        }
        state.is_running = true;
        drop(state);

        // Connect to the Substrate node with automatic retry
        let connect_result = if self.config.max_reconnect_attempts > 0 {
            self.connection.connect_with_retry(self.config.max_reconnect_attempts).await
        } else {
            self.connection.connect().await
        };

        if let Err(e) = connect_result {
            // Reset is_running flag on connection failure
            let mut state = self.state.write().await;
            state.is_running = false;
            return Err(e);
        }

        // Initialize detectors
        let detectors = self.initialize_detectors();

        // Start monitoring tasks
        if self.config.enable_mempool {
            self.start_mempool_monitoring(detectors.clone()).await?;
        }

        if self.config.enable_blocks {
            self.start_block_monitoring(detectors.clone()).await?;
        }

        if self.config.enable_events {
            self.start_event_monitoring(detectors).await?;
        }

        tracing::info!("Monitoring engine started successfully");
        Ok(())
    }

    /// Stop the monitoring engine
    pub async fn stop(&self) -> Result<()> {
        tracing::info!("Stopping monitoring engine");

        let mut state = self.state.write().await;
        state.is_running = false;
        drop(state);

        // Disconnect from the node
        self.connection.disconnect().await;

        tracing::info!("Monitoring engine stopped");
        Ok(())
    }

    /// Get current engine statistics
    pub async fn get_stats(&self) -> EngineStats {
        let state = self.state.read().await;
        EngineStats {
            is_running: state.is_running,
            blocks_processed: state.blocks_processed,
            transactions_analyzed: state.transactions_analyzed,
            alerts_triggered: state.alerts_triggered,
        }
    }

    /// Initialize attack pattern detectors
    fn initialize_detectors(&self) -> Arc<Vec<Box<dyn detectors::Detector + Send + Sync>>> {
        let detectors: Vec<Box<dyn detectors::Detector + Send + Sync>> = vec![
            Box::new(detectors::FlashLoanDetector::new()),
            Box::new(detectors::MevDetector::new()),
            Box::new(detectors::VolumeAnomalyDetector::new()),
            Box::new(detectors::FrontRunningDetector::new()),
        ];

        Arc::new(detectors)
    }

    /// Start mempool monitoring
    async fn start_mempool_monitoring(
        &self,
        _detectors: Arc<Vec<Box<dyn detectors::Detector + Send + Sync>>>,
    ) -> Result<()> {
        tracing::info!("Starting mempool monitoring");
        // TODO: Implement mempool monitoring using subxt
        Ok(())
    }

    /// Start block monitoring
    async fn start_block_monitoring(
        &self,
        _detectors: Arc<Vec<Box<dyn detectors::Detector + Send + Sync>>>,
    ) -> Result<()> {
        tracing::info!("Starting block monitoring");

        let client = self.connection.get_client().await
            .ok_or_else(|| Error::ConnectionError("Not connected to node".to_string()))?;

        let state = self.state.clone();
        let chain_name = self.config.chain_name.clone();

        // Spawn background task for block subscription
        tokio::spawn(async move {
            match Self::subscribe_to_blocks(client, state, chain_name).await {
                Ok(_) => tracing::info!("Block subscription ended"),
                Err(e) => tracing::error!("Block subscription error: {}", e),
            }
        });

        Ok(())
    }

    /// Subscribe to finalized blocks
    async fn subscribe_to_blocks(
        client: subxt::OnlineClient<subxt::PolkadotConfig>,
        state: Arc<RwLock<EngineState>>,
        chain_name: String,
    ) -> Result<()> {
        tracing::info!("Subscribing to finalized blocks on {}", chain_name);

        // Create transaction extractor
        let extractor = Arc::new(transaction::TransactionExtractor::new(Arc::new(client.clone())));

        let mut blocks_sub = client
            .blocks()
            .subscribe_finalized()
            .await
            .map_err(|e| Error::SubscriptionError(format!("Failed to subscribe to blocks: {}", e)))?;

        while let Some(block_result) = blocks_sub.next().await {
            match block_result {
                Ok(block) => {
                    let block_number = block.number();
                    let block_hash = block.hash();

                    tracing::debug!(
                        "Received block #{} (hash: {:?}) on {}",
                        block_number,
                        block_hash,
                        chain_name
                    );

                    // Update block statistics
                    let mut state_lock = state.write().await;
                    state_lock.blocks_processed += 1;
                    drop(state_lock);

                    // Extract transactions from block
                    match extractor.extract_from_block(block_hash, block.number() as u64).await {
                        Ok(transactions) => {
                            if !transactions.is_empty() {
                                tracing::debug!(
                                    "Extracted {} transactions from block #{}",
                                    transactions.len(),
                                    block_number
                                );

                                // Update transaction statistics
                                let mut state_lock = state.write().await;
                                state_lock.transactions_analyzed += transactions.len() as u64;
                                drop(state_lock);

                                // TODO: Pass transactions to detectors
                                // for tx in transactions {
                                //     Self::process_transaction(tx, &detectors).await?;
                                // }
                            }
                        }
                        Err(e) => {
                            tracing::warn!(
                                "Failed to extract transactions from block #{}: {}",
                                block_number,
                                e
                            );
                        }
                    }
                }
                Err(e) => {
                    tracing::error!("Error receiving block: {}", e);
                    return Err(Error::SubscriptionError(format!("Block stream error: {}", e)));
                }
            }
        }

        Ok(())
    }

    /// Start event monitoring
    async fn start_event_monitoring(
        &self,
        _detectors: Arc<Vec<Box<dyn detectors::Detector + Send + Sync>>>,
    ) -> Result<()> {
        tracing::info!("Starting event monitoring");

        let client = self.connection.get_client().await
            .ok_or_else(|| Error::ConnectionError("Not connected to node".to_string()))?;

        let chain_name = self.config.chain_name.clone();

        // Spawn background task for event subscription
        tokio::spawn(async move {
            match Self::subscribe_to_events(client, chain_name).await {
                Ok(_) => tracing::info!("Event subscription ended"),
                Err(e) => tracing::error!("Event subscription error: {}", e),
            }
        });

        Ok(())
    }

    /// Subscribe to runtime events
    async fn subscribe_to_events(
        client: subxt::OnlineClient<subxt::PolkadotConfig>,
        chain_name: String,
    ) -> Result<()> {
        tracing::info!("Subscribing to runtime events on {}", chain_name);

        let mut blocks_sub = client
            .blocks()
            .subscribe_finalized()
            .await
            .map_err(|e| Error::SubscriptionError(format!("Failed to subscribe to blocks for events: {}", e)))?;

        while let Some(block_result) = blocks_sub.next().await {
            match block_result {
                Ok(block) => {
                    let block_number = block.number();

                    // Get events from this block
                    match block.events().await {
                        Ok(events) => {
                            let event_count = events.iter().count();

                            if event_count > 0 {
                                tracing::debug!(
                                    "Block #{} on {} contains {} events",
                                    block_number,
                                    chain_name,
                                    event_count
                                );
                            }

                            // TODO: Process events and pass to detectors
                            // for event in events.iter() {
                            //     match event {
                            //         Ok(event_details) => {
                            //             // Process event
                            //         }
                            //         Err(e) => {
                            //             tracing::warn!("Error decoding event: {}", e);
                            //         }
                            //     }
                            // }
                        }
                        Err(e) => {
                            tracing::error!("Error fetching events for block #{}: {}", block_number, e);
                        }
                    }
                }
                Err(e) => {
                    tracing::error!("Error receiving block for events: {}", e);
                    return Err(Error::SubscriptionError(format!("Event stream error: {}", e)));
                }
            }
        }

        Ok(())
    }
}

/// Engine statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineStats {
    pub is_running: bool,
    pub blocks_processed: u64,
    pub transactions_analyzed: u64,
    pub alerts_triggered: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_engine_creation() {
        let config = MonitorConfig::default();
        let engine = MonitoringEngine::new(config);

        let stats = engine.get_stats().await;
        assert!(!stats.is_running);
        assert_eq!(stats.blocks_processed, 0);
    }

    #[tokio::test]
    async fn test_engine_start_stop() {
        let config = MonitorConfig::default();
        let engine = MonitoringEngine::new(config);

        let result = engine.stop().await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_default_config() {
        let config = MonitorConfig::default();
        assert_eq!(config.ws_endpoint, "ws://localhost:9944");
        assert_eq!(config.chain_name, "local");
        assert!(config.enable_mempool);
    }
}
