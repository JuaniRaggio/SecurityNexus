//! Hydration DeFi attack detectors
//!
//! Detects DeFi attack patterns specific to Hydration protocol:
//! 1. Omnipool manipulation (sandwich attacks, flash loans, oracle deviation)
//! 2. Liquidity drain attacks (massive withdrawals, rug pulls)
//! 3. Collateral manipulation (liquidation cascades, ratio manipulation)

use crate::detectors::Detector;
use crate::types::{AttackPattern, DetectionResult, TransactionContext};
use async_trait::async_trait;
use std::collections::HashMap;

/// Detector for Omnipool manipulation attacks
pub struct OmnipoolManipulationDetector {
    enabled: bool,
}

/// Omnipool attack indicators
struct OmnipoolIndicators {
    has_swap: bool,
    has_add_liquidity: bool,
    has_remove_liquidity: bool,
    swap_count: usize,
    large_price_impact: bool,
    flash_loan_pattern: bool,
    rapid_swaps: bool,
    oracle_deviation: bool,
}

impl OmnipoolManipulationDetector {
    pub fn new() -> Self {
        Self { enabled: true }
    }

    /// Analyze events for Omnipool indicators
    fn analyze_events(ctx: &TransactionContext) -> OmnipoolIndicators {
        let mut has_swap = false;
        let mut has_add_liquidity = false;
        let mut has_remove_liquidity = false;
        let mut swap_count = 0;
        let mut large_price_impact = false;
        let mut flash_loan_pattern = false;
        let mut oracle_deviation = false;

        for event in &ctx.events {
            let event_name_lower = event.event_name.to_lowercase();
            let pallet_lower = event.pallet.to_lowercase();

            // Detect Omnipool/Hydration specific events
            if pallet_lower.contains("omnipool") || pallet_lower.contains("hydration") {
                // Detect swaps
                if event_name_lower.contains("swap") || event_name_lower.contains("trade") {
                    has_swap = true;
                    swap_count += 1;

                    // Check for large price impact in event data
                    if let Some(data) = &event.event_data {
                        if let Some(impact) = data.get("price_impact").and_then(|v| v.as_f64()) {
                            if impact > 0.05 { // More than 5% price impact
                                large_price_impact = true;
                            }
                        }

                        // Check for oracle deviation
                        if let Some(deviation) = data.get("oracle_deviation").and_then(|v| v.as_f64()) {
                            if deviation > 0.03 { // More than 3% deviation
                                oracle_deviation = true;
                            }
                        }
                    }
                }

                // Detect liquidity operations
                if event_name_lower.contains("addliquidity") || event_name_lower.contains("add_liquidity") {
                    has_add_liquidity = true;
                }

                if event_name_lower.contains("removeliquidity") || event_name_lower.contains("remove_liquidity") {
                    has_remove_liquidity = true;
                }
            }

            // Detect flash loan pattern (borrow + repay in same tx)
            if event_name_lower.contains("borrow") || event_name_lower.contains("loan") {
                // Check if there's also a repay in the same transaction
                let has_repay = ctx.events.iter().any(|e| {
                    e.event_name.to_lowercase().contains("repay") ||
                    e.event_name.to_lowercase().contains("repaid")
                });
                if has_repay {
                    flash_loan_pattern = true;
                }
            }
        }

        OmnipoolIndicators {
            has_swap,
            has_add_liquidity,
            has_remove_liquidity,
            swap_count,
            large_price_impact,
            flash_loan_pattern,
            rapid_swaps: swap_count > 2, // More than 2 swaps in one tx
            oracle_deviation,
        }
    }

    /// Calculate confidence score
    fn calculate_confidence(indicators: &OmnipoolIndicators) -> f64 {
        let mut confidence: f64 = 0.0;

        // Flash loan + swap is highly suspicious
        if indicators.flash_loan_pattern && indicators.has_swap {
            confidence += 0.6;
        }

        // Large price impact
        if indicators.large_price_impact {
            confidence += 0.3;
        }

        // Rapid swaps (sandwich pattern)
        if indicators.rapid_swaps {
            confidence += 0.25;
        }

        // Oracle deviation exploitation
        if indicators.oracle_deviation {
            confidence += 0.2;
        }

        // Add + Remove liquidity in same tx (potential wash trading)
        if indicators.has_add_liquidity && indicators.has_remove_liquidity {
            confidence += 0.15;
        }

        confidence.min(1.0)
    }

