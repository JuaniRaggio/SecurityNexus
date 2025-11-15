//! MEV (Maximal Extractable Value) attack detector
//!
//! Detects MEV attack patterns by analyzing transaction sequences:
//! 1. Sandwich attacks: Same caller surrounds victim (buy → victim → sell)
//! 2. Frontrunning: Similar transaction executed before victim
//! 3. Back-running: Transaction immediately after large trade
//! 4. Position-based exploitation

use crate::detectors::Detector;
use crate::types::{AttackPattern, DetectionResult, TransactionContext};
use async_trait::async_trait;
use std::collections::HashMap;

/// Detector for MEV attacks (front-running, sandwich attacks, etc.)
pub struct MevDetector {
    enabled: bool,
}

/// MEV pattern indicators from batch analysis
#[derive(Debug, Clone)]
struct MevIndicators {
    /// Sandwich attack pattern detected
    is_sandwich: bool,
    /// Frontrunning pattern detected
    is_frontrunning: bool,
    /// Back-running pattern detected
    is_backrunning: bool,
    /// Confidence level
    confidence: f64,
    /// Evidence for detection
    evidence: Vec<String>,
}

impl MevDetector {
    pub fn new() -> Self {
        Self { enabled: true }
    }

    /// Analyze a batch of transactions for MEV patterns
    ///
    /// MEV detection requires analyzing multiple transactions together
    /// to identify exploitation patterns
    fn analyze_batch_for_mev(contexts: &[TransactionContext]) -> HashMap<usize, MevIndicators> {
        let mut indicators_map = HashMap::new();

        if contexts.len() < 2 {
            return indicators_map; // Need at least 2 transactions for MEV
        }

        // Check for sandwich attacks
        Self::detect_sandwich_attacks(contexts, &mut indicators_map);

        // Check for frontrunning
        Self::detect_frontrunning(contexts, &mut indicators_map);

        // Check for back-running
        Self::detect_backrunning(contexts, &mut indicators_map);

        indicators_map
    }

    /// Detect sandwich attack patterns
    ///
    /// Pattern: Same caller executes transactions before and after victim
    fn detect_sandwich_attacks(
        contexts: &[TransactionContext],
        indicators_map: &mut HashMap<usize, MevIndicators>,
    ) {
        for i in 0..contexts.len() {
            if i + 2 >= contexts.len() {
                break;
            }

            let before = &contexts[i];
            let middle = &contexts[i + 1];
            let after = &contexts[i + 2];

            // Check if before and after are from same caller
            let same_attacker = before.transaction.caller == after.transaction.caller;

            // Check if middle is different caller (victim)
            let different_victim = middle.transaction.caller != before.transaction.caller;

            // Check if transactions are sequential or close together
            let sequential = (middle.transaction.index == before.transaction.index + 1)
                && (after.transaction.index == middle.transaction.index + 1);

            let close_indices = (middle.transaction.index <= before.transaction.index + 3)
                && (after.transaction.index <= middle.transaction.index + 3);

            // Check if all are DEX/swap operations
            let all_dex = Self::is_dex_operation(&before.transaction)
                && Self::is_dex_operation(&middle.transaction)
                && Self::is_dex_operation(&after.transaction);

            if same_attacker && different_victim && (sequential || close_indices) && all_dex {
                let mut confidence: f64 = 0.7; // Base confidence for sandwich pattern
                let mut evidence = Vec::new();

                evidence.push(format!(
                    "Same caller ({}) surrounds different caller ({})",
                    before.transaction.caller, middle.transaction.caller
                ));

                if sequential {
                    confidence += 0.15;
                    evidence.push("Transactions are sequential in block".to_string());
                }

                // Check for sequential nonces (strong indicator)
                if let (Some(nonce1), Some(nonce2)) =
                    (before.transaction.nonce, after.transaction.nonce)
                {
                    if nonce2 == nonce1 + 1 {
                        confidence += 0.1;
                        evidence.push("Sequential nonces from attacker".to_string());
                    }
                }

                evidence.push("Classic sandwich attack pattern detected".to_string());

                // Mark all three transactions
                for idx in [i, i + 1, i + 2] {
                    indicators_map.insert(
                        idx,
                        MevIndicators {
                            is_sandwich: true,
                            is_frontrunning: false,
                            is_backrunning: false,
                            confidence: confidence.min(1.0),
                            evidence: evidence.clone(),
                        },
                    );
                }
            }
        }
    }

