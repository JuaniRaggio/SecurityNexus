//! Mempool monitoring and analysis

use crate::detectors::Detector;
use crate::types::{Alert, AlertSeverity, DetectionResult, Transaction, TransactionContext};
use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Mempool monitor tracks pending transactions and analyzes for attacks
pub struct MempoolMonitor {
    pending_txs: RwLock<VecDeque<Transaction>>,
    max_size: usize,
    /// Detectors for analyzing mempool transactions
    detectors: Arc<Vec<Box<dyn Detector + Send + Sync>>>,
    /// Alert manager for triggering security alerts
    alert_manager: Option<Arc<crate::alerts::AlertManager>>,
}

impl MempoolMonitor {
    /// Create a new mempool monitor
    pub fn new(max_size: usize) -> Self {
        Self {
            pending_txs: RwLock::new(VecDeque::with_capacity(max_size)),
            max_size,
            detectors: Arc::new(Vec::new()),
            alert_manager: None,
        }
    }

    /// Create a mempool monitor with detectors
    pub fn with_detectors(
        max_size: usize,
        detectors: Arc<Vec<Box<dyn Detector + Send + Sync>>>,
        alert_manager: Option<Arc<crate::alerts::AlertManager>>,
    ) -> Self {
        Self {
            pending_txs: RwLock::new(VecDeque::with_capacity(max_size)),
            max_size,
            detectors,
            alert_manager,
        }
    }

    /// Add a transaction to the mempool
    pub async fn add_transaction(&self, tx: Transaction) {
        let mut pending = self.pending_txs.write().await;

        // Remove oldest if at capacity
        if pending.len() >= self.max_size {
            pending.pop_front();
        }

        pending.push_back(tx);
    }

    /// Get all pending transactions
    pub async fn get_pending_transactions(&self) -> Vec<Transaction> {
        let pending = self.pending_txs.read().await;
        pending.iter().cloned().collect()
    }

    /// Get pending transaction count
    pub async fn pending_count(&self) -> usize {
        let pending = self.pending_txs.read().await;
        pending.len()
    }

    /// Clear confirmed transactions
    pub async fn clear_confirmed(&self, confirmed_hashes: &[String]) {
        let mut pending = self.pending_txs.write().await;
        pending.retain(|tx| !confirmed_hashes.contains(&tx.hash));
    }

    /// Clear all pending transactions
    pub async fn clear_all(&self) {
        let mut pending = self.pending_txs.write().await;
        pending.clear();
    }

    /// Analyze a transaction context with all enabled detectors
    pub async fn analyze_transaction(&self, ctx: &TransactionContext) -> Vec<DetectionResult> {
        let mut results = Vec::new();

        for detector in self.detectors.iter() {
            if detector.is_enabled() {
                let result = detector.analyze_transaction(ctx).await;

                if result.detected {
                    tracing::info!(
                        "Mempool detector '{}' found pattern: {} (confidence: {:.2}%)",
                        detector.name(),
                        result.pattern,
                        result.confidence * 100.0
                    );

                    // Trigger alert if alert manager is configured
                    if let Some(alert_manager) = &self.alert_manager {
                        let severity = Self::confidence_to_severity(result.confidence);
                        let alert = Self::detection_to_alert(&result, ctx, severity);
                        alert_manager.trigger_alert(alert).await;
                    }
                }

                results.push(result);
            }
        }

        results
    }

    /// Analyze a batch of transactions for cross-transaction patterns
    pub async fn analyze_batch(&self, contexts: &[TransactionContext]) -> Vec<Vec<DetectionResult>> {
        let mut all_results = Vec::new();

        for ctx in contexts {
            let results = self.analyze_transaction(ctx).await;
            all_results.push(results);
        }

        all_results
    }

    /// Convert confidence score to alert severity
    fn confidence_to_severity(confidence: f64) -> AlertSeverity {
        if confidence >= 0.9 {
            AlertSeverity::Critical
        } else if confidence >= 0.7 {
            AlertSeverity::High
        } else if confidence >= 0.5 {
            AlertSeverity::Medium
        } else {
            AlertSeverity::Low
        }
    }

