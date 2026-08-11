//! Promote underlying types of public type aliases into reachability.
//!
//! `pub type Public = Hidden` makes inherent methods on `Hidden` ordinary public
//! ceremony API even when `Hidden` itself was never re-exported by name.

use super::crate_modules::ModuleGraph;
use super::public_reachability::{Reachability, ReachableItemKey};
use super::use_binding_resolution::{expand_use_tree, resolve_module_path};
use std::collections::BTreeSet;
use syn::Item;

/// Fixed-point: each reachable type alias pulls its local RHS type into reachability.
pub(super) fn promote_type_alias_underlying_types(
    graph: &ModuleGraph,
    reachability: &mut Reachability,
) {
    let mut changed = true;
    while changed {
        changed = false;
        let keys: Vec<ReachableItemKey> = reachability.items.iter().cloned().collect();
        for key in keys {
            let Some(node) = graph.modules.get(&key.module_path) else {
                continue;
            };
            for item in &node.items {
                let Item::Type(item_type) = item else {
                    continue;
                };
                if item_type.ident != key.item_name {
                    continue;
                }
                if type_starts_with_generic_parameter(&item_type.ty, &item_type.generics) {
                    continue;
                }
                if let Some(underlying) =
                    resolve_local_type_key(graph, &key.module_path, &item_type.ty)
                {
                    if reachability.items.insert(underlying) {
                        changed = true;
                    }
                }
            }
        }
    }
}

fn type_starts_with_generic_parameter(ty: &syn::Type, generics: &syn::Generics) -> bool {
    let syn::Type::Path(path) = ty else {
        return false;
    };
    let Some(first) = path.path.segments.first() else {
        return false;
    };
    path.path.leading_colon.is_none()
        && generics
            .type_params()
            .any(|parameter| parameter.ident == first.ident)
}

/// Resolve a type path to a local type definition key when the path is crate-local.
fn resolve_local_type_key(
    graph: &ModuleGraph,
    from_module: &[String],
    ty: &syn::Type,
) -> Option<ReachableItemKey> {
    let syn::Type::Path(type_path) = ty else {
        return None;
    };
    if type_path.qself.is_some() {
        return None;
    }
    let segments: Vec<String> = type_path
        .path
        .segments
        .iter()
        .map(|segment| segment.ident.to_string())
        .collect();
    if segments.is_empty() {
        return None;
    }
    let (unresolved_module, type_name) = resolve_path_to_module_and_name(from_module, &segments)?;
    let module_path = resolve_module_path(graph, &unresolved_module)?;
    // Prefer a type definition in the resolved module; if the name is itself a
    // type alias, the fixed-point pass will chase the next hop.
    find_named_type_item(graph, &module_path, &type_name).or_else(|| {
        (segments.len() == 1)
            .then(|| resolve_imported_type(graph, from_module, &type_name, &mut BTreeSet::new()))
            .flatten()
    })
}

fn resolve_imported_type(
    graph: &ModuleGraph,
    module_path: &[String],
    local_name: &str,
    visited: &mut BTreeSet<(Vec<String>, String)>,
) -> Option<ReachableItemKey> {
    if !visited.insert((module_path.to_vec(), local_name.to_owned())) {
        return None;
    }
    if let Some(key) = find_named_type_item(graph, module_path, local_name) {
        return Some(key);
    }
    let node = graph.modules.get(module_path)?;
    for item in &node.items {
        let Item::Use(item_use) = item else {
            continue;
        };
        for (target_module, target_name, import_name) in
            expand_use_tree(module_path, &item_use.tree)
        {
            let sought = if import_name == local_name && target_name != "*" {
                target_name.as_str()
            } else {
                continue;
            };
            if let Some(key) = resolve_imported_type(graph, &target_module, sought, visited) {
                return Some(key);
            }
        }
    }
    None
}

fn resolve_path_to_module_and_name(
    from_module: &[String],
    segments: &[String],
) -> Option<(Vec<String>, String)> {
    if segments.is_empty() {
        return None;
    }
    let mut idx = 0;
    let mut module_path = from_module.to_vec();
    match segments[0].as_str() {
        "crate" => {
            module_path.clear();
            idx = 1;
        }
        "self" => {
            idx = 1;
        }
        "super" => {
            module_path.pop()?;
            idx = 1;
            while idx < segments.len() && segments[idx] == "super" {
                module_path.pop()?;
                idx += 1;
            }
        }
        _ => {}
    }
    if idx >= segments.len() {
        return None;
    }
    // Intermediate segments are module path; final segment is the type name.
    while idx + 1 < segments.len() {
        module_path.push(segments[idx].clone());
        idx += 1;
    }
    Some((module_path, segments[idx].clone()))
}

fn find_named_type_item(
    graph: &ModuleGraph,
    module_path: &[String],
    name: &str,
) -> Option<ReachableItemKey> {
    let node = graph.modules.get(module_path)?;
    for item in &node.items {
        match item {
            Item::Struct(i) if i.ident == name => {}
            Item::Enum(i) if i.ident == name => {}
            Item::Union(i) if i.ident == name => {}
            Item::Trait(i) if i.ident == name => {}
            Item::Type(i) if i.ident == name => {}
            _ => continue,
        }
        return Some(ReachableItemKey {
            module_path: module_path.to_vec(),
            item_name: name.to_owned(),
        });
    }
    None
}
