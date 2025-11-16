//! Monitoring Engine Binary

use monitoring_engine::{MonitoringEngine, api::start_api_server, config, database::DatabaseClient};
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

    // Load configuration from saved file, or use default
    let mut config = config::load_monitor_config();

    // Allow environment variables to override saved configuration
    if let Ok(ws_endpoint) = std::env::var("WS_ENDPOINT") {
        tracing::info!("Overriding WebSocket endpoint from environment variable");
        config.ws_endpoint = ws_endpoint;
    }
    if let Ok(chain_name) = std::env::var("CHAIN_NAME") {
        tracing::info!("Overriding chain name from environment variable");
        config.chain_name = chain_name;
    }
    if let Ok(webhook) = std::env::var("ALERT_WEBHOOK") {
        config.alert_webhook = Some(webhook);
    }

    tracing::info!("Configuration:");
    tracing::info!("  WebSocket: {}", config.ws_endpoint);
    tracing::info!("  Chain: {}", config.chain_name);
    tracing::info!("  Mempool monitoring: {}", config.enable_mempool);
    tracing::info!("  Block monitoring: {}", config.enable_blocks);
    tracing::info!("  Event monitoring: {}", config.enable_events);

    // Initialize database client if DATABASE_URL is provided
    let database = if let Ok(database_url) = std::env::var("DATABASE_URL") {
        let max_connections = std::env::var("DATABASE_MAX_CONNECTIONS")
            .unwrap_or_else(|_| "10".to_string())
            .parse::<usize>()
            .unwrap_or(10);

        match DatabaseClient::new(&database_url, max_connections).await {
            Ok(client) => {
                tracing::info!("Successfully connected to TimescaleDB");
                Some(Arc::new(client))
            }
            Err(e) => {
                tracing::warn!("Failed to connect to database: {}. Running without database support.", e);
                None
            }
        }
    } else {
        tracing::info!("DATABASE_URL not provided. Running without database support.");
        None
    };

    // Create and start monitoring engine
    let engine = Arc::new(if let Some(db) = database {
        MonitoringEngine::with_database(config, db)
    } else {
        MonitoringEngine::new(config)
    });

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
