//! One reusable externally-callable surface closure for BC7001.
//!
//! Local crate-root reachability and external re-export seeds both reduce to a
//! `Reachability` inventory; this module is the single inspection walk that seals
//! declaration, impl, trait-member, foreign-item, macro, attribute, and
//! extern-crate surfaces.

use super::authority_sealing_surface::{
    inspect_foreign_mod, inspect_reachable_impl, inspect_reachable_item, public_foreign_item_attrs,
    SurfaceHit,
};
use super::crate_modules::ModuleGraph;
use super::forbidden_aliases::cumulative_aliases;
use super::opaque_attributes::first_opaque_attribute;
use super::public_reachability::{
    item_name, module_is_public_chain, Reachability, ReachableItemKey,
};
use syn::{Attribute, Item, Visibility};

/// One sealing violation on a named externally-callable surface.
#[derive(Clone, Debug)]
pub(super) struct SurfaceViolation {
    pub(super) key: ReachableItemKey,
    pub(super) relative_source: String,
    pub(super) hit: SurfaceHit,
}

/// Exhaustive seal of every signature-producing position under `reachability`.
pub(super) fn collect_surface_violations(
    graph: &ModuleGraph,
    reachability: &Reachability,
) -> Vec<SurfaceViolation> {
    let mut violations = Vec::new();

    for key in &reachability.items {
        let Some(node) = graph.modules.get(&key.module_path) else {
            continue;
        };
        let aliases = cumulative_aliases(&reachability.forbidden_aliases, &key.module_path);
        for item in &node.items {
            if item_name(item).as_deref() != Some(key.item_name.as_str()) {
                continue;
            }
            if let Some(hit) = inspect_reachable_item(item, &aliases) {
                violations.push(SurfaceViolation {
                    key: key.clone(),
                    relative_source: node.relative_source.clone(),
                    hit,
                });
            }
            // Opaque attrs on declaration-site public items.
            if let Some(attr_path) = first_opaque_attribute(item_attrs(item)) {
                violations.push(SurfaceViolation {
                    key: key.clone(),
                    relative_source: node.relative_source.clone(),
                    hit: SurfaceHit::OpaqueMacroExpansion {
                        macro_path: attr_path,
                    },
                });
            }
            // Trait members: opaque attrs + macro members.
            if let Item::Trait(item_trait) = item {
                violations.extend(trait_member_surface_hits(
                    key,
                    &node.relative_source,
                    item_trait,
                    &aliases,
                ));
            }
        }
    }

    // Impl blocks, foreign modules, item macros, and pub extern crate live
    // outside the named declaration-item set but still produce callable surface.
    for (module_path, node) in &graph.modules {
        if !module_contributes_callable_surface(graph, module_path, reachability) {
            continue;
        }
        let aliases = cumulative_aliases(&reachability.forbidden_aliases, module_path);
        for item in &node.items {
            match item {
                Item::ExternCrate(extern_crate)
                    if matches!(extern_crate.vis, Visibility::Public(_)) =>
                {
                    let export_name = extern_crate
                        .rename
                        .as_ref()
                        .map(|(_, ident)| ident.to_string())
                        .unwrap_or_else(|| extern_crate.ident.to_string());
                    violations.push(SurfaceViolation {
                        key: ReachableItemKey {
                            module_path: module_path.clone(),
                            item_name: format!("extern_crate:{export_name}"),
                        },
                        relative_source: node.relative_source.clone(),
                        hit: SurfaceHit::PublicExternCrate {
                            crate_ident: extern_crate.ident.to_string(),
                        },
                    });
                }
                Item::ForeignMod(foreign) => {
                    for (item_name, hit) in inspect_foreign_mod(foreign, &aliases) {
                        violations.push(SurfaceViolation {
                            key: ReachableItemKey {
                                module_path: module_path.clone(),
                                item_name: format!("foreign:{item_name}"),
                            },
                            relative_source: node.relative_source.clone(),
                            hit,
                        });
                    }
                    for foreign_item in &foreign.items {
                        if let Some((name, attrs)) = public_foreign_item_attrs(foreign_item) {
                            if let Some(attr_path) = first_opaque_attribute(attrs) {
                                violations.push(SurfaceViolation {
                                    key: ReachableItemKey {
                                        module_path: module_path.clone(),
                                        item_name: format!("foreign:{name}"),
                                    },
                                    relative_source: node.relative_source.clone(),
                                    hit: SurfaceHit::OpaqueMacroExpansion {
                                        macro_path: attr_path,
                                    },
                                });
                            }
                        }
                    }
                }
                Item::Macro(item_macro) if item_macro.ident.is_none() => {
                    if let Some(hit) = inspect_reachable_item(item, &aliases) {
                        violations.push(SurfaceViolation {
                            key: ReachableItemKey {
                                module_path: module_path.clone(),
                                item_name: format!("macro:{}", macro_path_name(item)),
                            },
                            relative_source: node.relative_source.clone(),
                            hit,
                        });
                    }
                }
                Item::Impl(item_impl) => {
                    // Opaque attribute on a reachable impl is a ceremony factory site.
                    if self_type_reachable(&item_impl.self_ty, reachability) {
                        if let Some(attr_path) = first_opaque_attribute(&item_impl.attrs) {
                            violations.push(SurfaceViolation {
                                key: ReachableItemKey {
                                    module_path: module_path.clone(),
                                    item_name: "impl".to_owned(),
                                },
                                relative_source: node.relative_source.clone(),
                                hit: SurfaceHit::OpaqueMacroExpansion {
                                    macro_path: attr_path,
                                },
                            });
                        }
                        // Opaque attrs on public methods / associated items.
                        for impl_item in &item_impl.items {
                            if let Some((name, attrs)) =
                                public_impl_item_attrs(impl_item, item_impl)
                            {
                                if let Some(attr_path) = first_opaque_attribute(attrs) {
                                    violations.push(SurfaceViolation {
                                        key: ReachableItemKey {
                                            module_path: module_path.clone(),
                                            item_name: name,
                                        },
                                        relative_source: node.relative_source.clone(),
                                        hit: SurfaceHit::OpaqueMacroExpansion {
                                            macro_path: attr_path,
                                        },
                                    });
                                }
                            }
                        }
                    }
                    for (item_name, hit) in
                        inspect_reachable_impl(item_impl, &aliases, reachability)
                    {
                        violations.push(SurfaceViolation {
                            key: ReachableItemKey {
                                module_path: module_path.clone(),
                                item_name,
                            },
                            relative_source: node.relative_source.clone(),
                            hit,
                        });
                    }
                }
                _ => {}
            }
        }
    }

    violations.sort_by(|a, b| {
        (
            &a.relative_source,
            &a.key.module_path,
            &a.key.item_name,
            format!("{:?}", a.hit),
        )
            .cmp(&(
                &b.relative_source,
                &b.key.module_path,
                &b.key.item_name,
                format!("{:?}", b.hit),
            ))
    });
    violations.dedup_by(|a, b| {
        a.key == b.key
            && a.relative_source == b.relative_source
            && format!("{:?}", a.hit) == format!("{:?}", b.hit)
    });
    violations
}

