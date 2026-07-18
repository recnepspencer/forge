//! Forbidden authority-trait alias inventory for sealing.
//!
//! Tracks names that resolve to sealed authority traits via local uses, path-
//! dependency renames, trait aliases/supertraits, and blanket laundering
//! (see `blanket_launder`). Sealed spellings are projected to the crate root.

use super::authority_sealing_surface::FORBIDDEN_TRAITS;
use super::blanket_launder::blanket_impl_launders_forbidden;
use super::crate_modules::ModuleGraph;
use super::dependency_authority::{
    dep_path_is_sealed, dep_sealed_index, DepAuthorityCache, DepSealedIndex,
};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;
use syn::{Item, TypeParamBound, UseTree};

/// Module path → local simple names that alias a sealed authority trait.
pub(super) type AliasInventory = BTreeMap<Vec<String>, BTreeSet<String>>;

/// Collect sealed-trait aliases for a governed (or dependency) crate root.
pub(super) fn collect_forbidden_aliases(
    graph: &ModuleGraph,
    crate_root: &Path,
) -> Result<AliasInventory, String> {
    let mut cache = DepAuthorityCache::default();
    collect_forbidden_aliases_local(graph, crate_root, &mut cache)
}

/// Local collection with a shared dependency cache (used recursively for path deps).
pub(super) fn collect_forbidden_aliases_local(
    graph: &ModuleGraph,
    crate_root: &Path,
    cache: &mut DepAuthorityCache,
) -> Result<AliasInventory, String> {
    // Fail closed: dependency-index errors must surface as BC7001, never empty.
    let dep_index = dep_sealed_index(crate_root, cache)?;
    Ok(collect_with_dep_index(graph, &dep_index))
}

