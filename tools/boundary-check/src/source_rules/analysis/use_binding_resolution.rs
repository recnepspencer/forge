//! Syntactic `use` expansion and local module-alias binding for BC7001.

use super::crate_modules::ModuleGraph;
use std::collections::BTreeSet;
use syn::{Item, UseTree};

/// Expand a use tree into `(module_path, imported_name, bound_name)` triples.
pub(super) fn expand_use_tree(
    current_module: &[String],
    tree: &UseTree,
) -> Vec<(Vec<String>, String, String)> {
    expand_use_tree_rec(current_module, tree)
}

fn expand_use_tree_rec(
    prefix_module: &[String],
    tree: &UseTree,
) -> Vec<(Vec<String>, String, String)> {
    match tree {
        UseTree::Path(path) => {
            let ident = path.ident.to_string();
            let next = match ident.as_str() {
                "self" => prefix_module.to_vec(),
                "super" => {
                    let mut parent = prefix_module.to_vec();
                    parent.pop();
                    parent
                }
                "crate" => Vec::new(),
                other => {
                    let mut child = prefix_module.to_vec();
                    child.push(other.to_owned());
                    child
                }
            };
            expand_use_tree_rec(&next, &path.tree)
        }
        UseTree::Name(name) => {
            let name = name.ident.to_string();
            vec![(prefix_module.to_vec(), name.clone(), name)]
        }
        UseTree::Rename(rename) => vec![(
            prefix_module.to_vec(),
            rename.ident.to_string(),
            rename.rename.to_string(),
        )],
        UseTree::Glob(_) => {
            vec![(prefix_module.to_vec(), "*".to_owned(), "*".to_owned())]
        }
        UseTree::Group(group) => group
            .items
            .iter()
            .flat_map(|item| expand_use_tree_rec(prefix_module, item))
            .collect(),
    }
}

/// Resolve a physical module path or a path containing lexical module aliases.
pub(super) fn resolve_module_path(graph: &ModuleGraph, path: &[String]) -> Option<Vec<String>> {
    resolve_module_path_inner(graph, path, &mut BTreeSet::new())
}

fn resolve_module_path_inner(
    graph: &ModuleGraph,
    path: &[String],
    visited: &mut BTreeSet<(Vec<String>, String)>,
) -> Option<Vec<String>> {
    let mut resolved = Vec::new();
    for segment in path {
        let mut child = resolved.clone();
        child.push(segment.clone());
        if graph.modules.contains_key(&child) {
            resolved = child;
            continue;
        }
        let key = (resolved.clone(), segment.clone());
        if !visited.insert(key) {
            return None;
        }
        resolved = resolve_named_module_binding(graph, &resolved, segment, visited)?;
    }
    graph.modules.contains_key(&resolved).then_some(resolved)
}

fn resolve_named_module_binding(
    graph: &ModuleGraph,
    module_path: &[String],
    bound_name: &str,
    visited: &mut BTreeSet<(Vec<String>, String)>,
) -> Option<Vec<String>> {
    let node = graph.modules.get(module_path)?;
    let mut candidates = BTreeSet::new();
    for item in &node.items {
        let Item::Use(item_use) = item else {
            continue;
        };
        for (target_module, target_name, import_name) in
            expand_use_tree(module_path, &item_use.tree)
        {
            if import_name != bound_name || target_name == "*" {
                continue;
            }
            let mut candidate = target_module;
            candidate.push(target_name);
            if let Some(candidate) = resolve_module_path_inner(graph, &candidate, visited) {
                candidates.insert(candidate);
            }
        }
    }
    (candidates.len() == 1)
        .then(|| candidates.into_iter().next())
        .flatten()
}
