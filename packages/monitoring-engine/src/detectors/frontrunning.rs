//! Front-running and Sandwich attack detector
//!
//! Detects mempool-based attacks by analyzing:
//! 1. Transaction ordering and timing
//! 2. Duplicate or similar transactions from different senders
//! 3. Sandwich patterns (front-run + victim + back-run)
//! 4. High gas prices indicating priority transactions

use crate::detectors::Detector;
use crate::types::{AttackPattern, DetectionResult, ParsedTransaction, TransactionContext};
use async_trait::async_trait;
use std::collections::HashMap;

/// Detector for front-running and sandwich attacks
pub struct FrontRunningDetector {
    enabled: bool,
    /// Recently seen transactions for pattern matching
    recent_transactions: std::sync::Arc<tokio::sync::RwLock<Vec<ParsedTransaction>>>,
}

/// Front-running pattern indicators
struct FrontRunningIndicators {
    has_duplicate_call: bool,
    is_high_priority: bool,
    has_sandwich_pattern: bool,
    similar_transaction_count: usize,
    same_target_count: usize,
}

impl FrontRunningDetector {
    pub fn new() -> Self {
        Self {
            enabled: true,
            recent_transactions: std::sync::Arc::new(tokio::sync::RwLock::new(Vec::with_capacity(100))),
        }
    }

    /// Add transaction to history for pattern detection
    async fn add_to_history(&self, tx: &ParsedTransaction) {
        let mut history = self.recent_transactions.write().await;

        // Keep only last 100 transactions
        if history.len() >= 100 {
            history.remove(0);
        }

        history.push(tx.clone());
    }

    /// Analyze mempool for front-running patterns
    async fn analyze_mempool_pattern(&self, ctx: &TransactionContext) -> FrontRunningIndicators {
        let history = self.recent_transactions.read().await;

        let mut has_duplicate_call = false;
        let mut similar_transaction_count = 0;
        let mut same_target_count = 0;

        // Check for similar transactions in recent history
        for recent_tx in history.iter() {
            // Skip if same transaction
            if recent_tx.hash == ctx.transaction.hash {
                continue;
            }

            // Check if targeting same pallet and call
            if recent_tx.pallet == ctx.transaction.pallet
                && recent_tx.call == ctx.transaction.call {
                same_target_count += 1;

                // Check if from different caller (potential front-run)
                if recent_tx.caller != ctx.transaction.caller {
                    has_duplicate_call = true;
                    similar_transaction_count += 1;
                }
            }

            // Check for sandwich pattern:
            // Multiple transactions from same caller targeting same pallet
            if recent_tx.caller == ctx.transaction.caller
                && recent_tx.pallet == ctx.transaction.pallet
                && recent_tx.block_number == ctx.transaction.block_number {
                similar_transaction_count += 1;
            }
        }

        // Detect sandwich pattern:
        // Transaction appears between two transactions from same attacker
        let has_sandwich_pattern = Self::detect_sandwich_pattern(&history, ctx);

        FrontRunningIndicators {
            has_duplicate_call,
            is_high_priority: false, // TODO: Implement priority detection based on nonce/fees
            has_sandwich_pattern,
            similar_transaction_count,
            same_target_count,
        }
    }

    /// Detect sandwich attack pattern
    fn detect_sandwich_pattern(
        history: &[ParsedTransaction],
        ctx: &TransactionContext,
    ) -> bool {
        if history.len() < 2 {
            return false;
        }

        // Look for pattern: TxA (attacker) -> TxV (victim) -> TxB (attacker)
        // Where TxA and TxB target same pallet but TxV is from different caller

        let current_block = ctx.transaction.block_number;
        let current_pallet = &ctx.transaction.pallet;
        let current_caller = &ctx.transaction.caller;

        // Find transactions in same block
        let block_txs: Vec<_> = history
            .iter()
            .filter(|tx| tx.block_number == current_block)
            .collect();

        if block_txs.len() < 2 {
            return false;
        }

        // Check if current transaction is sandwiched
        for i in 0..block_txs.len() {
            if block_txs[i].hash == ctx.transaction.hash {
                // Check if surrounded by transactions from same attacker
                if i > 0 && i < block_txs.len() - 1 {
                    let prev = &block_txs[i - 1];
                    let next = &block_txs[i + 1];

                    if prev.caller == next.caller
                        && prev.caller != *current_caller
                        && prev.pallet == *current_pallet
                        && next.pallet == *current_pallet
                    {
                        return true;
                    }
                }
            }
        }

        false
    }

    /// Calculate confidence score based on indicators
    fn calculate_confidence(indicators: &FrontRunningIndicators) -> f64 {
        let mut confidence = 0.0;

        // Sandwich pattern is strong indicator (70%)
        if indicators.has_sandwich_pattern {
            confidence += 0.7;
        }

        // Duplicate calls from different senders (30%)
        if indicators.has_duplicate_call {
            confidence += 0.3;
        }

        // Multiple similar transactions (up to +20%)
        if indicators.similar_transaction_count > 1 {
            confidence += 0.1 * (indicators.similar_transaction_count as f64).min(2.0);
        }

        // High priority transaction (+10%)
        if indicators.is_high_priority {
            confidence += 0.1;
        }

        // Same target being hit multiple times (+10%)
        if indicators.same_target_count > 2 {
            confidence += 0.1;
        }

        confidence.min(1.0) // Cap at 100%
    }

