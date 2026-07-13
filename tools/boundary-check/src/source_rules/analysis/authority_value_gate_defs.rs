//! Type-definition mintability facts for value-gated ceremony markers.

use super::authority_value_identity::local_type_key;
use super::crate_modules::ModuleGraph;
use super::public_reachability::{item_name, Reachability};
use std::collections::{BTreeMap, BTreeSet};
use syn::{Fields, Item, ReturnType, Type, Visibility};

pub(super) type DefinitionKey = (Vec<String>, String);

pub(super) struct TypeDef {
    pub(super) is_public: bool,
    pub(super) kind: TypeDefKind,
}

pub(super) enum TypeDefKind {
    Struct { constructible: bool },
    Enum { constructible: bool },
    Alias { target: Option<DefinitionKey> },
    Union { constructible: bool },
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
                            },
                        },
                    );
                }
                Item::Enum(item_enum) => {
                    let constructible = item_enum
                        .variants
                        .iter()
                        .any(|variant| fields_are_caller_constructible(&variant.fields));
                    map.insert(
                        (module_path.clone(), item_enum.ident.to_string()),
                        TypeDef {
                            is_public: matches!(item_enum.vis, Visibility::Public(_)),
                            kind: TypeDefKind::Enum { constructible },
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
                            kind: TypeDefKind::Union { constructible },
                        },
                    );
                }
                Item::Type(item_type) => {
                    let target = local_type_key(graph, module_path, &item_type.ty);
                    map.insert(
                        (module_path.clone(), item_type.ident.to_string()),
                        TypeDef {
                            is_public: matches!(item_type.vis, Visibility::Public(_)),
                            kind: TypeDefKind::Alias { target },
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
            unnamed.unnamed.iter().all(|field| {
                matches!(field.vis, Visibility::Public(_))
                    || matches!(field.vis, Visibility::Inherited)
            })
        }
    }
}

pub(super) fn collect_default_impls(graph: &ModuleGraph) -> BTreeSet<DefinitionKey> {
    let mut defaults = BTreeSet::new();
    for (module_path, node) in &graph.modules {
        for item in &node.items {
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
                Item::Const(i) => Some(i.ty.as_ref()),
                Item::Static(i) => Some(i.ty.as_ref()),
                _ => None,
            };
            if let Some(marker) = ty.and_then(|ty| local_type_key(graph, &key.module_path, ty)) {
                values.insert(marker);
            }
        }
    }
    for (module_path, node) in &graph.modules {
        for item in &node.items {
            let Item::Impl(item_impl) = item else {
                continue;
            };
            if !self_type_reachable(&item_impl.self_ty, reachability) {
                continue;
            }
            for member in &item_impl.items {
                let syn::ImplItem::Const(assoc) = member else {
                    continue;
                };
                let public =
                    matches!(assoc.vis, Visibility::Public(_)) || item_impl.trait_.is_some();
                if !public {
                    continue;
                }
                if let Some(marker) = local_type_key(graph, module_path, &assoc.ty) {
                    values.insert(marker);
                }
            }
        }
    }
    values
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
                    if let Some(marker) = local_type_key(graph, &key.module_path, ty) {
                        factories.insert(marker);
                    }
                }
            }
        }
    }
    for (module_path, node) in &graph.modules {
        for item in &node.items {
            let Item::Impl(item_impl) = item else {
                continue;
            };
            if item_impl.trait_.is_none() || !self_type_reachable(&item_impl.self_ty, reachability)
            {
                continue;
            }
            for member in &item_impl.items {
                let syn::ImplItem::Fn(method) = member else {
                    continue;
                };
                if let ReturnType::Type(_, ty) = &method.sig.output {
                    if let Some(marker) = local_type_key(graph, module_path, ty) {
                        factories.insert(marker);
                    }
                }
            }
        }
    }
    factories
}

fn self_type_reachable(ty: &Type, reachability: &Reachability) -> bool {
    simple_type_ident(ty)
        .is_some_and(|name| reachability.items.iter().any(|key| key.item_name == name))
}

pub(super) fn collect_public_self_constructors(graph: &ModuleGraph) -> BTreeSet<DefinitionKey> {
    let mut constructors = BTreeSet::new();
    for (module_path, node) in &graph.modules {
        for item in &node.items {
            let Item::Impl(item_impl) = item else {
                continue;
            };
            if item_impl.trait_.is_some() {
                continue;
            }
            let Some(self_name) = simple_type_ident(&item_impl.self_ty) else {
                continue;
            };
            let Some(self_key) = local_type_key(graph, module_path, &item_impl.self_ty) else {
                continue;
            };
            for impl_item in &item_impl.items {
                let syn::ImplItem::Fn(method) = impl_item else {
                    continue;
                };
                if !matches!(method.vis, Visibility::Public(_)) {
                    continue;
                }
                if signature_returns_self(&method.sig.output, &self_name) {
                    constructors.insert(self_key.clone());
                    break;
                }
            }
        }
    }
    constructors
}

fn signature_returns_self(output: &ReturnType, self_name: &str) -> bool {
    match output {
        ReturnType::Default => false,
        ReturnType::Type(_, ty) => match ty.as_ref() {
            Type::Path(type_path) => {
                let last = type_path
                    .path
                    .segments
                    .last()
                    .map(|s| s.ident.to_string())
                    .unwrap_or_default();
                last == "Self" || last == self_name
            }
            _ => false,
        },
    }
}

pub(super) fn mintability_reason_for(
    marker_key: &DefinitionKey,
    def: &TypeDef,
    defaults: &BTreeSet<DefinitionKey>,
    constructors: &BTreeSet<DefinitionKey>,
    public_values: &BTreeSet<DefinitionKey>,
    trait_factories: &BTreeSet<DefinitionKey>,
    definitions: &BTreeMap<DefinitionKey, TypeDef>,
) -> Option<String> {
    if let TypeDefKind::Alias {
        target: Some(target),
    } = &def.kind
    {
        if let Some(underlying) = definitions.get(target) {
            if !underlying.is_public {
                return None;
            }
            return mintability_reason_for(
                target,
                underlying,
                defaults,
                constructors,
                public_values,
                trait_factories,
                definitions,
            );
        }
        return Some("unresolved_alias_target".to_owned());
    }

    if defaults.contains(marker_key) {
        return Some("public_default".to_owned());
    }
    if constructors.contains(marker_key) {
        return Some("public_constructor".to_owned());
    }
    if public_values.contains(marker_key) {
        return Some("public_const_or_static".to_owned());
    }
    if trait_factories.contains(marker_key) {
        return Some("public_trait_factory".to_owned());
    }

    match &def.kind {
        TypeDefKind::Struct { constructible } if *constructible => {
            Some("public_unit_or_fields".to_owned())
        }
        TypeDefKind::Enum { constructible } if *constructible => {
            Some("public_enum_variant".to_owned())
        }
        TypeDefKind::Union { constructible } if *constructible => {
            Some("public_union_field".to_owned())
        }
        TypeDefKind::Alias { target: None } => Some("opaque_type_alias".to_owned()),
        _ => None,
    }
}

fn simple_type_ident(ty: &Type) -> Option<String> {
    match ty {
        Type::Path(type_path) => type_path
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
