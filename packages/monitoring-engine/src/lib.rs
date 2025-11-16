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
pub mod config;
pub mod database;
pub mod ml;

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
        Self::westend()
    }
}

/// Chain configuration presets
impl MonitorConfig {
    /// Westend testnet configuration
    pub fn westend() -> Self {
        Self {
            ws_endpoint: "wss://westend-rpc.polkadot.io".to_string(),
            chain_name: "westend".to_string(),
            enable_mempool: true,
            enable_blocks: true,
            enable_events: true,
            alert_webhook: None,
            min_alert_severity: AlertSeverity::Medium,
            buffer_size: 1000,
            max_reconnect_attempts: 5,
        }
    }

    /// Asset Hub (Westend) configuration
    pub fn asset_hub() -> Self {
        Self {
            ws_endpoint: "wss://westend-asset-hub-rpc.polkadot.io".to_string(),
            chain_name: "asset-hub".to_string(),
            enable_mempool: true,
            enable_blocks: true,
            enable_events: true,
            alert_webhook: None,
            min_alert_severity: AlertSeverity::Medium,
            buffer_size: 1000,
            max_reconnect_attempts: 5,
        }
    }

    /// Polkadot mainnet configuration
    pub fn polkadot() -> Self {
        Self {
            ws_endpoint: "wss://rpc.polkadot.io".to_string(),
            chain_name: "polkadot".to_string(),
            enable_mempool: true,
            enable_blocks: true,
            enable_events: true,
            alert_webhook: None,
            min_alert_severity: AlertSeverity::Medium,
            buffer_size: 1000,
            max_reconnect_attempts: 5,
        }
    }

    /// Kusama configuration
    pub fn kusama() -> Self {
        Self {
            ws_endpoint: "wss://kusama-rpc.polkadot.io".to_string(),
            chain_name: "kusama".to_string(),
            enable_mempool: true,
            enable_blocks: true,
            enable_events: true,
            alert_webhook: None,
            min_alert_severity: AlertSeverity::Medium,
            buffer_size: 1000,
            max_reconnect_attempts: 5,
        }
    }

    /// Get chain config by name
    pub fn from_chain_name(name: &str) -> Option<Self> {
        match name.to_lowercase().as_str() {
            "westend" => Some(Self::westend()),
            "asset-hub" | "asset_hub" | "assethub" => Some(Self::asset_hub()),
            "polkadot" => Some(Self::polkadot()),
            "kusama" => Some(Self::kusama()),
            _ => None,
        }
    }

    /// Get list of available chain presets
    pub fn available_chains() -> Vec<ChainInfo> {
        vec![
            ChainInfo {
                name: "westend".to_string(),
                display_name: "Westend Testnet".to_string(),
                endpoint: "wss://westend-rpc.polkadot.io".to_string(),
                description: "Polkadot's primary testnet for protocol development".to_string(),
            },
            ChainInfo {
                name: "asset-hub".to_string(),
                display_name: "Asset Hub (Westend)".to_string(),
                endpoint: "wss://westend-asset-hub-rpc.polkadot.io".to_string(),
                description: "Asset management parachain on Westend".to_string(),
            },
            ChainInfo {
                name: "polkadot".to_string(),
                display_name: "Polkadot Mainnet".to_string(),
                endpoint: "wss://rpc.polkadot.io".to_string(),
                description: "Polkadot relay chain (production network)".to_string(),
            },
            ChainInfo {
                name: "kusama".to_string(),
                display_name: "Kusama".to_string(),
                endpoint: "wss://kusama-rpc.polkadot.io".to_string(),
                description: "Polkadot's canary network".to_string(),
            },
        ]
    }
}

/// Chain information for API responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainInfo {
    pub name: String,
    pub display_name: String,
    pub endpoint: String,
    pub description: String,
}

/// Main monitoring engine
pub struct MonitoringEngine {
    pub config: MonitorConfig,
    state: Arc<RwLock<EngineState>>,
    pub alert_manager: Arc<alerts::AlertManager>,
    pub connection: Arc<connection::ConnectionManager>,
    pub database: Option<Arc<database::DatabaseClient>>,
}