    /// Build evidence
    fn build_evidence(indicators: &OmnipoolIndicators) -> Vec<String> {
        let mut evidence = Vec::new();

        if indicators.flash_loan_pattern && indicators.has_swap {
            evidence.push("Flash loan combined with Omnipool swap (potential price manipulation)".to_string());
        }

        if indicators.large_price_impact {
            evidence.push("Large price impact detected (>5%)".to_string());
        }

        if indicators.rapid_swaps {
            evidence.push(format!("{} swaps in single transaction (potential sandwich attack)", indicators.swap_count));
        }

        if indicators.oracle_deviation {
            evidence.push("Oracle price deviation detected (>3%)".to_string());
        }

        if indicators.has_add_liquidity && indicators.has_remove_liquidity {
            evidence.push("Liquidity added and removed in same transaction".to_string());
        }

        evidence
    }
}

#[async_trait]
impl Detector for OmnipoolManipulationDetector {
    fn name(&self) -> &str {
        "Omnipool Manipulation Detector"
    }

    async fn analyze_transaction(&self, ctx: &TransactionContext) -> DetectionResult {
        if !self.enabled {
            return DetectionResult::safe();
        }

        let indicators = Self::analyze_events(ctx);

        // Only analyze if there's Omnipool activity
        if !indicators.has_swap && !indicators.has_add_liquidity && !indicators.has_remove_liquidity {
            return DetectionResult::safe();
        }

        let confidence = Self::calculate_confidence(&indicators);
        let evidence = Self::build_evidence(&indicators);
        let detected = confidence > 0.5;

        DetectionResult {
            detected,
            confidence,
            pattern: AttackPattern::OmnipoolManipulation,
            description: if detected {
                "Suspicious Omnipool activity detected. Potential sandwich attack, flash loan manipulation, or oracle exploitation.".to_string()
            } else {
                "Omnipool activity appears normal.".to_string()
            },
            evidence,
            metadata: HashMap::new(),
        }
    }

    fn is_enabled(&self) -> bool {
        self.enabled
    }
}

/// Detector for liquidity drain attacks
pub struct LiquidityDrainDetector {
    enabled: bool,
}

/// Liquidity drain indicators
struct LiquidityDrainIndicators {
    large_withdrawal: bool,
    multiple_withdrawals: usize,
    rapid_succession: bool,
    pool_depletion_risk: bool,
    suspicious_timing: bool,
}

impl LiquidityDrainDetector {
    pub fn new() -> Self {
        Self { enabled: true }
    }

    /// Analyze events for liquidity drain indicators
    fn analyze_events(ctx: &TransactionContext) -> LiquidityDrainIndicators {
        let mut withdrawal_count = 0;
        let mut large_withdrawal = false;
        let mut pool_depletion_risk = false;
        let mut suspicious_timing = false;

        for event in &ctx.events {
            let event_name_lower = event.event_name.to_lowercase();
            let pallet_lower = event.pallet.to_lowercase();

            if pallet_lower.contains("omnipool") || pallet_lower.contains("hydration") || pallet_lower.contains("liquidity") {
                // Detect liquidity removals
                if event_name_lower.contains("removeliquidity") ||
                   event_name_lower.contains("remove_liquidity") ||
                   event_name_lower.contains("withdraw") {
                    withdrawal_count += 1;

                    // Check withdrawal size
                    if let Some(data) = &event.event_data {
                        if let Some(amount) = data.get("amount").and_then(|v| v.as_f64()) {
                            // Check if withdrawal is more than 10% of pool (simplified)
                            if amount > 1_000_000.0 { // Large amount threshold
                                large_withdrawal = true;
                            }
                        }

                        // Check pool state
                        if let Some(remaining) = data.get("remaining_liquidity").and_then(|v| v.as_f64()) {
                            if remaining < 100_000.0 { // Low liquidity threshold
                                pool_depletion_risk = true;
                            }
                        }

                        // Check timing (e.g., right after a major event)
                        if let Some(timing_flag) = data.get("suspicious_timing").and_then(|v| v.as_bool()) {
                            suspicious_timing = timing_flag;
                        }
                    }
                }
            }
        }

        LiquidityDrainIndicators {
            large_withdrawal,
            multiple_withdrawals: withdrawal_count,
            rapid_succession: withdrawal_count > 1,
            pool_depletion_risk,
            suspicious_timing,
        }
    }