    /// Detect frontrunning patterns
    ///
    /// Pattern: Similar function call executed before another transaction
    fn detect_frontrunning(
        contexts: &[TransactionContext],
        indicators_map: &mut HashMap<usize, MevIndicators>,
    ) {
        for i in 0..contexts.len().saturating_sub(1) {
            // Skip if already detected as sandwich (higher priority)
            if indicators_map.contains_key(&i) {
                continue;
            }

            let current = &contexts[i];
            let next = &contexts[i + 1];

            // Same function call
            let same_function = current.transaction.call == next.transaction.call
                && current.transaction.pallet == next.transaction.pallet;

            // Different callers
            let different_callers = current.transaction.caller != next.transaction.caller;

            // Sequential or close indices
            let close_indices = next.transaction.index <= current.transaction.index + 2;

            // Second transaction failed (victim got frontrun)
            let victim_failed = !next.transaction.success && current.transaction.success;

            if same_function && different_callers && close_indices {
                let mut confidence: f64 = 0.5; // Base confidence
                let mut evidence = Vec::new();

                evidence.push(format!(
                    "Same function call ({}.{}) executed by different callers",
                    current.transaction.pallet, current.transaction.call
                ));

                if victim_failed {
                    confidence += 0.25;
                    evidence.push("Victim transaction failed after frontrunner succeeded".to_string());
                }

                if next.transaction.index == current.transaction.index + 1 {
                    confidence += 0.1;
                    evidence.push("Frontrunner executed immediately before victim".to_string());
                }

                evidence.push("Potential frontrunning pattern detected".to_string());

                indicators_map.insert(
                    i,
                    MevIndicators {
                        is_sandwich: false,
                        is_frontrunning: true,
                        is_backrunning: false,
                        confidence: confidence.min(1.0),
                        evidence: evidence.clone(),
                    },
                );
            }
        }
    }

    /// Detect back-running patterns
    ///
    /// Pattern: Transaction immediately after large price-moving trade
    fn detect_backrunning(
        contexts: &[TransactionContext],
        indicators_map: &mut HashMap<usize, MevIndicators>,
    ) {
        for i in 0..contexts.len().saturating_sub(1) {
            let current = &contexts[i];
            let next = &contexts[i + 1];

            // Current transaction has large state changes (price impact)
            let has_large_changes = !current.state_changes.is_empty()
                && current
                    .state_changes
                    .iter()
                    .any(|sc| Self::is_large_state_change(sc));

            // Next transaction is DEX operation
            let next_is_dex = Self::is_dex_operation(&next.transaction);

            // Immediately following
            let immediately_after = next.transaction.index == current.transaction.index + 1;

            if has_large_changes && next_is_dex && immediately_after {
                let confidence = 0.6; // Moderate confidence for back-running
                let evidence = vec![
                    "Large price-moving transaction detected".to_string(),
                    "Followed immediately by DEX operation".to_string(),
                    "Potential back-running MEV pattern".to_string(),
                ];

                indicators_map.insert(
                    i + 1,
                    MevIndicators {
                        is_sandwich: false,
                        is_frontrunning: false,
                        is_backrunning: true,
                        confidence,
                        evidence,
                    },
                );
            }
        }
    }

    /// Check if transaction is a DEX operation
    fn is_dex_operation(tx: &crate::types::ParsedTransaction) -> bool {
        let pallet_lower = tx.pallet.to_lowercase();
        let call_lower = tx.call.to_lowercase();

        pallet_lower.contains("dex")
            || call_lower.contains("swap")
            || call_lower.contains("trade")
            || call_lower.contains("exchange")
    }

    /// Check if state change is large (>50% change)
    fn is_large_state_change(sc: &crate::types::StateChange) -> bool {
        if let (Some(old_val), Some(new_val)) = (&sc.old_value, &sc.new_value) {
            if old_val.len() == new_val.len() && !old_val.is_empty() {
                // Simple heuristic: check if values differ significantly
                let differs = old_val
                    .iter()
                    .zip(new_val.iter())
                    .filter(|(a, b)| a != b)
                    .count();

                // If more than 25% of bytes differ, consider it large
                differs > old_val.len() / 4
            } else {
                false
            }
        } else {
            false
        }
    }
}

impl Default for MevDetector {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Detector for MevDetector {
    fn name(&self) -> &str {
        "MevDetector"
    }

    async fn analyze_transaction(&self, _ctx: &TransactionContext) -> DetectionResult {
        // MEV detection requires batch context
        // Single transaction analysis returns no detection
        // Use analyze_batch() for proper MEV detection
        DetectionResult::no_detection()
    }

    async fn analyze_batch(&self, contexts: &[TransactionContext]) -> Vec<DetectionResult> {
        // Perform batch MEV analysis
        let indicators_map = Self::analyze_batch_for_mev(contexts);

        // Generate results for each transaction
        contexts
            .iter()
            .enumerate()
            .map(|(idx, _ctx)| {
                if let Some(indicators) = indicators_map.get(&idx) {
                    // Determine attack pattern
                    let pattern = if indicators.is_sandwich {
                        AttackPattern::Sandwich
                    } else if indicators.is_frontrunning {
                        AttackPattern::FrontRunning
                    } else if indicators.is_backrunning {
                        AttackPattern::Mev
                    } else {
                        AttackPattern::Mev
                    };

                    let description = if indicators.is_sandwich {
                        "Sandwich attack: Attacker surrounds victim transaction".to_string()
                    } else if indicators.is_frontrunning {
                        "Frontrunning: Transaction executed before victim".to_string()
                    } else {
                        "MEV extraction: Back-running detected".to_string()
                    };

                    DetectionResult::detected(
                        pattern,
                        indicators.confidence,
                        description,
                        indicators.evidence.clone(),
                    )
                } else {
                    DetectionResult::no_detection()
                }
            })
            .collect()
    }

    fn is_enabled(&self) -> bool {
        self.enabled
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mev_detector() {
        let detector = MevDetector::new();
        assert_eq!(detector.name(), "MevDetector");
        assert!(detector.is_enabled());
    }
}
