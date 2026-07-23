use std::path::Path;

use syn::visit::{self, Visit};
use syn::{ExprMethodCall, ExprPath, File, ImplItem, Item};

use super::WorkspaceSourceInventory;

pub(super) fn collect_method_names(
    inventory: &WorkspaceSourceInventory,
    path: &Path,
) -> Vec<String> {
    let parsed = parse_rust_file(inventory, path);
    let mut collector = MethodCallCollector::default();
    collector.visit_file(&parsed);
    collector.method_names
}

pub(super) fn collect_method_names_for_function(
    inventory: &WorkspaceSourceInventory,
    path: &Path,
    function_name: &str,
) -> Vec<String> {
    let parsed = parse_rust_file(inventory, path);
    let mut collector = MethodCallCollector::default();

    for item in parsed.items {
        match item {
            Item::Fn(item_fn) if item_fn.sig.ident == function_name => {
                collector.visit_block(&item_fn.block);
            }
            Item::Impl(item_impl) => {
                for impl_item in item_impl.items {
                    if let ImplItem::Fn(function) = impl_item {
                        if function.sig.ident == function_name {
                            collector.visit_block(&function.block);
                        }
                    }
                }
            }
            _ => {}
        }
    }

    collector.method_names
}

pub(super) fn collect_paths_for_function(
    inventory: &WorkspaceSourceInventory,
    path: &Path,
    function_name: &str,
) -> Vec<Vec<String>> {
    let parsed = parse_rust_file(inventory, path);
    let mut collector = PathCollector::default();

    for item in parsed.items {
        match item {
            Item::Fn(item_fn) if item_fn.sig.ident == function_name => {
                collector.visit_block(&item_fn.block);
            }
            Item::Impl(item_impl) => {
                for impl_item in item_impl.items {
                    if let ImplItem::Fn(function) = impl_item {
                        if function.sig.ident == function_name {
                            collector.visit_block(&function.block);
                        }
                    }
                }
            }
            _ => {}
        }
    }

    collector.paths
}

pub(super) fn ends_with_path(segments: &[String], suffix: &[&str]) -> bool {
    segments.len() >= suffix.len()
        && segments[segments.len() - suffix.len()..]
            .iter()
            .map(String::as_str)
            .eq(suffix.iter().copied())
}

fn parse_rust_file(inventory: &WorkspaceSourceInventory, path: &Path) -> File {
    let text = inventory.text(path);
    syn::parse_file(text).unwrap_or_else(|error| {
        panic!("{} should parse as Rust source: {error}", path.display());
    })
}

#[derive(Default)]
struct MethodCallCollector {
    method_names: Vec<String>,
}

impl Visit<'_> for MethodCallCollector {
    fn visit_expr_method_call(&mut self, method_call: &ExprMethodCall) {
        self.method_names.push(method_call.method.to_string());
        visit::visit_expr_method_call(self, method_call);
    }
}

#[derive(Default)]
struct PathCollector {
    paths: Vec<Vec<String>>,
}

impl Visit<'_> for PathCollector {
    fn visit_expr_path(&mut self, expr_path: &ExprPath) {
        self.paths.push(
            expr_path
                .path
                .segments
                .iter()
                .map(|segment| segment.ident.to_string())
                .collect(),
        );
        visit::visit_expr_path(self, expr_path);
    }
}
