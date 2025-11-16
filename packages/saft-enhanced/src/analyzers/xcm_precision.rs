//! XCM Decimal Precision vulnerability detector
//!
//! Detects potential decimal precision issues in cross-chain transfers via XCM.
//! Different parachains may have different decimal configurations for the same asset,
//! leading to loss of funds or incorrect amounts if not properly converted.

use crate::{
    Location, Result, Severity, Vulnerability, VulnerabilityCategory,
};
use std::path::Path;
use syn::{visit::Visit, Expr, ExprCall, ExprMethodCall, File, Lit};

/// Patterns that indicate XCM-related operations
const XCM_PATTERNS: &[&str] = &[
    "transfer_multiasset",
    "transfer_asset",
    "withdraw_asset",
    "deposit_asset",
    "reserve_asset_deposited",
    "receive_teleported_asset",
    "xcm_transfer",
    "transfer_to_para",
    "transfer_to_relay",
    "do_xcm_transfer",
    "MultiAsset",
    "MultiLocation",
];

/// Patterns that indicate proper decimal conversion
const SAFE_PATTERNS: &[&str] = &[
    "convert_balance",
    "scale_balance",
    "adjust_decimals",
    "from_decimals",
    "to_decimals",
    "Balance::from_decimals",
    "normalize_decimals",
];

/// Visitor for XCM-related function calls
#[derive(Default)]
struct XcmCallVisitor {
    xcm_calls: Vec<XcmCallInfo>,
}

#[derive(Debug, Clone)]
struct XcmCallInfo {
    function_name: String,
    line: Option<usize>,
    has_hardcoded_amount: bool,
    has_decimal_conversion: bool,
    expression: String,
}

