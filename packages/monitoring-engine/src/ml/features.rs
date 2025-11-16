//! Feature extraction for machine learning models
//!
//! Extracts numerical and categorical features from transaction contexts
//! for ML-based attack detection and prediction.

use crate::types::TransactionContext;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Extracted features for ML models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionFeatures {
    // Transaction Metadata Features
    pub block_number: f64,
    pub tx_index: f64,
    pub tx_success: f64, // 1.0 if success, 0.0 if failed
    pub has_signature: f64, // 1.0 if has signature, 0.0 otherwise
    pub nonce: f64,

    // Temporal Features
    pub hour_of_day: f64,
    pub day_of_week: f64,
    pub timestamp: f64,

    // Event Features
    pub event_count: f64,
    pub unique_event_types: f64,
    pub has_swap_events: f64,
    pub has_transfer_events: f64,
    pub has_borrow_events: f64,
    pub has_liquidation_events: f64,
    pub has_bridge_events: f64,

    // State Change Features
    pub state_change_count: f64,
    pub state_change_magnitude: f64, // Average number of changed bytes
    pub max_state_change: f64,

    // Behavioral Features
    pub is_dex_interaction: f64,
    pub is_lending_interaction: f64,
    pub is_bridge_interaction: f64,
    pub is_governance_interaction: f64,
    pub is_batch_call: f64,
    pub is_utility_call: f64,

    // Pattern Indicators
    pub rapid_succession_indicator: f64, // Based on nonce and index
    pub flash_loan_pattern: f64, // Has borrow + repay in events
    pub sandwich_risk: f64, // Based on DEX + sequential position
    pub cross_chain_activity: f64, // Has ISMP/Hyperbridge events

    // Complexity Metrics
    pub call_depth: f64, // Number of nested calls (estimated)
    pub data_size: f64, // Size of transaction args
    pub event_diversity: f64, // Shannon entropy of event types

    // Network Features
    pub caller_hash: f64, // Numeric hash of caller address
    pub pallet_category: f64, // Encoded pallet category

    // Additional context (not used directly in ML, but useful for analysis)
    pub tx_hash: String,
    pub caller: String,
    pub pallet: String,
    pub call: String,
}

/// Feature extractor for transaction contexts
pub struct FeatureExtractor {
    // Historical data for context-aware features
    caller_history: HashMap<String, CallerHistory>,
}

/// Historical information about a caller
#[derive(Debug, Clone, Default)]
struct CallerHistory {
    transaction_count: u64,
    last_seen_block: u64,
    attack_count: u64,
    avg_nonce_gap: f64,
}

impl FeatureExtractor {
    pub fn new() -> Self {
        Self {
            caller_history: HashMap::new(),
        }
    }

