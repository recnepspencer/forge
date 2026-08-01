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
    cost: crate::declaration::UiIntentRouteResolutionCost,
    evidence_reference: Option<worth_ui_inspection::UiIntentEvidenceReference>,
}

pub struct UiResolvedConfirmationIntentRoute {
    graph_node: UiGraphNodeIdentity,
    definition_id: UiIntentId,
    declaration: Arc<UiCanonicalIntentDeclaration>,
    source: crate::runtime::interaction::UiSemanticInteraction,
    cost: crate::declaration::UiIntentRouteResolutionCost,
    evidence_reference: Option<worth_ui_inspection::UiIntentEvidenceReference>,
}

pub(crate) struct UiResolvedProductIntentRouteInput {
    pub(crate) graph_node: UiGraphNodeIdentity,
    pub(crate) interaction: UiSemanticInteractionFamily,
    pub(crate) definition_id: UiIntentId,
    pub(crate) declaration: Arc<UiCanonicalIntentDeclaration>,
    pub(crate) source: crate::runtime::interaction::UiSemanticInteraction,
    pub(crate) cost: crate::declaration::UiIntentRouteResolutionCost,
}

pub(crate) struct UiResolvedConfirmationIntentRouteInput {
    pub(crate) graph_node: UiGraphNodeIdentity,
    pub(crate) definition_id: UiIntentId,
    pub(crate) declaration: Arc<UiCanonicalIntentDeclaration>,
    pub(crate) source: crate::runtime::interaction::UiSemanticInteraction,
    pub(crate) cost: crate::declaration::UiIntentRouteResolutionCost,
}

impl UiResolvedProductIntentRoute {
    pub(crate) fn new(input: UiResolvedProductIntentRouteInput) -> Self {
        Self {
            graph_node: input.graph_node,
            interaction: input.interaction,
            definition_id: input.definition_id,
            declaration: input.declaration,
            source: input.source,
            cost: input.cost,
            evidence_reference: None,
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

    pub const fn cost(&self) -> crate::declaration::UiIntentRouteResolutionCost {
        self.cost
    }

    pub const fn evidence_reference(
        &self,
    ) -> Option<worth_ui_inspection::UiIntentEvidenceReference> {
        self.evidence_reference
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        UiGraphNodeIdentity,
        Arc<UiCanonicalIntentDeclaration>,
        crate::runtime::interaction::UiSemanticInteraction,
        crate::declaration::UiIntentRouteResolutionCost,
        Option<worth_ui_inspection::UiIntentEvidenceReference>,
    ) {
        (
            self.graph_node,
            self.declaration,
            self.source,
            self.cost,
            self.evidence_reference,
        )
    }
}

impl UiResolvedConfirmationIntentRoute {
    pub(crate) fn new(input: UiResolvedConfirmationIntentRouteInput) -> Self {
        Self {
            graph_node: input.graph_node,
            definition_id: input.definition_id,
            declaration: input.declaration,
            source: input.source,
            cost: input.cost,
            evidence_reference: None,
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

    pub const fn cost(&self) -> crate::declaration::UiIntentRouteResolutionCost {
        self.cost
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        UiGraphNodeIdentity,
        UiIntentId,
        Arc<UiCanonicalIntentDeclaration>,
        crate::runtime::interaction::UiSemanticInteraction,
        crate::declaration::UiIntentRouteResolutionCost,
        Option<worth_ui_inspection::UiIntentEvidenceReference>,
    ) {
        (
            self.graph_node,
            self.definition_id,
            self.declaration,
            self.source,
            self.cost,
            self.evidence_reference,
        )
    }
}

impl UiIntentRouteResolution {
    pub(crate) fn with_evidence_reference(
        mut self,
        reference: Option<worth_ui_inspection::UiIntentEvidenceReference>,
    ) -> Self {
        match &mut self {
            Self::Product(route) => route.evidence_reference = reference,
            Self::Confirmation(route) => route.evidence_reference = reference,
        }
        self
    }
}