    /// Calculate confidence
    fn calculate_confidence(indicators: &LiquidityDrainIndicators) -> f64 {
        let mut confidence: f64 = 0.0;

        // Large withdrawal is concerning
        if indicators.large_withdrawal {
            confidence += 0.4;
        }

        // Multiple withdrawals in one tx
        if indicators.rapid_succession {
            confidence += 0.3;
        }

        // Pool depletion risk
        if indicators.pool_depletion_risk {
            confidence += 0.4;
        }

        // Suspicious timing
        if indicators.suspicious_timing {
            confidence += 0.2;
        }

        confidence.min(1.0)
    }

    /// Build evidence
    fn build_evidence(indicators: &LiquidityDrainIndicators) -> Vec<String> {
        let mut evidence = Vec::new();

        if indicators.large_withdrawal {
            evidence.push("Large liquidity withdrawal detected".to_string());
        }

        if indicators.rapid_succession {
            evidence.push(format!("{} withdrawals in single transaction", indicators.multiple_withdrawals));
        }

        if indicators.pool_depletion_risk {
            evidence.push("Pool depletion risk - remaining liquidity critically low".to_string());
        }

        if indicators.suspicious_timing {
            evidence.push("Withdrawal timing appears suspicious".to_string());
        }

        evidence
    }
}

#[async_trait]
impl Detector for LiquidityDrainDetector {
    fn name(&self) -> &str {
        "Liquidity Drain Detector"
    }

    async fn analyze_transaction(&self, ctx: &TransactionContext) -> DetectionResult {
        if !self.enabled {
            return DetectionResult::safe();
        }

        let indicators = Self::analyze_events(ctx);

        // Only analyze if there are withdrawals
        if indicators.multiple_withdrawals == 0 {
            return DetectionResult::safe();
        }

        let confidence = Self::calculate_confidence(&indicators);
        let evidence = Self::build_evidence(&indicators);
        let detected = confidence > 0.5;

        DetectionResult {
            detected,
            confidence,
            pattern: AttackPattern::LiquidityDrain,
            description: if detected {
                "Suspicious liquidity withdrawal pattern detected. Potential liquidity drain or rug pull attempt.".to_string()
            } else {
                "Liquidity withdrawal appears normal.".to_string()
            },
            evidence,
            metadata: HashMap::new(),
        }
    }

    fn is_enabled(&self) -> bool {
        self.enabled
    }
}

/// Detector for collateral manipulation attacks
pub struct CollateralManipulationDetector {
    enabled: bool,
}

/// Collateral manipulation indicators
struct CollateralIndicators {
    has_liquidation: bool,
    liquidation_count: usize,
    has_collateral_change: bool,
    health_factor_drop: bool,
    flash_loan_liquidation: bool,
    cascade_risk: bool,
}

impl CollateralManipulationDetector {
    pub fn new() -> Self {
        Self { enabled: true }
    }

    /// Analyze events for collateral indicators
    fn analyze_events(ctx: &TransactionContext) -> CollateralIndicators {
        let mut has_liquidation = false;
        let mut liquidation_count = 0;
        let mut has_collateral_change = false;
        let mut health_factor_drop = false;
        let mut flash_loan_liquidation = false;
        let mut cascade_risk = false;

        // Check for flash loans
        let has_flash_loan = ctx.events.iter().any(|e| {
            let name = e.event_name.to_lowercase();
            (name.contains("borrow") || name.contains("loan")) &&
            ctx.events.iter().any(|e2| e2.event_name.to_lowercase().contains("repay"))
        });

        for event in &ctx.events {
            let event_name_lower = event.event_name.to_lowercase();
            let pallet_lower = event.pallet.to_lowercase();

            // Detect lending/borrowing protocol events
            if pallet_lower.contains("lending") || pallet_lower.contains("loan") || pallet_lower.contains("collateral") {
                // Detect liquidations
                if event_name_lower.contains("liquidat") {
                    has_liquidation = true;
                    liquidation_count += 1;

                    if has_flash_loan {
                        flash_loan_liquidation = true;
                    }

                    // Check for cascade indicators
                    if let Some(data) = &event.event_data {
                        if let Some(cascade) = data.get("cascade_risk").and_then(|v| v.as_bool()) {
                            cascade_risk = cascade;
                        }
                    }
                }

                // Detect collateral changes
                if event_name_lower.contains("collateral") {
                    has_collateral_change = true;

                    // Check health factor
                    if let Some(data) = &event.event_data {
                        if let Some(health_before) = data.get("health_factor_before").and_then(|v| v.as_f64()) {
                            if let Some(health_after) = data.get("health_factor_after").and_then(|v| v.as_f64()) {
                                // Significant drop in health factor
                                if health_before - health_after > 0.3 {
                                    health_factor_drop = true;
                                }
                            }
                        }
                    }
                }
            }
        }

        CollateralIndicators {
            has_liquidation,
            liquidation_count,
            has_collateral_change,
            health_factor_drop,
            flash_loan_liquidation,
            cascade_risk,
        }
    }

