//! Advanced AST visitors for security analysis

use quote::ToTokens;
use syn::{
    visit::Visit, Expr, ExprBinary, ExprCall, ExprMethodCall, ItemFn, ItemMod,
};

/// Visitor for detecting arithmetic operations
#[derive(Default)]
pub struct ArithmeticVisitor {
    /// Detected arithmetic operations (add, sub, mul, div)
    pub operations: Vec<ArithmeticOp>,
    /// Track if we're currently in a test module
    in_test_module: bool,
    /// Track current function name
    current_function: Option<String>,
}

/// An arithmetic operation found in the code
#[derive(Debug, Clone)]
pub struct ArithmeticOp {
    pub operation: ArithmeticKind,
    pub line: Option<usize>,
    pub in_test_module: bool,
    pub function_name: Option<String>,
    pub involves_literals: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ArithmeticKind {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    CheckedAdd,
    CheckedSub,
    CheckedMul,
    CheckedDiv,
    SaturatingAdd,
    SaturatingSub,
    SaturatingMul,
}

impl<'ast> Visit<'ast> for ArithmeticVisitor {
    fn visit_item_mod(&mut self, node: &'ast ItemMod) {
        // Check if this is a test module
        let is_test = node.attrs.iter().any(|attr| {
            attr.path().is_ident("cfg") &&
            attr.meta.to_token_stream().to_string().contains("test")
        });

        let was_in_test = self.in_test_module;
        if is_test {
            self.in_test_module = true;
        }

        syn::visit::visit_item_mod(self, node);

        self.in_test_module = was_in_test;
    }

    fn visit_item_fn(&mut self, node: &'ast ItemFn) {
        // Check if this is a test function
        let is_test = node.attrs.iter().any(|attr| attr.path().is_ident("test"));

        let previous_fn = self.current_function.clone();
        let was_in_test = self.in_test_module;

        self.current_function = Some(node.sig.ident.to_string());
        if is_test {
            self.in_test_module = true;
        }

        syn::visit::visit_item_fn(self, node);

        self.current_function = previous_fn;
        self.in_test_module = was_in_test;
    }

    fn visit_expr_binary(&mut self, node: &'ast ExprBinary) {
        let op_kind = match node.op {
            syn::BinOp::Add(_) => Some(ArithmeticKind::Add),
            syn::BinOp::Sub(_) => Some(ArithmeticKind::Sub),
            syn::BinOp::Mul(_) => Some(ArithmeticKind::Mul),
            syn::BinOp::Div(_) => Some(ArithmeticKind::Div),
            syn::BinOp::Rem(_) => Some(ArithmeticKind::Rem),
            _ => None,
        };

        if let Some(operation) = op_kind {
            // Check if either operand is a literal
            let involves_literals = is_literal_expr(&node.left) || is_literal_expr(&node.right);

            self.operations.push(ArithmeticOp {
                operation,
                line: None, // TODO: Extract line number from span
                in_test_module: self.in_test_module,
                function_name: self.current_function.clone(),
                involves_literals,
            });
        }

        syn::visit::visit_expr_binary(self, node);
    }

    fn visit_expr_method_call(&mut self, node: &'ast ExprMethodCall) {
        let method_name = node.method.to_string();
        let op_kind = match method_name.as_str() {
            "checked_add" => Some(ArithmeticKind::CheckedAdd),
            "checked_sub" => Some(ArithmeticKind::CheckedSub),
            "checked_mul" => Some(ArithmeticKind::CheckedMul),
            "checked_div" => Some(ArithmeticKind::CheckedDiv),
            "saturating_add" => Some(ArithmeticKind::SaturatingAdd),
            "saturating_sub" => Some(ArithmeticKind::SaturatingSub),
            "saturating_mul" => Some(ArithmeticKind::SaturatingMul),
            _ => None,
        };

        if let Some(operation) = op_kind {
            self.operations.push(ArithmeticOp {
                operation,
                line: None,
                in_test_module: self.in_test_module,
                function_name: self.current_function.clone(),
                involves_literals: false,
            });
        }

        syn::visit::visit_expr_method_call(self, node);
    }
}

/// Helper function to check if an expression is a literal
fn is_literal_expr(expr: &Expr) -> bool {
    matches!(expr, Expr::Lit(_))
}

/// Visitor for detecting external calls
#[derive(Default)]
pub struct ExternalCallVisitor {
    /// Detected external calls
    pub calls: Vec<ExternalCall>,
}

#[derive(Debug, Clone)]
pub struct ExternalCall {
    pub function_name: String,
    pub is_checked: bool,
    pub line: Option<usize>,
}

impl<'ast> Visit<'ast> for ExternalCallVisitor {
    fn visit_expr_call(&mut self, node: &'ast ExprCall) {
        // Extract function name if possible
        let function_name = if let Expr::Path(expr_path) = &*node.func {
            expr_path
                .path
                .segments
                .last()
                .map(|seg| seg.ident.to_string())
                .unwrap_or_default()
        } else {
            String::new()
        };

        self.calls.push(ExternalCall {
            function_name,
            is_checked: false, // Will be determined by context analysis
            line: None,
        });

        syn::visit::visit_expr_call(self, node);
    }

