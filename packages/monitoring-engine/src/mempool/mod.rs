//! Mempool monitoring and analysis

use crate::types::Transaction;
use std::collections::VecDeque;
use tokio::sync::RwLock;

/// Mempool monitor tracks pending transactions
pub struct MempoolMonitor {
    pending_txs: RwLock<VecDeque<Transaction>>,
    max_size: usize,
}

impl MempoolMonitor {
    /// Create a new mempool monitor
    pub fn new(max_size: usize) -> Self {
        Self {
            pending_txs: RwLock::new(VecDeque::with_capacity(max_size)),
            max_size,
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
