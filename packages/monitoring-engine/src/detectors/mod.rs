//! Attack pattern detectors

pub mod flash_loan;
pub mod mev;
pub mod volume;

pub use flash_loan::FlashLoanDetector;
pub use mev::MevDetector;
pub use volume::VolumeAnomalyDetector;

use crate::types::{DetectionResult, Transaction};
use async_trait::async_trait;

/// Trait for attack pattern detectors
#[async_trait]
pub trait Detector: Send + Sync {
    /// Get the name of this detector
    fn name(&self) -> &str;

    /// Analyze a transaction for suspicious patterns
    async fn analyze_transaction(&self, tx: &Transaction) -> DetectionResult;

    /// Analyze a batch of transactions for patterns
    async fn analyze_batch(&self, transactions: &[Transaction]) -> Vec<DetectionResult> {
        let mut results = Vec::new();
        for tx in transactions {
            results.push(self.analyze_transaction(tx).await);
        }
        results
    }

    /// Check if this detector is enabled
    fn is_enabled(&self) -> bool {
        true
    }
}
