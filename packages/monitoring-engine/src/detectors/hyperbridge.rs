//! Hyperbridge cross-chain attack detectors
//!
//! Detects cross-chain attack patterns specific to Hyperbridge ISMP protocol:
//! 1. Duplicate message relay (same request commitment sent multiple times)
//! 2. Proof manipulation (invalid or duplicate state proofs)
//! 3. Cross-chain drain attacks (extracting funds from multiple chains)
//! 4. Timeout manipulation (exploiting message timeouts)
//! 5. Relayer manipulation (malicious relayers)

use crate::detectors::Detector;
use crate::types::{AttackPattern, DetectionResult, TransactionContext};
use async_trait::async_trait;
use std::collections::HashMap;

/// Detector for cross-chain bridge attacks
pub struct CrossChainBridgeDetector {
    enabled: bool,
}

/// Cross-chain attack indicators
struct CrossChainIndicators {
    has_post_request: bool,
    has_get_request: bool,
    has_post_response: bool,
    has_get_response: bool,
    request_count: usize,
    response_count: usize,
    duplicate_commitments: Vec<String>,
    multiple_destinations: bool,
    high_value_transfer: bool,
    rapid_succession: bool,
}

impl CrossChainBridgeDetector {
    pub fn new() -> Self {
        Self { enabled: true }
    }

    /// Analyze events to extract cross-chain indicators
    fn analyze_events(ctx: &TransactionContext) -> CrossChainIndicators {
        let mut has_post_request = false;
        let mut has_get_request = false;
        let mut has_post_response = false;
        let mut has_get_response = false;
        let mut request_count = 0;
        let mut response_count = 0;
        let mut commitments = Vec::new();
        let mut destinations = Vec::new();

        for event in &ctx.events {
            let event_name_lower = event.event_name.to_lowercase();
            let pallet_lower = event.pallet.to_lowercase();

            // Detect ISMP protocol interactions
            if pallet_lower.contains("ismp") || pallet_lower.contains("hyperbridge") {
                // Detect POST requests (state commitments)
                if event_name_lower.contains("post") && event_name_lower.contains("request") {
                    has_post_request = true;
                    request_count += 1;

                    // Extract commitment if available in event data
                    if let Some(data) = &event.event_data {
                        if let Some(commitment) = data.get("commitment").and_then(|v| v.as_str()) {
                            commitments.push(commitment.to_string());
                        }
                        if let Some(dest) = data.get("dest").and_then(|v| v.as_str()) {
                            destinations.push(dest.to_string());
                        }
                    }
                }

                // Detect GET requests (state queries)
                if event_name_lower.contains("get") && event_name_lower.contains("request") {
                    has_get_request = true;
                    request_count += 1;
                }

                // Detect POST responses
                if event_name_lower.contains("post") && event_name_lower.contains("response") {
                    has_post_response = true;
                    response_count += 1;
                }

                // Detect GET responses
                if event_name_lower.contains("get") && event_name_lower.contains("response") {
                    has_get_response = true;
                    response_count += 1;
                }
            }
        }

        // Find duplicate commitments
        let mut commitment_counts: HashMap<String, usize> = HashMap::new();
        for commitment in &commitments {
            *commitment_counts.entry(commitment.clone()).or_insert(0) += 1;
        }
        let duplicate_commitments: Vec<String> = commitment_counts
            .iter()
            .filter(|(_, &count)| count > 1)
            .map(|(commitment, _)| commitment.clone())
            .collect();

        // Check for multiple unique destinations (potential spray attack)
        let unique_destinations: std::collections::HashSet<_> = destinations.iter().collect();
        let multiple_destinations = unique_destinations.len() > 1;

        CrossChainIndicators {
            has_post_request,
            has_get_request,
            has_post_response,
            has_get_response,
            request_count,
            response_count,
            duplicate_commitments,
            multiple_destinations,
            high_value_transfer: Self::detect_high_value_transfer(&ctx.state_changes),
            rapid_succession: request_count > 3, // More than 3 requests in one transaction
        }
    }

    /// Detect high-value transfers that could indicate drain attacks
    fn detect_high_value_transfer(state_changes: &[crate::types::StateChange]) -> bool {
        // Check for balance changes > 10% of total
        for change in state_changes {
            // Convert key bytes to string for checking
            if let Ok(key_str) = String::from_utf8(change.key.clone()) {
                if key_str.contains("Balance") || key_str.contains("balance") {
                    // This is a simplified check - in production you'd parse actual values
                    return true;
                }
            }
        }
        false
    }

