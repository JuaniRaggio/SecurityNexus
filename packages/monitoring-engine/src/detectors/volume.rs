//! Volume anomaly detector

use crate::detectors::Detector;
use crate::types::{AttackPattern, DetectionResult, TransactionContext};
use async_trait::async_trait;

/// Detector for unusual volume spikes
pub struct VolumeAnomalyDetector {
    enabled: bool,
}

impl VolumeAnomalyDetector {
    pub fn new() -> Self {
        Self { enabled: true }
    }
}

impl Default for VolumeAnomalyDetector {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Detector for VolumeAnomalyDetector {
    fn name(&self) -> &str {
        "Volume Anomaly Detector"
    }

    async fn analyze_transaction(&self, ctx: &TransactionContext) -> DetectionResult {
        let tx = &ctx.transaction;
        let mut suspicion_score: f64 = 0.0;
        let mut evidence = Vec::new();

        // Check for balance transfers (very common in Westend)
        if tx.pallet.to_lowercase() == "balances" {
            suspicion_score += 0.3;
            evidence.push(format!("Balance operation detected: {}", tx.call));

            // Higher suspicion for transfer_keep_alive or transfer_all
            if tx.call.contains("transfer") {
                suspicion_score += 0.2;
                evidence.push("Transfer operation - potential volume activity".to_string());
            }
        }

        // Check for asset-related transactions
        if tx.pallet.to_lowercase().contains("asset") {
            suspicion_score += 0.4;
            evidence.push("Asset-related transaction detected".to_string());
        }

        // Check for staking operations (can involve large amounts)
        if tx.pallet.to_lowercase() == "staking" {
            suspicion_score += 0.35;
            evidence.push(format!("Staking operation: {}", tx.call));
        }

        // Check for utility batch calls (multiple operations at once)
        if tx.pallet.to_lowercase() == "utility" && tx.call.contains("batch") {
            suspicion_score += 0.5;
            evidence.push("Batch transaction detected - multiple operations".to_string());
        }

        // Check for XCM (cross-chain) transfers
        if tx.pallet.to_lowercase().contains("xcm") || tx.pallet.to_lowercase().contains("xtokens") {
            suspicion_score += 0.6;
            evidence.push("Cross-chain transfer detected".to_string());
        }

        // Detect if transaction has many arguments (complex operation)
        if tx.args.len() > 3 {
            suspicion_score += 0.2;
            evidence.push(format!("Complex transaction with {} arguments", tx.args.len()));
        }

        // If we have any suspicion, report it
        if suspicion_score > 0.5 {
            DetectionResult {
                detected: true,
                confidence: suspicion_score.min(0.95), // Cap at 0.95
                pattern: AttackPattern::VolumeAnomaly,
                description: format!(
                    "Potentially suspicious volume activity detected in {}::{} transaction",
                    tx.pallet, tx.call
                ),
                evidence,
                metadata: std::collections::HashMap::new(),
            }
        } else {
            DetectionResult::no_detection()
        }
    }

    fn is_enabled(&self) -> bool {
        self.enabled
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_volume_detector() {
        let detector = VolumeAnomalyDetector::new();
        assert_eq!(detector.name(), "Volume Anomaly Detector");
        assert!(detector.is_enabled());
    }
}
