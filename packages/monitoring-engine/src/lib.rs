//! Monitoring Engine
//!
//! Real-time security monitoring for Polkadot parachains.
//! Detects suspicious patterns, potential attacks, and anomalies in blockchain activity.

pub mod detectors;
pub mod mempool;
pub mod alerts;
pub mod types;
pub mod connection;

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::RwLock;

pub use types::{Alert, AlertSeverity, AttackPattern, ChainEvent, DetectionResult, Transaction};

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
        }
    }
}

/// Main monitoring engine
pub struct MonitoringEngine {
    config: MonitorConfig,
    state: Arc<RwLock<EngineState>>,
    alert_manager: Arc<alerts::AlertManager>,
    connection: Arc<connection::ConnectionManager>,
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

        // Connect to the Substrate node
        if let Err(e) = self.connection.connect().await {
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
        let mut detectors: Vec<Box<dyn detectors::Detector + Send + Sync>> = Vec::new();

        // Add flash loan detector
        detectors.push(Box::new(detectors::FlashLoanDetector::new()));

        // Add MEV detector
        detectors.push(Box::new(detectors::MevDetector::new()));

        // Add unusual volume detector
        detectors.push(Box::new(detectors::VolumeAnomalyDetector::new()));

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
        // TODO: Implement block monitoring using subxt
        Ok(())
    }

    /// Start event monitoring
    async fn start_event_monitoring(
        &self,
        _detectors: Arc<Vec<Box<dyn detectors::Detector + Send + Sync>>>,
    ) -> Result<()> {
        tracing::info!("Starting event monitoring");
        // TODO: Implement event monitoring using subxt
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