/// Internal engine state
struct EngineState {
    is_running: bool,
    blocks_processed: u64,
    transactions_analyzed: u64,
    alerts_triggered: u64,
    detector_stats: std::collections::HashMap<String, DetectorStatsInternal>,
    feature_extractor: ml::FeatureExtractor,
}

#[derive(Debug, Default, Clone)]
struct DetectorStatsInternal {
    detections: u64,
    last_detection: Option<u64>,
}

impl Default for EngineState {
    fn default() -> Self {
        let mut detector_stats = std::collections::HashMap::new();
        detector_stats.insert("Flash Loan Detector".to_string(), DetectorStatsInternal::default());
        detector_stats.insert("MEV Detector".to_string(), DetectorStatsInternal::default());
        detector_stats.insert("Volume Anomaly Detector".to_string(), DetectorStatsInternal::default());
        detector_stats.insert("FrontRunning Detector".to_string(), DetectorStatsInternal::default());
        detector_stats.insert("Cross-Chain Bridge Detector".to_string(), DetectorStatsInternal::default());
        detector_stats.insert("State Proof Verification Detector".to_string(), DetectorStatsInternal::default());
        detector_stats.insert("Omnipool Manipulation Detector".to_string(), DetectorStatsInternal::default());
        detector_stats.insert("Liquidity Drain Detector".to_string(), DetectorStatsInternal::default());
        detector_stats.insert("Collateral Manipulation Detector".to_string(), DetectorStatsInternal::default());

        Self {
            is_running: false,
            blocks_processed: 0,
            transactions_analyzed: 0,
            alerts_triggered: 0,
            detector_stats,
            feature_extractor: ml::FeatureExtractor::new(),
        }
    }
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
            database: None,
        }
    }

    /// Create a new monitoring engine with database support
    pub fn with_database(config: MonitorConfig, database: Arc<database::DatabaseClient>) -> Self {
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
            database: Some(database),
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

    /// Get statistics for all detectors
    pub async fn get_detector_stats(&self) -> AllDetectorStats {
        let state = self.state.read().await;
        let detectors = vec![
            DetectorStats {
                name: "Flash Loan Detector".to_string(),
                enabled: true,
                detections: state.detector_stats.get("Flash Loan Detector")
                    .map(|s| s.detections)
                    .unwrap_or(0),
                last_detection: state.detector_stats.get("Flash Loan Detector")
                    .and_then(|s| s.last_detection),
            },
            DetectorStats {
                name: "MEV Detector".to_string(),
                enabled: true,
                detections: state.detector_stats.get("MEV Detector")
                    .map(|s| s.detections)
                    .unwrap_or(0),
                last_detection: state.detector_stats.get("MEV Detector")
                    .and_then(|s| s.last_detection),
            },
            DetectorStats {
                name: "Volume Anomaly Detector".to_string(),
                enabled: true,
                detections: state.detector_stats.get("Volume Anomaly Detector")
                    .map(|s| s.detections)
                    .unwrap_or(0),
                last_detection: state.detector_stats.get("Volume Anomaly Detector")
                    .and_then(|s| s.last_detection),
            },
            DetectorStats {
                name: "FrontRunning Detector".to_string(),
                enabled: true,
                detections: state.detector_stats.get("FrontRunning Detector")
                    .map(|s| s.detections)
                    .unwrap_or(0),
                last_detection: state.detector_stats.get("FrontRunning Detector")
                    .and_then(|s| s.last_detection),
            },
            DetectorStats {
                name: "Cross-Chain Bridge Detector".to_string(),
                enabled: true,
                detections: state.detector_stats.get("Cross-Chain Bridge Detector")
                    .map(|s| s.detections)
                    .unwrap_or(0),
                last_detection: state.detector_stats.get("Cross-Chain Bridge Detector")
                    .and_then(|s| s.last_detection),
            },
            DetectorStats {
                name: "State Proof Verification Detector".to_string(),
                enabled: true,
                detections: state.detector_stats.get("State Proof Verification Detector")
                    .map(|s| s.detections)
                    .unwrap_or(0),
                last_detection: state.detector_stats.get("State Proof Verification Detector")
                    .and_then(|s| s.last_detection),
            },
        ];

        AllDetectorStats { detectors }
    }

    /// Initialize attack pattern detectors
    fn initialize_detectors(&self) -> Arc<Vec<Box<dyn detectors::Detector + Send + Sync>>> {
        let detectors: Vec<Box<dyn detectors::Detector + Send + Sync>> = vec![
            Box::new(detectors::FlashLoanDetector::new()),
            Box::new(detectors::MevDetector::new()),
            Box::new(detectors::VolumeAnomalyDetector::new()),
            Box::new(detectors::FrontRunningDetector::new()),
            Box::new(detectors::CrossChainBridgeDetector::new()),
            Box::new(detectors::StateProofVerificationDetector::new()),
            Box::new(detectors::OmnipoolManipulationDetector::new()),
            Box::new(detectors::LiquidityDrainDetector::new()),
            Box::new(detectors::CollateralManipulationDetector::new()),
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
        detectors: Arc<Vec<Box<dyn detectors::Detector + Send + Sync>>>,
    ) -> Result<()> {
        tracing::info!("Starting block monitoring");

        let client = self.connection.get_client().await
            .ok_or_else(|| Error::ConnectionError("Not connected to node".to_string()))?;

        let state = self.state.clone();
        let chain_name = self.config.chain_name.clone();
        let alert_manager = self.alert_manager.clone();
        let database = self.database.clone();

        // Spawn background task for block subscription
        tokio::spawn(async move {
            match Self::subscribe_to_blocks(client, state, chain_name, detectors, alert_manager, database).await {
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
        detectors: Arc<Vec<Box<dyn detectors::Detector + Send + Sync>>>,
        alert_manager: Arc<alerts::AlertManager>,
        database: Option<Arc<database::DatabaseClient>>,
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

                    tracing::info!(
                        "Processing block #{} (hash: 0x{}) on {}",
                        block_number,
                        hex::encode(&block_hash.0[..8]),
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
                                tracing::info!(
                                    "Extracted {} transactions from block #{}",
                                    transactions.len(),
                                    block_number
                                );

                                // Update transaction statistics
                                let mut state_lock = state.write().await;
                                state_lock.transactions_analyzed += transactions.len() as u64;
                                drop(state_lock);

                                // Process each transaction through detectors
                                for tx in transactions {
                                    Self::process_transaction(
                                        tx,
                                        &detectors,
                                        &state,
                                        &alert_manager,
                                        &chain_name,
                                        &database
                                    ).await;
                                }
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

    /// Process a transaction through all detectors
    async fn process_transaction(
        tx: ParsedTransaction,
        detectors: &[Box<dyn detectors::Detector + Send + Sync>],
        state: &Arc<RwLock<EngineState>>,
        alert_manager: &Arc<alerts::AlertManager>,
        chain_name: &str,
        database: &Option<Arc<database::DatabaseClient>>,
    ) {
        // Store transaction in database if available
        if let Some(db) = database {
            // Convert args bytes to JSON Value
            let args_json = if !tx.args.is_empty() {
                match serde_json::from_slice::<serde_json::Value>(&tx.args) {
                    Ok(value) => Some(value),
                    Err(_) => {
                        // If parsing fails, store as hex string
                        Some(serde_json::json!({
                            "raw": hex::encode(&tx.args)
                        }))
                    }
                }
            } else {
                None
            };

            let db_tx = database::models::Transaction {
                timestamp: chrono::Utc::now(),
                tx_hash: tx.hash.clone(),
                block_number: tx.block_number as i64,
                chain: chain_name.to_string(),
                pallet: tx.pallet.clone(),
                call_name: tx.call.clone(),
                caller: tx.caller.clone(),
                success: tx.success,
                args: args_json,
                gas_used: None,  // TODO: Extract from transaction
                fee_paid: None,  // TODO: Extract from transaction
            };

            if let Err(e) = db.insert_transaction(&db_tx).await {
                tracing::warn!("Failed to store transaction in database: {}", e);
            }
        }

        // Create transaction context (simplified - no events/state changes for now)
        let ctx = TransactionContext {
            transaction: tx.clone(),
            events: vec![],
            state_changes: vec![],
        };

        // Extract ML features and store in database
        if database.is_some() {
            let mut state_lock = state.write().await;
            let features = state_lock.feature_extractor.extract_features(&ctx);
            drop(state_lock);

            if let Some(db) = database {
                if let Err(e) = db.insert_ml_features(&features).await {
                    tracing::warn!("Failed to store ML features in database: {}", e);
                }
            }
        }

        // Run all detectors
        for detector in detectors {
            let result = detector.analyze_transaction(&ctx).await;

            if result.detected && result.confidence > 0.5 {
                let detector_name = detector.name();
                tracing::warn!(
                    "🚨 {} detected suspicious activity in tx {}",
                    detector_name,
                    tx.hash
                );
                tracing::warn!("   Confidence: {:.2}%", result.confidence * 100.0);
                tracing::warn!("   Description: {}", result.description);
                tracing::warn!("   Evidence: {:?}", result.evidence);

                // Determine severity based on confidence
                let severity = if result.confidence >= 0.9 {
                    AlertSeverity::Critical
                } else if result.confidence >= 0.75 {
                    AlertSeverity::High
                } else if result.confidence >= 0.6 {
                    AlertSeverity::Medium
                } else {
                    AlertSeverity::Low
                };

                // Update detector statistics
                let mut state_lock = state.write().await;
                if let Some(detector_stat) = state_lock.detector_stats.get_mut(detector_name) {
                    detector_stat.detections += 1;
                    detector_stat.last_detection = Some(
                        std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap()
                            .as_secs()
                    );
                }
                state_lock.alerts_triggered += 1;
                drop(state_lock);

                // Create recommended actions from evidence
                let recommended_actions = if !result.evidence.is_empty() {
                    vec![
                        "Review transaction details and evidence".to_string(),
                        format!("Investigate pattern: {}", result.pattern),
                        "Monitor related transactions from same sender".to_string(),
                    ]
                } else {
                    vec!["Review transaction for suspicious activity".to_string()]
                };

                // Create and trigger alert
                let alert_id = uuid::Uuid::new_v4().to_string();
                let alert = Alert {
                    id: alert_id.clone(),
                    timestamp: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs(),
                    severity: severity.clone(),
                    pattern: result.pattern.clone(),
                    description: result.description.clone(),
                    transaction_hash: Some(tx.hash.clone()),
                    block_number: Some(tx.block_number),
                    chain: chain_name.to_string(),
                    metadata: std::collections::HashMap::new(),
                    recommended_actions,
                    acknowledged: false,
                };

                // Store detection in database if available
                if let Some(db) = database {
                    let detection = database::models::Detection {
                        timestamp: chrono::Utc::now(),
                        detection_id: alert_id.clone(),
                        tx_hash: tx.hash.clone(),
                        detector_name: detector_name.to_string(),
                        attack_pattern: result.pattern.clone().to_string(),
                        confidence: result.confidence,
                        severity: match severity {
                            AlertSeverity::Critical => "critical",
                            AlertSeverity::High => "high",
                            AlertSeverity::Medium => "medium",
                            AlertSeverity::Low => "low",
                        }.to_string(),
                        description: Some(result.description.clone()),
                        evidence: Some(serde_json::json!(result.evidence)),
                        metadata: None,
                        acknowledged: false,
                    };

                    if let Err(e) = db.insert_detection(&detection).await {
                        tracing::warn!("Failed to store detection in database: {}", e);
                    }
                }

                alert_manager.trigger_alert(alert).await;
            }
        }
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

/// Statistics for a specific detector
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectorStats {
    pub name: String,
    pub enabled: bool,
    pub detections: u64,
    pub last_detection: Option<u64>, // Unix timestamp
}

/// Collection of all detector statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllDetectorStats {
    pub detectors: Vec<DetectorStats>,
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
        assert_eq!(config.ws_endpoint, "wss://westend-rpc.polkadot.io");
        assert_eq!(config.chain_name, "westend");
        assert!(config.enable_mempool);
    }
}