    /// Calculate confidence
    fn calculate_confidence(indicators: &CollateralIndicators) -> f64 {
        let mut confidence: f64 = 0.0;

        // Flash loan + liquidation is highly suspicious
        if indicators.flash_loan_liquidation {
            confidence += 0.7;
        }

        // Multiple liquidations in one tx
        if indicators.liquidation_count > 1 {
            confidence += 0.4;
        }

        // Cascade risk
        if indicators.cascade_risk {
            confidence += 0.5;
        }

        // Significant health factor drop
        if indicators.health_factor_drop {
            confidence += 0.3;
        }

        confidence.min(1.0)
    }

    /// Build evidence
    fn build_evidence(indicators: &CollateralIndicators) -> Vec<String> {
        let mut evidence = Vec::new();

        if indicators.flash_loan_liquidation {
            evidence.push("Flash loan used in combination with liquidation (potential manipulation)".to_string());
        }

        if indicators.liquidation_count > 1 {
            evidence.push(format!("{} liquidations in single transaction", indicators.liquidation_count));
        }

        if indicators.cascade_risk {
            evidence.push("Liquidation cascade risk detected".to_string());
        }

        if indicators.health_factor_drop {
            evidence.push("Significant health factor drop detected (>30%)".to_string());
        }

        evidence
    }
}

#[async_trait]
impl Detector for CollateralManipulationDetector {
    fn name(&self) -> &str {
        "Collateral Manipulation Detector"
    }

    async fn analyze_transaction(&self, ctx: &TransactionContext) -> DetectionResult {
        if !self.enabled {
            return DetectionResult::safe();
        }

        let indicators = Self::analyze_events(ctx);

        // Only analyze if there's lending/collateral activity
        if !indicators.has_liquidation && !indicators.has_collateral_change {
            return DetectionResult::safe();
        }

        let confidence = Self::calculate_confidence(&indicators);
        let evidence = Self::build_evidence(&indicators);
        let detected = confidence > 0.5;

        DetectionResult {
            detected,
            confidence,
            pattern: AttackPattern::CollateralManipulation,
            description: if detected {
                "Suspicious collateral/liquidation activity detected. Potential flash loan liquidation or cascade manipulation.".to_string()
            } else {
                "Collateral activity appears normal.".to_string()
            },
            evidence,
            metadata: HashMap::new(),
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

    #[tokio::test]
    async fn test_omnipool_detector_safe() {
        let detector = OmnipoolManipulationDetector::new();
        let ctx = TransactionContext {
            transaction: ParsedTransaction {
                hash: "0x123".to_string(),
                block_number: 100,
                block_hash: "0xabc".to_string(),
                index: 0,
                caller: "Alice".to_string(),
                pallet: "Balances".to_string(),
                call: "transfer".to_string(),
                args: vec![],
                success: true,
                timestamp: 0,
                signature: None,
                nonce: None,
            },
            events: vec![],
            state_changes: vec![],
        };

        let result = detector.analyze_transaction(&ctx).await;
        assert!(!result.detected);
    }

    #[tokio::test]
    async fn test_liquidity_drain_detector() {
        let detector = LiquidityDrainDetector::new();
        let ctx = TransactionContext {
            transaction: ParsedTransaction {
                hash: "0x456".to_string(),
                block_number: 101,
                block_hash: "0xdef".to_string(),
                index: 0,
                caller: "Attacker".to_string(),
                pallet: "Omnipool".to_string(),
                call: "remove_liquidity".to_string(),
                args: vec![],
                success: true,
                timestamp: 0,
                signature: None,
                nonce: None,
            },
            events: vec![
                ChainEvent {
                    pallet: "Omnipool".to_string(),
                    event_name: "LiquidityRemoved".to_string(),
                    event_data: Some(serde_json::json!({
                        "amount": 2000000.0,
                        "remaining_liquidity": 50000.0
                    })),
                },
            ],
            state_changes: vec![],
        };

        let result = detector.analyze_transaction(&ctx).await;
        assert!(result.detected);
        assert!(result.confidence >= 0.5);
    }
}
