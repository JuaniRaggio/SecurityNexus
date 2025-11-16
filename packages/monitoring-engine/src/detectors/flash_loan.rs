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
    use crate::types::{ChainEvent, ParsedTransaction, StateChange};

    fn create_test_transaction(hash: &str, pallet: &str, call: &str) -> ParsedTransaction {
        ParsedTransaction {
            hash: hash.to_string(),
            block_number: 1000,
            block_hash: "0xblock1000".to_string(),
            index: 0,
            caller: "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY".to_string(),
            pallet: pallet.to_string(),
            call: call.to_string(),
            args: vec![],
            signature: None,
            nonce: Some(1),
            timestamp: 1234567890,
            success: true,
        }
    }

    fn create_event(pallet: &str, event_name: &str, data: &str) -> ChainEvent {
        ChainEvent {
            block_number: 1000,
            event_index: 0,
            extrinsic_index: Some(0),
            pallet: pallet.to_string(),
            event_name: event_name.to_string(),
            data: data.as_bytes().to_vec(),
            topics: vec![],
        }
    }

    fn create_state_change(key: &str, old_value: Vec<u8>, new_value: Vec<u8>) -> StateChange {
        StateChange {
            key: key.as_bytes().to_vec(),
            old_value: Some(old_value),
            new_value: Some(new_value),
        }
    }

    #[tokio::test]
    async fn test_flash_loan_detector_basic() {
        let detector = FlashLoanDetector::new();
        assert_eq!(detector.name(), "FlashLoanDetector");
        assert!(detector.is_enabled());
    }

    #[tokio::test]
    async fn test_classic_flash_loan_attack() {
        let detector = FlashLoanDetector::new();

        // Classic flash loan: borrow → swap → swap → repay
        let tx = create_test_transaction(
            "0xflashloan1",
            "Lending",
            "borrow_flash",
        );

        let events = vec![
            create_event("Lending", "Borrowed", "amount: 1000000 USDT"),
            create_event("DEX", "Swap", "USDT -> DOT"),
            create_event("DEX", "Swap", "DOT -> USDT"),
            create_event("Lending", "Repaid", "amount: 1000000 USDT"),
        ];

        // Large balance changes indicating manipulation
        let state_changes = vec![
            create_state_change(
                "balance:USDT",
                1000u64.to_be_bytes().to_vec(),
                2000000u64.to_be_bytes().to_vec(), // 2000x increase
            ),
            create_state_change(
                "balance:DOT",
                5000u64.to_be_bytes().to_vec(),
                10000u64.to_be_bytes().to_vec(), // 2x increase
            ),
        ];

        let ctx = TransactionContext {
            transaction: tx,
            events,
            state_changes,
        };

        let result = detector.analyze_transaction(&ctx).await;

        // Should detect with high confidence
        assert!(result.detected, "Should detect classic flash loan pattern");
        assert!(result.confidence >= 0.7, "Confidence should be high (>=70%), got {}", result.confidence);
        assert_eq!(result.pattern, AttackPattern::FlashLoan);
        assert!(!result.evidence.is_empty(), "Should have evidence");

        // Verify evidence contains key indicators
        let evidence_text = result.evidence.join(" ");
        assert!(evidence_text.contains("borrow") || evidence_text.contains("Borrow"));
        assert!(evidence_text.contains("repay") || evidence_text.contains("Repay"));
    }

    #[tokio::test]
    async fn test_price_manipulation_flash_loan() {
        let detector = FlashLoanDetector::new();

        let tx = create_test_transaction(
            "0xmanipulation1",
            "DeFi",
            "execute_strategy",
        );

        // Flash loan used for price manipulation
        let events = vec![
            create_event("Lending", "FlashLoanBorrowed", "10000 DOT"),
            create_event("DEX", "Trade", "DOT -> KSM"),
            create_event("DEX", "Trade", "KSM -> USDT"),
            create_event("DEX", "Trade", "USDT -> DOT"),
            create_event("Lending", "FlashLoanRepaid", "10000 DOT + 10 DOT fee"),
        ];

        // Multiple large balance changes
        let state_changes = vec![
            create_state_change(
                "price:DOT",
                100u64.to_be_bytes().to_vec(),
                180u64.to_be_bytes().to_vec(), // 80% increase
            ),
            create_state_change(
                "balance:user",
                500u64.to_be_bytes().to_vec(),
                1500u64.to_be_bytes().to_vec(), // 3x profit
            ),
        ];

        let ctx = TransactionContext {
            transaction: tx,
            events,
            state_changes,
        };

        let result = detector.analyze_transaction(&ctx).await;

        assert!(result.detected, "Should detect price manipulation");
        assert!(result.confidence >= 0.6, "Should have good confidence");
        assert!(result.evidence.len() >= 3, "Should have multiple pieces of evidence");
    }

    #[tokio::test]
    async fn test_liquidation_cascade_flash_loan() {
        let detector = FlashLoanDetector::new();

        let tx = create_test_transaction(
            "0xliquidation1",
            "Lending",
            "liquidate_positions",
        );

        // Flash loan to trigger liquidation cascade
        let events = vec![
            create_event("Lending", "Borrow", "flash loan: 5000000 USDT"),
            create_event("Lending", "Liquidation", "position 1 liquidated"),
            create_event("Lending", "Liquidation", "position 2 liquidated"),
            create_event("Lending", "Liquidation", "position 3 liquidated"),
            create_event("DEX", "Swap", "collateral sold"),
            create_event("Lending", "Repay", "flash loan repaid"),
        ];

        let state_changes = vec![
            create_state_change(
                "collateral:vault",
                10000u64.to_be_bytes().to_vec(),
                1000u64.to_be_bytes().to_vec(), // 90% decrease
            ),
        ];

        let ctx = TransactionContext {
            transaction: tx,
            events,
            state_changes,
        };

        let result = detector.analyze_transaction(&ctx).await;

        assert!(result.detected, "Should detect liquidation cascade attack");
        assert!(result.confidence >= 0.5, "Should detect complex lending activity");
    }

    #[tokio::test]
    async fn test_no_flash_loan_normal_borrow() {
        let detector = FlashLoanDetector::new();

        // Normal borrow without repayment in same transaction
        let tx = create_test_transaction(
            "0xnormal1",
            "Lending",
            "borrow",
        );

        let events = vec![
            create_event("Lending", "Borrowed", "1000 USDT"),
            // No repayment - this is a normal borrow
        ];

        let ctx = TransactionContext {
            transaction: tx,
            events,
            state_changes: vec![],
        };

        let result = detector.analyze_transaction(&ctx).await;

        // Should have low confidence or not detect
        assert!(result.confidence < 0.3, "Normal borrow should have low confidence");
    }

    #[tokio::test]
    async fn test_no_flash_loan_simple_swap() {
        let detector = FlashLoanDetector::new();

        // Simple DEX swap - no flash loan
        let tx = create_test_transaction(
            "0xswap1",
            "DEX",
            "swap",
        );

        let events = vec![
            create_event("DEX", "Swap", "DOT -> USDT"),
        ];

        let ctx = TransactionContext {
            transaction: tx,
            events,
            state_changes: vec![],
        };

        let result = detector.analyze_transaction(&ctx).await;

        assert!(!result.detected, "Simple swap should not be detected as flash loan");
        assert_eq!(result.confidence, 0.0);
    }

    #[tokio::test]
    async fn test_partial_flash_loan_pattern() {
        let detector = FlashLoanDetector::new();

        // Borrow with DEX interactions but no repayment (suspicious but not flash loan)
        let tx = create_test_transaction(
            "0xpartial1",
            "Lending",
            "borrow_and_trade",
        );

        let events = vec![
            create_event("Lending", "Borrowed", "10000 DOT"),
            create_event("DEX", "Swap", "DOT -> KSM"),
            create_event("DEX", "Swap", "KSM -> USDT"),
            // No repayment
        ];

        let ctx = TransactionContext {
            transaction: tx,
            events,
            state_changes: vec![],
        };

        let result = detector.analyze_transaction(&ctx).await;

        // Should have very low confidence (incomplete pattern)
        assert!(result.confidence < 0.3, "Partial pattern should have low confidence");
    }

    #[tokio::test]
    async fn test_multiple_dex_interactions() {
        let detector = FlashLoanDetector::new();

        // Flash loan with many arbitrage swaps
        let tx = create_test_transaction(
            "0xarbitrage1",
            "DeFi",
            "arbitrage",
        );

        let events = vec![
            create_event("Lending", "FlashBorrow", "100000 USDT"),
            create_event("DEX", "Swap", "USDT -> DOT"),
            create_event("DEX", "Swap", "DOT -> KSM"),
            create_event("DEX", "Swap", "KSM -> ACA"),
            create_event("DEX", "Swap", "ACA -> USDT"),
            create_event("Lending", "FlashRepay", "100000 USDT"),
        ];

        let state_changes = vec![
            create_state_change(
                "profit",
                0u64.to_be_bytes().to_vec(),
                5000u64.to_be_bytes().to_vec(), // Profit from arbitrage
            ),
        ];

        let ctx = TransactionContext {
            transaction: tx,
            events,
            state_changes,
        };

        let result = detector.analyze_transaction(&ctx).await;

        assert!(result.detected, "Should detect multi-swap flash loan");
        assert!(result.confidence >= 0.7, "High confidence for classic pattern with many swaps");

        // Should mention classic pattern
        let evidence_text = result.evidence.join(" ");
        assert!(evidence_text.contains("Classic flash loan") || evidence_text.contains("classic"));
    }

    #[tokio::test]
    async fn test_balance_change_calculation() {
        let detector = FlashLoanDetector::new();

        // Test with extreme balance changes
        let tx = create_test_transaction("0xtest1", "Test", "test");

        let state_changes = vec![
            create_state_change(
                "balance:1",
                100u64.to_be_bytes().to_vec(),
                500u64.to_be_bytes().to_vec(), // 5x increase (>50%)
            ),
            create_state_change(
                "balance:2",
                1000u64.to_be_bytes().to_vec(),
                1100u64.to_be_bytes().to_vec(), // 10% increase (<50%)
            ),
            create_state_change(
                "balance:3",
                10000u64.to_be_bytes().to_vec(),
                2000u64.to_be_bytes().to_vec(), // 5x decrease (>50%)
            ),
        ];

        let count = FlashLoanDetector::count_large_balance_changes(&state_changes);

        // Should detect 2 large changes (5x increase and 5x decrease)
        assert_eq!(count, 2, "Should count 2 large balance changes (>50%)");
    }
}
