//! Example: Monitoring Engine with REST API Server
//!
//! This example demonstrates how to run the monitoring engine
//! with a REST API server for dashboard integration.
//!
//! Usage:
//!   cargo run --example api_server
//!
//! API Endpoints:
//!   GET http://localhost:8080/api/health - Health check
//!   GET http://localhost:8080/api/stats  - Monitoring statistics
//!
//! Environment Variables:
//!   WS_ENDPOINT - Substrate node endpoint (default: ws://127.0.0.1:9944)
//!   CHAIN_NAME  - Chain identifier (default: development)
//!   API_PORT    - API server port (default: 8080)

use monitoring_engine::{MonitorConfig, MonitoringEngine, AlertSeverity};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    tracing::info!("Starting Monitoring Engine with REST API");

    // Get configuration from environment or use defaults
    let ws_endpoint = std::env::var("WS_ENDPOINT")
        .unwrap_or_else(|_| "ws://127.0.0.1:9944".to_string());
    let chain_name = std::env::var("CHAIN_NAME")
        .unwrap_or_else(|_| "development".to_string());
    let api_port = std::env::var("API_PORT")
        .unwrap_or_else(|_| "8080".to_string());

    // Create monitoring configuration
    let config = MonitorConfig {
        ws_endpoint,
        chain_name,
        enable_mempool: false, // Not implemented yet
        enable_blocks: true,
        enable_events: true,
        alert_webhook: None,
        min_alert_severity: AlertSeverity::Low,
        buffer_size: 1000,
        max_reconnect_attempts: 5,
    };

    tracing::info!("Configuration:");
    tracing::info!("  Endpoint: {}", config.ws_endpoint);
    tracing::info!("  Chain: {}", config.chain_name);
    tracing::info!("  API Port: {}", api_port);

    // Create and start monitoring engine
    let engine = Arc::new(MonitoringEngine::new(config));

    tracing::info!("Starting monitoring engine...");
    match engine.start().await {
        Ok(_) => tracing::info!("Monitoring engine started successfully"),
        Err(e) => {
            tracing::error!("Failed to start monitoring engine: {}", e);
            tracing::info!("Continuing with API server in offline mode...");
        }
    }

    // Print initial stats
    let stats = engine.get_stats().await;
    tracing::info!("Initial stats:");
    tracing::info!("  Running: {}", stats.is_running);
    tracing::info!("  Blocks processed: {}", stats.blocks_processed);
    tracing::info!("  Transactions analyzed: {}", stats.transactions_analyzed);
    tracing::info!("  Alerts triggered: {}", stats.alerts_triggered);

    // Start API server
    let bind_address = format!("0.0.0.0:{}", api_port);
    tracing::info!("Starting API server on {}", bind_address);
    tracing::info!("Dashboard should connect to: http://localhost:{}", api_port);
    tracing::info!("");
    tracing::info!("API Endpoints:");
    tracing::info!("  http://localhost:{}/api/health", api_port);
    tracing::info!("  http://localhost:{}/api/stats", api_port);
    tracing::info!("");
    tracing::info!("Press Ctrl+C to stop");

    // Start API server (this will block)
    if let Err(e) = monitoring_engine::api::start_api_server(engine.clone(), &bind_address).await {
        tracing::error!("API server error: {}", e);
    }

    // Cleanup
    tracing::info!("Shutting down...");
    engine.stop().await?;

    Ok(())
}