fn trait_member_surface_hits(
    trait_key: &ReachableItemKey,
    relative_source: &str,
    item_trait: &syn::ItemTrait,
    aliases: &std::collections::BTreeSet<String>,
) -> Vec<SurfaceViolation> {
    let mut violations = Vec::new();
    for trait_item in &item_trait.items {
        match trait_item {
            syn::TraitItem::Fn(method) => {
                if let Some(attr_path) = first_opaque_attribute(&method.attrs) {
                    violations.push(SurfaceViolation {
                        key: ReachableItemKey {
                            module_path: trait_key.module_path.clone(),
                            item_name: format!("{}::{}", trait_key.item_name, method.sig.ident),
                        },
                        relative_source: relative_source.to_owned(),
                        hit: SurfaceHit::OpaqueMacroExpansion {
                            macro_path: attr_path,
                        },
                    });
                }
            }
            syn::TraitItem::Type(assoc) => {
                if let Some(attr_path) = first_opaque_attribute(&assoc.attrs) {
                    violations.push(SurfaceViolation {
                        key: ReachableItemKey {
                            module_path: trait_key.module_path.clone(),
                            item_name: format!("{}::{}", trait_key.item_name, assoc.ident),
                        },
                        relative_source: relative_source.to_owned(),
                        hit: SurfaceHit::OpaqueMacroExpansion {
                            macro_path: attr_path,
                        },
                    });
                }
            }
            syn::TraitItem::Const(assoc) => {
                if let Some(attr_path) = first_opaque_attribute(&assoc.attrs) {
                    violations.push(SurfaceViolation {
                        key: ReachableItemKey {
                            module_path: trait_key.module_path.clone(),
                            item_name: format!("{}::{}", trait_key.item_name, assoc.ident),
                        },
                        relative_source: relative_source.to_owned(),
                        hit: SurfaceHit::OpaqueMacroExpansion {
                            macro_path: attr_path,
                        },
                    });
                }
            }
            syn::TraitItem::Macro(mac) => {
                let _ = aliases;
                violations.push(SurfaceViolation {
                    key: ReachableItemKey {
                        module_path: trait_key.module_path.clone(),
                        item_name: format!(
                            "{}::macro:{}",
                            trait_key.item_name,
                            path_display(&mac.mac.path)
                        ),
                    },
                    relative_source: relative_source.to_owned(),
                    hit: SurfaceHit::OpaqueMacroExpansion {
                        macro_path: path_display(&mac.mac.path),
                    },
                });
            }
            _ => {}
        }
    }
    violations
}

