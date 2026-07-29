//! Runtime lifecycle lanes: launch → replacement → planning → activation → execution → host observation.

pub(crate) mod activation;
pub(crate) use activation::committed_allocation_attempt::UiCommittedAllocationActivationAttempt;
pub(crate) use activation::UiAllocationCatalogDeltaActivationInput;
pub(crate) use activation::WorthUiApplicationPlanSwap;
pub(crate) use activation::WorthUiInitialMountedAllocationActivationDenial;
pub(crate) use activation::WorthUiPreparedApplicationPlanSwap;
pub(crate) use activation::WorthUiPreparedApplicationPublication;
pub(crate) use activation::WorthUiPreparedQueryAwarePlanOutcome;
mod active;
pub(crate) use active::WorthUiActiveExecutionPlan;
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
#[cfg(any(test, feature = "certification-support"))]
pub(crate) use launch::WorthUiRuntimeLaunchAuthority;
pub(crate) use launch::{
    WorthUiInitialMountedCatalogPreparationDenial, WorthUiMountedAllocationActivationBasis,
};
mod measurement;
pub(crate) mod observation;
pub(crate) mod persistent_index;
pub(crate) mod planning;
mod portal_anchored_allocation;
pub(crate) mod rebind;
pub mod replacement;
pub(crate) mod scroll_owned_allocation;
pub(crate) mod session;
mod source_ingress;
pub(crate) use source_ingress::WorthUiAuthoredSourceBasis;
pub use source_ingress::{
    UiSourceCompilationDenialReceipt, UiSourceRebindAttemptBasis, UiSourceRebindAttemptDenial,
    UiSourceRebindAttemptDenialReceipt, UiSourceRebindAttemptFailure, UiSourceRebindAttemptOutcome,
};
mod stream_policy;
mod viewport_resize;

pub use drag_resize::*;
pub use scroll_owned_allocation::*;

pub(crate) use allocation_frame_dispatch::UiPendingMountedPreviewTransition;
pub(crate) use allocation_receipt::project_allocation_preview;
pub(crate) use allocation_receipt::UiAllocationTruthRevision;
pub(crate) use allocation_receipt::UiCommittedAllocationCatalogActivationRow;
pub(crate) use allocation_receipt::UiCommittedScrollActivationSource;
pub(crate) use allocation_receipt::UiMountedAllocationExactDelta;
pub(crate) use allocation_receipt::UiMountedAllocationProjectionCatalog;
pub(crate) use allocation_receipt::UiMountedAllocationProjectionDelta;
pub(crate) use allocation_receipt::UiMountedAllocationProjectionDenial;
pub(crate) use allocation_receipt::UiMountedAllocationProjectionSource;
pub use exports::*;

#[cfg(test)]
pub(crate) use replacement::state_inventory::WorthUiTransientInteractionAdmission;

#[cfg(test)]
pub(crate) mod tests;
