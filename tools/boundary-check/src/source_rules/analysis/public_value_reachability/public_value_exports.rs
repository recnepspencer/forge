//! Inventory every externally reachable public value definition in one Cargo world.

use super::super::crate_modules::ModuleGraph;
use super::super::production_availability::{
    item_is_production_available, module_is_production_available,
};
use super::super::public_reachability::{Reachability, ReachableItemKey};
use syn::Item;

#[derive(Clone, Debug)]
pub(super) struct PublicValueExport {
    pub(super) key: ReachableItemKey,
    pub(super) relative_source: String,
}

pub(super) fn collect(graph: &ModuleGraph, reachable: &Reachability) -> Vec<PublicValueExport> {
    let mut exports = Vec::new();
    for key in &reachable.items {
        if !module_is_production_available(graph, &key.module_path) {
            continue;
        }
        let Some(node) = graph.modules.get(&key.module_path) else {
            continue;
        };
        if node
            .items
            .iter()
            .any(|item| item_is_production_available(item) && item_defines_value(item, key))
        {
            exports.push(PublicValueExport {
                key: key.clone(),
                relative_source: node.relative_source.clone(),
            });
        }
    }
    exports.sort_by(|left, right| left.key.cmp(&right.key));
    exports
}

fn item_defines_value(item: &Item, key: &ReachableItemKey) -> bool {
    match item {
        Item::Struct(item) => item.ident == key.item_name,
        Item::Enum(item) => item.ident == key.item_name,
        Item::Union(item) => item.ident == key.item_name,
        _ => false,
    }
}
