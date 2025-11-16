//! ZK circuits for vulnerability proofs

use ark_ff::PrimeField;
use ark_r1cs_std::prelude::*;
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};

/// Circuit for proving knowledge of a vulnerability without revealing its details
///
/// This circuit proves:
/// 1. Knowledge of vulnerability severity and description hash
/// 2. That severity is within valid range (0-3: Low, Medium, High, Critical)
/// 3. That the commitment is correctly formed as: Hash(severity || description_hash || blinding)
pub struct VulnerabilityCircuit<F: PrimeField> {
    /// Private: The vulnerability severity (0=Low, 1=Medium, 2=High, 3=Critical)
    pub severity: Option<F>,
    /// Private: Hash of vulnerability description
    pub description_hash: Option<F>,
    /// Private: Blinding factor for commitment
    pub blinding_factor: Option<F>,
    /// Public: Commitment to the vulnerability (Hash of all private inputs)
    pub commitment: Option<F>,
}

impl<F: PrimeField> VulnerabilityCircuit<F> {
    /// Create a new vulnerability circuit with witness values
    pub fn new(
        severity: F,
        description_hash: F,
        blinding_factor: F,
        commitment: F,
    ) -> Self {
        Self {
            severity: Some(severity),
            description_hash: Some(description_hash),
            blinding_factor: Some(blinding_factor),
            commitment: Some(commitment),
        }
    }

    /// Create an empty circuit (for setup phase)
    pub fn empty() -> Self {
        Self {
            severity: None,
            description_hash: None,
            blinding_factor: None,
            commitment: None,
        }
    }
}

impl<F: PrimeField> ConstraintSynthesizer<F> for VulnerabilityCircuit<F> {
    fn generate_constraints(self, cs: ConstraintSystemRef<F>) -> Result<(), SynthesisError> {
        // Allocate private witness variables
        let severity = FpVar::new_witness(cs.clone(), || {
            self.severity.ok_or(SynthesisError::AssignmentMissing)
        })?;

        let description_hash = FpVar::new_witness(cs.clone(), || {
            self.description_hash
                .ok_or(SynthesisError::AssignmentMissing)
        })?;

        let blinding_factor = FpVar::new_witness(cs.clone(), || {
            self.blinding_factor
                .ok_or(SynthesisError::AssignmentMissing)
        })?;

        // Allocate public input variable
        let commitment_public = FpVar::new_input(cs.clone(), || {
            self.commitment.ok_or(SynthesisError::AssignmentMissing)
        })?;

        // Constraint 1: Severity must be in range [0, 3]
        // We'll enforce this by checking: severity * (severity - 1) * (severity - 2) * (severity - 3) == 0
        let zero = FpVar::zero();
        let one = FpVar::one();
        let two = &one + &one;
        let three = &two + &one;

        let diff_0 = &severity - &zero;
        let diff_1 = &severity - &one;
        let diff_2 = &severity - &two;
        let diff_3 = &severity - &three;

        let product = &diff_0 * &diff_1;
        let product = &product * &diff_2;
        let product = &product * &diff_3;

        product.enforce_equal(&zero)?;

        // Constraint 2: Compute commitment as simple hash-like function
        // commitment = severity + description_hash * 2 + blinding_factor * 3
        // (This is a simplified commitment scheme for demonstration)
        let commitment_computed = &severity
            + &(&description_hash * &two)
            + &(&blinding_factor * &three);

        // Constraint 3: Computed commitment must equal public commitment
        commitment_computed.enforce_equal(&commitment_public)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_bn254::Fr;
    use ark_relations::r1cs::ConstraintSystem;
    use ark_std::UniformRand;

    #[test]
    fn test_circuit_creation() {
        let circuit = VulnerabilityCircuit::<Fr>::empty();
        assert!(circuit.severity.is_none());
        assert!(circuit.description_hash.is_none());
    }

    #[test]
    fn test_circuit_with_valid_witness() {
        let mut rng = ark_std::test_rng();

        // Severity = 2 (High)
        let severity = Fr::from(2u64);
        let description_hash = Fr::rand(&mut rng);
        let blinding_factor = Fr::rand(&mut rng);

        // Compute commitment: severity + description_hash * 2 + blinding_factor * 3
        let commitment = severity
            + description_hash * Fr::from(2u64)
            + blinding_factor * Fr::from(3u64);

        let circuit = VulnerabilityCircuit::new(
            severity,
            description_hash,
            blinding_factor,
            commitment,
        );

        let cs = ConstraintSystem::<Fr>::new_ref();
        circuit.generate_constraints(cs.clone()).unwrap();

        assert!(cs.is_satisfied().unwrap());
    }

    #[test]
    fn test_circuit_with_invalid_severity() {
        let mut rng = ark_std::test_rng();

        // Severity = 5 (invalid, must be 0-3)
        let severity = Fr::from(5u64);
        let description_hash = Fr::rand(&mut rng);
        let blinding_factor = Fr::rand(&mut rng);

        let commitment = severity
            + description_hash * Fr::from(2u64)
            + blinding_factor * Fr::from(3u64);

        let circuit = VulnerabilityCircuit::new(
            severity,
            description_hash,
            blinding_factor,
            commitment,
        );

        let cs = ConstraintSystem::<Fr>::new_ref();
        circuit.generate_constraints(cs.clone()).unwrap();

        // Should not be satisfied because severity is out of range
        assert!(!cs.is_satisfied().unwrap());
    }

    #[test]
    fn test_circuit_with_wrong_commitment() {
        let mut rng = ark_std::test_rng();

        let severity = Fr::from(1u64);
        let description_hash = Fr::rand(&mut rng);
        let blinding_factor = Fr::rand(&mut rng);

        // Wrong commitment (random value instead of correct formula)
        let wrong_commitment = Fr::rand(&mut rng);

        let circuit = VulnerabilityCircuit::new(
            severity,
            description_hash,
            blinding_factor,
            wrong_commitment,
        );

        let cs = ConstraintSystem::<Fr>::new_ref();
        circuit.generate_constraints(cs.clone()).unwrap();

        // Should not be satisfied because commitment is wrong
        assert!(!cs.is_satisfied().unwrap());
    }
}
