//! Privacy Layer
//!
//! Zero-knowledge proof system for private vulnerability reporting.
//! Allows security researchers to prove they found a vulnerability without
//! revealing the exploit details publicly.

pub mod circuits;
pub mod credentials;
pub mod proofs;
pub mod types;

use ark_bn254::Bn254;
use ark_groth16::{Groth16, Proof, ProvingKey, VerifyingKey};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use ark_snark::SNARK;
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub use types::{VulnerabilityProof, VulnerabilityReport, ReportCommitment};

/// Type alias for the pairing-friendly elliptic curve
pub type PairingCurve = Bn254;

/// Main error type for the privacy layer
#[derive(Error, Debug)]
pub enum Error {
    #[error("Proof generation failed: {0}")]
    ProofGenerationError(String),

    #[error("Proof verification failed: {0}")]
    ProofVerificationError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Invalid witness: {0}")]
    InvalidWitness(String),

    #[error("Circuit error: {0}")]
    CircuitError(String),

    #[error("Cryptographic error: {0}")]
    CryptoError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

/// Result type alias
pub type Result<T> = std::result::Result<T, Error>;

/// Zero-knowledge proof system for vulnerability reporting
pub struct PrivacyLayer {
    proving_key: Option<ProvingKey<PairingCurve>>,
    verifying_key: Option<VerifyingKey<PairingCurve>>,
}

impl PrivacyLayer {
    /// Create a new privacy layer instance
    pub fn new() -> Self {
        Self {
            proving_key: None,
            verifying_key: None,
        }
    }

    /// Setup the proving and verifying keys using Groth16
    pub fn setup(&mut self) -> Result<()> {
        use ark_std::rand::SeedableRng;
        use crate::circuits::VulnerabilityCircuit;

        tracing::info!("Setting up privacy layer with Groth16...");

        // Create an empty circuit for setup
        let circuit = VulnerabilityCircuit::empty();

        // Generate random parameters (in production, use MPC ceremony)
        let mut rng = ark_std::rand::rngs::StdRng::seed_from_u64(0u64);

        // Generate proving and verifying keys
        let (pk, vk) = Groth16::<PairingCurve>::circuit_specific_setup(circuit, &mut rng)
            .map_err(|e| Error::CircuitError(format!("Setup failed: {}", e)))?;

        self.proving_key = Some(pk);
        self.verifying_key = Some(vk);

        tracing::info!("Privacy layer setup complete");
        Ok(())
    }

    /// Generate a zero-knowledge proof for a vulnerability report
    pub fn generate_proof(
        &self,
        report: &VulnerabilityReport,
    ) -> Result<VulnerabilityProof> {
        use ark_bn254::Fr;
        use ark_ff::PrimeField;
        use ark_std::rand::SeedableRng;
        use ark_std::UniformRand;
        use blake2::{Blake2b512, Digest};
        use crate::circuits::VulnerabilityCircuit;

        tracing::debug!("Generating ZK proof for vulnerability report");

        let proving_key = self.proving_key.as_ref().ok_or_else(|| {
            Error::ProofGenerationError(
                "Proving key not initialized. Call setup() first.".to_string(),
            )
        })?;

        // Convert severity to field element (0=Low, 1=Medium, 2=High, 3=Critical)
        let severity_value = match report.severity {
            types::Severity::Low => 0u64,
            types::Severity::Medium => 1u64,
            types::Severity::High => 2u64,
            types::Severity::Critical => 3u64,
        };
        let severity_fr = Fr::from(severity_value);

        // Hash the description to get description_hash
        let mut hasher = Blake2b512::new();
        hasher.update(report.description.as_bytes());
        let desc_hash_bytes = hasher.finalize();
        let description_hash_fr = Fr::from_le_bytes_mod_order(&desc_hash_bytes[..32]);

        // Generate random blinding factor
        let mut rng = ark_std::rand::rngs::StdRng::seed_from_u64(chrono::Utc::now().timestamp() as u64);
        let blinding_factor_fr = Fr::rand(&mut rng);

        // Compute commitment: severity + description_hash * 2 + blinding_factor * 3
        let commitment_fr = severity_fr
            + description_hash_fr * Fr::from(2u64)
            + blinding_factor_fr * Fr::from(3u64);

        // Create circuit with witness
        let circuit = VulnerabilityCircuit::new(
            severity_fr,
            description_hash_fr,
            blinding_factor_fr,
            commitment_fr,
        );

        // Generate proof
        let proof = Groth16::<PairingCurve>::prove(proving_key, circuit, &mut rng)
            .map_err(|e| Error::ProofGenerationError(format!("Proof generation failed: {}", e)))?;

        // Serialize proof
        let mut proof_bytes = Vec::new();
        proof.serialize_compressed(&mut proof_bytes)
            .map_err(|e| Error::SerializationError(format!("Proof serialization failed: {}", e)))?;

        // Serialize public inputs (just the commitment)
        let mut commitment_bytes = Vec::new();
        commitment_fr.serialize_compressed(&mut commitment_bytes)
            .map_err(|e| Error::SerializationError(format!("Commitment serialization failed: {}", e)))?;

        Ok(VulnerabilityProof {
            commitment: ReportCommitment {
                hash: hex::encode(&commitment_bytes),
                blinding_factor: {
                    let mut bf_bytes = Vec::new();
                    blinding_factor_fr.serialize_compressed(&mut bf_bytes).ok();
                    bf_bytes
                },
            },
            proof_data: proof_bytes,
            public_inputs: vec![hex::encode(&commitment_bytes)],
            metadata: ProofMetadata {
                created_at: chrono::Utc::now().timestamp() as u64,
                circuit_version: "v1".to_string(),
                curve: "BN254".to_string(),
            },
        })
    }

