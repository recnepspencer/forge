//! Definition-backed identity for value-gated carriers and local marker types.

use super::crate_modules::ModuleGraph;
use std::collections::{BTreeMap, BTreeSet};
use syn::{Item, Type, UseTree};

pub(super) const CARRIERS: &[(&str, usize)] = &[
    ("AuthorityWitness", 0),
    ("CapabilityWitness", 0),
    ("Proof", 1),
];

pub(super) type CarrierAliases = BTreeMap<Vec<String>, BTreeMap<String, usize>>;

/// Resolve aliases only when their source roots in the Cargo-identified
/// `worth-proof` package. Same-named local/foreign carriers are not authority.
pub(super) fn carrier_aliases(
    graph: &ModuleGraph,
    worth_proof_idents: &BTreeSet<String>,
) -> CarrierAliases {
    let mut aliases = CarrierAliases::new();
    let mut changed = true;
    while changed {
        changed = false;
        for (module_path, node) in &graph.modules {
            for item in &node.items {
                match item {
                    Item::Use(item_use) => {
                        for binding in use_bindings(&item_use.tree) {
                            let Some(arg_index) = carrier_source(
                                module_path,
                                &binding.source,
                                worth_proof_idents,
                                &aliases,
                            ) else {
                                continue;
                            };
                            changed |= insert_carrier_alias(
                                &mut aliases,
                                module_path,
                                binding.local,
                                arg_index,
                            );
                        }
                    }
                    Item::Type(item_type) => {
                        let Some(arg_index) = carrier_type_alias_argument(
                            module_path,
                            item_type,
                            worth_proof_idents,
                            &aliases,
                        ) else {
                            continue;
                        };
                        changed |= insert_carrier_alias(
                            &mut aliases,
                            module_path,
                            item_type.ident.to_string(),
                            arg_index,
                        );
                    }
                    _ => {}
                }
            }
        }
    }
    aliases
}

fn insert_carrier_alias(
    aliases: &mut CarrierAliases,
    module_path: &[String],
    local: String,
    arg_index: usize,
) -> bool {
    let slot = aliases.entry(module_path.to_vec()).or_default();
    match slot.get(&local) {
        Some(existing) if *existing == arg_index => false,
        _ => {
            slot.insert(local, arg_index);
            true
        }
    }
}

fn carrier_type_alias_argument(
    module_path: &[String],
    alias: &syn::ItemType,
    worth_proof_idents: &BTreeSet<String>,
    aliases: &CarrierAliases,
) -> Option<usize> {
    let Type::Path(target) = alias.ty.as_ref() else {
        return None;
    };
    let carrier_index = carrier_argument(&target.path, module_path, worth_proof_idents, aliases)?;
    let segment = target.path.segments.last()?;
    let syn::PathArguments::AngleBracketed(arguments) = &segment.arguments else {
        return None;
    };
    let carrier_arg = arguments
        .args
        .iter()
        .filter_map(|arg| match arg {
            syn::GenericArgument::Type(ty) => Some(ty),
            _ => None,
        })
        .nth(carrier_index)?;
    let Type::Path(param_path) = carrier_arg else {
        return None;
    };
    if param_path.path.segments.len() != 1 {
        return None;
    }
    let param = param_path.path.segments[0].ident.to_string();
    alias
        .generics
        .params
        .iter()
        .filter_map(|generic| match generic {
            syn::GenericParam::Type(ty) => Some(ty.ident.to_string()),
            _ => None,
        })
        .position(|name| name == param)
}

pub(super) fn carrier_argument(
    path: &syn::Path,
    module_path: &[String],
    worth_proof_idents: &BTreeSet<String>,
    aliases: &CarrierAliases,
) -> Option<usize> {
    let segments: Vec<String> = path.segments.iter().map(|s| s.ident.to_string()).collect();
    let last = segments.last()?;
    let direct = CARRIERS
        .iter()
        .find(|(name, _)| name == last)
        .map(|(_, i)| *i);
    if segments.len() >= 2 && worth_proof_idents.contains(&segments[0]) && direct.is_some() {
        return direct;
    }
    if segments.len() == 1 {
        return aliases.get(module_path)?.get(last).copied();
    }
    let (target_module, local) = split_local_path(module_path, &segments)?;
    aliases.get(&target_module)?.get(&local).copied()
}

pub(super) fn local_type_key(
    graph: &ModuleGraph,
    module_path: &[String],
    ty: &Type,
) -> Option<(Vec<String>, String)> {
    let Type::Path(type_path) = peel_type(ty) else {
        return None;
    };
    if type_path.qself.is_some() {
        return None;
    }
    resolve_local_path(graph, module_path, &type_path.path, 0)
}

