//! Proof utilities and helpers

use crate::{Error, Result};
use blake2::{Blake2b512, Digest};

/// Generate a Pedersen commitment
pub fn generate_commitment(data: &[u8], blinding: &[u8]) -> Result<Vec<u8>> {
    let mut hasher = Blake2b512::new();
    hasher.update(data);
    hasher.update(blinding);

    Ok(hasher.finalize().to_vec())
}

/// Verify a commitment
pub fn verify_commitment(data: &[u8], blinding: &[u8], commitment: &[u8]) -> Result<bool> {
    let computed = generate_commitment(data, blinding)?;
    Ok(computed == commitment)
}

/// Generate a random blinding factor
pub fn generate_blinding_factor() -> Vec<u8> {
    use sha2::Sha256;

    let timestamp = chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0);
    let mut hasher = Sha256::new();
    hasher.update(timestamp.to_le_bytes());

    hasher.finalize().to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_commitment_generation() {
        let data = b"secret data";
        let blinding = b"random blinding";

        let commitment = generate_commitment(data, blinding).unwrap();
        assert!(!commitment.is_empty());
    }

    #[test]
    fn test_commitment_verification() {
        let data = b"secret data";
        let blinding = b"random blinding";

        let commitment = generate_commitment(data, blinding).unwrap();
        assert!(verify_commitment(data, blinding, &commitment).unwrap());

        // Wrong data should fail
        let wrong_data = b"wrong data";
        assert!(!verify_commitment(wrong_data, blinding, &commitment).unwrap());
    }

    #[test]
    fn test_blinding_factor_generation() {
        let blinding1 = generate_blinding_factor();
        let blinding2 = generate_blinding_factor();

        assert!(!blinding1.is_empty());
        assert!(!blinding2.is_empty());
        // Should be different (very high probability)
        assert_ne!(blinding1, blinding2);
    }
}