    /// Extract features from a transaction context
    pub fn extract_features(&mut self, ctx: &TransactionContext) -> TransactionFeatures {
        let tx = &ctx.transaction;

        // Transaction metadata
        let block_number = tx.block_number as f64;
        let tx_index = tx.index as f64;
        let tx_success = if tx.success { 1.0 } else { 0.0 };
        let has_signature = if tx.signature.is_some() { 1.0 } else { 0.0 };
        let nonce = tx.nonce.unwrap_or(0) as f64;

        // Temporal features
        let timestamp = tx.timestamp as f64;
        let datetime = chrono::DateTime::from_timestamp(tx.timestamp as i64, 0)
            .unwrap_or_else(|| chrono::Utc::now());
        let hour_of_day = datetime.hour() as f64;
        let day_of_week = datetime.weekday().num_days_from_monday() as f64;

        // Event features
        let event_count = ctx.events.len() as f64;
        let unique_event_types = Self::count_unique_event_types(&ctx.events);
        let has_swap_events = Self::has_event_type(&ctx.events, &["swap", "trade", "exchange"]);
        let has_transfer_events = Self::has_event_type(&ctx.events, &["transfer", "transferred"]);
        let has_borrow_events = Self::has_event_type(&ctx.events, &["borrow", "loan"]);
        let has_liquidation_events = Self::has_event_type(&ctx.events, &["liquidation", "liquidated"]);
        let has_bridge_events = Self::has_event_type(&ctx.events, &["request", "response", "ismp", "hyperbridge"]);

        // State change features
        let state_change_count = ctx.state_changes.len() as f64;
        let (state_change_magnitude, max_state_change) = Self::analyze_state_changes(&ctx.state_changes);

        // Behavioral features
        let pallet_lower = tx.pallet.to_lowercase();
        let call_lower = tx.call.to_lowercase();

        let is_dex_interaction = if pallet_lower.contains("dex") ||
            pallet_lower.contains("swap") ||
            pallet_lower.contains("omnipool") {
            1.0
        } else {
            0.0
        };

        let is_lending_interaction = if pallet_lower.contains("lending") ||
            pallet_lower.contains("borrow") ||
            call_lower.contains("borrow") ||
            call_lower.contains("lend") {
            1.0
        } else {
            0.0
        };

        let is_bridge_interaction = if pallet_lower.contains("ismp") ||
            pallet_lower.contains("hyperbridge") ||
            pallet_lower.contains("bridge") {
            1.0
        } else {
            0.0
        };

        let is_governance_interaction = if pallet_lower.contains("governance") ||
            pallet_lower.contains("democracy") ||
            pallet_lower.contains("council") {
            1.0
        } else {
            0.0
        };

        let is_batch_call = if call_lower.contains("batch") { 1.0 } else { 0.0 };
        let is_utility_call = if pallet_lower == "utility" { 1.0 } else { 0.0 };

        // Pattern indicators
        let rapid_succession_indicator = if nonce > 0.0 && nonce < 5.0 { 1.0 } else { 0.0 };

        let flash_loan_pattern = if Self::detect_flash_loan_pattern(&ctx.events) {
            1.0
        } else {
            0.0
        };

        let sandwich_risk = if is_dex_interaction > 0.0 && tx_index > 0.0 {
            0.5 // Medium risk if DEX + not first transaction
        } else {
            0.0
        };

        let cross_chain_activity = if has_bridge_events > 0.0 { 1.0 } else { 0.0 };

        // Complexity metrics
        let call_depth = Self::estimate_call_depth(&ctx.events);
        let data_size = tx.args.len() as f64;
        let event_diversity = Self::calculate_event_diversity(&ctx.events);

        // Network features
        let caller_hash = Self::hash_address(&tx.caller);
        let pallet_category = Self::encode_pallet_category(&tx.pallet);

        // Update caller history
        self.update_caller_history(&tx.caller, tx.block_number, nonce as u64);

        TransactionFeatures {
            block_number,
            tx_index,
            tx_success,
            has_signature,
            nonce,
            hour_of_day,
            day_of_week,
            timestamp,
            event_count,
            unique_event_types,
            has_swap_events,
            has_transfer_events,
            has_borrow_events,
            has_liquidation_events,
            has_bridge_events,
            state_change_count,
            state_change_magnitude,
            max_state_change,
            is_dex_interaction,
            is_lending_interaction,
            is_bridge_interaction,
            is_governance_interaction,
            is_batch_call,
            is_utility_call,
            rapid_succession_indicator,
            flash_loan_pattern,
            sandwich_risk,
            cross_chain_activity,
            call_depth,
            data_size,
            event_diversity,
            caller_hash,
            pallet_category,
            tx_hash: tx.hash.clone(),
            caller: tx.caller.clone(),
            pallet: tx.pallet.clone(),
            call: tx.call.clone(),
        }
    }

    /// Get feature vector as array (for ML models)
    pub fn to_vector(features: &TransactionFeatures) -> Vec<f64> {
        vec![
            features.block_number,
            features.tx_index,
            features.tx_success,
            features.has_signature,
            features.nonce,
            features.hour_of_day,
            features.day_of_week,
            features.timestamp,
            features.event_count,
            features.unique_event_types,
            features.has_swap_events,
            features.has_transfer_events,
            features.has_borrow_events,
            features.has_liquidation_events,
            features.has_bridge_events,
            features.state_change_count,
            features.state_change_magnitude,
            features.max_state_change,
            features.is_dex_interaction,
            features.is_lending_interaction,
            features.is_bridge_interaction,
            features.is_governance_interaction,
            features.is_batch_call,
            features.is_utility_call,
            features.rapid_succession_indicator,
            features.flash_loan_pattern,
            features.sandwich_risk,
            features.cross_chain_activity,
            features.call_depth,
            features.data_size,
            features.event_diversity,
            features.caller_hash,
            features.pallet_category,
        ]
    }

