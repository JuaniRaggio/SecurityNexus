//! Monitoring Engine Binary

use monitoring_engine::{MonitorConfig, MonitoringEngine, api::start_api_server};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set tracing subscriber");

    tracing::info!("Starting Polkadot Security Nexus - Monitoring Engine");

    // Create configuration
    let config = MonitorConfig {
        ws_endpoint: std::env::var("WS_ENDPOINT")
            .unwrap_or_else(|_| "wss://westend-rpc.polkadot.io".to_string()),
        chain_name: std::env::var("CHAIN_NAME")
            .unwrap_or_else(|_| "westend".to_string()),
        enable_mempool: true,
        enable_blocks: true,
        enable_events: true,
        alert_webhook: std::env::var("ALERT_WEBHOOK").ok(),
        ..Default::default()
    };

    tracing::info!("Configuration:");
    tracing::info!("  WebSocket: {}", config.ws_endpoint);
    tracing::info!("  Chain: {}", config.chain_name);
    tracing::info!("  Mempool monitoring: {}", config.enable_mempool);
    tracing::info!("  Block monitoring: {}", config.enable_blocks);
    tracing::info!("  Event monitoring: {}", config.enable_events);

    // Create and start monitoring engine
    let engine = Arc::new(MonitoringEngine::new(config));

    match engine.start().await {
        Ok(_) => {
            tracing::info!("Monitoring engine started.");

            // Start API server
            let api_bind = std::env::var("API_BIND_ADDRESS")
                .unwrap_or_else(|_| "0.0.0.0:8080".to_string());

            tracing::info!("Press Ctrl+C to stop.");

            // Run API server (blocks until shutdown)
            let api_result = tokio::select! {
                result = start_api_server(engine.clone(), &api_bind) => {
                    result
                }
                _ = tokio::signal::ctrl_c() => {
                    tracing::info!("Shutdown signal received");
                    Ok(())
                }
            };

            // Stop the monitoring engine
            engine.stop().await?;

            // Print final statistics
            let stats = engine.get_stats().await;
            tracing::info!("Final statistics:");
            tracing::info!("  Blocks processed: {}", stats.blocks_processed);
            tracing::info!("  Transactions analyzed: {}", stats.transactions_analyzed);
            tracing::info!("  Alerts triggered: {}", stats.alerts_triggered);

            api_result?;
            Ok(())
        }
        Err(e) => {
            tracing::error!("Failed to start monitoring engine: {}", e);
            Err(e.into())
        }
    }
}
