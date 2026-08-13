//! Type-definition mintability facts for value-gated ceremony markers.

use super::authority_value_gate_returns::return_type_keys;
use super::authority_value_identity::local_type_key;
use super::crate_modules::ModuleGraph;
use std::collections::{BTreeMap, BTreeSet};
use syn::{Fields, Item, Visibility};

pub(super) use super::authority_value_gate_producers::{
    collect_public_self_constructors, collect_public_values, collect_trait_factories,
};

pub(super) type DefinitionKey = (Vec<String>, String);

pub(super) struct TypeDef {
    pub(super) is_public: bool,
    pub(super) kind: TypeDefKind,
}

pub(super) enum TypeDefKind {
    Struct {
        constructible: bool,
        owned_payloads: BTreeSet<DefinitionKey>,
    },
    Enum {
        constructible: bool,
        owned_payloads: BTreeSet<DefinitionKey>,
    },
    Alias {
        targets: BTreeSet<DefinitionKey>,
    },
    Union {
        constructible: bool,
        owned_payloads: BTreeSet<DefinitionKey>,
    },
}

pub(super) fn index_type_definitions(graph: &ModuleGraph) -> BTreeMap<DefinitionKey, TypeDef> {
    let mut map = BTreeMap::new();
    for (module_path, node) in &graph.modules {
        for item in &node.items {
            match item {
                Item::Struct(item_struct) => {
                    map.insert(
                        (module_path.clone(), item_struct.ident.to_string()),
                        TypeDef {
                            is_public: matches!(item_struct.vis, Visibility::Public(_)),
                            kind: TypeDefKind::Struct {
                                constructible: fields_are_caller_constructible(&item_struct.fields),
                                owned_payloads:
                                    super::authority_value_gate_payloads::public_field_payloads(
                                        graph,
                                        module_path,
                                        &item_struct.fields,
                                    ),
                            },
                        },
                    );
                }
                Item::Enum(item_enum) => {
                    let constructible = item_enum
                        .variants
                        .iter()
                        .any(|variant| enum_variant_is_caller_constructible(&variant.fields));
                    map.insert(
                        (module_path.clone(), item_enum.ident.to_string()),
                        TypeDef {
                            is_public: matches!(item_enum.vis, Visibility::Public(_)),
                            kind: TypeDefKind::Enum {
                                constructible,
                                owned_payloads: super::authority_value_gate_payloads::enum_payloads(
                                    graph,
                                    module_path,
                                    item_enum,
                                ),
                            },
                        },
                    );
                }
                Item::Union(item_union) => {
                    let constructible = item_union
                        .fields
                        .named
                        .iter()
                        .any(|field| matches!(field.vis, Visibility::Public(_)));
                    map.insert(
                        (module_path.clone(), item_union.ident.to_string()),
                        TypeDef {
                            is_public: matches!(item_union.vis, Visibility::Public(_)),
                            kind: TypeDefKind::Union {
                                constructible,
                                owned_payloads:
                                    super::authority_value_gate_payloads::public_named_payloads(
                                        graph,
                                        module_path,
                                        &item_union.fields.named,
                                    ),
                            },
                        },
                    );
                }
                Item::Type(item_type) => {
                    let targets = return_type_keys(graph, module_path, &item_type.ty, None);
                    map.insert(
                        (module_path.clone(), item_type.ident.to_string()),
                        TypeDef {
                            is_public: matches!(item_type.vis, Visibility::Public(_)),
                            kind: TypeDefKind::Alias { targets },
                        },
                    );
                }
                _ => {}
            }
        }
    }
    map
}

/// True when an external caller can construct a value of this fields shape.
fn fields_are_caller_constructible(fields: &Fields) -> bool {
    match fields {
        Fields::Unit => true,
        Fields::Named(named) => {
            if named.named.is_empty() {
                return true;
            }
            named
                .named
                .iter()
                .all(|field| matches!(field.vis, Visibility::Public(_)))
        }
        Fields::Unnamed(unnamed) => {
            if unnamed.unnamed.is_empty() {
                return true;
            }
            unnamed
                .unnamed
                .iter()
                .all(|field| matches!(field.vis, Visibility::Public(_)))
        }
    }
}

fn enum_variant_is_caller_constructible(fields: &Fields) -> bool {
    // Every variant of an inhabited public enum is nameable by a caller.
    // `Variant {}` and `Variant()` are constructible just like unit variants.
    match fields {
        Fields::Unit | Fields::Named(_) | Fields::Unnamed(_) => true,
    }
}

pub(super) fn collect_default_impls(graph: &ModuleGraph) -> BTreeSet<DefinitionKey> {
    let mut defaults = BTreeSet::new();
    for (module_path, node) in &graph.modules {
        for item in &node.items {
            let derived_default = match item {
                Item::Struct(item) => derives_default(&item.attrs)
                    .then(|| (module_path.clone(), item.ident.to_string())),
                Item::Enum(item) => derives_default(&item.attrs)
                    .then(|| (module_path.clone(), item.ident.to_string())),
                Item::Union(item) => derives_default(&item.attrs)
                    .then(|| (module_path.clone(), item.ident.to_string())),
                _ => None,
            };
            if let Some(key) = derived_default {
                defaults.insert(key);
            }
            let Item::Impl(item_impl) = item else {
                continue;
            };
            let Some((_, trait_path, _)) = &item_impl.trait_ else {
                continue;
            };
            let trait_name = trait_path
                .segments
                .last()
                .map(|s| s.ident.to_string())
                .unwrap_or_default();
            if trait_name != "Default" {
                continue;
            }
            if let Some(key) = local_type_key(graph, module_path, &item_impl.self_ty) {
                defaults.insert(key);
            }
        }
    }
    defaults
}

fn derives_default(attrs: &[syn::Attribute]) -> bool {
    attrs.iter().any(|attr| {
        attr.path().is_ident("derive")
            && attr
                .parse_args_with(
                    syn::punctuated::Punctuated::<syn::Path, syn::Token![,]>::parse_terminated,
                )
                .is_ok_and(|traits| {
                    traits.iter().any(|path| {
                        path.segments
                            .last()
                            .is_some_and(|segment| segment.ident == "Default")
                    })
                })
    })
}
