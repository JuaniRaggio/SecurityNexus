//! Flash loan attack detector
//!
//! Detects flash loan attack patterns by analyzing:
//! 1. Borrow and repayment events in the same transaction
//! 2. Multiple DeFi protocol interactions (swaps, liquidations)
//! 3. Large balance changes (>50% threshold)
//! 4. Transaction complexity and manipulation indicators

use crate::detectors::Detector;
use crate::types::{AttackPattern, DetectionResult, TransactionContext};
use async_trait::async_trait;

/// Detector for flash loan attacks
pub struct FlashLoanDetector {
    enabled: bool,
}

/// Flash loan pattern indicators
struct FlashLoanIndicators {
    has_borrow: bool,
    has_repay: bool,
    dex_interaction_count: usize,
    lending_protocol_interactions: usize,
    large_balance_changes: usize,
}

impl FlashLoanDetector {
    pub fn new() -> Self {
        Self { enabled: true }
    }

    /// Analyze events to extract flash loan indicators
    fn analyze_events(ctx: &TransactionContext) -> FlashLoanIndicators {
        let mut has_borrow = false;
        let mut has_repay = false;
        let mut dex_interaction_count = 0;
        let mut lending_protocol_interactions = 0;

        for event in &ctx.events {
            let event_name_lower = event.event_name.to_lowercase();
            let pallet_lower = event.pallet.to_lowercase();

            // Detect borrow events
            if event_name_lower.contains("borrow") {
                has_borrow = true;
                lending_protocol_interactions += 1;
            }

            // Detect repayment events
            if event_name_lower.contains("repay") || event_name_lower.contains("repaid") {
                has_repay = true;
                lending_protocol_interactions += 1;
            }

            // Detect DEX interactions (swaps, trades)
            if pallet_lower.contains("dex")
                || event_name_lower.contains("swap")
                || event_name_lower.contains("trade")
            {
                dex_interaction_count += 1;
            }

            // Other lending protocol events
            if pallet_lower.contains("lending") || pallet_lower.contains("loan") {
                lending_protocol_interactions += 1;
            }
        }

        FlashLoanIndicators {
            has_borrow,
            has_repay,
            dex_interaction_count,
            lending_protocol_interactions,
            large_balance_changes: Self::count_large_balance_changes(&ctx.state_changes),
        }
    }

    /// Count balance changes larger than 50%
    fn count_large_balance_changes(state_changes: &[crate::types::StateChange]) -> usize {
        let mut count = 0;

        for change in state_changes {
            if let (Some(old_val), Some(new_val)) = (&change.old_value, &change.new_value) {
                // Simple heuristic: if values are different lengths or differ significantly
                if old_val.len() == new_val.len() && !old_val.is_empty() {
                    // Convert to u64 for comparison (simplified)
                    let old_num = Self::bytes_to_u64(old_val);
                    let new_num = Self::bytes_to_u64(new_val);

                    if old_num > 0 {
                        let change_ratio = if new_num > old_num {
                            (new_num as f64) / (old_num as f64)
                        } else {
                            (old_num as f64) / (new_num as f64)
                        };

                        // >50% change threshold
                        if change_ratio > 1.5 {
                            count += 1;
                        }
                    }
                }
            }
        }

        count
    }

    /// Convert byte array to u64 (simplified, takes last 8 bytes)
    fn bytes_to_u64(bytes: &[u8]) -> u64 {
        let len = bytes.len();
        if len >= 8 {
            u64::from_be_bytes([
                bytes[len - 8],
                bytes[len - 7],
                bytes[len - 6],
                bytes[len - 5],
                bytes[len - 4],
                bytes[len - 3],
                bytes[len - 2],
                bytes[len - 1],
            ])
        } else {
            // For smaller arrays, pad with zeros
            let mut padded = [0u8; 8];
            padded[8 - len..].copy_from_slice(bytes);
            u64::from_be_bytes(padded)
        }
    }

    /// Calculate confidence score based on indicators
    fn calculate_confidence(indicators: &FlashLoanIndicators) -> f64 {
        let mut confidence = 0.0;

        // Core pattern: borrow + repay (50% confidence)
        if indicators.has_borrow && indicators.has_repay {
            confidence += 0.5;

            // Multiple DEX interactions increase confidence (up to +30%)
            if indicators.dex_interaction_count >= 2 {
                confidence += 0.15 * (indicators.dex_interaction_count as f64).min(4.0) / 2.0;
            }

            // Large balance changes indicate manipulation (+20%)
            if indicators.large_balance_changes > 0 {
                confidence += 0.2;
            }

            // Multiple lending protocol interactions (+10%)
            if indicators.lending_protocol_interactions > 2 {
                confidence += 0.1;
            }
        } else if indicators.has_borrow {
            // Borrow without repayment (incomplete pattern, low confidence)
            confidence = 0.2;
        }

        confidence.min(1.0) // Cap at 100%
    }

    /// Build evidence list from indicators
    fn build_evidence(indicators: &FlashLoanIndicators) -> Vec<String> {
        let mut evidence = Vec::new();

        if indicators.has_borrow {
            evidence.push("Detected borrow event from lending protocol".to_string());
        }

        if indicators.has_repay {
            evidence.push("Detected repayment event in same transaction".to_string());
        }

        if indicators.dex_interaction_count > 0 {
            evidence.push(format!(
                "Multiple DeFi protocol interactions: {} swap/trade events",
                indicators.dex_interaction_count
            ));
        }

        if indicators.large_balance_changes > 0 {
            evidence.push(format!(
                "Large balance changes detected: {} changes >50%",
                indicators.large_balance_changes
            ));
        }

        if indicators.lending_protocol_interactions > 2 {
            evidence.push(format!(
                "Complex lending protocol usage: {} interactions",
                indicators.lending_protocol_interactions
            ));
        }

        if indicators.has_borrow && indicators.has_repay && indicators.dex_interaction_count >= 2 {
            evidence.push(
                "Classic flash loan pattern: borrow → manipulate → repay".to_string(),
            );
        }

        evidence
    }
}

impl Default for FlashLoanDetector {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Detector for FlashLoanDetector {
    fn name(&self) -> &str {
        "FlashLoanDetector"
    }

    async fn analyze_transaction(&self, ctx: &TransactionContext) -> DetectionResult {
        // Extract indicators from the transaction context
        let indicators = Self::analyze_events(ctx);

        // Calculate confidence score
        let confidence = Self::calculate_confidence(&indicators);

        // Only report if we have reasonable confidence (>30%)
        if confidence >= 0.3 {
            let evidence = Self::build_evidence(&indicators);

            let description = if indicators.has_borrow && indicators.has_repay {
                format!(
                    "Potential flash loan attack: borrow + {} DeFi interactions + repay in single transaction",
                    indicators.dex_interaction_count
                )
            } else {
                format!(
                    "Suspicious lending activity: {} protocol interactions detected",
                    indicators.lending_protocol_interactions
                )
            };

            DetectionResult::detected(AttackPattern::FlashLoan, confidence, description, evidence)
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
    async fn test_flash_loan_detector() {
        let detector = FlashLoanDetector::new();
        assert_eq!(detector.name(), "FlashLoanDetector");
        assert!(detector.is_enabled());
    }
}
