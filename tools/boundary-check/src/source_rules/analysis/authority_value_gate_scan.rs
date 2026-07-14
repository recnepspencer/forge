//! Scan public surfaces for concrete ceremony-marker admissions.

use super::authority_value_gate::CeremonyAdmission;
use super::authority_value_identity::{carrier_argument, CarrierAliases};
use super::crate_modules::ModuleGraph;
use super::public_reachability::{
    item_name, module_is_public_chain, Reachability, ReachableItemKey,
};
use std::collections::BTreeSet;
use syn::visit::Visit;
use syn::{GenericArgument, GenericParam, Item, PathArguments, Type, TypePath, Visibility};

/// Ceremony carrier → index of the authority/capability type argument.
pub(super) fn collect_ceremony_admissions(
    graph: &ModuleGraph,
    reachability: &Reachability,
    worth_proof_idents: &BTreeSet<String>,
    carrier_aliases: &CarrierAliases,
) -> Vec<CeremonyAdmission> {
    let mut admissions = Vec::new();

    for key in &reachability.items {
        let Some(node) = graph.modules.get(&key.module_path) else {
            continue;
        };
        for item in &node.items {
            if item_name(item).as_deref() != Some(key.item_name.as_str()) {
                continue;
            }
            let generic_params = item_type_param_names(item);
            let mut visitor = CeremonyMarkerVisitor {
                generic_params: &generic_params,
                module_path: &key.module_path,
                worth_proof_idents,
                carrier_aliases,
                markers: Vec::new(),
            };
            visit_item_types(item, &mut visitor);
            for marker_type in visitor.markers {
                admissions.push(CeremonyAdmission {
                    key: key.clone(),
                    relative_source: node.relative_source.clone(),
                    marker_type,
                });
            }
        }
    }

    for (module_path, node) in &graph.modules {
        if !module_contributes(graph, module_path, reachability) {
            continue;
        }
        for item in &node.items {
            let Item::Impl(item_impl) = item else {
                continue;
            };
            if !self_type_reachable(&item_impl.self_ty, reachability) {
                continue;
            }
            let impl_generics = type_param_names_from_generics(&item_impl.generics);
            for impl_item in &item_impl.items {
                let syn::ImplItem::Fn(method) = impl_item else {
                    continue;
                };
                let method_public =
                    matches!(method.vis, Visibility::Public(_)) || item_impl.trait_.is_some();
                if !method_public {
                    continue;
                }
                let mut generic_params = impl_generics.clone();
                generic_params.extend(type_param_names_from_generics(&method.sig.generics));
                let mut visitor = CeremonyMarkerVisitor {
                    generic_params: &generic_params,
                    module_path,
                    worth_proof_idents,
                    carrier_aliases,
                    markers: Vec::new(),
                };
                visitor.visit_signature(&method.sig);
                for marker_type in visitor.markers {
                    admissions.push(CeremonyAdmission {
                        key: ReachableItemKey {
                            module_path: module_path.clone(),
                            item_name: method.sig.ident.to_string(),
                        },
                        relative_source: node.relative_source.clone(),
                        marker_type,
                    });
                }
            }
        }
    }

    admissions
}

fn module_contributes(
    graph: &ModuleGraph,
    module_path: &[String],
    reachability: &Reachability,
) -> bool {
    if module_path.is_empty() {
        return true;
    }
    if reachability.public_modules.contains(module_path) {
        return true;
    }
    if reachability
        .items
        .iter()
        .any(|k| k.module_path.starts_with(module_path) || k.module_path == module_path)
    {
        return true;
    }
    module_is_public_chain(graph, module_path)
}

