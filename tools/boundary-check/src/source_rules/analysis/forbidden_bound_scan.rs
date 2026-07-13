//! Scan signatures, types, fields, and generics for sealed authority trait bounds.

use super::authority_sealing_surface::SurfaceHit;
use super::forbidden_aliases::path_is_forbidden;
use std::collections::BTreeSet;
use syn::visit::Visit;
use syn::{
    Field, Fields, FnArg, GenericParam, Generics, Path, Signature, TraitBound, Type,
    TypeParamBound, WherePredicate,
};

pub(super) fn find_forbidden_in_signature(
    sig: &Signature,
    aliases: &BTreeSet<String>,
) -> Option<SurfaceHit> {
    if let Some(hit) = find_forbidden_in_generics(&sig.generics, aliases) {
        return Some(hit);
    }
    for input in &sig.inputs {
        if let Some(hit) = find_forbidden_in_fn_arg(input, aliases) {
            return Some(hit);
        }
    }
    if let syn::ReturnType::Type(_, ty) = &sig.output {
        return find_forbidden_in_type(ty, aliases);
    }
    None
}

fn find_forbidden_in_fn_arg(input: &FnArg, aliases: &BTreeSet<String>) -> Option<SurfaceHit> {
    match input {
        FnArg::Receiver(_) => None,
        FnArg::Typed(pat_type) => find_forbidden_in_type(&pat_type.ty, aliases),
    }
}

pub(super) fn find_forbidden_in_fields(
    fields: &Fields,
    aliases: &BTreeSet<String>,
) -> Option<SurfaceHit> {
    match fields {
        Fields::Named(named) => {
            for field in &named.named {
                if let Some(hit) = find_forbidden_in_field(field, aliases) {
                    return Some(hit);
                }
            }
        }
        Fields::Unnamed(unnamed) => {
            for field in &unnamed.unnamed {
                if let Some(hit) = find_forbidden_in_field(field, aliases) {
                    return Some(hit);
                }
            }
        }
        Fields::Unit => {}
    }
    None
}

pub(super) fn find_forbidden_in_enum_variant_fields(
    fields: &Fields,
    aliases: &BTreeSet<String>,
) -> Option<SurfaceHit> {
    match fields {
        Fields::Named(named) => {
            for field in &named.named {
                if let Some(hit) = find_forbidden_in_type(&field.ty, aliases) {
                    return Some(hit);
                }
            }
        }
        Fields::Unnamed(unnamed) => {
            for field in &unnamed.unnamed {
                if let Some(hit) = find_forbidden_in_type(&field.ty, aliases) {
                    return Some(hit);
                }
            }
        }
        Fields::Unit => {}
    }
    None
}

fn find_forbidden_in_field(field: &Field, aliases: &BTreeSet<String>) -> Option<SurfaceHit> {
    // Only public fields are externally reachable carriers on a public struct/union.
    if !matches!(field.vis, syn::Visibility::Public(_)) {
        // Tuple-struct fields inherit visibility; treat as public when parent is.
        if field.ident.is_some() {
            return None;
        }
    }
    find_forbidden_in_type(&field.ty, aliases)
}

pub(super) fn find_forbidden_in_generics(
    generics: &Generics,
    aliases: &BTreeSet<String>,
) -> Option<SurfaceHit> {
    for param in &generics.params {
        if let GenericParam::Type(type_param) = param {
            for bound in &type_param.bounds {
                if let Some(hit) = type_param_bound_hit(bound, aliases) {
                    return Some(hit);
                }
            }
        }
    }
    if let Some(where_clause) = &generics.where_clause {
        for predicate in &where_clause.predicates {
            if let WherePredicate::Type(pred) = predicate {
                for bound in &pred.bounds {
                    if let Some(hit) = type_param_bound_hit(bound, aliases) {
                        return Some(hit);
                    }
                }
            }
        }
    }
    None
}

pub(super) fn find_forbidden_in_type(ty: &Type, aliases: &BTreeSet<String>) -> Option<SurfaceHit> {
    let mut visitor = ForbiddenBoundVisitor { aliases, hit: None };
    visitor.visit_type(ty);
    visitor.hit
}

pub(super) fn type_param_bound_hit(
    bound: &TypeParamBound,
    aliases: &BTreeSet<String>,
) -> Option<SurfaceHit> {
    match bound {
        TypeParamBound::Trait(trait_bound) => trait_bound_path_hit(&trait_bound.path, aliases),
        _ => None,
    }
}

fn trait_bound_path_hit(path: &Path, aliases: &BTreeSet<String>) -> Option<SurfaceHit> {
    // Definition-resolved: final segment, full path (`dep::Gate`), and crate-root
    // projected aliases are all checked via path_is_forbidden.
    if path_is_forbidden(path, aliases) {
        let spelling = path
            .segments
            .iter()
            .map(|s| s.ident.to_string())
            .collect::<Vec<_>>()
            .join("::");
        return Some(SurfaceHit::ForbiddenBound {
            trait_spelling: spelling,
        });
    }
    None
}

struct ForbiddenBoundVisitor<'a> {
    aliases: &'a BTreeSet<String>,
    hit: Option<SurfaceHit>,
}

impl<'a, 'ast> Visit<'ast> for ForbiddenBoundVisitor<'a> {
    fn visit_type_param_bound(&mut self, bound: &'ast TypeParamBound) {
        if self.hit.is_some() {
            return;
        }
        if let Some(hit) = type_param_bound_hit(bound, self.aliases) {
            self.hit = Some(hit);
            return;
        }
        syn::visit::visit_type_param_bound(self, bound);
    }

    fn visit_trait_bound(&mut self, bound: &'ast TraitBound) {
        if self.hit.is_some() {
            return;
        }
        if let Some(hit) = trait_bound_path_hit(&bound.path, self.aliases) {
            self.hit = Some(hit);
            return;
        }
        syn::visit::visit_trait_bound(self, bound);
    }

    fn visit_path(&mut self, path: &'ast Path) {
        if self.hit.is_some() {
            return;
        }
        if let Some(hit) = trait_bound_path_hit(path, self.aliases) {
            self.hit = Some(hit);
            return;
        }
        syn::visit::visit_path(self, path);
    }
}