fn public_impl_item_attrs<'a>(
    impl_item: &'a syn::ImplItem,
    item_impl: &syn::ItemImpl,
) -> Option<(String, &'a [Attribute])> {
    match impl_item {
        syn::ImplItem::Fn(method) => {
            let method_public =
                matches!(method.vis, Visibility::Public(_)) || item_impl.trait_.is_some();
            if !method_public {
                return None;
            }
            Some((method.sig.ident.to_string(), method.attrs.as_slice()))
        }
        syn::ImplItem::Type(assoc) => {
            let public = matches!(assoc.vis, Visibility::Public(_)) || item_impl.trait_.is_some();
            if !public {
                return None;
            }
            Some((assoc.ident.to_string(), assoc.attrs.as_slice()))
        }
        syn::ImplItem::Const(assoc) => {
            let public = matches!(assoc.vis, Visibility::Public(_)) || item_impl.trait_.is_some();
            if !public {
                return None;
            }
            Some((assoc.ident.to_string(), assoc.attrs.as_slice()))
        }
        _ => None,
    }
}

fn self_type_reachable(self_ty: &syn::Type, reachability: &Reachability) -> bool {
    match simple_type_ident(self_ty) {
        None => true,
        Some(name) => reachability.items.iter().any(|key| key.item_name == name),
    }
}

fn simple_type_ident(ty: &syn::Type) -> Option<String> {
    match ty {
        syn::Type::Path(type_path) => type_path
            .path
            .segments
            .last()
            .map(|segment| segment.ident.to_string()),
        syn::Type::Reference(reference) => simple_type_ident(&reference.elem),
        syn::Type::Paren(paren) => simple_type_ident(&paren.elem),
        syn::Type::Group(group) => simple_type_ident(&group.elem),
        _ => None,
    }
}

fn module_contributes_callable_surface(
    graph: &ModuleGraph,
    path: &[String],
    reachability: &Reachability,
) -> bool {
    if path.is_empty() {
        return true;
    }
    if reachability
        .items
        .iter()
        .any(|key| key.module_path.starts_with(path) || key.module_path == path)
    {
        return true;
    }
    // Seeded public modules (external re-export of a module) may have no named
    // items yet when only their inherent impls are under inspection; honor the
    // public chain when the module itself is a reachability seed.
    if reachability.public_modules.contains(path) {
        return true;
    }
    module_is_public_chain(graph, path)
}

fn item_attrs(item: &Item) -> &[Attribute] {
    match item {
        Item::Const(i) => &i.attrs,
        Item::Enum(i) => &i.attrs,
        Item::Fn(i) => &i.attrs,
        Item::Static(i) => &i.attrs,
        Item::Struct(i) => &i.attrs,
        Item::Trait(i) => &i.attrs,
        Item::TraitAlias(i) => &i.attrs,
        Item::Type(i) => &i.attrs,
        Item::Union(i) => &i.attrs,
        _ => &[],
    }
}

fn macro_path_name(item: &Item) -> String {
    match item {
        Item::Macro(item_macro) => path_display(&item_macro.mac.path),
        _ => "unknown".to_owned(),
    }
}

fn path_display(path: &syn::Path) -> String {
    path.segments
        .iter()
        .map(|segment| segment.ident.to_string())
        .collect::<Vec<_>>()
        .join("::")
}