    /// Verify a zero-knowledge proof
    pub fn verify_proof(&self, proof: &VulnerabilityProof) -> Result<bool> {
        use ark_bn254::Fr;

        tracing::debug!("Verifying ZK proof");

        let verifying_key = self.verifying_key.as_ref().ok_or_else(|| {
            Error::ProofVerificationError(
                "Verifying key not initialized. Call setup() first.".to_string(),
            )
        })?;

        // Deserialize the proof
        let groth_proof = Proof::<PairingCurve>::deserialize_compressed(&proof.proof_data[..])
            .map_err(|e| Error::ProofVerificationError(format!("Proof deserialization failed: {}", e)))?;

        // Deserialize public inputs (commitment)
        if proof.public_inputs.is_empty() {
            return Err(Error::ProofVerificationError(
                "No public inputs provided".to_string(),
            ));
        }

        let commitment_bytes = hex::decode(&proof.public_inputs[0])
            .map_err(|e| Error::ProofVerificationError(format!("Public input decode failed: {}", e)))?;

        let commitment_fr = Fr::deserialize_compressed(&commitment_bytes[..])
            .map_err(|e| Error::ProofVerificationError(format!("Commitment deserialization failed: {}", e)))?;

        // Prepare public inputs for verification
        let public_inputs = vec![commitment_fr];

        // Verify the proof
        let is_valid = Groth16::<PairingCurve>::verify(verifying_key, &public_inputs, &groth_proof)
            .map_err(|e| Error::ProofVerificationError(format!("Verification failed: {}", e)))?;

        tracing::debug!("Proof verification result: {}", is_valid);
        Ok(is_valid)
    }

    /// Create a commitment to vulnerability details
    fn create_commitment(&self, report: &VulnerabilityReport) -> Result<ReportCommitment> {
        use blake2::{Blake2b512, Digest};

        let mut hasher = Blake2b512::new();
        hasher.update(report.severity.as_bytes());
        hasher.update(report.category.as_bytes());
        hasher.update(report.description.as_bytes());

        let hash = hasher.finalize();

        Ok(ReportCommitment {
            hash: hex::encode(hash),
            blinding_factor: vec![], // Placeholder for actual blinding factor
        })
    }

    /// Load proving key from file
    pub fn load_proving_key(&mut self, path: &str) -> Result<()> {
        tracing::info!("Loading proving key from {}", path);
        // TODO: Implement key loading
        Ok(())
    }

    /// Load verifying key from file
    pub fn load_verifying_key(&mut self, path: &str) -> Result<()> {
        tracing::info!("Loading verifying key from {}", path);
        // TODO: Implement key loading
        Ok(())
    }

    /// Save proving key to file
    pub fn save_proving_key(&self, path: &str) -> Result<()> {
        tracing::info!("Saving proving key to {}", path);
        // TODO: Implement key saving
        Ok(())
    }

