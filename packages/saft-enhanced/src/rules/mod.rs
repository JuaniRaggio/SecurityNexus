//! Security rules and patterns for FRAME pallets

use crate::{Severity, VulnerabilityCategory};
use serde::{Deserialize, Serialize};

/// A security rule definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityRule {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: VulnerabilityCategory,
    pub severity: Severity,
    pub enabled: bool,
}

/// Collection of all security rules
pub struct RuleSet {
    rules: Vec<SecurityRule>,
}

impl RuleSet {
    /// Create a new rule set with default rules
    pub fn default() -> Self {
        Self {
            rules: vec![
                SecurityRule {
                    id: "SAFT-001".to_string(),
                    name: "Unchecked Arithmetic".to_string(),
                    description: "Arithmetic operations without overflow checks".to_string(),
                    category: VulnerabilityCategory::IntegerOverflow,
                    severity: Severity::High,
                    enabled: true,
                },
                SecurityRule {
                    id: "SAFT-002".to_string(),
                    name: "Missing Origin Check".to_string(),
                    description: "Dispatchable function without origin verification".to_string(),
                    category: VulnerabilityCategory::AccessControl,
                    severity: Severity::Critical,
                    enabled: true,
                },
                SecurityRule {
                    id: "SAFT-003".to_string(),
                    name: "Reentrancy Risk".to_string(),
                    description: "External call before state update".to_string(),
                    category: VulnerabilityCategory::Reentrancy,
                    severity: Severity::High,
                    enabled: true,
                },
                SecurityRule {
                    id: "SAFT-004".to_string(),
                    name: "Unchecked Error Handling".to_string(),
                    description: "Use of unwrap() or expect() in production code".to_string(),
                    category: VulnerabilityCategory::ErrorHandling,
                    severity: Severity::Medium,
                    enabled: true,
                },
            ],
        }
    }

    /// Get all enabled rules
    pub fn enabled_rules(&self) -> Vec<&SecurityRule> {
        self.rules.iter().filter(|r| r.enabled).collect()
    }

    /// Get rule by ID
    pub fn get_rule(&self, id: &str) -> Option<&SecurityRule> {
        self.rules.iter().find(|r| r.id == id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_ruleset() {
        let ruleset = RuleSet::default();
        assert!(!ruleset.rules.is_empty());
        assert!(ruleset.get_rule("SAFT-001").is_some());
    }

    #[test]
    fn test_enabled_rules() {
        let ruleset = RuleSet::default();
        let enabled = ruleset.enabled_rules();
        assert_eq!(enabled.len(), ruleset.rules.len());
    }
}
