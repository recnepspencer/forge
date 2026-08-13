//! Local trait-bound applicability for generic associated projections.

use syn::{Item, PathArguments, Type};

use super::authority_value_gate_projection_identity::resolve_trait_key;
use super::authority_value_gate_projection_matching::{unify_impl_projection, TypeBindings};
use super::crate_modules::ModuleGraph;

pub(super) fn local_type_parameter_bounds_hold(
    graph: &ModuleGraph,
    impl_module: &[String],
    item_impl: &syn::ItemImpl,
    bindings: &TypeBindings,
) -> bool {
    let inline = item_impl.generics.type_params().all(|parameter| {
        let Some(actual) = bindings.get(&parameter.ident.to_string()) else {
            return true;
        };
        bounds_hold(graph, impl_module, &parameter.bounds, actual)
    });
    inline
        && item_impl
            .generics
            .where_clause
            .iter()
            .flat_map(|clause| &clause.predicates)
            .all(|predicate| {
                let syn::WherePredicate::Type(predicate) = predicate else {
                    return true;
                };
                let Type::Path(path) = &predicate.bounded_ty else {
                    return true;
                };
                if path.qself.is_some() || path.path.segments.len() != 1 {
                    return true;
                }
                let Some(actual) = bindings.get(&path.path.segments[0].ident.to_string()) else {
                    return true;
                };
                bounds_hold(graph, impl_module, &predicate.bounds, actual)
            })
}

fn bounds_hold(
    graph: &ModuleGraph,
    impl_module: &[String],
    bounds: &syn::punctuated::Punctuated<syn::TypeParamBound, syn::Token![+]>,
    actual: &Type,
) -> bool {
    bounds.iter().all(|bound| {
        let syn::TypeParamBound::Trait(bound) = bound else {
            return true;
        };
        local_trait_bound_holds(graph, impl_module, &bound.path, actual)
    })
}

fn local_trait_bound_holds(
    graph: &ModuleGraph,
    bound_module: &[String],
    bound_path: &syn::Path,
    actual: &Type,
) -> bool {
    let segments = bound_path
        .segments
        .iter()
        .map(|segment| segment.ident.to_string())
        .collect::<Vec<_>>();
    let Some(bound_key) = resolve_trait_key(graph, bound_module, &segments) else {
        // External bounds are not modeled by the local source graph. Retaining
        // the candidate is fail-closed for authority producer inventory.
        return true;
    };
    let bound_arguments = bound_path
        .segments
        .last()
        .map(|segment| &segment.arguments)
        .unwrap_or(&PathArguments::None);
    graph.modules.iter().any(|(candidate_module, node)| {
        node.items.iter().any(|item| {
            let Item::Impl(candidate) = item else {
                return false;
            };
            let Some((_, candidate_trait, _)) = &candidate.trait_ else {
                return false;
            };
            let candidate_segments = candidate_trait
                .segments
                .iter()
                .map(|segment| segment.ident.to_string())
                .collect::<Vec<_>>();
            resolve_trait_key(graph, candidate_module, &candidate_segments).as_ref()
                == Some(&bound_key)
                && unify_impl_projection(
                    graph,
                    candidate_module,
                    candidate,
                    bound_module,
                    actual,
                    bound_arguments,
                )
                .is_some()
        })
    })
}
