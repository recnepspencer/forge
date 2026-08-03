use crate::capability::{UiIntentId, UiSemanticInteractionFamily};
use crate::declaration::UiCanonicalIntentDeclaration;
use crate::graph::UiGraphNodeIdentity;
use std::sync::Arc;

pub enum UiIntentRouteResolution {
    Product(UiResolvedProductIntentRoute),
    Confirmation(UiResolvedConfirmationIntentRoute),
}

pub struct UiResolvedProductIntentRoute {
    graph_node: UiGraphNodeIdentity,
    interaction: UiSemanticInteractionFamily,
    definition_id: UiIntentId,
    declaration: Arc<UiCanonicalIntentDeclaration>,
    source: crate::runtime::interaction::UiSemanticInteraction,
}

pub struct UiResolvedConfirmationIntentRoute {
    graph_node: UiGraphNodeIdentity,
    definition_id: UiIntentId,
    declaration: Arc<UiCanonicalIntentDeclaration>,
    source: crate::runtime::interaction::UiSemanticInteraction,
}

impl UiResolvedProductIntentRoute {
    pub(crate) const fn new(
        graph_node: UiGraphNodeIdentity,
        interaction: UiSemanticInteractionFamily,
        definition_id: UiIntentId,
        declaration: Arc<UiCanonicalIntentDeclaration>,
        source: crate::runtime::interaction::UiSemanticInteraction,
    ) -> Self {
        Self {
            graph_node,
            interaction,
            definition_id,
            declaration,
            source,
        }
    }

    pub const fn graph_node(&self) -> UiGraphNodeIdentity {
        self.graph_node
    }

    pub const fn interaction(&self) -> UiSemanticInteractionFamily {
        self.interaction
    }

    pub const fn definition_id(&self) -> UiIntentId {
        self.definition_id
    }

    pub fn declaration_identity(&self) -> &str {
        self.declaration.identity().as_str()
    }

    pub const fn source(&self) -> &crate::runtime::interaction::UiSemanticInteraction {
        &self.source
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        Arc<UiCanonicalIntentDeclaration>,
        crate::runtime::interaction::UiSemanticInteraction,
    ) {
        (self.declaration, self.source)
    }
}

impl UiResolvedConfirmationIntentRoute {
    pub(crate) const fn new(
        graph_node: UiGraphNodeIdentity,
        definition_id: UiIntentId,
        declaration: Arc<UiCanonicalIntentDeclaration>,
        source: crate::runtime::interaction::UiSemanticInteraction,
    ) -> Self {
        Self {
            graph_node,
            definition_id,
            declaration,
            source,
        }
    }

    pub const fn graph_node(&self) -> UiGraphNodeIdentity {
        self.graph_node
    }

    pub const fn definition_id(&self) -> UiIntentId {
        self.definition_id
    }

    pub fn declaration_identity(&self) -> &str {
        self.declaration.identity().as_str()
    }

    pub const fn source(&self) -> &crate::runtime::interaction::UiSemanticInteraction {
        &self.source
    }
}