fn collect_with_dep_index(graph: &ModuleGraph, dep_index: &DepSealedIndex) -> AliasInventory {
    let mut aliases = AliasInventory::new();
    // Seed qualified dependency sealed exports (`dep::Gate`) only.
    for names in dep_index.values() {
        for name in names {
            if name.contains("::") {
                let _ = insert_alias(&mut aliases, &[], name.clone());
            }
        }
    }

    let mut changed = true;
    while changed {
        changed = false;
        for (module_path, node) in &graph.modules {
            let available = cumulative_aliases(&aliases, module_path);
            for item in &node.items {
                match item {
                    Item::Use(item_use) => {
                        for binding in collect_use_bindings(module_path, &item_use.tree) {
                            let sealed = is_forbidden_spelling(&binding.source, &available)
                                || dep_path_is_sealed(
                                    &binding.path_prefix,
                                    &binding.source,
                                    dep_index,
                                );
                            if sealed && project_alias(&mut aliases, module_path, binding.local) {
                                changed = true;
                            }
                        }
                        for target in expand_use_glob_targets(module_path, &item_use.tree) {
                            if let Some(crate_ident) = target.first() {
                                if target.len() == 1 {
                                    if let Some(sealed) = dep_index.get(crate_ident) {
                                        for name in sealed {
                                            let simple =
                                                name.rsplit("::").next().unwrap_or(name).to_owned();
                                            if project_alias(&mut aliases, module_path, simple) {
                                                changed = true;
                                            }
                                        }
                                    }
                                }
                            }
                            let imported: Vec<String> = aliases
                                .get(&target)
                                .map(|set| set.iter().cloned().collect())
                                .unwrap_or_default();
                            for name in imported {
                                if project_alias(&mut aliases, module_path, name) {
                                    changed = true;
                                }
                            }
                        }
                    }
                    Item::TraitAlias(trait_alias) => {
                        if trait_alias_is_forbidden(trait_alias, &available)
                            && project_alias(
                                &mut aliases,
                                module_path,
                                trait_alias.ident.to_string(),
                            )
                        {
                            changed = true;
                        }
                    }
                    Item::Trait(item_trait) => {
                        if trait_supertraits_forbidden(item_trait, &available)
                            && project_alias(
                                &mut aliases,
                                module_path,
                                item_trait.ident.to_string(),
                            )
                        {
                            changed = true;
                        }
                    }
                    Item::Impl(item_impl) => {
                        if let Some(gate) = blanket_impl_launders_forbidden(item_impl, &available) {
                            if project_alias(&mut aliases, module_path, gate) {
                                changed = true;
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    aliases
}

/// Union of aliases declared in `module_path` and all ancestor modules.
pub(super) fn cumulative_aliases(
    aliases: &AliasInventory,
    module_path: &[String],
) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    for depth in 0..=module_path.len() {
        let prefix = module_path[..depth].to_vec();
        if let Some(set) = aliases.get(&prefix) {
            out.extend(set.iter().cloned());
        }
    }
    out
}

/// Trait path is sealed as a final segment or as a full qualified spelling.
pub(super) fn path_is_forbidden(path: &syn::Path, available: &BTreeSet<String>) -> bool {
    let segments: Vec<String> = path.segments.iter().map(|s| s.ident.to_string()).collect();
    if segments.is_empty() {
        return false;
    }
    let last = segments.last().unwrap();
    if FORBIDDEN_TRAITS.contains(&last.as_str()) || available.contains(last) {
        return true;
    }
    let full = segments.join("::");
    available.contains(&full)
}

fn project_alias(aliases: &mut AliasInventory, module_path: &[String], local: String) -> bool {
    let a = insert_alias(aliases, module_path, local.clone());
    let b = insert_alias(aliases, &[], local);
    a || b
}

fn insert_alias(aliases: &mut AliasInventory, module_path: &[String], local: String) -> bool {
    aliases
        .entry(module_path.to_vec())
        .or_default()
        .insert(local)
}

fn is_forbidden_spelling(name: &str, available: &BTreeSet<String>) -> bool {
    FORBIDDEN_TRAITS.contains(&name) || available.contains(name)
}

fn trait_alias_is_forbidden(
    trait_alias: &syn::ItemTraitAlias,
    available: &BTreeSet<String>,
) -> bool {
    trait_alias
        .bounds
        .iter()
        .any(|bound| bound_is_forbidden(bound, available))
}

fn trait_supertraits_forbidden(item_trait: &syn::ItemTrait, available: &BTreeSet<String>) -> bool {
    item_trait
        .supertraits
        .iter()
        .any(|bound| bound_is_forbidden(bound, available))
}

fn bound_is_forbidden(bound: &TypeParamBound, available: &BTreeSet<String>) -> bool {
    let TypeParamBound::Trait(trait_bound) = bound else {
        return false;
    };
    path_is_forbidden(&trait_bound.path, available)
}

struct UseBinding {
    local: String,
    source: String,
    path_prefix: Vec<String>,
}

fn collect_use_bindings(current_module: &[String], tree: &UseTree) -> Vec<UseBinding> {
    collect_use_bindings_rec(current_module, &[], tree)
}

fn collect_use_bindings_rec(
    current_module: &[String],
    prefix_module: &[String],
    tree: &UseTree,
) -> Vec<UseBinding> {
    match tree {
        UseTree::Path(path) => {
            let ident = path.ident.to_string();
            let next = match ident.as_str() {
                "self" if prefix_module.is_empty() => current_module.to_vec(),
                "self" => prefix_module.to_vec(),
                "super" => {
                    let mut p = if prefix_module.is_empty() {
                        current_module.to_vec()
                    } else {
                        prefix_module.to_vec()
                    };
                    p.pop();
                    p
                }
                "crate" => Vec::new(),
                other => {
                    let mut p = prefix_module.to_vec();
                    p.push(other.to_owned());
                    p
                }
            };
            collect_use_bindings_rec(current_module, &next, &path.tree)
        }
        UseTree::Name(name) => {
            let n = name.ident.to_string();
            vec![UseBinding {
                local: n.clone(),
                source: n,
                path_prefix: prefix_module.to_vec(),
            }]
        }
        UseTree::Rename(rename) => vec![UseBinding {
            local: rename.rename.to_string(),
            source: rename.ident.to_string(),
            path_prefix: prefix_module.to_vec(),
        }],
        UseTree::Glob(_) => Vec::new(),
        UseTree::Group(group) => group
            .items
            .iter()
            .flat_map(|item| collect_use_bindings_rec(current_module, prefix_module, item))
            .collect(),
    }
}

fn expand_use_glob_targets(current_module: &[String], tree: &UseTree) -> Vec<Vec<String>> {
    expand_use_glob_rec(current_module, &[], tree)
}

fn expand_use_glob_rec(
    current_module: &[String],
    prefix_module: &[String],
    tree: &UseTree,
) -> Vec<Vec<String>> {
    match tree {
        UseTree::Path(path) => {
            let ident = path.ident.to_string();
            let next = match ident.as_str() {
                "self" if prefix_module.is_empty() => current_module.to_vec(),
                "self" => prefix_module.to_vec(),
                "super" => {
                    let mut p = if prefix_module.is_empty() {
                        current_module.to_vec()
                    } else {
                        prefix_module.to_vec()
                    };
                    p.pop();
                    p
                }
                "crate" => Vec::new(),
                other => {
                    let mut p = prefix_module.to_vec();
                    p.push(other.to_owned());
                    p
                }
            };
            expand_use_glob_rec(current_module, &next, &path.tree)
        }
        UseTree::Glob(_) => vec![prefix_module.to_vec()],
        UseTree::Group(group) => group
            .items
            .iter()
            .flat_map(|item| expand_use_glob_rec(current_module, prefix_module, item))
            .collect(),
        UseTree::Name(_) | UseTree::Rename(_) => Vec::new(),
    }
}
