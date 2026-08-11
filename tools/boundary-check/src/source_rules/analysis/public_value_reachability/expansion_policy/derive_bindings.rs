//! Resolve derive spellings through the world's exact lexical import graph.

use super::super::super::crate_modules::ModuleGraph;
use super::super::super::module_path_resolution::expand_use_tree;
use super::super::super::public_reachability::item_name;
use std::collections::BTreeSet;
use syn::Item;

pub(super) fn require_unbound(
    graph: &ModuleGraph,
    module_path: &[String],
    derive_name: &str,
) -> Result<(), String> {
    let mut visited = BTreeSet::new();
    if let Some(origin) = resolve_explicit_binding(graph, module_path, derive_name, &mut visited) {
        return Err(format!(
            "derive `{derive_name}` has explicit import binding `{origin}`; compiler built-in identity is not proven"
        ));
    }
    Ok(())
}

fn resolve_explicit_binding(
    graph: &ModuleGraph,
    module_path: &[String],
    local_name: &str,
    visited: &mut BTreeSet<(Vec<String>, String)>,
) -> Option<String> {
    let key = (module_path.to_vec(), local_name.to_owned());
    if !visited.insert(key.clone()) {
        return Some(format!(
            "{}::{local_name} (cyclic)",
            display_module(module_path)
        ));
    }
    let node = graph.modules.get(module_path)?;
    for item_use in node.items.iter().filter_map(|item| match item {
        Item::Use(item_use) => Some(item_use),
        _ => None,
    }) {
        for (target_module, target_name, import_name) in
            expand_use_tree(module_path, &item_use.tree)
        {
            if import_name == local_name {
                return Some(resolve_named_origin(
                    graph,
                    &target_module,
                    &target_name,
                    visited,
                ));
            }
            if import_name == "*" {
                if graph.modules.contains_key(&target_module) {
                    if let Some(origin) =
                        resolve_local_export(graph, &target_module, local_name, visited)
                    {
                        return Some(origin);
                    }
                } else {
                    return Some(format!("{}::*", display_module(&target_module)));
                }
            }
        }
    }
    visited.remove(&key);
    None
}

fn resolve_named_origin(
    graph: &ModuleGraph,
    target_module: &[String],
    target_name: &str,
    visited: &mut BTreeSet<(Vec<String>, String)>,
) -> String {
    if graph.modules.contains_key(target_module) {
        if let Some(origin) = resolve_local_export(graph, target_module, target_name, visited) {
            return origin;
        }
    }
    format!("{}::{target_name}", display_module(target_module))
}

fn resolve_local_export(
    graph: &ModuleGraph,
    module_path: &[String],
    name: &str,
    visited: &mut BTreeSet<(Vec<String>, String)>,
) -> Option<String> {
    let node = graph.modules.get(module_path)?;
    if node
        .items
        .iter()
        .any(|item| item_name(item).as_deref() == Some(name))
    {
        return Some(format!("{}::{name}", display_module(module_path)));
    }
    resolve_explicit_binding(graph, module_path, name, visited)
}

fn display_module(module_path: &[String]) -> String {
    if module_path.is_empty() {
        "crate".to_owned()
    } else {
        module_path.join("::")
    }
}