    /// Get feature names (for model interpretation)
    pub fn feature_names() -> Vec<&'static str> {
        vec![
            "block_number",
            "tx_index",
            "tx_success",
            "has_signature",
            "nonce",
            "hour_of_day",
            "day_of_week",
            "timestamp",
            "event_count",
            "unique_event_types",
            "has_swap_events",
            "has_transfer_events",
            "has_borrow_events",
            "has_liquidation_events",
            "has_bridge_events",
            "state_change_count",
            "state_change_magnitude",
            "max_state_change",
            "is_dex_interaction",
            "is_lending_interaction",
            "is_bridge_interaction",
            "is_governance_interaction",
            "is_batch_call",
            "is_utility_call",
            "rapid_succession_indicator",
            "flash_loan_pattern",
            "sandwich_risk",
            "cross_chain_activity",
            "call_depth",
            "data_size",
            "event_diversity",
            "caller_hash",
            "pallet_category",
        ]
    }

    // Helper methods

    fn count_unique_event_types(events: &[crate::types::ChainEvent]) -> f64 {
        let unique: std::collections::HashSet<_> = events
            .iter()
            .map(|e| (&e.pallet, &e.event_name))
            .collect();
        unique.len() as f64
    }

    fn has_event_type(events: &[crate::types::ChainEvent], keywords: &[&str]) -> f64 {
        for event in events {
            let event_lower = event.event_name.to_lowercase();
            let pallet_lower = event.pallet.to_lowercase();

            for keyword in keywords {
                if event_lower.contains(keyword) || pallet_lower.contains(keyword) {
                    return 1.0;
                }
            }
        }
        0.0
    }

    fn analyze_state_changes(state_changes: &[crate::types::StateChange]) -> (f64, f64) {
        if state_changes.is_empty() {
            return (0.0, 0.0);
        }

        let mut total_magnitude = 0.0;
        let mut max_change = 0.0;

        for sc in state_changes {
            // Calculate magnitude as number of bytes that changed
            let magnitude = if let (Some(old), Some(new)) = (&sc.old_value, &sc.new_value) {
                let changed_bytes = old.iter()
                    .zip(new.iter())
                    .filter(|(a, b)| a != b)
                    .count();
                changed_bytes as f64
            } else {
                sc.key.len() as f64 // If no old/new value, use key size
            };

            total_magnitude += magnitude;
            max_change = max_change.max(magnitude);
        }

        let avg_magnitude = total_magnitude / state_changes.len() as f64;
        (avg_magnitude, max_change)
    }

    fn detect_flash_loan_pattern(events: &[crate::types::ChainEvent]) -> bool {
        let has_borrow = events.iter().any(|e| {
            let event_lower = e.event_name.to_lowercase();
            event_lower.contains("borrow") || event_lower.contains("loan")
        });

        let has_repay = events.iter().any(|e| {
            let event_lower = e.event_name.to_lowercase();
            event_lower.contains("repay") || event_lower.contains("return")
        });

        has_borrow && has_repay
    }

    fn estimate_call_depth(events: &[crate::types::ChainEvent]) -> f64 {
        // Estimate based on number of unique pallets involved
        let unique_pallets: std::collections::HashSet<_> = events
            .iter()
            .map(|e| &e.pallet)
            .collect();
        unique_pallets.len() as f64
    }

    fn calculate_event_diversity(events: &[crate::types::ChainEvent]) -> f64 {
        if events.is_empty() {
            return 0.0;
        }

        // Count frequency of each event type
        let mut counts: HashMap<String, usize> = HashMap::new();
        for event in events {
            let key = format!("{}.{}", event.pallet, event.event_name);
            *counts.entry(key).or_insert(0) += 1;
        }

        // Calculate Shannon entropy
        let total = events.len() as f64;
        let mut entropy = 0.0;

        for count in counts.values() {
            let p = *count as f64 / total;
            if p > 0.0 {
                entropy -= p * p.log2();
            }
        }

        entropy
    }

    fn hash_address(address: &str) -> f64 {
        // Simple hash function for address
        let hash = address.bytes().fold(0u64, |acc, b| {
            acc.wrapping_mul(31).wrapping_add(b as u64)
        });

        // Normalize to [0, 1]
        (hash % 1_000_000) as f64 / 1_000_000.0
    }

    fn encode_pallet_category(pallet: &str) -> f64 {
        let pallet_lower = pallet.to_lowercase();

        // Encode common pallet categories as numbers
        if pallet_lower.contains("balance") {
            1.0
        } else if pallet_lower.contains("dex") || pallet_lower.contains("swap") || pallet_lower.contains("omnipool") {
            2.0
        } else if pallet_lower.contains("lending") || pallet_lower.contains("borrow") {
            3.0
        } else if pallet_lower.contains("bridge") || pallet_lower.contains("ismp") || pallet_lower.contains("hyperbridge") {
            4.0
        } else if pallet_lower.contains("governance") || pallet_lower.contains("democracy") {
            5.0
        } else if pallet_lower.contains("staking") {
            6.0
        } else if pallet_lower.contains("utility") {
            7.0
        } else if pallet_lower.contains("system") {
            8.0
        } else {
            0.0 // Unknown
        }
    }

    fn update_caller_history(&mut self, caller: &str, block: u64, nonce: u64) {
        let history = self.caller_history
            .entry(caller.to_string())
            .or_insert_with(CallerHistory::default);

        history.transaction_count += 1;

        // Update nonce gap average
        if history.last_seen_block > 0 && block > history.last_seen_block {
            let gap = block - history.last_seen_block;
            history.avg_nonce_gap = (history.avg_nonce_gap * 0.8) + (gap as f64 * 0.2);
        }

        history.last_seen_block = block;
    }

    /// Get caller history for contextual features
    pub fn get_caller_history(&self, caller: &str) -> Option<&CallerHistory> {
        self.caller_history.get(caller)
    }
}

