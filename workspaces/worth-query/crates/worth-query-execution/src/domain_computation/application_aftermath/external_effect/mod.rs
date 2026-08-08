//! External-effect causality: postures, correlation, outbox, classification.
//!
//! CDC is intentionally not the delivery substrate (R8.8). Dispatch work is
//! derived from Query-owned outbox records co-committed with the mutation.

mod causal_event;
pub(crate) mod classification;
mod correlation;
mod dispatch;
mod identity;
mod identity_derivation;
mod observation;
mod outbox;
mod posture;
mod transport;

#[cfg(test)]
pub(crate) mod tests;

pub use classification::{ExternalEffectClassification, ExternalRailTransportFault};
pub use correlation::{
    derive_external_effect_correlation_identity, ExternalEffectCorrelationBasis,
    ExternalEffectCorrelationIdentity,
};
pub(in crate::domain_computation) use dispatch::dispatch_external_effect;
pub use dispatch::{
    WorthQueryExternalDispatchPosture, WorthQueryExternalDispatchPostureKind,
    WorthQueryExternalEffectCausalLadder, WorthQueryExternalEffectDispatch,
};
pub use identity::{ExternalEffectCausalLink, ExternalEffectPostureIdentity};
pub(crate) use outbox::WorthQueryDispatchOutboxRestoredFields;
pub use outbox::{
    dispatch_outbox_create_intent, WorthQueryDispatchOutboxLayout, WorthQueryDispatchOutboxRecord,
};
pub use posture::{ExternalEffectPosture, ExternalEffectPostureKind};
pub use transport::{
    WorthQueryExternalDispatchRequest, WorthQueryExternalEffectTransport,
    WorthQueryExternalTransportOutcome,
};
