//! Typed intent meaning exposed to product composition roots.

pub use worth_ui_runtime::facade::intent::{
    FrozenIntentDefinitionCapabilities, IntentDefinitionDescriptor, UiIntent,
    UiIntentAcceptedInteractions, UiIntentApplicationFact, UiIntentApplicationFactIdentityError,
    UiIntentApplicationFactRegistrationError, UiIntentApplicationFactUpdateDenial,
    UiIntentApplicationFactUpdateReceipt, UiIntentBoolean, UiIntentCatalogMetrics,
    UiIntentCatalogPreparationDenial, UiIntentConfirmationRouteBinding, UiIntentDeclaration,
    UiIntentDeclarationConstructionError, UiIntentDeclarationIdentity, UiIntentDefinition,
    UiIntentDefinitionRegistrationError, UiIntentExecutionDestination, UiIntentId,
    UiIntentInputBasisReceipt, UiIntentInteractionPayloadSourceKind, UiIntentPayload,
    UiIntentPayloadField, UiIntentPayloadFieldDescriptor, UiIntentPayloadFieldKind,
    UiIntentPayloadFieldSet, UiIntentPayloadProjection, UiIntentPayloadProjectionCost,
    UiIntentPayloadProjectionViolation, UiIntentPayloadSchemaViolation, UiIntentPayloadSource,
    UiIntentPayloadStop, UiIntentPayloadValueKind, UiIntentProductOutcome, UiIntentRouteBinding,
    UiIntentRouteResolution, UiIntentRouteResolutionStop, UiIntentRouteSource,
    UiIntentRuntimeServiceDestination, UiIntentSchema, UiIntentSelection, UiIntentSelectionValue,
    UiIntentText, UiIntentTransitionDestination, UiIntentUnsigned64, UiPreparedIntentPayload,
    UiResolvedConfirmationIntentRoute, UiResolvedProductIntentRoute, UiSemanticInteractionFamily,
    UI_INTENT_PAYLOAD_FIELD_LIMIT, UI_INTENT_PAYLOAD_TEXT_BYTE_LIMIT,
};