fn visit_item_types(item: &Item, visitor: &mut CeremonyMarkerVisitor<'_>) {
    match item {
        Item::Fn(item_fn) => visitor.visit_signature(&item_fn.sig),
        Item::Struct(item_struct) => {
            for field in item_struct.fields.iter() {
                if matches!(field.vis, Visibility::Public(_)) || field.ident.is_none() {
                    visitor.visit_type(&field.ty);
                }
            }
        }
        Item::Enum(item_enum) => {
            for variant in &item_enum.variants {
                for field in variant.fields.iter() {
                    visitor.visit_type(&field.ty);
                }
            }
        }
        Item::Type(item_type) => visitor.visit_type(&item_type.ty),
        Item::Const(item_const) => visitor.visit_type(&item_const.ty),
        Item::Static(item_static) => visitor.visit_type(&item_static.ty),
        Item::Trait(item_trait) => {
            for trait_item in &item_trait.items {
                match trait_item {
                    syn::TraitItem::Fn(method) => visitor.visit_signature(&method.sig),
                    syn::TraitItem::Type(assoc) => {
                        if let Some((_, ty)) = &assoc.default {
                            visitor.visit_type(ty);
                        }
                    }
                    syn::TraitItem::Const(assoc) => visitor.visit_type(&assoc.ty),
                    _ => {}
                }
            }
        }
        Item::Union(item_union) => {
            for field in &item_union.fields.named {
                if matches!(field.vis, Visibility::Public(_)) {
                    visitor.visit_type(&field.ty);
                }
            }
        }
        _ => {}
    }
}

struct CeremonyMarkerVisitor<'a> {
    generic_params: &'a BTreeSet<String>,
    module_path: &'a [String],
    worth_proof_idents: &'a BTreeSet<String>,
    carrier_aliases: &'a CarrierAliases,
    markers: Vec<Type>,
}

impl<'a, 'ast> Visit<'ast> for CeremonyMarkerVisitor<'a> {
    fn visit_type_path(&mut self, type_path: &'ast TypePath) {
        if let Some(marker) = ceremony_marker_from_path(
            &type_path.path,
            self.generic_params,
            self.module_path,
            self.worth_proof_idents,
            self.carrier_aliases,
        ) {
            self.markers.push(marker);
        }
        syn::visit::visit_type_path(self, type_path);
    }
}

fn ceremony_marker_from_path(
    path: &syn::Path,
    generic_params: &BTreeSet<String>,
    module_path: &[String],
    worth_proof_idents: &BTreeSet<String>,
    aliases: &CarrierAliases,
) -> Option<Type> {
    let segment = path.segments.last()?;
    let arg_index = carrier_argument(path, module_path, worth_proof_idents, aliases)?;

    let PathArguments::AngleBracketed(args) = &segment.arguments else {
        return None;
    };
    let type_args: Vec<&Type> = args
        .args
        .iter()
        .filter_map(|arg| match arg {
            GenericArgument::Type(ty) => Some(ty),
            _ => None,
        })
        .collect();
    let marker_ty = type_args.get(arg_index).copied()?;
    concrete_type_name(marker_ty, generic_params).map(|_| marker_ty.clone())
}

pub(super) fn concrete_type_name(ty: &Type, generic_params: &BTreeSet<String>) -> Option<String> {
    match ty {
        Type::Path(type_path) if type_path.qself.is_none() => {
            let name = type_path.path.segments.last()?.ident.to_string();
            if generic_params.contains(&name) {
                return None;
            }
            Some(name)
        }
        Type::Reference(reference) => concrete_type_name(&reference.elem, generic_params),
        Type::Paren(paren) => concrete_type_name(&paren.elem, generic_params),
        Type::Group(group) => concrete_type_name(&group.elem, generic_params),
        _ => None,
    }
}

fn item_type_param_names(item: &Item) -> BTreeSet<String> {
    match item {
        Item::Fn(item_fn) => type_param_names_from_generics(&item_fn.sig.generics),
        Item::Struct(s) => type_param_names_from_generics(&s.generics),
        Item::Enum(e) => type_param_names_from_generics(&e.generics),
        Item::Type(t) => type_param_names_from_generics(&t.generics),
        Item::Trait(t) => type_param_names_from_generics(&t.generics),
        Item::Union(u) => type_param_names_from_generics(&u.generics),
        _ => BTreeSet::new(),
    }
}

fn type_param_names_from_generics(generics: &syn::Generics) -> BTreeSet<String> {
    generics
        .params
        .iter()
        .filter_map(|param| match param {
            GenericParam::Type(type_param) => Some(type_param.ident.to_string()),
            _ => None,
        })
        .collect()
}

fn self_type_reachable(self_ty: &Type, reachability: &Reachability) -> bool {
    match simple_type_ident(self_ty) {
        None => true,
        Some(name) => reachability.items.iter().any(|key| key.item_name == name),
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
