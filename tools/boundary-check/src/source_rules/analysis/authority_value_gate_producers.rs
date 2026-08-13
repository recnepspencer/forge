//! Reachable public value and factory producers for authority-value gates.

use std::collections::BTreeSet;

use syn::{Item, ReturnType, Type, Visibility};

use super::authority_value_gate_defs::DefinitionKey;
use super::authority_value_gate_returns::{return_type_keys, return_type_keys_for_impl};
use super::authority_value_identity::local_type_key;
use super::crate_modules::ModuleGraph;
use super::public_reachability::{item_name, Reachability};

pub(super) fn collect_public_values(
    graph: &ModuleGraph,
    reachability: &Reachability,
) -> BTreeSet<DefinitionKey> {
    let mut values = BTreeSet::new();
    for key in &reachability.items {
        let Some(node) = graph.modules.get(&key.module_path) else {
            continue;
        };
        for item in &node.items {
            if item_name(item).as_deref() != Some(key.item_name.as_str()) {
                continue;
            }
            let ty = match item {
                Item::Const(item) => Some(item.ty.as_ref()),
                Item::Static(item) => Some(item.ty.as_ref()),
                _ => None,
            };
            if let Some(ty) = ty {
                values.extend(return_type_keys(graph, &key.module_path, ty, None));
            }
        }
    }
    collect_associated_values(graph, reachability, &mut values);
    collect_trait_values(graph, reachability, &mut values);
    values
}

fn collect_associated_values(
    graph: &ModuleGraph,
    reachability: &Reachability,
    values: &mut BTreeSet<DefinitionKey>,
) {
    for (module_path, node) in &graph.modules {
        for item in &node.items {
            let Item::Impl(item_impl) = item else {
                continue;
            };
            if !impl_surface_reachable(graph, item_impl, reachability) {
                continue;
            }
            for member in &item_impl.items {
                let syn::ImplItem::Const(assoc) = member else {
                    continue;
                };
                if matches!(assoc.vis, Visibility::Public(_)) || item_impl.trait_.is_some() {
                    values.extend(return_type_keys_for_impl(
                        graph,
                        module_path,
                        &assoc.ty,
                        item_impl,
                    ));
                }
            }
        }
    }
}

fn collect_trait_values(
    graph: &ModuleGraph,
    reachability: &Reachability,
    values: &mut BTreeSet<DefinitionKey>,
) {
    for key in &reachability.items {
        let Some(node) = graph.modules.get(&key.module_path) else {
            continue;
        };
        for item in &node.items {
            if item_name(item).as_deref() != Some(key.item_name.as_str()) {
                continue;
            }
            let Item::Trait(item_trait) = item else {
                continue;
            };
            for member in &item_trait.items {
                let syn::TraitItem::Const(assoc) = member else {
                    continue;
                };
                values.extend(return_type_keys(graph, &key.module_path, &assoc.ty, None));
            }
        }
    }
}

pub(super) fn collect_trait_factories(
    graph: &ModuleGraph,
    reachability: &Reachability,
) -> BTreeSet<DefinitionKey> {
    let mut factories = BTreeSet::new();
    for key in &reachability.items {
        let Some(node) = graph.modules.get(&key.module_path) else {
            continue;
        };
        for item in &node.items {
            if item_name(item).as_deref() != Some(key.item_name.as_str()) {
                continue;
            }
            let Item::Trait(item_trait) = item else {
                continue;
            };
            for member in &item_trait.items {
                let syn::TraitItem::Fn(method) = member else {
                    continue;
                };
                if let ReturnType::Type(_, ty) = &method.sig.output {
                    factories.extend(return_type_keys(graph, &key.module_path, ty, None));
                }
            }
        }
    }
    collect_trait_impl_factories(graph, reachability, &mut factories);
    factories
}

fn collect_trait_impl_factories(
    graph: &ModuleGraph,
    reachability: &Reachability,
    factories: &mut BTreeSet<DefinitionKey>,
) {
    for (module_path, node) in &graph.modules {
        for item in &node.items {
            let Item::Impl(item_impl) = item else {
                continue;
            };
            if item_impl.trait_.is_none() || !impl_surface_reachable(graph, item_impl, reachability)
            {
                continue;
            }
            for member in &item_impl.items {
                let syn::ImplItem::Fn(method) = member else {
                    continue;
                };
                if let ReturnType::Type(_, ty) = &method.sig.output {
                    factories.extend(return_type_keys_for_impl(graph, module_path, ty, item_impl));
                }
            }
        }
    }
}

pub(super) fn collect_public_self_constructors(
    graph: &ModuleGraph,
    reachability: &Reachability,
) -> BTreeSet<DefinitionKey> {
    let mut constructors = BTreeSet::new();
    for key in &reachability.items {
        let Some(node) = graph.modules.get(&key.module_path) else {
            continue;
        };
        for item in &node.items {
            if item_name(item).as_deref() != Some(key.item_name.as_str()) {
                continue;
            }
            let Item::Fn(function) = item else { continue };
            if let ReturnType::Type(_, ty) = &function.sig.output {
                constructors.extend(return_type_keys(graph, &key.module_path, ty, None));
            }
        }
    }
    collect_inherent_constructors(graph, reachability, &mut constructors);
    constructors
}

fn collect_inherent_constructors(
    graph: &ModuleGraph,
    reachability: &Reachability,
    constructors: &mut BTreeSet<DefinitionKey>,
) {
    for (module_path, node) in &graph.modules {
        for item in &node.items {
            let Item::Impl(item_impl) = item else {
                continue;
            };
            if item_impl.trait_.is_some()
                || local_type_key(graph, module_path, &item_impl.self_ty).is_none()
                || !self_type_reachable(&item_impl.self_ty, reachability)
            {
                continue;
            }
            for member in &item_impl.items {
                let syn::ImplItem::Fn(method) = member else {
                    continue;
                };
                if !matches!(method.vis, Visibility::Public(_)) {
                    continue;
                }
                if let ReturnType::Type(_, ty) = &method.sig.output {
                    constructors.extend(return_type_keys(
                        graph,
                        module_path,
                        ty,
                        Some(&item_impl.self_ty),
                    ));
                }
            }
        }
    }
}

fn impl_surface_reachable(
    graph: &ModuleGraph,
    item_impl: &syn::ItemImpl,
    reachability: &Reachability,
) -> bool {
    self_type_reachable(&item_impl.self_ty, reachability)
        || item_impl.trait_.as_ref().is_some_and(|(_, path, _)| {
            let Some(trait_name) = path
                .segments
                .last()
                .map(|segment| segment.ident.to_string())
            else {
                return false;
            };
            reachability.items.iter().any(|key| {
                key.item_name == trait_name
                    && graph.modules.get(&key.module_path).is_some_and(|module| {
                        module.items.iter().any(
                            |item| matches!(item, Item::Trait(item) if item.ident == trait_name),
                        )
                    })
            })
        })
}

fn self_type_reachable(ty: &Type, reachability: &Reachability) -> bool {
    simple_type_ident(ty)
        .is_some_and(|name| reachability.items.iter().any(|key| key.item_name == name))
}

fn simple_type_ident(ty: &Type) -> Option<String> {
    match ty {
        Type::Path(path) => path
            .path
            .segments
            .last()
            .map(|segment| segment.ident.to_string()),
        Type::Reference(reference) => simple_type_ident(&reference.elem),
        Type::Paren(paren) => simple_type_ident(&paren.elem),
        Type::Group(group) => simple_type_ident(&group.elem),
        _ => None,
    }
}