    fn visit_expr_method_call(&mut self, node: &'ast ExprMethodCall) {
        let method_name = node.method.to_string();

        self.calls.push(ExternalCall {
            function_name: method_name,
            is_checked: false,
            line: None,
        });

        syn::visit::visit_expr_method_call(self, node);
    }
}

/// Visitor for detecting storage access patterns
#[derive(Default)]
pub struct StorageAccessVisitor {
    /// Detected storage accesses
    pub accesses: Vec<StorageAccess>,
}

#[derive(Debug, Clone)]
pub struct StorageAccess {
    pub storage_name: String,
    pub access_type: StorageAccessType,
    pub line: Option<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StorageAccessType {
    Read,
    Write,
    Mutate,
    Take,
    Kill,
}

impl<'ast> Visit<'ast> for StorageAccessVisitor {
    fn visit_expr_method_call(&mut self, node: &'ast ExprMethodCall) {
        let method_name = node.method.to_string();
        let access_type = match method_name.as_str() {
            "get" | "try_get" => Some(StorageAccessType::Read),
            "put" | "set" => Some(StorageAccessType::Write),
            "mutate" | "try_mutate" | "mutate_exists" => Some(StorageAccessType::Mutate),
            "take" => Some(StorageAccessType::Take),
            "kill" => Some(StorageAccessType::Kill),
            _ => None,
        };

        if let Some(access_type) = access_type {
            // Try to extract storage name from receiver
            let storage_name = if let Expr::Path(expr_path) = &*node.receiver {
                expr_path
                    .path
                    .segments
                    .last()
                    .map(|seg| seg.ident.to_string())
                    .unwrap_or_default()
            } else {
                String::new()
            };

            self.accesses.push(StorageAccess {
                storage_name,
                access_type,
                line: None,
            });
        }

        syn::visit::visit_expr_method_call(self, node);
    }
}

/// Visitor for detecting function definitions
#[derive(Default)]
pub struct FunctionVisitor {
    /// Detected functions
    pub functions: Vec<FunctionInfo>,
}

#[derive(Debug, Clone)]
pub struct FunctionInfo {
    pub name: String,
    pub is_public: bool,
    pub is_async: bool,
    pub param_count: usize,
}

impl<'ast> Visit<'ast> for FunctionVisitor {
    fn visit_item_fn(&mut self, node: &'ast ItemFn) {
        self.functions.push(FunctionInfo {
            name: node.sig.ident.to_string(),
            is_public: matches!(node.vis, syn::Visibility::Public(_)),
            is_async: node.sig.asyncness.is_some(),
            param_count: node.sig.inputs.len(),
        });

        syn::visit::visit_item_fn(self, node);
    }
}

/// Visitor for detecting error handling patterns
#[derive(Default)]
pub struct ErrorHandlingVisitor {
    /// Detected unwrap/expect calls
    pub unwraps: Vec<UnwrapCall>,
    /// Detected error returns
    pub error_returns: Vec<ErrorReturn>,
}

#[derive(Debug, Clone)]
pub struct UnwrapCall {
    pub method: String,
    pub line: Option<usize>,
}

#[derive(Debug, Clone)]
pub struct ErrorReturn {
    pub error_type: String,
    pub line: Option<usize>,
}

impl<'ast> Visit<'ast> for ErrorHandlingVisitor {
    fn visit_expr_method_call(&mut self, node: &'ast ExprMethodCall) {
        let method_name = node.method.to_string();
        if method_name == "unwrap" || method_name == "expect" {
            self.unwraps.push(UnwrapCall {
                method: method_name,
                line: None,
            });
        }

        syn::visit::visit_expr_method_call(self, node);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    #[test]
    fn test_arithmetic_visitor() {
        let expr: Expr = parse_quote! {
            a + b
        };

        let mut visitor = ArithmeticVisitor::default();
        visitor.visit_expr(&expr);

        assert_eq!(visitor.operations.len(), 1);
        assert_eq!(visitor.operations[0].operation, ArithmeticKind::Add);
    }

    #[test]
    fn test_checked_arithmetic_visitor() {
        let expr: Expr = parse_quote! {
            a.checked_add(b)
        };

        let mut visitor = ArithmeticVisitor::default();
        visitor.visit_expr(&expr);

        assert_eq!(visitor.operations.len(), 1);
        assert_eq!(visitor.operations[0].operation, ArithmeticKind::CheckedAdd);
    }

    #[test]
    fn test_storage_access_visitor() {
        let expr: Expr = parse_quote! {
            MyStorage::<T>::get()
        };

        let mut visitor = StorageAccessVisitor::default();
        visitor.visit_expr(&expr);

        assert_eq!(visitor.accesses.len(), 1);
        assert_eq!(visitor.accesses[0].access_type, StorageAccessType::Read);
    }

    #[test]
    fn test_unwrap_detection() {
        let expr: Expr = parse_quote! {
            value.unwrap()
        };

        let mut visitor = ErrorHandlingVisitor::default();
        visitor.visit_expr(&expr);

        assert_eq!(visitor.unwraps.len(), 1);
        assert_eq!(visitor.unwraps[0].method, "unwrap");
    }
}