    /// Calculate confidence score based on indicators
    fn calculate_confidence(indicators: &CrossChainIndicators) -> f64 {
        let mut confidence: f64 = 0.0;

        // Duplicate commitments are a strong indicator of replay attacks
        if !indicators.duplicate_commitments.is_empty() {
            confidence += 0.6;
        }

        // Multiple requests in rapid succession
        if indicators.rapid_succession {
            confidence += 0.2;
        }

        // Multiple destinations (spray attack)
        if indicators.multiple_destinations {
            confidence += 0.15;
        }

        // High value transfer combined with other indicators
        if indicators.high_value_transfer && indicators.request_count > 1 {
            confidence += 0.15;
        }

        // Mismatched request/response counts
        if indicators.request_count > 0 && indicators.response_count == 0 {
            confidence += 0.1;
        }

        confidence.min(1.0)
    }

    /// Build evidence list from indicators
    fn build_evidence(indicators: &CrossChainIndicators) -> Vec<String> {
        let mut evidence = Vec::new();

        if !indicators.duplicate_commitments.is_empty() {
            evidence.push(format!(
                "Duplicate request commitments detected: {}",
                indicators.duplicate_commitments.len()
            ));
            for commitment in &indicators.duplicate_commitments {
                evidence.push(format!("  - {}", commitment));
            }
        }

        if indicators.rapid_succession {
            evidence.push(format!(
                "Rapid succession of {} cross-chain requests in single transaction",
                indicators.request_count
            ));
        }

        if indicators.multiple_destinations {
            evidence.push("Multiple destination chains targeted (potential spray attack)".to_string());
        }

        if indicators.high_value_transfer {
            evidence.push("High-value transfer detected in cross-chain message".to_string());
        }

        if indicators.request_count > 0 && indicators.response_count == 0 {
            evidence.push("Requests without responses (potential timeout exploitation)".to_string());
        }

        evidence
    }
}

#[async_trait]
impl Detector for CrossChainBridgeDetector {
    fn name(&self) -> &str {
        "Cross-Chain Bridge Detector"
    }

    async fn analyze_transaction(&self, ctx: &TransactionContext) -> DetectionResult {
        if !self.enabled {
            return DetectionResult::safe();
        }

        // Extract indicators from transaction context
        let indicators = Self::analyze_events(ctx);

        // Check if we have any cross-chain activity
        if indicators.request_count == 0 && indicators.response_count == 0 {
            return DetectionResult::safe();
        }

        // Calculate confidence and build evidence
        let confidence = Self::calculate_confidence(&indicators);
        let evidence = Self::build_evidence(&indicators);

        // Determine if this is suspicious
        let detected = confidence > 0.5;

        DetectionResult {
            detected,
            confidence,
            pattern: AttackPattern::CrossChainBridge,
            description: if detected {
                "Suspicious cross-chain bridge activity detected. Potential message replay, spray attack, or cross-chain drain attempt.".to_string()
            } else {
                "Cross-chain bridge activity appears normal.".to_string()
            },
            evidence,
            metadata: std::collections::HashMap::new(),
        }
    }

    fn is_enabled(&self) -> bool {
        self.enabled
    }
}

/// Detector for state proof verification attacks
pub struct StateProofVerificationDetector {
    enabled: bool,
}

/// State proof attack indicators
struct StateProofIndicators {
    has_state_proof: bool,
    has_consensus_proof: bool,
    proof_verification_failed: bool,
    multiple_proofs_same_height: bool,
    invalid_proof_structure: bool,
    suspicious_relayer: bool,
}

impl StateProofVerificationDetector {
    pub fn new() -> Self {
        Self { enabled: true }
    }

