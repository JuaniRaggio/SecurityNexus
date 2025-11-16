//! Integration tests for transaction extraction
//!
//! These tests verify that we can correctly extract and parse transactions
//! from Substrate blocks.

use monitoring_engine::MonitoringEngine;
use std::sync::Arc;

mod common;
use common::test_config;

#[tokio::test]
async fn test_extract_transactions_from_block() {
    skip_if_no_chain!();

    // Given: A monitoring engine connected to a chain
    let config = common::test_config();
    let engine = Arc::new(MonitoringEngine::new(config));

    // Start the engine
    engine.start().await.expect("Failed to start engine");

    // Wait for at least one block to be processed
    tokio::time::sleep(tokio::time::Duration::from_secs(15)).await;

    // When: We check the stats
    let stats = engine.get_stats().await;

    // Then: We should have processed blocks AND extracted transactions
    assert!(stats.blocks_processed > 0, "No blocks processed");

    // We expect at least SOME transactions (system extrinsics count)
    // Even empty blocks have inherent extrinsics (timestamp, etc)
    assert!(
        stats.transactions_analyzed >= stats.blocks_processed,
        "Expected at least one transaction per block (inherents)"
    );

    // Cleanup
    engine.stop().await.expect("Failed to stop engine");
}

#[tokio::test]
async fn test_transaction_has_metadata() {
    skip_if_no_chain!();

    // This test verifies that extracted transactions have proper metadata
    // We'll need to add a method to get recent transactions for this

    // Given: Engine running
    let config = common::test_config();
    let engine = Arc::new(MonitoringEngine::new(config));
    engine.start().await.expect("Failed to start engine");

    // Wait for blocks
    tokio::time::sleep(tokio::time::Duration::from_secs(15)).await;

    let stats = engine.get_stats().await;

    // Then: Transactions should be analyzed
    assert!(stats.transactions_analyzed > 0, "No transactions analyzed");

    engine.stop().await.expect("Failed to stop engine");
}

#[tokio::test]
async fn test_transaction_statistics_increment() {
    skip_if_no_chain!();

    // Given: Engine starts with zero transactions
    let config = common::test_config();
    let engine = Arc::new(MonitoringEngine::new(config));

    let initial_stats = engine.get_stats().await;
    assert_eq!(initial_stats.transactions_analyzed, 0);

    // When: We start processing blocks
    engine.start().await.expect("Failed to start engine");
    tokio::time::sleep(tokio::time::Duration::from_secs(20)).await;

    // Then: Transaction count should increase
    let final_stats = engine.get_stats().await;
    assert!(
        final_stats.transactions_analyzed > initial_stats.transactions_analyzed,
        "Transaction count should increase"
    );

    engine.stop().await.expect("Failed to stop engine");
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test_transaction_context_creation() {
        // Unit test for creating transaction context
        // This will be implemented when we add the TransactionExtractor

        // For now, this is a placeholder that should compile
        assert!(true);
    }

    #[test]
    fn test_parse_transaction_metadata() {
        // Unit test for parsing transaction metadata
        // Will be implemented with TransactionExtractor

        assert!(true);
    }
}
