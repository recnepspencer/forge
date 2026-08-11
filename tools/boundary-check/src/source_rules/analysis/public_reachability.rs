//! Compute externally reachable items from a parsed module graph.
//!
//! Public means reachable from crate-root (or explicit seeds) through `pub`
//! visibility chains and same-crate `pub use` re-exports (including renames,
//! groups, and nested globs). Seeds allow external re-exports to reuse the same
//! closure without re-deriving ownership rules per re-export site.

use super::crate_modules::{is_public_visibility, ModuleGraph};
use super::module_path_resolution::expand_resolved_use_tree;
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::path::Path;
use syn::{Item, Visibility};

/// One externally reachable declaration site.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub(super) struct ReachableItemKey {
    /// Module path segments from crate root to the defining module.
    pub(super) module_path: Vec<String>,
    /// Local item name in that module (original definition name).
    pub(super) item_name: String,
}

/// Seeds that open an ordinary public callable surface inside a module graph.
#[derive(Clone, Debug, Default)]
pub(super) struct ReachabilitySeeds {
    /// Modules treated as already externally public (e.g. re-exported module).
    pub(super) modules: BTreeSet<Vec<String>>,
    /// Named items treated as already externally public (e.g. re-exported type).
    pub(super) items: BTreeSet<ReachableItemKey>,
}

/// Reachability result: definition keys, public modules, and import aliases.
#[derive(Debug, Default)]
pub(super) struct Reachability {
    pub(super) items: BTreeSet<ReachableItemKey>,
    /// Modules that contribute ordinary public callable surface.
    pub(super) public_modules: BTreeSet<Vec<String>>,
    /// Local simple names that resolve to a forbidden authority trait spelling.
    pub(super) forbidden_aliases: BTreeMap<Vec<String>, BTreeSet<String>>,
}

/// Full crate-root ordinary public surface.
///
/// Returns `Err` when dependency authority indexing fails (fail closed for BC7001).
pub(super) fn externally_reachable_items(
    graph: &ModuleGraph,
    crate_root: &Path,
) -> Result<Reachability, String> {
    let mut seeds = ReachabilitySeeds::default();
    if graph.modules.contains_key(&Vec::<String>::new()) {
        seeds.modules.insert(Vec::new());
    }
    let mut reachability = reachability_from_seeds(graph, seeds);
    // Full-crate mode: also promote any module whose pub chain reaches root.
    promote_public_chain_modules(graph, &mut reachability);
    reachability.forbidden_aliases =
        super::forbidden_aliases::collect_forbidden_aliases(graph, crate_root)?;
    Ok(reachability)
}

/// Ordinary public surface opened by explicit module/item seeds (external re-exports).
pub(super) fn reachability_from_seeds(
    graph: &ModuleGraph,
    seeds: ReachabilitySeeds,
) -> Reachability {
    let mut reachability = Reachability::default();
    let mut public_modules: BTreeSet<Vec<String>> = seeds.modules;
    let mut queue: VecDeque<Vec<String>> = public_modules.iter().cloned().collect();
    let mut glob_expanded: BTreeSet<Vec<String>> = BTreeSet::new();

    for item in seeds.items {
        reachability.items.insert(item);
    }
    reachability.public_modules = public_modules.clone();

    while let Some(module_path) = queue.pop_front() {
        walk_public_module(
            graph,
            &module_path,
            &mut reachability,
            &mut public_modules,
            &mut queue,
            &mut glob_expanded,
        );
    }

    // Public type aliases make underlying local types ordinary public carriers.
    super::type_alias_reachability::promote_type_alias_underlying_types(graph, &mut reachability);

    reachability.public_modules = public_modules;
    reachability
}

/// Whether every prefix of `path` is a public child of its parent.
pub(super) fn module_is_public_chain(graph: &ModuleGraph, path: &[String]) -> bool {
    for depth in 1..=path.len() {
        let prefix = path[..depth].to_vec();
        let Some(node) = graph.modules.get(&prefix) else {
            return false;
        };
        if !node.public_from_parent {
            return false;
        }
    }
    true
}

fn promote_public_chain_modules(graph: &ModuleGraph, reachability: &mut Reachability) {
    let mut queue: VecDeque<Vec<String>> = VecDeque::new();
    let mut public_modules = reachability.public_modules.clone();
    let mut glob_expanded: BTreeSet<Vec<String>> = BTreeSet::new();

    for path in graph.modules.keys() {
        if path.is_empty() {
            continue;
        }
        if module_is_public_chain(graph, path) && public_modules.insert(path.clone()) {
            queue.push_back(path.clone());
        }
    }
    while let Some(module_path) = queue.pop_front() {
        walk_public_module(
            graph,
            &module_path,
            reachability,
            &mut public_modules,
            &mut queue,
            &mut glob_expanded,
        );
    }
    reachability.public_modules = public_modules;
}

