//! Exact local trait identity for associated projections.

use std::collections::BTreeSet;

use syn::Item;

use super::authority_value_gate_defs::DefinitionKey;
use super::crate_modules::ModuleGraph;
use super::use_binding_resolution::{expand_use_tree, resolve_module_path};

pub(super) fn resolve_trait_key(
    graph: &ModuleGraph,
    current_module: &[String],
    segments: &[String],
) -> Option<DefinitionKey> {
    resolve_trait_key_inner(graph, current_module, segments, &mut BTreeSet::new())
}

fn resolve_trait_key_inner(
    graph: &ModuleGraph,
    current_module: &[String],
    segments: &[String],
    visited: &mut BTreeSet<(Vec<String>, String)>,
) -> Option<DefinitionKey> {
    let (trait_name, module_segments) = segments.split_last()?;
    if module_segments.is_empty() {
        return resolve_trait_name(graph, current_module, trait_name, visited).or_else(|| {
            (!current_module.is_empty())
                .then(|| resolve_trait_name(graph, &[], trait_name, visited))
                .flatten()
        });
    }
    let lexical = lexical_module_path(current_module, module_segments)?;
    let module_path = resolve_module_path(graph, &lexical)?;
    resolve_trait_name(graph, &module_path, trait_name, visited)
}

fn resolve_trait_name(
    graph: &ModuleGraph,
    module_path: &[String],
    name: &str,
    visited: &mut BTreeSet<(Vec<String>, String)>,
) -> Option<DefinitionKey> {
    if !visited.insert((module_path.to_vec(), name.to_owned())) {
        return None;
    }
    let mut candidates = BTreeSet::new();
    if trait_exists(graph, module_path, name) {
        candidates.insert((module_path.to_vec(), name.to_owned()));
    }
    let node = graph.modules.get(module_path)?;
    for item in &node.items {
        let Item::Use(item_use) = item else { continue };
        for (target_module, target_name, bound_name) in expand_use_tree(module_path, &item_use.tree)
        {
            if bound_name != name || target_name == "*" {
                continue;
            }
            if let Some(key) = resolve_trait_name(graph, &target_module, &target_name, visited) {
                candidates.insert(key);
            }
        }
    }
    (candidates.len() == 1)
        .then(|| candidates.into_iter().next())
        .flatten()
}

fn lexical_module_path(current_module: &[String], segments: &[String]) -> Option<Vec<String>> {
    let mut module_path = current_module.to_vec();
    let mut index = 0;
    match segments.first()?.as_str() {
        "crate" => {
            module_path.clear();
            index = 1;
        }
        "self" => index = 1,
        "super" => {
            module_path.pop()?;
            index = 1;
            while segments
                .get(index)
                .is_some_and(|segment| segment == "super")
            {
                module_path.pop()?;
                index += 1;
            }
        }
        _ => {}
    }
    module_path.extend_from_slice(&segments[index..]);
    Some(module_path)
}

fn trait_exists(graph: &ModuleGraph, module_path: &[String], name: &str) -> bool {
    graph.modules.get(module_path).is_some_and(|node| {
        node.items
            .iter()
            .any(|item| matches!(item, Item::Trait(item_trait) if item_trait.ident == name))
    })
}
