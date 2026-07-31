//! Stable product-facing intent definition contracts.

pub use crate::capability::{
    FrozenIntentDefinitionCapabilities, IntentDefinitionDescriptor, UiIntent,
    UiIntentAcceptedInteractions, UiIntentBoolean, UiIntentDefinition,
    UiIntentDefinitionRegistrationError, UiIntentExecutionDestination, UiIntentId, UiIntentPayload,
    UiIntentPayloadField, UiIntentPayloadFieldDescriptor, UiIntentPayloadFieldKind,
    UiIntentPayloadFieldSet, UiIntentPayloadProjection, UiIntentPayloadProjectionViolation,
    UiIntentPayloadSchemaViolation, UiIntentPayloadValueKind, UiIntentProductOutcome,
    UiIntentRuntimeServiceDestination, UiIntentSchema, UiIntentSelection, UiIntentSelectionValue,
    UiIntentText, UiIntentTransitionDestination, UiIntentUnsigned64, UiSemanticInteractionFamily,
    UI_INTENT_PAYLOAD_FIELD_LIMIT, UI_INTENT_PAYLOAD_TEXT_BYTE_LIMIT,
};
pub use crate::declaration::{
    UiIntentApplicationFact, UiIntentApplicationFactIdentityError,
    UiIntentApplicationFactRegistrationError, UiIntentCatalogMetrics,
    UiIntentCatalogPreparationDenial, UiIntentConfirmationRouteBinding, UiIntentDeclaration,
    UiIntentDeclarationConstructionError, UiIntentDeclarationIdentity,
    UiIntentInteractionPayloadSourceKind, UiIntentPayloadSource, UiIntentRouteBinding,
};
pub use crate::runtime::intent::{
    UiIntentApplicationFactUpdateDenial, UiIntentApplicationFactUpdateReceipt,
    UiIntentInputBasisReceipt, UiIntentPayloadProjectionCost, UiIntentPayloadStop,
    UiIntentRouteResolution, UiIntentRouteResolutionStop, UiPreparedIntentPayload,
    UiResolvedConfirmationIntentRoute, UiResolvedProductIntentRoute,
};
pub use crate::runtime::interaction::UiIntentRouteSource;
