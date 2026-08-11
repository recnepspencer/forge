//! Promote underlying types of public type aliases into reachability.
//!
//! `pub type Public = Hidden` makes inherent methods on `Hidden` ordinary public
//! ceremony API even when `Hidden` itself was never re-exported by name.

use super::crate_modules::{is_public_visibility, ModuleGraph};
use super::module_path_resolution::{expand_resolved_use_tree, resolve_module_path};
use super::public_reachability::{Reachability, ReachableItemKey};
use quote::ToTokens;
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
pub(super) fn resolve_local_type_key(
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
            .then(|| resolve_imported_type(graph, from_module, &type_name))
            .flatten()
    })
}

/// Resolve an absolute downstream type path back to its exact local definition.
///
/// The compiler remains authoritative for type arguments and visibility. This
/// resolver binds the configured external path (including public re-exports and
/// aliases) to the definition key inventoried by BC7004.
pub(super) fn resolve_public_type_key(
    graph: &ModuleGraph,
    public_type_path: &str,
) -> Option<ReachableItemKey> {
    let syn::Type::Path(path) = syn::parse_str::<syn::Type>(public_type_path).ok()? else {
        return None;
    };
    if path.qself.is_some() || path.path.leading_colon.is_none() {
        return None;
    }
    let mut segments = path.path.segments.iter();
    if segments.next()?.ident != "worth_proof" {
        return None;
    }
    let remaining = segments
        .map(|segment| segment.ident.to_string())
        .collect::<Vec<_>>();
    let (name, module_path) = remaining.split_last()?;
    let mut visited = BTreeSet::new();
    let key = resolve_exported_type(graph, module_path, name, &mut visited)?;
    resolve_type_alias_definition(graph, key, &mut BTreeSet::new())
}

pub(super) fn canonical_public_type_path(public_type_path: &str) -> Option<String> {
    let ty = syn::parse_str::<syn::Type>(public_type_path).ok()?;
    let syn::Type::Path(path) = &ty else {
        return None;
    };
    if path.qself.is_some()
        || path.path.leading_colon.is_none()
        || path.path.segments.first()?.ident != "worth_proof"
    {
        return None;
    }
    Some(ty.into_token_stream().to_string())
}

fn resolve_type_alias_definition(
    graph: &ModuleGraph,
    key: ReachableItemKey,
    visited: &mut BTreeSet<ReachableItemKey>,
) -> Option<ReachableItemKey> {
    if !visited.insert(key.clone()) {
        return None;
    }
    let node = graph.modules.get(&key.module_path)?;
    for item in &node.items {
        let Item::Type(alias) = item else {
            continue;
        };
        if alias.ident == key.item_name {
            let target = resolve_local_type_key(graph, &key.module_path, &alias.ty)?;
            return resolve_type_alias_definition(graph, target, visited);
        }
    }
    Some(key)
}

fn resolve_imported_type(
    graph: &ModuleGraph,
    from_module: &[String],
    local_name: &str,
) -> Option<ReachableItemKey> {
    let node = graph.modules.get(from_module)?;
    let mut visited = BTreeSet::new();
    for item in &node.items {
        let Item::Use(item_use) = item else {
            continue;
        };
        for (target_module, target_name, import_name) in
            expand_resolved_use_tree(graph, from_module, &item_use.tree)
        {
            if import_name == local_name && target_name != "*" {
                return resolve_exported_type(graph, &target_module, &target_name, &mut visited);
            }
            if import_name == "*" && target_name == "*" {
                if let Some(key) =
                    resolve_exported_type(graph, &target_module, local_name, &mut visited)
                {
                    return Some(key);
                }
            }
        }
    }
    None
}

fn resolve_exported_type(
    graph: &ModuleGraph,
    module_path: &[String],
    exported_name: &str,
    visited: &mut BTreeSet<(Vec<String>, String)>,
) -> Option<ReachableItemKey> {
    if !visited.insert((module_path.to_vec(), exported_name.to_owned())) {
        return None;
    }
    if let Some(key) = find_named_type_item(graph, module_path, exported_name) {
        return Some(key);
    }
    let node = graph.modules.get(module_path)?;
    for item in &node.items {
        let Item::Use(item_use) = item else {
            continue;
        };
        if !is_public_visibility(&item_use.vis) {
            continue;
        }
        for (target_module, target_name, public_name) in
            expand_resolved_use_tree(graph, module_path, &item_use.tree)
        {
            let sought = if public_name == exported_name && target_name != "*" {
                target_name.as_str()
            } else if public_name == "*" && target_name == "*" {
                exported_name
            } else {
                continue;
            };
            if let Some(key) = resolve_exported_type(graph, &target_module, sought, visited) {
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

pub(super) fn find_named_type_item(
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
