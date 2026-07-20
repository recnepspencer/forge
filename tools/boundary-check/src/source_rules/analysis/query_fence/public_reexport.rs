//! Governed public re-exports may not publish Query items.

use super::super::crate_modules::{GovernedCrate, ModuleGraph};
use super::super::public_reachability::Reachability;
use super::vocabulary::QueryVocabulary;
use crate::diagnostics::{Diagnostic, DiagnosticCode};
use proc_macro2::{TokenStream, TokenTree};
use syn::{Item, UseTree, Visibility};

pub(super) fn enforce(
    _governed: &GovernedCrate,
    graph: &ModuleGraph,
    reachable: &Reachability,
    vocabulary: &QueryVocabulary,
) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    for module_path in &reachable.public_modules {
        let Some(node) = graph.modules.get(module_path) else {
            continue;
        };
        for item in &node.items {
            if let Item::Macro(item_macro) = item {
                let exported_definition = item_macro
                    .attrs
                    .iter()
                    .any(|attr| attr.path().is_ident("macro_export"));
                let explicit_invocation = item_macro.ident.is_none();
                if exported_definition || explicit_invocation {
                    let names = token_identifiers(item_macro.mac.tokens.clone());
                    if names.iter().any(|name| name == "pub")
                        && names.iter().any(|name| name == "use")
                    {
                        for query_name in names
                            .into_iter()
                            .filter(|name| vocabulary.is_query_spelling(name))
                        {
                            diagnostics.push(reexport_diagnostic(
                                &node.relative_source,
                                format!("macro token {query_name}"),
                            ));
                        }
                    }
                }
                continue;
            }
            if let Item::ExternCrate(extern_crate) = item {
                if matches!(extern_crate.vis, Visibility::Public(_)) {
                    let root = extern_crate.ident.to_string();
                    if vocabulary.is_query_root(&root) {
                        diagnostics.push(reexport_diagnostic(
                            &node.relative_source,
                            format!("extern crate {root}"),
                        ));
                    }
                }
                continue;
            }
            let Item::Use(item_use) = item else {
                continue;
            };
            if !matches!(item_use.vis, Visibility::Public(_)) {
                continue;
            }
            let mut paths = Vec::new();
            collect_paths(&item_use.tree, Vec::new(), &mut paths);
            for path in paths {
                let Some(root) = path.first() else {
                    continue;
                };
                if vocabulary.is_query_root(root) {
                    diagnostics.push(reexport_diagnostic(&node.relative_source, path.join("::")));
                }
            }
        }
    }
    diagnostics
}

fn reexport_diagnostic(source: &str, spelling: String) -> Diagnostic {
    Diagnostic::new(
        DiagnosticCode::Bc3103QueryPublicReexport,
        source,
        format!(
            "public re-export of Query path `{spelling}` is denied; governed crates consume audience facades but do not re-export Query items; legal home: rule_contracts.query_audience and workspaces/worth-query/crates/worth-query-*/src/facade.rs"
        ),
    )
}

fn collect_paths(tree: &UseTree, prefix: Vec<String>, paths: &mut Vec<Vec<String>>) {
    match tree {
        UseTree::Path(path) => {
            let mut next = prefix;
            next.push(path.ident.to_string());
            collect_paths(&path.tree, next, paths);
        }
        UseTree::Name(name) => {
            let mut path = prefix;
            path.push(name.ident.to_string());
            paths.push(path);
        }
        UseTree::Rename(rename) => {
            let mut path = prefix;
            path.push(rename.ident.to_string());
            paths.push(path);
        }
        UseTree::Group(group) => {
            for item in &group.items {
                collect_paths(item, prefix.clone(), paths);
            }
        }
        UseTree::Glob(_) => {
            let mut path = prefix;
            path.push("*".to_owned());
            paths.push(path);
        }
    }
}

fn token_identifiers(tokens: TokenStream) -> Vec<String> {
    let mut names = Vec::new();
    for token in tokens {
        match token {
            TokenTree::Ident(ident) => names.push(ident.to_string()),
            TokenTree::Group(group) => names.extend(token_identifiers(group.stream())),
            TokenTree::Punct(_) | TokenTree::Literal(_) => {}
        }
    }
    names
}
