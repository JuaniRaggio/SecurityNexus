//! Core types for the privacy layer

use serde::{Deserialize, Serialize};

/// Severity level of a vulnerability
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
}

impl Severity {
    pub fn as_bytes(&self) -> &[u8] {
        match self {
            Severity::Low => b"low",
            Severity::Medium => b"medium",
            Severity::High => b"high",
            Severity::Critical => b"critical",
        }
    }
}

/// A vulnerability report (private data)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VulnerabilityReport {
    /// Severity of the vulnerability
    pub severity: Severity,
    /// Category (e.g., "integer_overflow", "reentrancy")
    pub category: String,
    /// Description of the vulnerability
    pub description: String,
    /// Affected code snippet or reference
    pub affected_code: String,
    /// Suggested remediation
    pub remediation: Option<String>,
    /// Reporter identifier (optional, for rewards)
    pub reporter_id: Option<String>,
}

/// Zero-knowledge proof of a vulnerability
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VulnerabilityProof {
    /// Commitment to the vulnerability details
    pub commitment: ReportCommitment,
    /// The actual ZK proof data
    pub proof_data: Vec<u8>,
    /// Public inputs to the circuit
    pub public_inputs: Vec<String>,
    /// Metadata about the proof
    pub metadata: crate::ProofMetadata,
}

/// Commitment to a vulnerability report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportCommitment {
    /// Hash of the committed data
    pub hash: String,
    /// Blinding factor for hiding
    pub blinding_factor: Vec<u8>,
}

/// Verifiable credential for a security researcher
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearcherCredential {
    /// Researcher's public identifier
    pub researcher_id: String,
    /// Reputation score
    pub reputation: u64,
    /// Number of verified vulnerabilities found
    pub vulnerabilities_found: u32,
    /// Credential issuance timestamp
    pub issued_at: u64,
    /// Credential expiration timestamp
    pub expires_at: u64,
    /// Digital signature from issuer
    pub signature: Vec<u8>,
}

/// Bug bounty claim with ZK proof
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BountyClaim {
    /// The vulnerability proof
    pub proof: VulnerabilityProof,
    /// Researcher credential
    pub credential: ResearcherCredential,
    /// Requested bounty amount
    pub bounty_amount: u64,
    /// Claim timestamp
    pub claimed_at: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_severity_bytes() {
        assert_eq!(Severity::Low.as_bytes(), b"low");
        assert_eq!(Severity::Critical.as_bytes(), b"critical");
    }

    #[test]
    fn test_vulnerability_report_creation() {
        let report = VulnerabilityReport {
            severity: Severity::High,
            category: "test".to_string(),
            description: "Test vuln".to_string(),
            affected_code: "code".to_string(),
            remediation: None,
            reporter_id: Some("researcher123".to_string()),
        };

        assert_eq!(report.severity, Severity::High);
        assert_eq!(report.category, "test");
    }
}
