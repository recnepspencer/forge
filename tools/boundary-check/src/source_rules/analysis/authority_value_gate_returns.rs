//! Recursive owned-return inventory for public marker producers.

use std::collections::BTreeSet;

use syn::Type;

use super::authority_value_gate_defs::DefinitionKey;
use super::authority_value_gate_projection::resolve_associated_projections;
use super::authority_value_identity::local_type_key;
use super::crate_modules::ModuleGraph;

pub(super) fn return_type_keys(
    graph: &ModuleGraph,
    module_path: &[String],
    ty: &Type,
    self_ty: Option<&Type>,
) -> BTreeSet<DefinitionKey> {
    let mut produced = BTreeSet::new();
    collect_owned_return_types(graph, module_path, ty, self_ty, None, &mut produced, 0);
    produced
}

pub(super) fn return_type_keys_for_impl(
    graph: &ModuleGraph,
    module_path: &[String],
    ty: &Type,
    item_impl: &syn::ItemImpl,
) -> BTreeSet<DefinitionKey> {
    let mut produced = BTreeSet::new();
    collect_owned_return_types(
        graph,
        module_path,
        ty,
        Some(&item_impl.self_ty),
        Some(item_impl),
        &mut produced,
        0,
    );
    produced
}

fn collect_owned_return_types(
    graph: &ModuleGraph,
    module_path: &[String],
    ty: &Type,
    self_ty: Option<&Type>,
    item_impl: Option<&syn::ItemImpl>,
    produced: &mut BTreeSet<DefinitionKey>,
    projection_depth: usize,
) {
    if projection_depth > 16 {
        return;
    }
    let Type::Path(type_path) = ty else {
        match ty {
            Type::Array(array) => collect_owned_return_types(
                graph,
                module_path,
                &array.elem,
                self_ty,
                item_impl,
                produced,
                projection_depth,
            ),
            Type::Slice(slice) => collect_owned_return_types(
                graph,
                module_path,
                &slice.elem,
                self_ty,
                item_impl,
                produced,
                projection_depth,
            ),
            Type::Tuple(tuple) => tuple.elems.iter().for_each(|element| {
                collect_owned_return_types(
                    graph,
                    module_path,
                    element,
                    self_ty,
                    item_impl,
                    produced,
                    projection_depth,
                )
            }),
            Type::Paren(paren) => collect_owned_return_types(
                graph,
                module_path,
                &paren.elem,
                self_ty,
                item_impl,
                produced,
                projection_depth,
            ),
            Type::Group(group) => collect_owned_return_types(
                graph,
                module_path,
                &group.elem,
                self_ty,
                item_impl,
                produced,
                projection_depth,
            ),
            Type::ImplTrait(opaque) => opaque.bounds.iter().for_each(|bound| {
                if let syn::TypeParamBound::Trait(trait_bound) = bound {
                    trait_bound.path.segments.iter().for_each(|segment| {
                        collect_path_arguments(
                            graph,
                            module_path,
                            &segment.arguments,
                            self_ty,
                            item_impl,
                            produced,
                            projection_depth,
                        )
                    });
                }
            }),
            Type::TraitObject(object) => object.bounds.iter().for_each(|bound| {
                if let syn::TypeParamBound::Trait(trait_bound) = bound {
                    trait_bound.path.segments.iter().for_each(|segment| {
                        collect_path_arguments(
                            graph,
                            module_path,
                            &segment.arguments,
                            self_ty,
                            item_impl,
                            produced,
                            projection_depth,
                        )
                    });
                }
            }),
            Type::BareFn(function) => {
                if let syn::ReturnType::Type(_, output) = &function.output {
                    collect_owned_return_types(
                        graph,
                        module_path,
                        output,
                        self_ty,
                        item_impl,
                        produced,
                        projection_depth,
                    );
                }
            }
            _ => {}
        }
        return;
    };
    if type_path.path.is_ident("Self") {
        if let Some(key) = self_ty.and_then(|ty| local_type_key(graph, module_path, ty)) {
            produced.insert(key);
        }
    } else if let Some(key) = local_type_key(graph, module_path, ty) {
        produced.insert(key);
    }
    collect_associated_projection(
        graph,
        module_path,
        type_path,
        self_ty,
        item_impl,
        produced,
        projection_depth,
    );
    if !is_standard_phantom_data(&type_path.path) {
        type_path.path.segments.iter().for_each(|segment| {
            collect_path_arguments(
                graph,
                module_path,
                &segment.arguments,
                self_ty,
                item_impl,
                produced,
                projection_depth,
            )
        });
    }
}

fn is_standard_phantom_data(path: &syn::Path) -> bool {
    let segments = path
        .segments
        .iter()
        .map(|segment| segment.ident.to_string())
        .collect::<Vec<_>>();
    matches!(
        segments.as_slice(),
        [root, marker, phantom]
            if (root == "core" || root == "std")
                && marker == "marker"
                && phantom == "PhantomData"
    )
}

fn collect_associated_projection(
    graph: &ModuleGraph,
    module_path: &[String],
    type_path: &syn::TypePath,
    self_ty: Option<&Type>,
    item_impl: Option<&syn::ItemImpl>,
    produced: &mut BTreeSet<DefinitionKey>,
    projection_depth: usize,
) {
    for projection in
        resolve_associated_projections(graph, module_path, type_path, self_ty, item_impl)
    {
        collect_owned_return_types(
            graph,
            projection.module_path(),
            projection.associated_type(),
            Some(projection.self_type()),
            Some(projection.item_impl()),
            produced,
            projection_depth + 1,
        );
    }
}

fn collect_path_arguments(
    graph: &ModuleGraph,
    module_path: &[String],
    arguments: &syn::PathArguments,
    self_ty: Option<&Type>,
    item_impl: Option<&syn::ItemImpl>,
    produced: &mut BTreeSet<DefinitionKey>,
    projection_depth: usize,
) {
    match arguments {
        syn::PathArguments::AngleBracketed(arguments) => {
            for argument in &arguments.args {
                match argument {
                    syn::GenericArgument::Type(ty) => collect_owned_return_types(
                        graph,
                        module_path,
                        ty,
                        self_ty,
                        item_impl,
                        produced,
                        projection_depth,
                    ),
                    syn::GenericArgument::AssocType(assoc) => collect_owned_return_types(
                        graph,
                        module_path,
                        &assoc.ty,
                        self_ty,
                        item_impl,
                        produced,
                        projection_depth,
                    ),
                    _ => {}
                }
            }
        }
        syn::PathArguments::Parenthesized(arguments) => {
            if let syn::ReturnType::Type(_, output) = &arguments.output {
                collect_owned_return_types(
                    graph,
                    module_path,
                    output,
                    self_ty,
                    item_impl,
                    produced,
                    projection_depth,
                );
            }
        }
        syn::PathArguments::None => {}
    }
}