impl Visit<'_> for XcmCallVisitor {
    fn visit_expr_call(&mut self, node: &ExprCall) {
        // Check function calls
        if let Expr::Path(path_expr) = &*node.func {
            if let Some(segment) = path_expr.path.segments.last() {
                let function_name = segment.ident.to_string();

                if XCM_PATTERNS.iter().any(|p| function_name.contains(p)) {
                    let has_hardcoded = Self::has_hardcoded_amount(&node.args);
                    let has_conversion = Self::has_decimal_conversion_in_args(&node.args);

                    self.xcm_calls.push(XcmCallInfo {
                        function_name: function_name.clone(),
                        line: None, // Will be set later if available
                        has_hardcoded_amount: has_hardcoded,
                        has_decimal_conversion: has_conversion,
                        expression: quote::quote!(#node).to_string(),
                    });
                }
            }
        }

        syn::visit::visit_expr_call(self, node);
    }

    fn visit_expr_method_call(&mut self, node: &ExprMethodCall) {
        let method_name = node.method.to_string();

        if XCM_PATTERNS.iter().any(|p| method_name.contains(p)) {
            let has_hardcoded = Self::has_hardcoded_amount(&node.args);
            let has_conversion = Self::has_decimal_conversion_in_args(&node.args);

            self.xcm_calls.push(XcmCallInfo {
                function_name: method_name.clone(),
                line: None,
                has_hardcoded_amount: has_hardcoded,
                has_decimal_conversion: has_conversion,
                expression: quote::quote!(#node).to_string(),
            });
        }

        syn::visit::visit_expr_method_call(self, node);
    }
}

impl XcmCallVisitor {
    /// Check if arguments contain hardcoded numeric literals (potential decimal issues)
    fn has_hardcoded_amount(args: &syn::punctuated::Punctuated<Expr, syn::token::Comma>) -> bool {
        for arg in args {
            if Self::contains_numeric_literal(arg) {
                return true;
            }
        }
        false
    }

    /// Check if arguments contain decimal conversion functions
    fn has_decimal_conversion_in_args(args: &syn::punctuated::Punctuated<Expr, syn::token::Comma>) -> bool {
        for arg in args {
            if Self::contains_decimal_conversion(arg) {
                return true;
            }
        }
        false
    }

    /// Recursively check if expression contains numeric literals
    fn contains_numeric_literal(expr: &Expr) -> bool {
        match expr {
            Expr::Lit(expr_lit) => matches!(expr_lit.lit, Lit::Int(_) | Lit::Float(_)),
            Expr::Binary(bin) => {
                Self::contains_numeric_literal(&bin.left) || Self::contains_numeric_literal(&bin.right)
            }
            Expr::Unary(unary) => Self::contains_numeric_literal(&unary.expr),
            Expr::Paren(paren) => Self::contains_numeric_literal(&paren.expr),
            Expr::Cast(cast) => Self::contains_numeric_literal(&cast.expr),
            Expr::Call(call) => call.args.iter().any(Self::contains_numeric_literal),
            Expr::MethodCall(method) => method.args.iter().any(Self::contains_numeric_literal),
            _ => false,
        }
    }

    /// Check if expression contains decimal conversion function calls
    fn contains_decimal_conversion(expr: &Expr) -> bool {
        let expr_str = quote::quote!(#expr).to_string();
        SAFE_PATTERNS.iter().any(|p| expr_str.contains(p))
    }
}

/// Analyze for XCM decimal precision vulnerabilities
pub fn analyze(ast: &File, file_path: &Path) -> Result<Vec<Vulnerability>> {
    let mut visitor = XcmCallVisitor::default();
    visitor.visit_file(ast);

    let mut vulnerabilities = Vec::new();

    for call in &visitor.xcm_calls {
        // Flag if XCM call has hardcoded amounts without decimal conversion
        if call.has_hardcoded_amount && !call.has_decimal_conversion {
            vulnerabilities.push(Vulnerability {
                id: "SAFT-004".to_string(),
                severity: Severity::Critical,
                category: VulnerabilityCategory::XcmDecimalPrecision,
                message: format!(
                    "XCM transfer '{}' uses hardcoded amount without decimal conversion",
                    call.function_name
                ),
                description: "Cross-chain transfers via XCM must account for different decimal configurations across parachains. Hardcoded amounts without proper decimal conversion can lead to incorrect transfer amounts, potentially resulting in loss of funds or failed transactions.".to_string(),
                location: Location {
                    file: file_path.to_path_buf(),
                    line: call.line.unwrap_or(0),
                    column: 0,
                    snippet: Some(call.expression.clone()),
                },
                remediation: Some(
                    "Use decimal conversion functions before XCM transfers. Example:\n\
                    1. Store decimal configurations for each parachain\n\
                    2. Use convert_balance(amount, source_decimals, dest_decimals)\n\
                    3. Validate converted amounts before transfer\n\
                    4. Consider using Balance::from_decimals() or similar utilities".to_string()
                ),
                references: vec![
                    "https://docs.substrate.io/reference/xcm-reference/".to_string(),
                    "https://github.com/paritytech/polkadot/tree/master/xcm".to_string(),
                ],
            });
        }

        // Also flag XCM calls without any decimal handling (even without hardcoded values)
        // This is lower severity as it might be intentional
        if !call.has_decimal_conversion && !call.has_hardcoded_amount {
            vulnerabilities.push(Vulnerability {
                id: "SAFT-004".to_string(),
                severity: Severity::Medium,
                category: VulnerabilityCategory::XcmDecimalPrecision,
                message: format!(
                    "XCM transfer '{}' may not handle decimal precision correctly",
                    call.function_name
                ),
                description: "This XCM transfer does not appear to use decimal conversion functions. Ensure that the amount being transferred is properly scaled for the destination parachain's decimal configuration.".to_string(),
                location: Location {
                    file: file_path.to_path_buf(),
                    line: call.line.unwrap_or(0),
                    column: 0,
                    snippet: Some(call.expression.clone()),
                },
                remediation: Some(
                    "Verify that decimal precision is handled correctly:\n\
                    1. Check source and destination parachain decimal configurations\n\
                    2. If decimals differ, use conversion functions\n\
                    3. Add unit tests with different decimal scenarios\n\
                    4. Document expected decimal behavior".to_string()
                ),
                references: vec![
                    "https://docs.substrate.io/reference/xcm-reference/".to_string(),
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
    fn test_detects_xcm_transfer_with_hardcoded_amount() {
        let source = r#"
            use xcm::latest::prelude::*;

            fn transfer_to_parachain(dest: ParaId) {
                let amount = 1_000_000_000_000u128; // Hardcoded 1 DOT (10 decimals)
                transfer_multiasset(dest, amount);
            }
        "#;

        let ast = parse_source(source).unwrap();
        let vulnerabilities = analyze(&ast, Path::new("test.rs")).unwrap();

        assert!(!vulnerabilities.is_empty());
        assert!(vulnerabilities.iter().any(|v| v.severity == Severity::Critical));
        assert!(vulnerabilities.iter().any(|v|
            v.message.contains("hardcoded amount without decimal conversion")
        ));
    }

    #[test]
    fn test_allows_xcm_transfer_with_decimal_conversion() {
        let source = r#"
            use xcm::latest::prelude::*;

            fn transfer_to_parachain(dest: ParaId, amount: u128) {
                let converted = convert_balance(amount, 10, 12); // DOT to GLMR decimals
                transfer_multiasset(dest, converted);
            }
        "#;

        let ast = parse_source(source).unwrap();
        let vulnerabilities = analyze(&ast, Path::new("test.rs")).unwrap();

        // Should not have critical vulnerabilities
        assert!(vulnerabilities.iter().all(|v| v.severity != Severity::Critical));
    }

    #[test]
    fn test_detects_multiple_xcm_calls() {
        let source = r#"
            fn example() {
                transfer_multiasset(para_id, 1_000_000);
                withdraw_asset(2_000_000);
                deposit_asset(3_000_000);
            }
        "#;

        let ast = parse_source(source).unwrap();
        let vulnerabilities = analyze(&ast, Path::new("test.rs")).unwrap();

        assert!(vulnerabilities.len() >= 3);
    }

    #[test]
    fn test_no_false_positives_on_non_xcm_code() {
        let source = r#"
            fn normal_transfer(dest: AccountId, amount: Balance) {
                T::Currency::transfer(&source, &dest, amount, ExistenceRequirement::KeepAlive)?;
            }
        "#;

        let ast = parse_source(source).unwrap();
        let vulnerabilities = analyze(&ast, Path::new("test.rs")).unwrap();

        assert!(vulnerabilities.is_empty());
    }

    #[test]
    fn test_detects_method_call_syntax() {
        let source = r#"
            fn example(xcm: Xcm) {
                xcm.transfer_asset(1_000_000_000);
            }
        "#;

        let ast = parse_source(source).unwrap();
        let vulnerabilities = analyze(&ast, Path::new("test.rs")).unwrap();

        assert!(!vulnerabilities.is_empty());
    }
}