impl Default for FeatureExtractor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{ChainEvent, ParsedTransaction};

    #[test]
    fn test_feature_extraction() {
        let mut extractor = FeatureExtractor::new();

        let ctx = TransactionContext {
            transaction: ParsedTransaction {
                hash: "0x123".to_string(),
                block_number: 1000,
                block_hash: "0xabc".to_string(),
                index: 5,
                caller: "Alice".to_string(),
                pallet: "Balances".to_string(),
                call: "transfer".to_string(),
                args: vec![1, 2, 3, 4],
                signature: Some("sig".to_string()),
                nonce: Some(10),
                timestamp: 1700000000,
                success: true,
            },
            events: vec![
                ChainEvent {
                    pallet: "Balances".to_string(),
                    event_name: "Transfer".to_string(),
                    event_data: None,
                },
            ],
            state_changes: vec![],
        };

        let features = extractor.extract_features(&ctx);

        assert_eq!(features.block_number, 1000.0);
        assert_eq!(features.tx_index, 5.0);
        assert_eq!(features.tx_success, 1.0);
        assert_eq!(features.has_signature, 1.0);
        assert_eq!(features.nonce, 10.0);
        assert_eq!(features.event_count, 1.0);
    }

    #[test]
    fn test_flash_loan_detection() {
        let events = vec![
            ChainEvent {
                pallet: "Lending".to_string(),
                event_name: "Borrow".to_string(),
                event_data: None,
            },
            ChainEvent {
                pallet: "Lending".to_string(),
                event_name: "Repay".to_string(),
                event_data: None,
            },
        ];

        assert!(FeatureExtractor::detect_flash_loan_pattern(&events));
    }
}
