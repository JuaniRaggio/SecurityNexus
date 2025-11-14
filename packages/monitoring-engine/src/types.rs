//! Core types for the monitoring engine

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Severity level for alerts
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AlertSeverity {
    Low,
    Medium,
    High,
    Critical,
}

impl std::fmt::Display for AlertSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AlertSeverity::Low => write!(f, "LOW"),
            AlertSeverity::Medium => write!(f, "MEDIUM"),
            AlertSeverity::High => write!(f, "HIGH"),
            AlertSeverity::Critical => write!(f, "CRITICAL"),
        }
    }
}

/// Type of attack pattern detected
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AttackPattern {
    /// Flash loan attack
    FlashLoan,
    /// MEV (Maximal Extractable Value) attack
    Mev,
    /// Front-running attack
    FrontRunning,
    /// Sandwich attack
    Sandwich,
    /// Oracle manipulation
    OracleManipulation,
    /// Governance attack
    GovernanceAttack,
    /// Reentrancy attack
    Reentrancy,
    /// Unusual volume spike
    VolumeAnomaly,
    /// Suspicious approval pattern
    SuspiciousApproval,
    /// Unknown pattern
    Unknown,
}

impl std::fmt::Display for AttackPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AttackPattern::FlashLoan => write!(f, "Flash Loan"),
            AttackPattern::Mev => write!(f, "MEV"),
            AttackPattern::FrontRunning => write!(f, "Front Running"),
            AttackPattern::Sandwich => write!(f, "Sandwich Attack"),
            AttackPattern::OracleManipulation => write!(f, "Oracle Manipulation"),
            AttackPattern::GovernanceAttack => write!(f, "Governance Attack"),
            AttackPattern::Reentrancy => write!(f, "Reentrancy"),
            AttackPattern::VolumeAnomaly => write!(f, "Volume Anomaly"),
            AttackPattern::SuspiciousApproval => write!(f, "Suspicious Approval"),
            AttackPattern::Unknown => write!(f, "Unknown"),
        }
    }
}

/// Security alert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alert {
    /// Unique alert ID
    pub id: String,
    /// Timestamp when alert was triggered
    pub timestamp: u64,
    /// Chain where the alert was detected
    pub chain: String,
    /// Severity level
    pub severity: AlertSeverity,
    /// Attack pattern detected
    pub pattern: AttackPattern,
    /// Human-readable description
    pub description: String,
    /// Related transaction hash (if applicable)
    pub transaction_hash: Option<String>,
    /// Related block number
    pub block_number: Option<u64>,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
    /// Suggested actions
    pub recommended_actions: Vec<String>,
}

/// A blockchain transaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    /// Transaction hash
    pub hash: String,
    /// Block number
    pub block_number: u64,
    /// Sender address
    pub from: String,
    /// Recipient address (if applicable)
    pub to: Option<String>,
    /// Value transferred
    pub value: String,
    /// Transaction data/call
    pub data: Vec<u8>,
    /// Gas used
    pub gas_used: Option<u64>,
    /// Success or failure
    pub success: bool,
    /// Timestamp
    pub timestamp: u64,
}

/// A blockchain event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainEvent {
    /// Block number
    pub block_number: u64,
    /// Event index in the block
    pub event_index: u32,
    /// Pallet name
    pub pallet: String,
    /// Event variant name
    pub event_name: String,
    /// Event data
    pub data: Vec<u8>,
    /// Topics for indexing
    pub topics: Vec<String>,
}

/// Pattern matching result
#[derive(Debug, Clone)]
pub struct DetectionResult {
    /// Whether a pattern was detected
    pub detected: bool,
    /// Confidence level (0.0 to 1.0)
    pub confidence: f64,
    /// Attack pattern type
    pub pattern: AttackPattern,
    /// Description of what was detected
    pub description: String,
    /// Evidence supporting the detection
    pub evidence: Vec<String>,
}

impl DetectionResult {
    /// Create a new detection result indicating no pattern was found
    pub fn no_detection() -> Self {
        Self {
            detected: false,
            confidence: 0.0,
            pattern: AttackPattern::Unknown,
            description: "No suspicious pattern detected".to_string(),
            evidence: Vec::new(),
        }
    }

    /// Create a detection result for a found pattern
    pub fn detected(
        pattern: AttackPattern,
        confidence: f64,
        description: String,
        evidence: Vec<String>,
    ) -> Self {
        Self {
            detected: true,
            confidence,
            pattern,
            description,
            evidence,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_severity_ordering() {
        assert!(AlertSeverity::Critical > AlertSeverity::High);
        assert!(AlertSeverity::High > AlertSeverity::Medium);
        assert!(AlertSeverity::Medium > AlertSeverity::Low);
    }

    #[test]
    fn test_detection_result() {
        let result = DetectionResult::no_detection();
        assert!(!result.detected);
        assert_eq!(result.confidence, 0.0);

        let detected = DetectionResult::detected(
            AttackPattern::FlashLoan,
            0.95,
            "Flash loan detected".to_string(),
            vec!["Large borrow followed by immediate repayment".to_string()],
        );
        assert!(detected.detected);
        assert_eq!(detected.confidence, 0.95);
    }
}