    /// Save verifying key to file
    pub fn save_verifying_key(&self, path: &str) -> Result<()> {
        tracing::info!("Saving verifying key to {}", path);
        // TODO: Implement key saving
        Ok(())
    }
}

impl Default for PrivacyLayer {
    fn default() -> Self {
        Self::new()
    }
}

/// Metadata about a generated proof
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofMetadata {
    pub created_at: u64,
    pub circuit_version: String,
    pub curve: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Severity;

    #[test]
    fn test_privacy_layer_creation() {
        let layer = PrivacyLayer::new();
        assert!(layer.proving_key.is_none());
        assert!(layer.verifying_key.is_none());
    }

    #[test]
    fn test_commitment_creation() {
        let layer = PrivacyLayer::new();

        let report = VulnerabilityReport {
            severity: Severity::High,
            category: "integer_overflow".to_string(),
            description: "Test vulnerability".to_string(),
            affected_code: "fn example() {}".to_string(),
            remediation: Some("Use checked arithmetic".to_string()),
            reporter_id: None,
        };

        let commitment = layer.create_commitment(&report);
        assert!(commitment.is_ok());
    }

    #[test]
    fn test_proof_generation_without_setup() {
        let layer = PrivacyLayer::new();

        let report = VulnerabilityReport {
            severity: Severity::High,
            category: "integer_overflow".to_string(),
            description: "Test vulnerability".to_string(),
            affected_code: "fn example() {}".to_string(),
            remediation: Some("Use checked arithmetic".to_string()),
            reporter_id: None,
        };

        let result = layer.generate_proof(&report);
        assert!(result.is_err());
    }

    #[test]
    fn test_end_to_end_proof_system() {
        // Initialize privacy layer
        let mut layer = PrivacyLayer::new();

        // Setup proving and verifying keys
        layer.setup().expect("Setup should succeed");

        // Create a vulnerability report
        let report = VulnerabilityReport {
            severity: Severity::Critical,
            category: "reentrancy".to_string(),
            description: "Re-entrancy vulnerability in withdraw function allows attacker to drain funds".to_string(),
            affected_code: "fn withdraw(amount: u128) { ... }".to_string(),
            remediation: Some("Add non-reentrant guard and CEI pattern".to_string()),
            reporter_id: Some("security_researcher_001".to_string()),
        };

        // Generate proof
        let proof = layer
            .generate_proof(&report)
            .expect("Proof generation should succeed");

        // Verify proof data exists
        assert!(!proof.proof_data.is_empty(), "Proof data should not be empty");
        assert!(!proof.public_inputs.is_empty(), "Public inputs should not be empty");
        assert_eq!(proof.metadata.curve, "BN254");
        assert_eq!(proof.metadata.circuit_version, "v1");

        // Verify the proof
        let is_valid = layer
            .verify_proof(&proof)
            .expect("Proof verification should succeed");

        assert!(is_valid, "Proof should be valid");
    }

    #[test]
    fn test_different_severity_levels() {
        let mut layer = PrivacyLayer::new();
        layer.setup().expect("Setup should succeed");

        // Test all severity levels
        let severities = vec![
            Severity::Low,
            Severity::Medium,
            Severity::High,
            Severity::Critical,
        ];

        for severity in severities {
            let report = VulnerabilityReport {
                severity,
                category: "test".to_string(),
                description: "Test vulnerability".to_string(),
                affected_code: "code".to_string(),
                remediation: None,
                reporter_id: None,
            };

            let proof = layer
                .generate_proof(&report)
                .expect("Proof generation should succeed");

            let is_valid = layer
                .verify_proof(&proof)
                .expect("Verification should succeed");

            assert!(is_valid, "Proof should be valid for severity {:?}", severity);
        }
    }

    #[test]
    fn test_proof_with_different_descriptions() {
        let mut layer = PrivacyLayer::new();
        layer.setup().expect("Setup should succeed");

        let descriptions = vec![
            "Integer overflow in arithmetic operation",
            "Missing access control on admin function",
            "XCM decimal precision mismatch",
        ];

        for desc in descriptions {
            let report = VulnerabilityReport {
                severity: Severity::High,
                category: "test".to_string(),
                description: desc.to_string(),
                affected_code: "code".to_string(),
                remediation: None,
                reporter_id: None,
            };

            let proof = layer
                .generate_proof(&report)
                .expect("Proof generation should succeed");

            let is_valid = layer
                .verify_proof(&proof)
                .expect("Verification should succeed");

            assert!(is_valid, "Proof should be valid");
        }
    }
}
