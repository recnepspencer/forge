mod accepted_registration;
mod definition;
mod execution_destination;
mod identity;
mod payload_schema;
mod registry;
mod result_schema;

pub(crate) use accepted_registration::IntentDefinitionAcceptedRegistrationProof;
pub use definition::{
    IntentDefinitionDescriptor, UiIntent, UiIntentDefinition, UiSemanticInteractionFamily,
};
pub use execution_destination::{
    UiIntentExecutionDestination, UiIntentRuntimeServiceDestination, UiIntentTransitionDestination,
};
pub use identity::UiIntentId;
pub use payload_schema::UiIntentPayload;
pub(crate) use registry::IntentDefinitionRegistry;
pub use registry::{FrozenIntentDefinitionCapabilities, UiIntentDefinitionRegistrationError};
pub use result_schema::{UiIntentProductOutcome, UiIntentSchema};