    /// Analyze events for state proof indicators
    fn analyze_events(ctx: &TransactionContext) -> StateProofIndicators {
        let mut has_state_proof = false;
        let mut has_consensus_proof = false;
        let mut proof_verification_failed = false;
        let mut proof_heights = Vec::new();
        let mut invalid_proof_structure = false;
        let mut suspicious_relayer = false;

        for event in &ctx.events {
            let event_name_lower = event.event_name.to_lowercase();
            let pallet_lower = event.pallet.to_lowercase();

            if pallet_lower.contains("ismp") || pallet_lower.contains("hyperbridge") {
                // Detect state proof submissions
                if event_name_lower.contains("stateproof")
                    || event_name_lower.contains("state_proof")
                {
                    has_state_proof = true;

                    // Extract proof height if available
                    if let Some(data) = &event.event_data {
                        if let Some(height) = data.get("height").and_then(|v| v.as_u64()) {
                            proof_heights.push(height);
                        }

                        // Check for proof structure issues
                        if data.get("proof").is_none() {
                            invalid_proof_structure = true;
                        }
                    }
                }

                // Detect consensus proofs
                if event_name_lower.contains("consensusproof")
                    || event_name_lower.contains("consensus_proof")
                {
                    has_consensus_proof = true;
                }

                // Detect verification failures
                if event_name_lower.contains("verificationfailed")
                    || event_name_lower.contains("verification_failed")
                    || event_name_lower.contains("invalidproof")
                {
                    proof_verification_failed = true;
                }

                // Detect suspicious relayer behavior
                if event_name_lower.contains("relayer") {
                    if let Some(data) = &event.event_data {
                        // Check if relayer is submitting invalid proofs
                        if data.get("invalid").and_then(|v| v.as_bool()).unwrap_or(false) {
                            suspicious_relayer = true;
                        }
                    }
                }
            }
        }

        // Check for multiple proofs at the same height (potential proof manipulation)
        let mut height_counts: HashMap<u64, usize> = HashMap::new();
        for height in &proof_heights {
            *height_counts.entry(*height).or_insert(0) += 1;
        }
        let multiple_proofs_same_height = height_counts.values().any(|&count| count > 1);

        StateProofIndicators {
            has_state_proof,
            has_consensus_proof,
            proof_verification_failed,
            multiple_proofs_same_height,
            invalid_proof_structure,
            suspicious_relayer,
        }
    }

    /// Calculate confidence score
    fn calculate_confidence(indicators: &StateProofIndicators) -> f64 {
        let mut confidence: f64 = 0.0;

        // Verification failure is a strong indicator
        if indicators.proof_verification_failed {
            confidence += 0.7;
        }

        // Multiple proofs for same height
        if indicators.multiple_proofs_same_height {
            confidence += 0.5;
        }

        // Invalid proof structure
        if indicators.invalid_proof_structure {
            confidence += 0.4;
        }

        // Suspicious relayer
        if indicators.suspicious_relayer {
            confidence += 0.3;
        }

        confidence.min(1.0)
    }

    /// Build evidence
    fn build_evidence(indicators: &StateProofIndicators) -> Vec<String> {
        let mut evidence = Vec::new();

        if indicators.proof_verification_failed {
            evidence.push("State proof verification failed".to_string());
        }

        if indicators.multiple_proofs_same_height {
            evidence.push("Multiple proofs submitted for the same block height".to_string());
        }

        if indicators.invalid_proof_structure {
            evidence.push("Invalid or malformed proof structure detected".to_string());
        }

        if indicators.suspicious_relayer {
            evidence.push("Suspicious relayer behavior detected".to_string());
        }

        evidence
    }
}

#[async_trait]
impl Detector for StateProofVerificationDetector {
    fn name(&self) -> &str {
        "State Proof Verification Detector"
    }

    async fn analyze_transaction(&self, ctx: &TransactionContext) -> DetectionResult {
        if !self.enabled {
            return DetectionResult::safe();
        }

        let indicators = Self::analyze_events(ctx);

        // Only analyze if we have proof-related activity
        if !indicators.has_state_proof && !indicators.has_consensus_proof {
            return DetectionResult::safe();
        }

        let confidence = Self::calculate_confidence(&indicators);
        let evidence = Self::build_evidence(&indicators);
        let detected = confidence > 0.5;

        DetectionResult {
            detected,
            confidence,
            pattern: AttackPattern::StateProofManipulation,
            description: if detected {
                "Suspicious state proof activity detected. Potential proof manipulation or invalid consensus proof.".to_string()
            } else {
                "State proof verification appears normal.".to_string()
            },
            evidence,
            metadata: std::collections::HashMap::new(),
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
    async fn test_cross_chain_detector_safe() {
        let detector = CrossChainBridgeDetector::new();
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
            },
            events: vec![],
            state_changes: vec![],
        };

        let result = detector.analyze_transaction(&ctx).await;
        assert!(!result.detected);
    }

    #[tokio::test]
    async fn test_state_proof_detector() {
        let detector = StateProofVerificationDetector::new();
        let ctx = TransactionContext {
            transaction: ParsedTransaction {
                hash: "0x456".to_string(),
                block_number: 101,
                block_hash: "0xdef".to_string(),
                index: 0,
                caller: "Relayer".to_string(),
                pallet: "ISMP".to_string(),
                call: "verify_proof".to_string(),
                args: vec![],
                success: false,
                timestamp: 0,
            },
            events: vec![ChainEvent {
                event_name: "VerificationFailed".to_string(),
                pallet: "ISMP".to_string(),
                event_data: None,
            }],
            state_changes: vec![],
        };

        let result = detector.analyze_transaction(&ctx).await;
        assert!(result.detected);
        assert!(result.confidence >= 0.5);
    }
}
