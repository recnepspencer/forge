//! Verify the one representation that can exempt an exported public value.

use super::super::crate_modules::ModuleGraph;
use super::super::public_reachability::ReachableItemKey;
use syn::Item;

pub(super) fn is_verified(graph: &ModuleGraph, key: &ReachableItemKey) -> bool {
    graph.modules.get(&key.module_path).is_some_and(|node| {
        node.items.iter().any(|item| match item {
            Item::Enum(item) => item.ident == key.item_name && item.variants.is_empty(),
            _ => false,
        })
    })
}
