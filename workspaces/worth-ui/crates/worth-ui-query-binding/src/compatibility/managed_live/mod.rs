//! Transitional managed-live compatibility boundary.
//!
//! Query 9.14 phases 17, 19, 23, and 24 are the exit trigger. Until then this
//! module preserves the real Query-owned resource lifecycle without presenting
//! it as operation-native snapshot consumption.

mod admission;
mod close;
pub(crate) mod declaration;
mod open;
mod projection_facts;
mod read;
mod resource;
mod retention;
mod retirement;

pub use crate::consumption::WorthUiQueryLiveProjectionOutcome;
pub use crate::declaration::WorthUiInstalledLiveQueryView;
pub use admission::{WorthUiQueryLiveAdmissionDenial, WorthUiQueryLiveAdmissionStop};
pub use close::{
    WorthUiQueryLiveAuthorityCloseStop, WorthUiQueryLiveCloseOutcome,
    WorthUiQueryLiveRuntimeCloseStop,
};
pub use open::{WorthUiQueryLiveOpenError, WorthUiQueryLiveOpenOutcome};
pub use projection_facts::*;
pub use read::WorthUiQueryLiveRead;
pub use resource::WorthUiQueryLiveResource;
pub use retention::{
    WorthUiExactManagedLiveResourceEvidence, WorthUiManagedLiveCompatibilityObservation,
};
pub use retirement::{
    WorthUiQueryLiveRetirement, WorthUiQueryLiveRetirementAuthorityStop,
    WorthUiQueryLiveRetirementCloseOutcome, WorthUiQueryLiveRetirementCloseReceipt,
    WorthUiQueryLiveRetirementRuntimeStop,
};

pub(crate) use retention::WorthUiManagedLiveCompatibilityRetention;
