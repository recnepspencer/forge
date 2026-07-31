//! Typed intent meaning exposed to product composition roots.

pub use worth_ui_runtime::facade::intent::{
    FrozenIntentDefinitionCapabilities, IntentDefinitionDescriptor, UiIntent,
    UiIntentAcceptedInteractions, UiIntentCatalogMetrics, UiIntentCatalogPreparationDenial,
    UiIntentConfirmationRouteBinding, UiIntentDeclaration, UiIntentDeclarationConstructionError,
    UiIntentDeclarationIdentity, UiIntentDefinition, UiIntentDefinitionRegistrationError,
    UiIntentExecutionDestination, UiIntentId, UiIntentPayload, UiIntentProductOutcome,
    UiIntentRouteBinding, UiIntentRouteResolution, UiIntentRouteResolutionStop,
    UiIntentRouteSource, UiIntentRuntimeServiceDestination, UiIntentSchema,
    UiIntentTransitionDestination, UiResolvedConfirmationIntentRoute, UiResolvedProductIntentRoute,
    UiSemanticInteractionFamily,
};
