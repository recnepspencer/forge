//! Resolve module paths through one projected Cargo world's lexical imports.

use super::crate_modules::{is_public_visibility, ModuleGraph};
use std::collections::BTreeSet;
use syn::{Item, UseTree};

type ModuleKey = Vec<String>;

#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
enum BindingSurface {
    Lexical,
    Public,
}

/// Expand one use tree, resolving every crate-local target module exactly.
pub(super) fn expand_resolved_use_tree(
    graph: &ModuleGraph,
    current_module: &[String],
    tree: &UseTree,
) -> Vec<(ModuleKey, String, String)> {
    expand_use_tree(current_module, tree)
        .into_iter()
        .filter_map(|(module, imported, local)| {
            resolve_module_path(graph, &module).map(|resolved| (resolved, imported, local))
        })
        .collect()
}

/// Resolve a normalized module path through local module import bindings.
///
/// The input path comes from [`expand_use_tree`] or an equivalent normalized
/// type path. `use` and `extern crate` bindings share one namespace decision:
/// explicit bindings shadow globs, while collisions, external roots, and cycles
/// fail closed.
pub(super) fn resolve_module_path(
    graph: &ModuleGraph,
    normalized_path: &[String],
) -> Option<ModuleKey> {
    ModulePathResolver {
        graph,
        resolving: BTreeSet::new(),
    }
    .resolve_path(normalized_path)
}

struct ModulePathResolver<'a> {
    graph: &'a ModuleGraph,
    resolving: BTreeSet<(ModuleKey, String, BindingSurface)>,
}

impl ModulePathResolver<'_> {
    fn resolve_path(&mut self, normalized_path: &[String]) -> Option<ModuleKey> {
        if self.graph.modules.contains_key(normalized_path) {
            return Some(normalized_path.to_vec());
        }
        let prefix_length = (0..normalized_path.len())
            .rev()
            .find(|length| self.graph.modules.contains_key(&normalized_path[..*length]))?;
        let mut resolved = self.resolve_module_name(
            &normalized_path[..prefix_length],
            &normalized_path[prefix_length],
            BindingSurface::Lexical,
        )?;
        for segment in &normalized_path[prefix_length + 1..] {
            resolved = self.resolve_module_name(&resolved, segment, BindingSurface::Lexical)?;
        }
        Some(resolved)
    }

    fn resolve_module_name(
        &mut self,
        module: &[String],
        name: &str,
        surface: BindingSurface,
    ) -> Option<ModuleKey> {
        let key = (module.to_vec(), name.to_owned(), surface);
        if !self.resolving.insert(key.clone()) {
            return None;
        }
        let node = self.graph.modules.get(module)?;
        let items = node.items.clone();
        let result = self.resolve_import_bindings(module, name, surface, items);
        self.resolving.remove(&key);
        result
    }

    fn resolve_import_bindings(
        &mut self,
        module: &[String],
        name: &str,
        surface: BindingSurface,
        items: Vec<Item>,
    ) -> Option<ModuleKey> {
        let mut explicit_binding_count = 0;
        let mut explicit_targets = BTreeSet::new();
        let mut glob_targets = BTreeSet::new();
        if let Some(child) = direct_child(self.graph, module, name) {
            explicit_binding_count += 1;
            if surface == BindingSurface::Lexical || self.graph.modules[&child].public_from_parent {
                explicit_targets.insert(child);
            }
        }
        for item in items {
            match item {
                Item::Use(item_use)
                    if surface == BindingSurface::Lexical
                        || is_public_visibility(&item_use.vis) =>
                {
                    for (target_module, target_name, import_name) in
                        expand_use_tree(module, &item_use.tree)
                    {
                        if import_name == name && target_name != "*" {
                            explicit_binding_count += 1;
                            let target = named_module_target(&target_module, &target_name);
                            if let Some(resolved) = self.resolve_path(&target) {
                                explicit_targets.insert(resolved);
                            }
                        } else if import_name == "*" && target_name == "*" {
                            self.collect_glob_target(&target_module, name, &mut glob_targets);
                        }
                    }
                }
                Item::ExternCrate(item_extern)
                    if (surface == BindingSurface::Lexical
                        || is_public_visibility(&item_extern.vis))
                        && extern_crate_local_name(&item_extern) == name =>
                {
                    explicit_binding_count += 1;
                    if item_extern.ident == "self" {
                        explicit_targets.insert(Vec::new());
                    }
                }
                _ => {}
            }
        }
        if explicit_binding_count > 0 {
            (explicit_binding_count == 1)
                .then(|| unique_target(explicit_targets))
                .flatten()
        } else {
            unique_target(glob_targets)
        }
    }

    fn collect_glob_target(
        &mut self,
        target_module: &[String],
        name: &str,
        glob_targets: &mut BTreeSet<ModuleKey>,
    ) {
        let Some(glob_module) = self.resolve_path(target_module) else {
            return;
        };
        if let Some(resolved) = self.resolve_module_name(&glob_module, name, BindingSurface::Public)
        {
            glob_targets.insert(resolved);
        }
    }
}

