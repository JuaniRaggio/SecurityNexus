//! Vulnerability analyzers for FRAME pallets

pub mod overflow;
pub mod access_control;
pub mod reentrancy;

use crate::{Result, Vulnerability};
use std::path::Path;
use syn::File;

/// Trait for vulnerability analyzers
pub trait VulnerabilityAnalyzer {
    /// Analyze an AST and return detected vulnerabilities
    fn analyze(&self, ast: &File, file_path: &Path) -> Result<Vec<Vulnerability>>;

    /// Get the name of this analyzer
    fn name(&self) -> &str;
}
