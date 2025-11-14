//! Monitoring Engine Binary

use monitoring_engine::{MonitorConfig, MonitoringEngine};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

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
            .unwrap_or_else(|_| "ws://localhost:9944".to_string()),
        chain_name: std::env::var("CHAIN_NAME")
            .unwrap_or_else(|_| "polkadot-local".to_string()),
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
    let engine = MonitoringEngine::new(config);

    match engine.start().await {
        Ok(_) => {
            tracing::info!("Monitoring engine running. Press Ctrl+C to stop.");

            // Wait for shutdown signal
            tokio::signal::ctrl_c().await?;

            tracing::info!("Shutdown signal received");
            engine.stop().await?;

            // Print final statistics
            let stats = engine.get_stats().await;
            tracing::info!("Final statistics:");
            tracing::info!("  Blocks processed: {}", stats.blocks_processed);
            tracing::info!("  Transactions analyzed: {}", stats.transactions_analyzed);
            tracing::info!("  Alerts triggered: {}", stats.alerts_triggered);

            Ok(())
        }
        Err(e) => {
            tracing::error!("Failed to start monitoring engine: {}", e);
            Err(e.into())
        }
    }
}
