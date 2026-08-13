//! Associated-projection resolution for public authority-marker producers.

use quote::ToTokens;
use syn::Type;

use super::authority_value_gate_projection_identity::resolve_trait_key;
use super::authority_value_gate_projection_matching::{substitute_type, unify_impl_projection};
use super::crate_modules::ModuleGraph;

fn same_type(left: &Type, right: &Type) -> bool {
    left.to_token_stream().to_string() == right.to_token_stream().to_string()
}

pub(super) struct ResolvedProjection {
    module_path: Vec<String>,
    associated_type: Type,
    item_impl: syn::ItemImpl,
}

impl ResolvedProjection {
    pub(super) fn module_path(&self) -> &[String] {
        &self.module_path
    }

    pub(super) fn associated_type(&self) -> &Type {
        &self.associated_type
    }

    pub(super) fn self_type(&self) -> &Type {
        &self.item_impl.self_ty
    }

    pub(super) const fn item_impl(&self) -> &syn::ItemImpl {
        &self.item_impl
    }
}

pub(super) fn resolve_associated_projections(
    graph: &ModuleGraph,
    module_path: &[String],
    type_path: &syn::TypePath,
    self_ty: Option<&Type>,
    enclosing_impl: Option<&syn::ItemImpl>,
) -> Vec<ResolvedProjection> {
    let Some(associated_name) = type_path.path.segments.last().map(|segment| &segment.ident) else {
        return Vec::new();
    };
    let projected_self = type_path
        .qself
        .as_ref()
        .map(|qself| qself.ty.as_ref())
        .or_else(|| {
            type_path
                .path
                .segments
                .first()
                .is_some_and(|segment| segment.ident == "Self")
                .then_some(self_ty)
                .flatten()
        });
    if let (Some(projected_self), Some(item_impl)) = (projected_self, enclosing_impl) {
        if same_type(projected_self, item_impl.self_ty.as_ref()) {
            for member in &item_impl.items {
                let syn::ImplItem::Type(associated) = member else {
                    continue;
                };
                if associated.ident == *associated_name {
                    return vec![ResolvedProjection {
                        module_path: module_path.to_vec(),
                        associated_type: associated.ty.clone(),
                        item_impl: item_impl.clone(),
                    }];
                }
            }
        }
    }
    let Some(projected_self) = projected_self else {
        return Vec::new();
    };
    let Some((projection_trait, projection_trait_arguments)) =
        projection_trait(graph, module_path, type_path)
    else {
        return Vec::new();
    };
    let mut resolved = Vec::new();
    for (impl_module, node) in &graph.modules {
        for item in &node.items {
            let syn::Item::Impl(item_impl) = item else {
                continue;
            };
            if !impl_matches_projection_trait(graph, impl_module, item_impl, &projection_trait) {
                continue;
            }
            let Some(bindings) = unify_impl_projection(
                graph,
                impl_module,
                item_impl,
                module_path,
                projected_self,
                projection_trait_arguments,
            ) else {
                continue;
            };
            for member in &item_impl.items {
                let syn::ImplItem::Type(associated) = member else {
                    continue;
                };
                if associated.ident == *associated_name {
                    resolved.push(ResolvedProjection {
                        module_path: impl_module.clone(),
                        associated_type: substitute_type(&associated.ty, &bindings),
                        item_impl: item_impl.clone(),
                    });
                }
            }
        }
    }
    resolved
}

fn projection_trait<'a>(
    graph: &ModuleGraph,
    module_path: &[String],
    type_path: &'a syn::TypePath,
) -> Option<(
    super::authority_value_gate_defs::DefinitionKey,
    &'a syn::PathArguments,
)> {
    let qself = type_path.qself.as_ref()?;
    let segment = qself
        .position
        .checked_sub(1)
        .and_then(|index| type_path.path.segments.iter().nth(index))?;
    let segments = type_path
        .path
        .segments
        .iter()
        .take(qself.position)
        .map(|segment| segment.ident.to_string())
        .collect::<Vec<_>>();
    Some((
        resolve_trait_key(graph, module_path, &segments)?,
        &segment.arguments,
    ))
}

fn impl_matches_projection_trait(
    graph: &ModuleGraph,
    impl_module: &[String],
    item_impl: &syn::ItemImpl,
    projection_trait: &super::authority_value_gate_defs::DefinitionKey,
) -> bool {
    let Some((_, trait_path, _)) = &item_impl.trait_ else {
        return false;
    };
    let segments = trait_path
        .segments
        .iter()
        .map(|segment| segment.ident.to_string())
        .collect::<Vec<_>>();
    resolve_trait_key(graph, impl_module, &segments).as_ref() == Some(projection_trait)
}
