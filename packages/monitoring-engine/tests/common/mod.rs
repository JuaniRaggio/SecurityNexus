// Common test utilities and helpers

use monitoring_engine::*;
use std::time::Duration;
use tokio::time::sleep;

/// Test configuration for local development chain
pub fn test_config() -> MonitorConfig {
    MonitorConfig {
        ws_endpoint: "ws://127.0.0.1:9944".to_string(),
        chain_name: "development".to_string(),
        enable_mempool: true,
        enable_blocks: true,
        enable_events: true,
        alert_webhook: None,
        min_alert_severity: AlertSeverity::Low,
        buffer_size: 100,
        max_reconnect_attempts: 3,
    }
}

/// Wait for the monitoring engine to start
pub async fn wait_for_engine_start() {
    sleep(Duration::from_millis(500)).await;
}

/// Create a test alert
#[allow(dead_code)]
pub fn create_test_alert(severity: AlertSeverity, pattern: AttackPattern) -> Alert {
    use std::collections::HashMap;

    let description = format!("Test alert for {:?}", pattern);

    Alert {
        id: uuid::Uuid::new_v4().to_string(),
        timestamp: chrono::Utc::now().timestamp() as u64,
        chain: "test-chain".to_string(),
        severity,
        pattern,
        description,
        transaction_hash: Some("0x1234".to_string()),
        block_number: Some(100),
        metadata: HashMap::new(),
        recommended_actions: vec!["Review transaction".to_string()],
    }
}

/// Check if a local development chain is running
pub async fn is_chain_running() -> bool {
    match tokio::time::timeout(
        Duration::from_secs(2),
        tokio::net::TcpStream::connect("127.0.0.1:9944")
    ).await {
        Ok(Ok(_)) => true,
        _ => false,
    }
}

/// Skip test if chain is not running
#[macro_export]
macro_rules! skip_if_no_chain {
    () => {
        if !common::is_chain_running().await {
            eprintln!("⚠️  Skipping test: Local chain not running at ws://127.0.0.1:9944");
            eprintln!("   Start chain with: substrate-contracts-node --dev");
            return;
        }
    };
}