    /// Build evidence list from indicators
    fn build_evidence(indicators: &FrontRunningIndicators, ctx: &TransactionContext) -> Vec<String> {
        let mut evidence = Vec::new();

        if indicators.has_sandwich_pattern {
            evidence.push(format!(
                "Sandwich attack pattern detected: transaction sandwiched between two transactions from same attacker"
            ));
        }

        if indicators.has_duplicate_call {
            evidence.push(format!(
                "Duplicate call detected: Same pallet.call ({}.{}) from different sender",
                ctx.transaction.pallet,
                ctx.transaction.call
            ));
        }

        if indicators.similar_transaction_count > 1 {
            evidence.push(format!(
                "Multiple similar transactions detected: {} similar calls in mempool",
                indicators.similar_transaction_count
            ));
        }

        if indicators.same_target_count > 2 {
            evidence.push(format!(
                "High competition for same target: {} transactions targeting {}.{}",
                indicators.same_target_count,
                ctx.transaction.pallet,
                ctx.transaction.call
            ));
        }

        if indicators.is_high_priority {
            evidence.push("High priority transaction detected (elevated fees/nonce)".to_string());
        }

        evidence
    }

    /// Determine attack pattern type
    fn determine_pattern(indicators: &FrontRunningIndicators) -> AttackPattern {
        if indicators.has_sandwich_pattern {
            AttackPattern::Sandwich
        } else if indicators.has_duplicate_call || indicators.similar_transaction_count > 1 {
            AttackPattern::FrontRunning
        } else {
            AttackPattern::Unknown
        }
    }
}

impl Default for FrontRunningDetector {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Detector for FrontRunningDetector {
    fn name(&self) -> &str {
        "FrontRunningDetector"
    }

    async fn analyze_transaction(&self, ctx: &TransactionContext) -> DetectionResult {
        // Add transaction to history for future pattern detection
        self.add_to_history(&ctx.transaction).await;

        // Analyze for front-running/sandwich patterns
        let indicators = self.analyze_mempool_pattern(ctx).await;

        // Calculate confidence score
        let confidence = Self::calculate_confidence(&indicators);

        // Only report if we have reasonable confidence (>40%)
        if confidence >= 0.4 {
            let evidence = Self::build_evidence(&indicators, ctx);
            let pattern = Self::determine_pattern(&indicators);

            let description = if indicators.has_sandwich_pattern {
                format!(
                    "Sandwich attack detected on {}.{}: victim transaction surrounded by attacker transactions",
                    ctx.transaction.pallet,
                    ctx.transaction.call
                )
            } else {
                format!(
                    "Front-running attempt detected: duplicate call to {}.{} from competing sender",
                    ctx.transaction.pallet,
                    ctx.transaction.call
                )
            };

            DetectionResult::detected(pattern, confidence, description, evidence)
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
    use crate::types::ChainEvent;

    fn create_test_transaction(hash: &str, caller: &str, pallet: &str, call: &str, block: u64, index: u32) -> ParsedTransaction {
        ParsedTransaction {
            hash: hash.to_string(),
            block_number: block,
            block_hash: format!("0xblock{}", block),
            index,
            caller: caller.to_string(),
            pallet: pallet.to_string(),
            call: call.to_string(),
            args: vec![],
            signature: None,
            nonce: Some(0),
            timestamp: 1234567890,
            success: true,
        }
    }

    fn create_test_context(tx: ParsedTransaction) -> TransactionContext {
        TransactionContext {
            transaction: tx,
            events: vec![],
            state_changes: vec![],
        }
    }

    #[tokio::test]
    async fn test_frontrunning_detector_basic() {
        let detector = FrontRunningDetector::new();
        assert_eq!(detector.name(), "FrontRunningDetector");
        assert!(detector.is_enabled());
    }

    #[tokio::test]
    async fn test_detect_duplicate_call() {
        let detector = FrontRunningDetector::new();

        // First transaction from Alice
        let tx1 = create_test_transaction("0x1", "alice", "Balances", "transfer", 100, 0);
        let ctx1 = create_test_context(tx1);
        detector.analyze_transaction(&ctx1).await;

        // Second transaction from Bob - same call (potential front-run)
        let tx2 = create_test_transaction("0x2", "bob", "Balances", "transfer", 100, 1);
        let ctx2 = create_test_context(tx2);
        let result = detector.analyze_transaction(&ctx2).await;

        // Should detect potential front-running
        assert!(result.detected);
        assert!(result.confidence > 0.0);
    }

    #[tokio::test]
    async fn test_no_detection_on_different_calls() {
        let detector = FrontRunningDetector::new();

        // Transaction 1: transfer
        let tx1 = create_test_transaction("0x1", "alice", "Balances", "transfer", 100, 0);
        let ctx1 = create_test_context(tx1);
        detector.analyze_transaction(&ctx1).await;

        // Transaction 2: different call
        let tx2 = create_test_transaction("0x2", "bob", "Balances", "transfer_keep_alive", 100, 1);
        let ctx2 = create_test_context(tx2);
        let result = detector.analyze_transaction(&ctx2).await;

        // Should not detect (different calls)
        assert!(!result.detected);
    }
}