    /// Convert detection result to alert
    fn detection_to_alert(
        result: &DetectionResult,
        ctx: &TransactionContext,
        severity: AlertSeverity,
    ) -> Alert {
        use std::collections::HashMap;

        let mut metadata = HashMap::new();
        metadata.insert("detector".to_string(), "mempool".to_string());
        metadata.insert("pallet".to_string(), ctx.transaction.pallet.clone());
        metadata.insert("call".to_string(), ctx.transaction.call.clone());
        metadata.insert("caller".to_string(), ctx.transaction.caller.clone());
        metadata.insert("confidence".to_string(), format!("{:.2}", result.confidence));

        Alert {
            id: format!("mempool-{}-{}", ctx.transaction.block_number, ctx.transaction.index),
            timestamp: ctx.transaction.timestamp,
            chain: "polkadot".to_string(), // TODO: Make configurable
            severity,
            pattern: result.pattern.clone(),
            description: result.description.clone(),
            transaction_hash: Some(ctx.transaction.hash.clone()),
            block_number: Some(ctx.transaction.block_number),
            metadata,
            recommended_actions: Self::generate_recommended_actions(&result.pattern),
            acknowledged: false,
        }
    }

    /// Generate recommended actions based on attack pattern
    fn generate_recommended_actions(pattern: &crate::types::AttackPattern) -> Vec<String> {
        use crate::types::AttackPattern;

        match pattern {
            AttackPattern::FrontRunning => vec![
                "Monitor subsequent transactions for profit extraction".to_string(),
                "Check if user transaction was affected".to_string(),
                "Consider implementing private transaction pool".to_string(),
            ],
            AttackPattern::Sandwich => vec![
                "Alert affected users immediately".to_string(),
                "Investigate attacker's wallet for other attacks".to_string(),
                "Consider implementing slippage protection".to_string(),
                "Review DEX parameters and fee structures".to_string(),
            ],
            AttackPattern::FlashLoan => vec![
                "Check for price manipulation in affected pools".to_string(),
                "Verify protocol solvency and reserves".to_string(),
                "Investigate if attack was profitable".to_string(),
            ],
            AttackPattern::Mev => vec![
                "Monitor for repeated MEV extraction patterns".to_string(),
                "Consider MEV-resistant transaction ordering".to_string(),
            ],
            _ => vec![
                "Investigate transaction for malicious activity".to_string(),
                "Monitor related addresses".to_string(),
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_transaction(hash: &str) -> Transaction {
        Transaction {
            hash: hash.to_string(),
            block_number: 0,
            from: "sender".to_string(),
            to: Some("recipient".to_string()),
            value: "1000".to_string(),
            data: vec![],
            gas_used: Some(21000),
            success: true,
            timestamp: 1234567890,
        }
    }

    #[tokio::test]
    async fn test_mempool_add_transaction() {
        let monitor = MempoolMonitor::new(100);

        let tx = create_test_transaction("0x123");
        monitor.add_transaction(tx).await;

        assert_eq!(monitor.pending_count().await, 1);
    }

    #[tokio::test]
    async fn test_mempool_max_size() {
        let monitor = MempoolMonitor::new(2);

        monitor.add_transaction(create_test_transaction("0x1")).await;
        monitor.add_transaction(create_test_transaction("0x2")).await;
        monitor.add_transaction(create_test_transaction("0x3")).await;

        assert_eq!(monitor.pending_count().await, 2);

        let pending = monitor.get_pending_transactions().await;
        assert_eq!(pending[0].hash, "0x2");
        assert_eq!(pending[1].hash, "0x3");
    }

    #[tokio::test]
    async fn test_clear_confirmed() {
        let monitor = MempoolMonitor::new(100);

        monitor.add_transaction(create_test_transaction("0x1")).await;
        monitor.add_transaction(create_test_transaction("0x2")).await;
        monitor.add_transaction(create_test_transaction("0x3")).await;

        assert_eq!(monitor.pending_count().await, 3);

        monitor.clear_confirmed(&["0x1".to_string(), "0x3".to_string()]).await;

        assert_eq!(monitor.pending_count().await, 1);

        let pending = monitor.get_pending_transactions().await;
        assert_eq!(pending[0].hash, "0x2");
    }
}
