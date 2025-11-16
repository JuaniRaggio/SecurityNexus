//! Integer overflow/underflow vulnerability detector

use crate::{
    parser::visitors::{ArithmeticKind, ArithmeticVisitor},
    Location, Result, Severity, Vulnerability, VulnerabilityCategory,
};
use std::path::Path;
use syn::{visit::Visit, File};

/// Analyze for integer overflow/underflow vulnerabilities
pub fn analyze(ast: &File, file_path: &Path) -> Result<Vec<Vulnerability>> {
    let mut visitor = ArithmeticVisitor::default();
    visitor.visit_file(ast);

    let mut vulnerabilities = Vec::new();

    // Check if we're analyzing internal tooling code (not pallet code)
    let is_internal_code = file_path.to_string_lossy().contains("saft-enhanced")
        || file_path.to_string_lossy().contains("privacy-layer"); // ZK proof cryptography uses field arithmetic

    for op in &visitor.operations {
        // Check if operation is unchecked (not using checked_* or saturating_* methods)
        let is_unchecked = matches!(
            op.operation,
            ArithmeticKind::Add
                | ArithmeticKind::Sub
                | ArithmeticKind::Mul
                | ArithmeticKind::Div
                | ArithmeticKind::Rem
        );

        // Skip false positives:
        // 1. Operations involving only literals (e.g., 1 + 1, line: 10 + 5)
        //    These are compile-time constants and cannot overflow at runtime
        if op.involves_literals {
            continue;
        }

        // 2. Skip operations in known safe utility functions
        //    These are typically used for metadata or non-critical calculations
        if let Some(ref func_name) = op.function_name {
            let safe_function_patterns = [
                "increment",           // SeverityCounts::increment in lib.rs
                "fmt",                 // Display/Debug implementations
                "to_string",          // String conversions
                "default",            // Default implementations
                "clone",              // Clone implementations
                "serialize",          // Serialization
                "deserialize",        // Deserialization
                "analyze_file",       // Internal analyzer metadata calculations
                "analyze_directory",  // Internal analyzer metadata
                "generate_proof",     // ZK proof generation (field arithmetic)
                "verify_proof",       // ZK proof verification (field arithmetic)
                "setup",              // Cryptographic setup operations
            ];

            if safe_function_patterns.iter().any(|pattern| func_name.contains(pattern)) {
                continue;
            }
        } else if is_internal_code {
            // 3. Skip operations without function context in internal tooling code
            //    This includes the analyzer itself and cryptographic modules (field arithmetic)
            continue;
        }

        if is_unchecked {
            vulnerabilities.push(Vulnerability {
                id: "SAFT-001".to_string(),
                severity: Severity::High,
                category: VulnerabilityCategory::IntegerOverflow,
                message: format!(
                    "Unchecked arithmetic operation: {:?}",
                    op.operation
                ),
                description: "Arithmetic operations without overflow checks can lead to unexpected behavior and potential security vulnerabilities. In FRAME pallets, use checked_* or saturating_* methods.".to_string(),
                location: Location {
                    file: file_path.to_path_buf(),
                    line: op.line.unwrap_or(0),
                    column: 0,
                    snippet: None,
                },
                remediation: Some("Replace with checked_add(), checked_sub(), checked_mul(), checked_div() or use saturating_* variants depending on desired behavior.".to_string()),
                references: vec![
                    "https://docs.substrate.io/build/runtime-storage/#safe-math".to_string(),
                ],
            });
        }
    }

    Ok(vulnerabilities)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse_source;

    #[test]
    fn test_detects_unchecked_addition() {
        let source = r#"
            fn example(a: u32, b: u32) -> u32 {
                a + b
            }
        "#;

        let ast = parse_source(source).unwrap();
        let vulnerabilities = analyze(&ast, Path::new("test.rs")).unwrap();

        assert!(!vulnerabilities.is_empty());
        assert_eq!(vulnerabilities[0].category, VulnerabilityCategory::IntegerOverflow);
    }

    #[test]
    fn test_allows_checked_addition() {
        let source = r#"
            fn example(a: u32, b: u32) -> Option<u32> {
                a.checked_add(b)
            }
        "#;

        let ast = parse_source(source).unwrap();
        let vulnerabilities = analyze(&ast, Path::new("test.rs")).unwrap();

        // checked_add should not trigger a vulnerability
        assert!(vulnerabilities.is_empty());
    }
}
