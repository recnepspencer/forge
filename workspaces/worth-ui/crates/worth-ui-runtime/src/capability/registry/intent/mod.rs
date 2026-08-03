mod accepted_interactions;
mod accepted_registration;
mod definition;
mod execution_destination;
mod identity;
mod interaction_family;
mod payload_schema;
mod registry;
mod result_schema;
mod semantic_digest;
mod stable_identity;

pub use accepted_interactions::UiIntentAcceptedInteractions;
pub(crate) use accepted_registration::IntentDefinitionAcceptedRegistrationProof;
pub(crate) use definition::UiRegisteredIntentDefinition;
pub use definition::{IntentDefinitionDescriptor, UiIntent, UiIntentDefinition};
pub use execution_destination::{
    UiIntentExecutionDestination, UiIntentRuntimeServiceDestination, UiIntentTransitionDestination,
};
pub use identity::UiIntentId;
pub use interaction_family::UiSemanticInteractionFamily;
pub use payload_schema::{
    UiIntentBoolean, UiIntentPayload, UiIntentPayloadField, UiIntentPayloadFieldDescriptor,
    UiIntentPayloadFieldKind, UiIntentPayloadFieldSet, UiIntentPayloadProjection,
    UiIntentPayloadProjectionViolation, UiIntentPayloadSchemaViolation, UiIntentPayloadValueKind,
    UiIntentProjectedValue, UiIntentSelection, UiIntentSelectionValue, UiIntentText,
    UiIntentUnsigned64, UI_INTENT_PAYLOAD_FIELD_LIMIT, UI_INTENT_PAYLOAD_TEXT_BYTE_LIMIT,
};
pub(crate) use payload_schema::{
    UiRegisteredIntentPayloadProjector, UiSealedIntentPayload, UiTypedIntentPayloadProjector,
};
pub(crate) use registry::IntentDefinitionRegistry;
pub(crate) use registry::UiIntentDefinitionSlot;
pub use registry::{FrozenIntentDefinitionCapabilities, UiIntentDefinitionRegistrationError};
pub use result_schema::{UiIntentProductOutcome, UiIntentSchema};
