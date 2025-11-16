// Integration tests for Substrate node connection
// Story 3.1: Parachain Node Connection

use monitoring_engine::*;

#[path = "common/mod.rs"]
mod common;
use common::*;

#[tokio::test]
async fn test_engine_creation_with_config() {
    let config = test_config();
    let engine = MonitoringEngine::new(config.clone());

    let stats = engine.get_stats().await;
    assert!(!stats.is_running);
    assert_eq!(stats.blocks_processed, 0);
    assert_eq!(stats.transactions_analyzed, 0);
}

#[tokio::test]
async fn test_engine_start_without_chain() {
    // This test verifies graceful handling when chain is not available
    let mut config = test_config();
    config.ws_endpoint = "ws://127.0.0.1:9999".to_string(); // Non-existent endpoint

    let engine = MonitoringEngine::new(config);

    // Engine should handle connection failure gracefully
    let result = tokio::time::timeout(
        std::time::Duration::from_secs(5),
        engine.start()
    ).await;

    // Should either timeout or return error, but not panic
    assert!(result.is_err() || result.unwrap().is_err());
}

#[tokio::test]
#[ignore] // Run with: cargo test -- --ignored --test-threads=1
async fn test_connection_to_local_chain() {
    skip_if_no_chain!();

    let config = test_config();
    let engine = MonitoringEngine::new(config);

    // This will be implemented when we add real subxt connection
    let result = engine.start().await;

    // For now, we expect this to work with the basic implementation
    assert!(result.is_ok());

    let stats = engine.get_stats().await;
    assert!(stats.is_running);

    // Clean up
    let stop_result = engine.stop().await;
    assert!(stop_result.is_ok());
}

#[tokio::test]
#[ignore]
async fn test_block_subscription() {
    skip_if_no_chain!();

    let config = test_config();
    let engine = MonitoringEngine::new(config);

    engine.start().await.expect("Failed to start engine");

    wait_for_engine_start().await;

    // Wait a bit for blocks to be processed
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;

    let stats = engine.get_stats().await;
    // In dev mode, blocks should be produced
    assert!(stats.blocks_processed > 0, "Expected blocks to be processed");

    engine.stop().await.expect("Failed to stop engine");
}

#[tokio::test]
async fn test_engine_double_start_prevention() {
    let config = test_config();
    let engine = MonitoringEngine::new(config);

    // First start should succeed (or fail gracefully if no chain)
    let _ = engine.start().await;

    // Second start should return error
    let result = engine.start().await;
    assert!(result.is_err());

    engine.stop().await.ok();
}

#[tokio::test]
async fn test_engine_stats_tracking() {
    let config = test_config();
    let engine = MonitoringEngine::new(config);

    // Initial stats
    let stats = engine.get_stats().await;
    assert_eq!(stats.blocks_processed, 0);
    assert_eq!(stats.transactions_analyzed, 0);
    assert_eq!(stats.alerts_triggered, 0);

    // Stats should be accessible even after stop
    engine.stop().await.ok();
    let stats_after = engine.get_stats().await;
    assert!(!stats_after.is_running);
}
