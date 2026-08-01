use std::collections::BTreeMap;

use super::{UiIntentCatalog, UiIntentCatalogSemanticComparison};

type DeclarationMeaning<'catalog> =
    BTreeMap<&'catalog str, &'catalog super::UiCanonicalIntentDeclaration>;
type ProductRouteMeaning<'catalog> = BTreeMap<
    (
        crate::graph::UiGraphNodeIdentity,
        crate::capability::UiSemanticInteractionFamily,
    ),
    &'catalog super::UiCanonicalIntentDeclaration,
>;
type ConfirmationRouteMeaning<'catalog> =
    BTreeMap<crate::graph::UiGraphNodeIdentity, &'catalog super::UiCanonicalIntentDeclaration>;

pub(super) fn compare(
    predecessor: &UiIntentCatalog,
    candidate: &UiIntentCatalog,
) -> UiIntentCatalogSemanticComparison {
    let equivalent = predecessor.definition_count == candidate.definition_count
        && declaration_meaning(predecessor) == declaration_meaning(candidate)
        && product_route_meaning(predecessor) == product_route_meaning(candidate)
        && confirmation_route_meaning(predecessor) == confirmation_route_meaning(candidate);
    if equivalent {
        UiIntentCatalogSemanticComparison::Equivalent
    } else {
        UiIntentCatalogSemanticComparison::Different
    }
}

fn declaration_meaning(catalog: &UiIntentCatalog) -> DeclarationMeaning<'_> {
    catalog
        .declarations
        .iter()
        .map(|declaration| (declaration.identity().as_str(), declaration.as_ref()))
        .collect()
}

fn product_route_meaning(catalog: &UiIntentCatalog) -> ProductRouteMeaning<'_> {
    catalog
        .product_routes
        .iter()
        .map(|route| {
            (
                (route.graph_node(), route.interaction()),
                catalog.declarations[route.declaration_index() as usize].as_ref(),
            )
        })
        .collect()
}

fn confirmation_route_meaning(catalog: &UiIntentCatalog) -> ConfirmationRouteMeaning<'_> {
    catalog
        .confirmation_routes
        .iter()
        .map(|route| {
            (
                route.graph_node(),
                catalog.declarations[route.declaration_index() as usize].as_ref(),
            )
        })
        .collect()
}
