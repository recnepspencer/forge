//! Detect blanket impls that launder sealed authority into a public gate trait.

use super::authority_sealing_surface::FORBIDDEN_TRAITS;
use super::forbidden_aliases::path_is_forbidden;
use std::collections::BTreeSet;
use syn::{GenericArgument, GenericParam, PathArguments, Type, TypeParamBound, WherePredicate};

/// If this impl makes a gate trait reachable from sealed authority, return the gate name.
pub(super) fn blanket_impl_launders_forbidden(
    item_impl: &syn::ItemImpl,
    available: &BTreeSet<String>,
) -> Option<String> {
    let (_, trait_path, _) = item_impl.trait_.as_ref()?;
    let gate_name = trait_path.segments.last()?.ident.to_string();
    if FORBIDDEN_TRAITS.contains(&gate_name.as_str()) {
        return None;
    }
    let forbidden_params = forbidden_bounded_type_params(&item_impl.generics, available);
    if forbidden_params.is_empty() {
        if where_forbids_self_shape(&item_impl.generics, &item_impl.self_ty, available) {
            return Some(gate_name);
        }
        return None;
    }
    for param in &forbidden_params {
        if type_mentions_param(&item_impl.self_ty, param) {
            return Some(gate_name);
        }
    }
    if self_type_is_opaque(&item_impl.self_ty) {
        return Some(gate_name);
    }
    None
}

fn bound_is_forbidden(bound: &TypeParamBound, available: &BTreeSet<String>) -> bool {
    let TypeParamBound::Trait(trait_bound) = bound else {
        return false;
    };
    path_is_forbidden(&trait_bound.path, available)
}

/// Params that carry forbidden dependence (inline, bare where, or non-bare
/// `where Wrapper<T>: AuthorityMarker` / associated projections).
fn forbidden_bounded_type_params(
    generics: &syn::Generics,
    available: &BTreeSet<String>,
) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    for param in &generics.params {
        let GenericParam::Type(type_param) = param else {
            continue;
        };
        let name = type_param.ident.to_string();
        let inline = type_param
            .bounds
            .iter()
            .any(|b| bound_is_forbidden(b, available));
        if inline || type_param_carries_forbidden_where(generics, &name, available) {
            out.insert(name);
        }
    }
    out
}

fn type_param_carries_forbidden_where(
    generics: &syn::Generics,
    param_name: &str,
    available: &BTreeSet<String>,
) -> bool {
    let Some(where_clause) = &generics.where_clause else {
        return false;
    };
    for predicate in &where_clause.predicates {
        let WherePredicate::Type(pred) = predicate else {
            continue;
        };
        let bound_forbidden = pred.bounds.iter().any(|b| bound_is_forbidden(b, available));
        if !bound_forbidden {
            continue;
        }
        if type_is_bare_param(&pred.bounded_ty, param_name)
            || type_mentions_param(&pred.bounded_ty, param_name)
        {
            return true;
        }
    }
    false
}

fn where_forbids_self_shape(
    generics: &syn::Generics,
    self_ty: &Type,
    available: &BTreeSet<String>,
) -> bool {
    let Some(where_clause) = &generics.where_clause else {
        return false;
    };
    for predicate in &where_clause.predicates {
        let WherePredicate::Type(pred) = predicate else {
            continue;
        };
        if !pred.bounds.iter().any(|b| bound_is_forbidden(b, available)) {
            continue;
        }
        if types_structurally_equal(&pred.bounded_ty, self_ty) {
            return true;
        }
    }
    false
}

fn types_structurally_equal(a: &Type, b: &Type) -> bool {
    match (a, b) {
        (Type::Path(pa), Type::Path(pb)) => path_last_ident(pa) == path_last_ident(pb),
        (Type::Reference(ra), Type::Reference(rb)) => types_structurally_equal(&ra.elem, &rb.elem),
        (Type::Tuple(ta), Type::Tuple(tb)) if ta.elems.len() == tb.elems.len() => ta
            .elems
            .iter()
            .zip(tb.elems.iter())
            .all(|(x, y)| types_structurally_equal(x, y)),
        _ => false,
    }
}

fn path_last_ident(type_path: &syn::TypePath) -> Option<String> {
    type_path.path.segments.last().map(|s| s.ident.to_string())
}

fn type_is_bare_param(ty: &Type, param_name: &str) -> bool {
    match ty {
        Type::Path(type_path)
            if type_path.qself.is_none() && type_path.path.segments.len() == 1 =>
        {
            let seg = &type_path.path.segments[0];
            seg.ident == param_name && matches!(seg.arguments, PathArguments::None)
        }
        Type::Reference(reference) => type_is_bare_param(&reference.elem, param_name),
        Type::Paren(paren) => type_is_bare_param(&paren.elem, param_name),
        Type::Group(group) => type_is_bare_param(&group.elem, param_name),
        _ => false,
    }
}

fn type_mentions_param(ty: &Type, param_name: &str) -> bool {
    match ty {
        Type::Path(type_path) => {
            if let Some(qself) = &type_path.qself {
                if type_mentions_param(&qself.ty, param_name) {
                    return true;
                }
            }
            if type_path.qself.is_none() {
                if let Some(seg) = type_path.path.segments.last() {
                    if seg.ident == param_name && matches!(seg.arguments, PathArguments::None) {
                        return true;
                    }
                }
            }
            for seg in &type_path.path.segments {
                if let PathArguments::AngleBracketed(args) = &seg.arguments {
                    for arg in &args.args {
                        if let GenericArgument::Type(inner) = arg {
                            if type_mentions_param(inner, param_name) {
                                return true;
                            }
                        }
                    }
                }
            }
            false
        }
        Type::Tuple(tuple) => tuple
            .elems
            .iter()
            .any(|elem| type_mentions_param(elem, param_name)),
        Type::Array(array) => type_mentions_param(&array.elem, param_name),
        Type::Slice(slice) => type_mentions_param(&slice.elem, param_name),
        Type::Reference(reference) => type_mentions_param(&reference.elem, param_name),
        Type::Ptr(ptr) => type_mentions_param(&ptr.elem, param_name),
        Type::Paren(paren) => type_mentions_param(&paren.elem, param_name),
        Type::Group(group) => type_mentions_param(&group.elem, param_name),
        Type::BareFn(bare) => {
            bare.inputs
                .iter()
                .any(|input| type_mentions_param(&input.ty, param_name))
                || matches!(
                    &bare.output,
                    syn::ReturnType::Type(_, ty) if type_mentions_param(ty, param_name)
                )
        }
        Type::TraitObject(obj) => obj.bounds.iter().any(
            |b| matches!(b, TypeParamBound::Trait(t) if path_mentions_param(&t.path, param_name)),
        ),
        Type::ImplTrait(impl_trait) => impl_trait.bounds.iter().any(
            |b| matches!(b, TypeParamBound::Trait(t) if path_mentions_param(&t.path, param_name)),
        ),
        _ => false,
    }
}

fn path_mentions_param(path: &syn::Path, param_name: &str) -> bool {
    for seg in &path.segments {
        if let PathArguments::AngleBracketed(args) = &seg.arguments {
            for arg in &args.args {
                if let GenericArgument::Type(inner) = arg {
                    if type_mentions_param(inner, param_name) {
                        return true;
                    }
                }
            }
        }
    }
    false
}

fn self_type_is_opaque(ty: &Type) -> bool {
    matches!(
        ty,
        Type::ImplTrait(_)
            | Type::TraitObject(_)
            | Type::Infer(_)
            | Type::Macro(_)
            | Type::Verbatim(_)
    )
}
