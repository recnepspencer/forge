//! All-source Query path classification below Cargo metadata.

use super::super::crate_modules::{GovernedCrate, ModuleGraph, ModuleNode};
use super::super::public_reachability::Reachability;
use super::vocabulary::QueryVocabulary;
use crate::diagnostics::{Diagnostic, DiagnosticCode};
use crate::naming::parse_crate_name;
use proc_macro2::{TokenStream, TokenTree};
use syn::visit::{self, Visit};
use syn::{ItemExternCrate, ItemUse, Path, UseTree, Visibility};

pub(super) fn enforce(
    governed: &GovernedCrate,
    graph: &ModuleGraph,
    reachable: &Reachability,
    vocabulary: &QueryVocabulary,
) -> Vec<Diagnostic> {
    let Ok(crate_name) = parse_crate_name(&governed.package) else {
        return Vec::new();
    };
    let mut diagnostics = Vec::new();
    for (module_path, node) in &graph.modules {
        diagnostics.extend(enforce_node(
            &crate_name.band,
            node,
            vocabulary,
            !reachable.public_modules.contains(module_path),
        ));
    }
    diagnostics
}

pub(super) fn enforce_nodes<N>(
    governed: &GovernedCrate,
    nodes: &[N],
    vocabulary: &QueryVocabulary,
    include_public_imports: bool,
) -> Vec<Diagnostic>
where
    N: std::borrow::Borrow<ModuleNode>,
{
    let Ok(crate_name) = parse_crate_name(&governed.package) else {
        return Vec::new();
    };
    let mut diagnostics = Vec::new();
    for node in nodes {
        let node = node.borrow();
        diagnostics.extend(enforce_node(
            &crate_name.band,
            node,
            vocabulary,
            include_public_imports,
        ));
    }
    diagnostics
}

fn enforce_node(
    band: &str,
    node: &ModuleNode,
    vocabulary: &QueryVocabulary,
    include_public_imports: bool,
) -> Vec<Diagnostic> {
    let mut visitor = QueryPathVisitor {
        band,
        source: &node.relative_source,
        vocabulary,
        include_public_imports,
        diagnostics: Vec::new(),
    };
    for item in &node.items {
        visitor.visit_item(item);
    }
    visitor.diagnostics
}

struct QueryPathVisitor<'a> {
    band: &'a str,
    source: &'a str,
    vocabulary: &'a QueryVocabulary,
    include_public_imports: bool,
    diagnostics: Vec<Diagnostic>,
}

impl QueryPathVisitor<'_> {
    fn inspect_root(&mut self, root: &str, spelling: String) {
        if !self.vocabulary.path_is_denied(root, self.band) {
            return;
        }
        self.diagnostics.push(Diagnostic::new(
            DiagnosticCode::Bc3101QuerySourcePath,
            self.source,
            format!(
                "Query-rooted source path `{spelling}` is denied in the `{}` band; consume Query only through an audience facade allowed by rule_contracts.query_audience",
                self.band
            ),
        ));
    }
}

impl<'ast> Visit<'ast> for QueryPathVisitor<'_> {
    fn visit_path(&mut self, path: &'ast Path) {
        if let Some(first) = path.segments.first() {
            let spelling = path
                .segments
                .iter()
                .map(|segment| segment.ident.to_string())
                .collect::<Vec<_>>()
                .join("::");
            self.inspect_root(&first.ident.to_string(), spelling);
        }
        visit::visit_path(self, path);
    }

    fn visit_item_use(&mut self, item: &'ast ItemUse) {
        if self.include_public_imports || !matches!(item.vis, Visibility::Public(_)) {
            let mut paths = Vec::new();
            collect_use_paths(&item.tree, Vec::new(), &mut paths);
            for path in paths {
                if let Some(root) = path.first() {
                    self.inspect_root(root, path.join("::"));
                }
            }
        }
    }

    fn visit_item_extern_crate(&mut self, item: &'ast ItemExternCrate) {
        if self.include_public_imports || !matches!(item.vis, Visibility::Public(_)) {
            let root = item.ident.to_string();
            self.inspect_root(&root, format!("extern crate {root}"));
        }
    }

    fn visit_macro(&mut self, mac: &'ast syn::Macro) {
        for name in token_identifiers(mac.tokens.clone()) {
            self.inspect_root(&name, format!("macro token `{name}`"));
        }
    }
}

fn collect_use_paths(tree: &UseTree, prefix: Vec<String>, paths: &mut Vec<Vec<String>>) {
    match tree {
        UseTree::Path(path) => {
            let mut next = prefix;
            next.push(path.ident.to_string());
            collect_use_paths(&path.tree, next, paths);
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
                collect_use_paths(item, prefix.clone(), paths);
            }
        }
        UseTree::Glob(_) => paths.push(prefix),
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
