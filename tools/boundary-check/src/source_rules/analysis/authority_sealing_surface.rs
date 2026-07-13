//! Exhaustive walk of externally reachable signature-bearing surfaces.

use super::forbidden_bound_scan::{
    find_forbidden_in_enum_variant_fields, find_forbidden_in_fields, find_forbidden_in_generics,
    find_forbidden_in_signature, find_forbidden_in_type, type_param_bound_hit,
};
use super::public_reachability::Reachability;
use std::collections::BTreeSet;
use syn::{ForeignItem, Item, ItemForeignMod, ItemImpl, Path, TraitItem, Type, Visibility};

pub(super) const FORBIDDEN_TRAITS: &[&str] = &[
    "AuthorityMarker",
    "CapabilityMarker",
    "AuthorityProves",
    "ProofSetAuthorizedBy",
];

/// One sealed-surface violation found on a public carrier.
#[derive(Clone, Debug)]
pub(super) enum SurfaceHit {
    ForbiddenBound {
        trait_spelling: String,
    },
    OpaqueMacroExpansion {
        macro_path: String,
    },
    PublicExternCrate {
        crate_ident: String,
    },
    /// Concrete ceremony marker is caller-constructible (not value-gated).
    MintableAuthority {
        marker_type: String,
        reason: String,
    },
}

/// Inspect a reachable public item for sealing violations (definition site).
pub(super) fn inspect_reachable_item(
    item: &Item,
    aliases: &BTreeSet<String>,
) -> Option<SurfaceHit> {
    match item {
        Item::Fn(item_fn) => find_forbidden_in_signature(&item_fn.sig, aliases),
        Item::Trait(item_trait) => inspect_trait_item(item_trait, aliases),
        Item::Type(item_type) => find_forbidden_in_generics(&item_type.generics, aliases)
            .or_else(|| find_forbidden_in_type(&item_type.ty, aliases)),
        Item::Struct(item_struct) => find_forbidden_in_generics(&item_struct.generics, aliases)
            .or_else(|| find_forbidden_in_fields(&item_struct.fields, aliases)),
        Item::Enum(item_enum) => {
            if let Some(hit) = find_forbidden_in_generics(&item_enum.generics, aliases) {
                return Some(hit);
            }
            // Enum variant fields are ordinary public API of a public enum.
            for variant in &item_enum.variants {
                if let Some(hit) = find_forbidden_in_enum_variant_fields(&variant.fields, aliases) {
                    return Some(hit);
                }
            }
            None
        }
        Item::Union(item_union) => find_forbidden_in_generics(&item_union.generics, aliases)
            .or_else(|| {
                for field in &item_union.fields.named {
                    if matches!(field.vis, syn::Visibility::Public(_)) || field.ident.is_none() {
                        if let Some(hit) = find_forbidden_in_type(&field.ty, aliases) {
                            return Some(hit);
                        }
                    }
                }
                None
            }),
        Item::Const(item_const) => find_forbidden_in_type(&item_const.ty, aliases),
        Item::Static(item_static) => find_forbidden_in_type(&item_static.ty, aliases),
        Item::TraitAlias(item_alias) => {
            if let Some(hit) = find_forbidden_in_generics(&item_alias.generics, aliases) {
                return Some(hit);
            }
            for bound in &item_alias.bounds {
                if let Some(hit) = type_param_bound_hit(bound, aliases) {
                    return Some(hit);
                }
            }
            None
        }
        // Item-position macro: expansion not visible — fail closed.
        Item::Macro(item_macro) if item_macro.ident.is_none() => {
            Some(SurfaceHit::OpaqueMacroExpansion {
                macro_path: path_display(&item_macro.mac.path),
            })
        }
        _ => None,
    }
}

/// Inspect public foreign items (`extern "…" { … }`) in an externally reachable module.
///
/// Foreign-item ceremonies are ordinary public callable surface: a `pub fn` inside
/// an extern block can carry open `AuthorityMarker` bounds identically to a Rust
/// `pub fn`, and must be sealed by the same signature walk.
pub(super) fn inspect_foreign_mod(
    foreign: &ItemForeignMod,
    aliases: &BTreeSet<String>,
) -> Vec<(String, SurfaceHit)> {
    let mut hits = Vec::new();
    for foreign_item in &foreign.items {
        match foreign_item {
            ForeignItem::Fn(item_fn) if matches!(item_fn.vis, Visibility::Public(_)) => {
                if let Some(hit) = find_forbidden_in_signature(&item_fn.sig, aliases) {
                    hits.push((item_fn.sig.ident.to_string(), hit));
                }
            }
            ForeignItem::Static(item_static)
                if matches!(item_static.vis, Visibility::Public(_)) =>
            {
                if let Some(hit) = find_forbidden_in_type(&item_static.ty, aliases) {
                    hits.push((item_static.ident.to_string(), hit));
                }
            }
            ForeignItem::Type(item_type) if matches!(item_type.vis, Visibility::Public(_)) => {
                if let Some(hit) = find_forbidden_in_generics(&item_type.generics, aliases) {
                    hits.push((item_type.ident.to_string(), hit));
                }
            }
            // Macro expansion inside a foreign block can mint unsealed public API.
            ForeignItem::Macro(mac) => {
                hits.push((
                    format!("foreign_macro:{}", path_display(&mac.mac.path)),
                    SurfaceHit::OpaqueMacroExpansion {
                        macro_path: path_display(&mac.mac.path),
                    },
                ));
            }
            _ => {}
        }
    }
    hits
}

