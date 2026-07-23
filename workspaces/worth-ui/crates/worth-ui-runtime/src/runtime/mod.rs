//! Runtime lifecycle lanes: launch → replacement → planning → activation → execution → host observation.

pub(crate) mod activation;
pub(crate) use activation::committed_allocation_attempt::{
    UiCommittedAllocationActivationAttempt, UiCommittedAllocationActivationIdentity,
};
pub(crate) use activation::UiAllocationCatalogDeltaActivationInput;
pub(crate) use activation::WorthUiPreparedApplicationPublication;
pub(crate) use activation::WorthUiQueryAwarePlanOutcome;
mod active;
mod allocation_catalog_successor;
mod allocation_frame_dispatch;
pub use allocation_catalog_successor::{
    UiAllocationCatalogDeltaClosureDenial, UiAllocationCatalogSuccessorReceipt,
};
mod allocation_receipt;
mod drag_resize;
pub(crate) mod execution;
pub(crate) mod exports;
pub(crate) mod host_observation;
mod invalidation_narrowing;
pub(crate) use invalidation_narrowing::{
    UiAdmittedScrollInvalidationBinding, UiAllocationInvalidationAuthority,
};
mod launch;
#[cfg(any(test, feature = "certification-support"))]
pub(crate) use launch::WorthUiActivationStagingPlans;
mod measurement;
pub(crate) mod persistent_index;
pub(crate) mod planning;
mod portal_anchored_allocation;
pub mod replacement;
pub(crate) mod scroll_owned_allocation;
mod source_ingress;
mod stream_policy;
mod viewport_resize;

pub use drag_resize::*;
pub use scroll_owned_allocation::*;

pub(crate) use allocation_receipt::project_allocation_preview;
pub(crate) use allocation_receipt::UiCommittedAllocationCatalogActivationRow;
pub(crate) use allocation_receipt::UiCommittedScrollActivationSource;
pub use exports::*;

#[cfg(test)]
pub(crate) use replacement::state_inventory::WorthUiTransientInteractionAdmission;

#[cfg(test)]
pub(crate) mod tests;
