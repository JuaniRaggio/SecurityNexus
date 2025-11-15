//! Attack pattern detectors

pub mod flash_loan;
pub mod mev;
pub mod volume;

pub use flash_loan::FlashLoanDetector;
pub use mev::MevDetector;
pub use volume::VolumeAnomalyDetector;

use crate::types::{DetectionResult, TransactionContext};
use async_trait::async_trait;

/// Trait for attack pattern detectors
#[async_trait]
pub trait Detector: Send + Sync {
    /// Get the name of this detector
    fn name(&self) -> &str;

    /// Analyze a transaction context for suspicious patterns
    ///
    /// The context includes the transaction itself, associated events,
    /// and state changes - providing full visibility into the transaction's behavior
    async fn analyze_transaction(&self, ctx: &TransactionContext) -> DetectionResult;

    /// Analyze a batch of transaction contexts for patterns
    async fn analyze_batch(&self, contexts: &[TransactionContext]) -> Vec<DetectionResult> {
        let mut results = Vec::new();
        for ctx in contexts {
            results.push(self.analyze_transaction(ctx).await);
        }
        results
    }

    /// Check if this detector is enabled
    fn is_enabled(&self) -> bool {
        true
    }
}