/// Public foreign-item attribute sites for opaque expansion fencing.
pub(super) fn public_foreign_item_attrs(
    foreign_item: &ForeignItem,
) -> Option<(String, &[syn::Attribute])> {
    match foreign_item {
        ForeignItem::Fn(item_fn) if matches!(item_fn.vis, Visibility::Public(_)) => {
            Some((item_fn.sig.ident.to_string(), item_fn.attrs.as_slice()))
        }
        ForeignItem::Static(item_static) if matches!(item_static.vis, Visibility::Public(_)) => {
            Some((item_static.ident.to_string(), item_static.attrs.as_slice()))
        }
        ForeignItem::Type(item_type) if matches!(item_type.vis, Visibility::Public(_)) => {
            Some((item_type.ident.to_string(), item_type.attrs.as_slice()))
        }
        _ => None,
    }
}

/// Inspect public methods on an impl when Self is externally reachable.
pub(super) fn inspect_reachable_impl(
    item_impl: &ItemImpl,
    aliases: &BTreeSet<String>,
    reachability: &Reachability,
) -> Vec<(String, SurfaceHit)> {
    if !self_type_is_externally_reachable(&item_impl.self_ty, reachability) {
        return Vec::new();
    }

    let mut hits = Vec::new();
    if let Some(hit) = find_forbidden_in_generics(&item_impl.generics, aliases) {
        hits.push(("impl".to_owned(), hit));
    }

    for impl_item in &item_impl.items {
        match impl_item {
            syn::ImplItem::Fn(method) => {
                let method_public =
                    matches!(method.vis, syn::Visibility::Public(_)) || item_impl.trait_.is_some();
                if !method_public {
                    continue;
                }
                if let Some(hit) = find_forbidden_in_signature(&method.sig, aliases) {
                    hits.push((method.sig.ident.to_string(), hit));
                }
            }
            syn::ImplItem::Type(assoc) => {
                let public =
                    matches!(assoc.vis, syn::Visibility::Public(_)) || item_impl.trait_.is_some();
                if !public {
                    continue;
                }
                if let Some(hit) = find_forbidden_in_generics(&assoc.generics, aliases)
                    .or_else(|| find_forbidden_in_type(&assoc.ty, aliases))
                {
                    hits.push((assoc.ident.to_string(), hit));
                }
            }
            syn::ImplItem::Const(assoc) => {
                let public =
                    matches!(assoc.vis, syn::Visibility::Public(_)) || item_impl.trait_.is_some();
                if !public {
                    continue;
                }
                if let Some(hit) = find_forbidden_in_type(&assoc.ty, aliases) {
                    hits.push((assoc.ident.to_string(), hit));
                }
            }
            // Macro inside a reachable impl expands to public ceremony API — fail closed.
            syn::ImplItem::Macro(mac) => {
                hits.push((
                    format!("macro:{}", path_display(&mac.mac.path)),
                    SurfaceHit::OpaqueMacroExpansion {
                        macro_path: path_display(&mac.mac.path),
                    },
                ));
            }
            _ => {}
        }
    }
    hits
}

fn inspect_trait_item(
    item_trait: &syn::ItemTrait,
    aliases: &BTreeSet<String>,
) -> Option<SurfaceHit> {
    if let Some(hit) = find_forbidden_in_generics(&item_trait.generics, aliases) {
        return Some(hit);
    }
    for bound in &item_trait.supertraits {
        if let Some(hit) = type_param_bound_hit(bound, aliases) {
            return Some(hit);
        }
    }
    for trait_item in &item_trait.items {
        if let Some(hit) = inspect_trait_member(trait_item, aliases) {
            return Some(hit);
        }
    }
    None
}

fn inspect_trait_member(trait_item: &TraitItem, aliases: &BTreeSet<String>) -> Option<SurfaceHit> {
    match trait_item {
        TraitItem::Fn(method) => find_forbidden_in_signature(&method.sig, aliases),
        TraitItem::Type(assoc) => {
            if let Some(hit) = find_forbidden_in_generics(&assoc.generics, aliases) {
                return Some(hit);
            }
            for bound in &assoc.bounds {
                if let Some(hit) = type_param_bound_hit(bound, aliases) {
                    return Some(hit);
                }
            }
            if let Some(default) = &assoc.default {
                return find_forbidden_in_type(&default.1, aliases);
            }
            None
        }
        TraitItem::Const(assoc) => find_forbidden_in_type(&assoc.ty, aliases),
        TraitItem::Macro(mac) => Some(SurfaceHit::OpaqueMacroExpansion {
            macro_path: path_display(&mac.mac.path),
        }),
        _ => None,
    }
}

fn self_type_is_externally_reachable(self_ty: &Type, reachability: &Reachability) -> bool {
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

fn path_display(path: &Path) -> String {
    path.segments
        .iter()
        .map(|segment| segment.ident.to_string())
        .collect::<Vec<_>>()
        .join("::")
}