fn walk_public_module(
    graph: &ModuleGraph,
    module_path: &[String],
    reachability: &mut Reachability,
    public_modules: &mut BTreeSet<Vec<String>>,
    queue: &mut VecDeque<Vec<String>>,
    glob_expanded: &mut BTreeSet<Vec<String>>,
) {
    let Some(node) = graph.modules.get(module_path) else {
        return;
    };
    for item in &node.items {
        match item {
            Item::Mod(item_mod) if is_public_visibility(&item_mod.vis) => {
                let mut child = module_path.to_vec();
                child.push(item_mod.ident.to_string());
                if public_modules.insert(child.clone()) {
                    queue.push_back(child);
                }
            }
            Item::Use(item_use) if is_public_visibility(&item_use.vis) => {
                for (target_module, target_name, _export_name) in
                    expand_resolved_use_tree(graph, module_path, &item_use.tree)
                {
                    if target_name == "*" {
                        promote_all_public_items(
                            graph,
                            &target_module,
                            reachability,
                            public_modules,
                            queue,
                            glob_expanded,
                        );
                    } else {
                        super::public_reexport_promotion::promote_item(
                            graph,
                            &target_module,
                            &target_name,
                            reachability,
                            public_modules,
                            queue,
                        );
                    }
                }
            }
            other if item_is_public_declaration(other) => {
                if let Some(name) = item_name(other) {
                    reachability.items.insert(ReachableItemKey {
                        module_path: module_path.to_vec(),
                        item_name: name,
                    });
                }
            }
            _ => {}
        }
    }
}

fn promote_all_public_items(
    graph: &ModuleGraph,
    target_module: &[String],
    reachability: &mut Reachability,
    public_modules: &mut BTreeSet<Vec<String>>,
    queue: &mut VecDeque<Vec<String>>,
    glob_expanded: &mut BTreeSet<Vec<String>>,
) {
    if !glob_expanded.insert(target_module.to_vec()) {
        return;
    }
    // Glob of a module makes that module's public surface ordinary API.
    public_modules.insert(target_module.to_vec());
    let Some(node) = graph.modules.get(target_module) else {
        return;
    };
    let items = node.items.clone();
    for item in &items {
        if item_is_public_declaration(item) {
            if let Some(name) = item_name(item) {
                reachability.items.insert(ReachableItemKey {
                    module_path: target_module.to_vec(),
                    item_name: name,
                });
            }
        }
        if let Item::Use(item_use) = item {
            if is_public_visibility(&item_use.vis) {
                for (nested_module, nested_name, _) in
                    expand_resolved_use_tree(graph, target_module, &item_use.tree)
                {
                    if nested_name == "*" {
                        promote_all_public_items(
                            graph,
                            &nested_module,
                            reachability,
                            public_modules,
                            queue,
                            glob_expanded,
                        );
                    } else {
                        super::public_reexport_promotion::promote_item(
                            graph,
                            &nested_module,
                            &nested_name,
                            reachability,
                            public_modules,
                            queue,
                        );
                    }
                }
            }
        }
        if let Item::Mod(item_mod) = item {
            if is_public_visibility(&item_mod.vis) {
                let mut child = target_module.to_vec();
                child.push(item_mod.ident.to_string());
                if public_modules.insert(child.clone()) {
                    queue.push_back(child);
                }
            }
        }
    }
}

pub(super) fn item_is_public_declaration(item: &Item) -> bool {
    item_visibility(item).is_some_and(is_public_visibility)
}

pub(super) fn item_visibility(item: &Item) -> Option<&Visibility> {
    match item {
        Item::Const(i) => Some(&i.vis),
        Item::Enum(i) => Some(&i.vis),
        Item::Fn(i) => Some(&i.vis),
        Item::Mod(i) => Some(&i.vis),
        Item::Static(i) => Some(&i.vis),
        Item::Struct(i) => Some(&i.vis),
        Item::Trait(i) => Some(&i.vis),
        Item::TraitAlias(i) => Some(&i.vis),
        Item::Type(i) => Some(&i.vis),
        Item::Union(i) => Some(&i.vis),
        Item::Use(i) => Some(&i.vis),
        _ => None,
    }
}

pub(super) fn item_name(item: &Item) -> Option<String> {
    match item {
        Item::Const(i) => Some(i.ident.to_string()),
        Item::Enum(i) => Some(i.ident.to_string()),
        Item::Fn(i) => Some(i.sig.ident.to_string()),
        Item::Mod(i) => Some(i.ident.to_string()),
        Item::Static(i) => Some(i.ident.to_string()),
        Item::Struct(i) => Some(i.ident.to_string()),
        Item::Trait(i) => Some(i.ident.to_string()),
        Item::TraitAlias(i) => Some(i.ident.to_string()),
        Item::Type(i) => Some(i.ident.to_string()),
        Item::Union(i) => Some(i.ident.to_string()),
        _ => None,
    }
}
