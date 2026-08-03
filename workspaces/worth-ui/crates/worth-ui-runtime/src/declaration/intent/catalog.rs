mod preparation;

use std::collections::HashMap;
use std::sync::Arc;

use crate::capability::UiSemanticInteractionFamily;

use super::{
    UiCanonicalIntentDeclaration, UiIntentCatalogPreparationDenial,
    UiIntentConfirmationRouteBinding, UiIntentRouteBinding,
};

pub(super) type RouteKey = (
    crate::graph::UiGraphNodeIdentity,
    UiSemanticInteractionFamily,
);

pub(crate) enum UiIntentCatalogResolvedRoute {
    Product {
        route: UiIntentRouteBinding,
        declaration: Arc<UiCanonicalIntentDeclaration>,
    },
    Confirmation {
        route: UiIntentConfirmationRouteBinding,
        declaration: Arc<UiCanonicalIntentDeclaration>,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiIntentCatalogMetrics {
    definitions: usize,
    declarations: usize,
    product_routes: usize,
    confirmation_routes: usize,
}

pub(crate) struct UiIntentCatalog {
    declarations: Box<[Arc<UiCanonicalIntentDeclaration>]>,
    product_routes: Box<[UiIntentRouteBinding]>,
    confirmation_routes: Box<[UiIntentConfirmationRouteBinding]>,
    product_index: HashMap<RouteKey, usize>,
    confirmation_index: HashMap<RouteKey, usize>,
    definition_count: usize,
}

impl UiIntentCatalog {
    pub(crate) fn prepare(
        material: &crate::runtime::WorthUiAuthoredIntentMaterial,
        definitions: &crate::capability::FrozenIntentDefinitionCapabilities,
        graph: &crate::graph::UiGraphSnapshot,
        query: &worth_ui_query_binding::WorthUiQueryBindingPlan,
        application_facts: &super::UiIntentApplicationFactPlan,
    ) -> Result<Self, UiIntentCatalogPreparationDenial> {
        preparation::prepare(material, definitions, graph, query, application_facts)
    }

    pub(crate) fn lookup(
        &self,
        graph_node: crate::graph::UiGraphNodeIdentity,
        interaction: UiSemanticInteractionFamily,
    ) -> Option<UiIntentCatalogResolvedRoute> {
        let key = (graph_node, interaction);
        if let Some(index) = self.product_index.get(&key).copied() {
            let route = self.product_routes[index];
            return Some(UiIntentCatalogResolvedRoute::Product {
                route,
                declaration: Arc::clone(&self.declarations[route.declaration_index() as usize]),
            });
        }
        self.confirmation_index.get(&key).copied().map(|index| {
            let route = self.confirmation_routes[index];
            UiIntentCatalogResolvedRoute::Confirmation {
                route,
                declaration: Arc::clone(&self.declarations[route.declaration_index() as usize]),
            }
        })
    }

    pub(crate) fn metrics(&self) -> UiIntentCatalogMetrics {
        UiIntentCatalogMetrics {
            definitions: self.definition_count,
            declarations: self.declarations.len(),
            product_routes: self.product_routes.len(),
            confirmation_routes: self.confirmation_routes.len(),
        }
    }
}

impl UiIntentCatalogMetrics {
    pub const fn definitions(self) -> usize {
        self.definitions
    }

    pub const fn declarations(self) -> usize {
        self.declarations
    }

    pub const fn product_routes(self) -> usize {
        self.product_routes
    }

    pub const fn confirmation_routes(self) -> usize {
        self.confirmation_routes
    }
}
