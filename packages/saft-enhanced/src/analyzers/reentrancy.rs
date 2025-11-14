//! Reentrancy vulnerability detector

use crate::{Result, Vulnerability};
use std::path::Path;
use syn::File;

/// Analyze for reentrancy vulnerabilities
pub fn analyze(_ast: &File, _file_path: &Path) -> Result<Vec<Vulnerability>> {
    // TODO: Implement reentrancy analysis
    // - Check for external calls before state changes
    // - Detect storage access patterns that could lead to reentrancy
    // - Verify checks-effects-interactions pattern

    Ok(Vec::new())
}
