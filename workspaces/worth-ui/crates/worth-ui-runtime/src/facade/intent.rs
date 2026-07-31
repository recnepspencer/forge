//! Stable product-facing intent definition contracts.

pub use crate::capability::{
    FrozenIntentDefinitionCapabilities, IntentDefinitionDescriptor, UiIntent,
    UiIntentAcceptedInteractions, UiIntentDefinition, UiIntentDefinitionRegistrationError,
    UiIntentExecutionDestination, UiIntentId, UiIntentPayload, UiIntentProductOutcome,
    UiIntentRuntimeServiceDestination, UiIntentSchema, UiIntentTransitionDestination,
    UiSemanticInteractionFamily,
};
pub use crate::declaration::{
    UiIntentCatalogMetrics, UiIntentCatalogPreparationDenial, UiIntentConfirmationRouteBinding,
    UiIntentDeclaration, UiIntentDeclarationConstructionError, UiIntentDeclarationIdentity,
    UiIntentRouteBinding,
};
pub use crate::runtime::intent::{
    UiIntentRouteResolution, UiIntentRouteResolutionStop, UiResolvedConfirmationIntentRoute,
    UiResolvedProductIntentRoute,
};
pub use crate::runtime::interaction::UiIntentRouteSource;
