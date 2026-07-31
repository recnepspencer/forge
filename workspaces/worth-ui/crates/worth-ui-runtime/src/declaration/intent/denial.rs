use crate::capability::{UiIntentSchema, UiSemanticInteractionFamily};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UiIntentCatalogPreparationDenial {
    DuplicateDeclaration {
        identity: Box<str>,
    },
    TooManyDeclarations {
        observed: usize,
        maximum: usize,
    },
    UnknownDefinition {
        declaration: Box<str>,
        definition: Box<str>,
    },
    PayloadSchemaMismatch {
        declaration: Box<str>,
        expected_identity: Box<str>,
        expected_version: u16,
        registered: UiIntentSchema,
    },
    OutcomeSchemaMismatch {
        declaration: Box<str>,
        expected_identity: Box<str>,
        expected_version: u16,
        registered: UiIntentSchema,
    },
    InteractionNotAccepted {
        declaration: Box<str>,
        interaction: UiSemanticInteractionFamily,
    },
    UnknownRouteDeclaration {
        declaration: Box<str>,
    },
    MissingRouteTarget {
        authored_provenance_digest: u64,
    },
    ProductInteractionMismatch {
        declaration: Box<str>,
        declared: UiSemanticInteractionFamily,
        routed: UiSemanticInteractionFamily,
    },
    ConfirmationRequiresActivate {
        declaration: Box<str>,
        routed: UiSemanticInteractionFamily,
    },
    AmbiguousProductRoute {
        graph_node: crate::graph::UiGraphNodeIdentity,
        interaction: UiSemanticInteractionFamily,
    },
    AmbiguousConfirmationRoute {
        graph_node: crate::graph::UiGraphNodeIdentity,
        interaction: UiSemanticInteractionFamily,
    },
    RouteKindCrossover {
        graph_node: crate::graph::UiGraphNodeIdentity,
        interaction: UiSemanticInteractionFamily,
    },
    RouteCapacityExceeded {
        observed: usize,
        maximum: usize,
    },
}