fn resolve_local_path(
    graph: &ModuleGraph,
    module_path: &[String],
    path: &syn::Path,
    depth: usize,
) -> Option<(Vec<String>, String)> {
    if depth > 8 {
        return None;
    }
    let segments: Vec<String> = path.segments.iter().map(|s| s.ident.to_string()).collect();
    if segments.is_empty() {
        return None;
    }
    if segments.len() == 1 {
        let name = &segments[0];
        if type_exists(graph, module_path, name) {
            return Some((module_path.to_vec(), name.clone()));
        }
        if let Some(resolved) = resolve_module_import(graph, module_path, name, depth) {
            return Some(resolved);
        }
        if type_exists(graph, &[], name) {
            return Some((Vec::new(), name.clone()));
        }
        return None;
    }
    let (target_module, name) = split_local_path(module_path, &segments)?;
    if type_exists(graph, &target_module, &name) {
        return Some((target_module, name));
    }
    resolve_module_import(graph, &target_module, &name, depth)
}

fn resolve_module_import(
    graph: &ModuleGraph,
    module_path: &[String],
    local: &str,
    depth: usize,
) -> Option<(Vec<String>, String)> {
    let node = graph.modules.get(module_path)?;
    for item in &node.items {
        let Item::Use(item_use) = item else { continue };
        for binding in use_bindings(&item_use.tree) {
            if binding.local == local {
                let rebound = path_from_segments(&binding.source)?;
                return resolve_local_path(graph, module_path, &rebound, depth + 1);
            }
        }
    }
    None
}

fn type_exists(graph: &ModuleGraph, module_path: &[String], name: &str) -> bool {
    graph.modules.get(module_path).is_some_and(|node| {
        node.items.iter().any(|item| match item {
            Item::Struct(i) => i.ident == name,
            Item::Enum(i) => i.ident == name,
            Item::Union(i) => i.ident == name,
            Item::Type(i) => i.ident == name,
            _ => false,
        })
    })
}

fn split_local_path(current: &[String], segments: &[String]) -> Option<(Vec<String>, String)> {
    let (mut module, start) = match segments.first()?.as_str() {
        "crate" => (Vec::new(), 1),
        "self" => (current.to_vec(), 1),
        "super" => {
            let mut parent = current.to_vec();
            parent.pop();
            (parent, 1)
        }
        _ => (current.to_vec(), 0),
    };
    if segments.len() <= start {
        return None;
    }
    for segment in &segments[start..segments.len() - 1] {
        module.push(segment.clone());
    }
    Some((module, segments.last()?.clone()))
}

fn carrier_source(
    current: &[String],
    source: &[String],
    worth_proof_idents: &BTreeSet<String>,
    aliases: &CarrierAliases,
) -> Option<usize> {
    let last = source.last()?;
    let direct = CARRIERS
        .iter()
        .find(|(name, _)| name == last)
        .map(|(_, i)| *i);
    if source.len() >= 2 && worth_proof_idents.contains(&source[0]) {
        return direct;
    }
    let (module, local) = split_local_path(current, source)?;
    aliases.get(&module)?.get(&local).copied()
}

struct UseBinding {
    source: Vec<String>,
    local: String,
}

fn use_bindings(tree: &UseTree) -> Vec<UseBinding> {
    fn walk(tree: &UseTree, prefix: &mut Vec<String>, out: &mut Vec<UseBinding>) {
        match tree {
            UseTree::Path(p) => {
                prefix.push(p.ident.to_string());
                walk(&p.tree, prefix, out);
                prefix.pop();
            }
            UseTree::Name(n) => {
                let local = n.ident.to_string();
                let mut source = prefix.clone();
                source.push(local.clone());
                out.push(UseBinding { source, local });
            }
            UseTree::Rename(r) => {
                let mut source = prefix.clone();
                source.push(r.ident.to_string());
                out.push(UseBinding {
                    source,
                    local: r.rename.to_string(),
                });
            }
            UseTree::Group(g) => {
                for item in &g.items {
                    walk(item, prefix, out);
                }
            }
            UseTree::Glob(_) => {}
        }
    }
    let mut out = Vec::new();
    walk(tree, &mut Vec::new(), &mut out);
    out
}

fn path_from_segments(segments: &[String]) -> Option<syn::Path> {
    syn::parse_str(&segments.join("::")).ok()
}

fn peel_type(ty: &Type) -> &Type {
    match ty {
        Type::Reference(r) => peel_type(&r.elem),
        Type::Paren(p) => peel_type(&p.elem),
        Type::Group(g) => peel_type(&g.elem),
        other => other,
    }
}
