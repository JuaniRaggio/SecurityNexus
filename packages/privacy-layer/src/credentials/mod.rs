//! Verifiable credentials for security researchers

use crate::types::ResearcherCredential;
use crate::{Error, Result};

/// Credential issuer for researcher identities
pub struct CredentialIssuer {
    issuer_id: String,
}

impl CredentialIssuer {
    pub fn new(issuer_id: String) -> Self {
        Self { issuer_id }
    }

    /// Issue a new credential to a researcher
    pub fn issue_credential(
        &self,
        researcher_id: String,
        reputation: u64,
        vulnerabilities_found: u32,
        validity_period: u64,
    ) -> Result<ResearcherCredential> {
        let now = chrono::Utc::now().timestamp() as u64;

        // TODO: Implement actual credential signing
        // Use a proper signature scheme (e.g., Ed25519 or BLS)

        Ok(ResearcherCredential {
            researcher_id,
            reputation,
            vulnerabilities_found,
            issued_at: now,
            expires_at: now + validity_period,
            signature: vec![], // Placeholder
        })
    }

    /// Verify a researcher credential
    pub fn verify_credential(&self, credential: &ResearcherCredential) -> Result<bool> {
        let now = chrono::Utc::now().timestamp() as u64;

        // Check expiration
        if credential.expires_at < now {
            return Ok(false);
        }

        // TODO: Verify signature

        Ok(true)
    }

    /// Update reputation score
    pub fn update_reputation(
        &self,
        mut credential: ResearcherCredential,
        new_reputation: u64,
    ) -> Result<ResearcherCredential> {
        credential.reputation = new_reputation;

        // TODO: Re-sign credential

        Ok(credential)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_credential_issuance() {
        let issuer = CredentialIssuer::new("test-issuer".to_string());

        let credential = issuer
            .issue_credential(
                "researcher-1".to_string(),
                100,
                5,
                86400, // 1 day
            )
            .unwrap();

        assert_eq!(credential.researcher_id, "researcher-1");
        assert_eq!(credential.reputation, 100);
        assert_eq!(credential.vulnerabilities_found, 5);
    }

    #[test]
    fn test_credential_verification() {
        let issuer = CredentialIssuer::new("test-issuer".to_string());

        let credential = issuer
            .issue_credential("researcher-1".to_string(), 100, 5, 86400)
            .unwrap();

        assert!(issuer.verify_credential(&credential).unwrap());
    }
}
