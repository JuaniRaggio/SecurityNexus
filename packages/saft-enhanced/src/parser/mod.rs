//! Parser module for FRAME pallets
//!
//! This module provides functionality to parse Substrate FRAME pallet code
//! using the `syn` crate and extract relevant structures for security analysis.

use crate::{Error, Result};
use std::path::Path;
use syn::{visit::Visit, File, Item, ItemMod};

pub mod pallet;
pub mod visitors;

pub use pallet::{FramePallet, PalletConfig, PalletStorage, PalletCall, PalletEvent, PalletError};

/// Parse a Rust source file into an AST
pub fn parse_file<P: AsRef<Path>>(path: P) -> Result<File> {
    let path = path.as_ref();
    let source = std::fs::read_to_string(path).map_err(|e| {
        Error::ParseError(format!("Failed to read file {}: {}", path.display(), e))
    })?;

    parse_source(&source)
}

/// Parse Rust source code into an AST
pub fn parse_source(source: &str) -> Result<File> {
    syn::parse_file(source).map_err(|e| Error::ParseError(format!("Syntax error: {}", e)))
}

/// Extract FRAME pallet information from a parsed AST
pub fn extract_pallet(ast: &File) -> Result<FramePallet> {
    let mut extractor = PalletExtractor::default();
    extractor.visit_file(ast);

    if !extractor.has_pallet_macro {
        return Err(Error::InvalidPallet(
            "No #[frame_support::pallet] macro found".to_string(),
        ));
    }

    Ok(FramePallet {
        name: extractor.pallet_name.clone(),
        config: extractor.config.clone(),
        storage_items: extractor.storage_items.clone(),
        calls: extractor.calls.clone(),
        events: extractor.events.clone(),
        errors: extractor.errors.clone(),
    })
}

/// Visitor that extracts FRAME pallet structures
#[derive(Default)]
struct PalletExtractor {
    has_pallet_macro: bool,
    pallet_name: Option<String>,
    config: Option<PalletConfig>,
    storage_items: Vec<PalletStorage>,
    calls: Vec<PalletCall>,
    events: Vec<PalletEvent>,
    errors: Vec<PalletError>,
}

impl<'ast> Visit<'ast> for PalletExtractor {
    fn visit_item_mod(&mut self, node: &'ast ItemMod) {
        // Check for #[frame_support::pallet] attribute
        for attr in &node.attrs {
            if let Ok(meta) = attr.parse_meta() {
                if is_pallet_macro(&meta) {
                    self.has_pallet_macro = true;
                    self.pallet_name = Some(node.ident.to_string());
                }
            }
        }

        syn::visit::visit_item_mod(self, node);
    }

    fn visit_item(&mut self, node: &'ast Item) {
        match node {
            Item::Struct(item_struct) => {
                // Check for storage items
                if has_storage_attribute(&item_struct.attrs) {
                    self.storage_items.push(PalletStorage {
                        name: item_struct.ident.to_string(),
                        ty: quote::quote!(#item_struct).to_string(),
                    });
                }

                // Check for Config trait
                if item_struct.ident == "Config" {
                    self.config = Some(PalletConfig {
                        bounds: extract_trait_bounds(&item_struct.attrs),
                    });
                }
            }
            Item::Enum(item_enum) => {
                // Check for Event enum
                if has_event_attribute(&item_enum.attrs) {
                    for variant in &item_enum.variants {
                        self.events.push(PalletEvent {
                            name: variant.ident.to_string(),
                        });
                    }
                }

                // Check for Error enum
                if has_error_attribute(&item_enum.attrs) {
                    for variant in &item_enum.variants {
                        self.errors.push(PalletError {
                            name: variant.ident.to_string(),
                        });
                    }
                }
            }
            Item::Impl(item_impl) => {
                // Check for Pallet implementation with dispatchable calls
                if has_call_attribute(&item_impl.attrs) {
                    for item in &item_impl.items {
                        if let syn::ImplItem::Method(method) = item {
                            // Check if method is a dispatchable call
                            if has_weight_attribute(&method.attrs) {
                                self.calls.push(PalletCall {
                                    name: method.sig.ident.to_string(),
                                    params: extract_call_params(&method.sig),
                                });
                            }
                        }
                    }
                }
            }
            _ => {}
        }

        syn::visit::visit_item(self, node);
    }
}

/// Check if a meta attribute is the pallet macro
fn is_pallet_macro(meta: &syn::Meta) -> bool {
    if let syn::Meta::Path(path) = meta {
        if let Some(segment) = path.segments.first() {
            if segment.ident == "frame_support" {
                if let Some(segment) = path.segments.iter().nth(1) {
                    return segment.ident == "pallet";
                }
            }
        }
    }
    false
}

/// Check if attributes contain storage macro
fn has_storage_attribute(attrs: &[syn::Attribute]) -> bool {
    attrs.iter().any(|attr| {
        attr.path
            .segments
            .last()
            .map(|seg| seg.ident == "storage")
            .unwrap_or(false)
    })
}

/// Check if attributes contain event macro
fn has_event_attribute(attrs: &[syn::Attribute]) -> bool {
    attrs.iter().any(|attr| {
        attr.path
            .segments
            .last()
            .map(|seg| seg.ident == "event")
            .unwrap_or(false)
    })
}

/// Check if attributes contain error macro
fn has_error_attribute(attrs: &[syn::Attribute]) -> bool {
    attrs.iter().any(|attr| {
        attr.path
            .segments
            .last()
            .map(|seg| seg.ident == "error")
            .unwrap_or(false)
    })
}

/// Check if attributes contain call macro
fn has_call_attribute(attrs: &[syn::Attribute]) -> bool {
    attrs.iter().any(|attr| {
        attr.path
            .segments
            .last()
            .map(|seg| seg.ident == "call" || seg.ident == "pallet")
            .unwrap_or(false)
    })
}

/// Check if attributes contain weight macro
fn has_weight_attribute(attrs: &[syn::Attribute]) -> bool {
    attrs.iter().any(|attr| {
        attr.path
            .segments
            .last()
            .map(|seg| seg.ident == "weight")
            .unwrap_or(false)
    })
}

/// Extract trait bounds from attributes
fn extract_trait_bounds(_attrs: &[syn::Attribute]) -> Vec<String> {
    // TODO: Implement proper trait bound extraction
    vec![]
}

/// Extract call parameters from function signature
fn extract_call_params(sig: &syn::Signature) -> Vec<String> {
    sig.inputs
        .iter()
        .filter_map(|arg| {
            if let syn::FnArg::Typed(pat_type) = arg {
                Some(quote::quote!(#pat_type).to_string())
            } else {
                None
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_source() {
        let source = r#"
            #[frame_support::pallet]
            pub mod pallet {
                use frame_support::pallet_prelude::*;

                #[pallet::config]
                pub trait Config: frame_system::Config {}

                #[pallet::storage]
                pub type MyStorage<T> = StorageValue<_, u32>;
            }
        "#;

        let result = parse_source(source);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_invalid_source() {
        let source = "invalid rust code {{{";
        let result = parse_source(source);
        assert!(result.is_err());
    }

    #[test]
    fn test_extract_pallet_without_macro() {
        let source = r#"
            pub mod my_module {
                pub fn test() {}
            }
        "#;

        let ast = parse_source(source).unwrap();
        let result = extract_pallet(&ast);
        assert!(result.is_err());
    }
}