fn direct_child(graph: &ModuleGraph, module: &[String], name: &str) -> Option<ModuleKey> {
    let mut child = module.to_vec();
    child.push(name.to_owned());
    graph.modules.contains_key(&child).then_some(child)
}

fn extern_crate_local_name(item: &syn::ItemExternCrate) -> String {
    item.rename
        .as_ref()
        .map_or_else(|| item.ident.to_string(), |(_, name)| name.to_string())
}

fn named_module_target(target_module: &[String], target_name: &str) -> ModuleKey {
    if target_name == "self" {
        return target_module.to_vec();
    }
    let mut target = target_module.to_vec();
    target.push(target_name.to_owned());
    target
}

fn unique_target(targets: BTreeSet<ModuleKey>) -> Option<ModuleKey> {
    (targets.len() == 1)
        .then(|| targets.into_iter().next())
        .flatten()
}

/// Expand a use tree into normalized `(module, imported, local)` triples.
pub(super) fn expand_use_tree(
    current_module: &[String],
    tree: &UseTree,
) -> Vec<(ModuleKey, String, String)> {
    expand_use_tree_rec(current_module, current_module, tree)
}

fn expand_use_tree_rec(
    current_module: &[String],
    prefix_module: &[String],
    tree: &UseTree,
) -> Vec<(ModuleKey, String, String)> {
    match tree {
        UseTree::Path(path) => {
            let next = match path.ident.to_string().as_str() {
                "self" if prefix_module.is_empty() => current_module.to_vec(),
                "self" => prefix_module.to_vec(),
                "super" => {
                    let mut parent = if prefix_module.is_empty() {
                        current_module.to_vec()
                    } else {
                        prefix_module.to_vec()
                    };
                    parent.pop();
                    parent
                }
                "crate" => Vec::new(),
                name => {
                    let mut child = prefix_module.to_vec();
                    child.push(name.to_owned());
                    child
                }
            };
            expand_use_tree_rec(current_module, &next, &path.tree)
        }
        UseTree::Name(name) => {
            let name = name.ident.to_string();
            vec![(prefix_module.to_vec(), name.clone(), name)]
        }
        UseTree::Rename(rename) => {
            let (target_module, target_name) =
                renamed_target(current_module, prefix_module, &rename.ident.to_string());
            vec![(target_module, target_name, rename.rename.to_string())]
        }
        UseTree::Glob(_) => vec![(prefix_module.to_vec(), "*".to_owned(), "*".to_owned())],
        UseTree::Group(group) => group
            .items
            .iter()
            .flat_map(|item| expand_use_tree_rec(current_module, prefix_module, item))
            .collect(),
    }
}

fn renamed_target(
    current_module: &[String],
    prefix_module: &[String],
    source_name: &str,
) -> (ModuleKey, String) {
    match source_name {
        "crate" => (Vec::new(), "self".to_owned()),
        "self" => (prefix_module.to_vec(), "self".to_owned()),
        "super" => {
            let mut parent = if prefix_module.is_empty() {
                current_module.to_vec()
            } else {
                prefix_module.to_vec()
            };
            parent.pop();
            (parent, "self".to_owned())
        }
        name => (prefix_module.to_vec(), name.to_owned()),
    }
}
