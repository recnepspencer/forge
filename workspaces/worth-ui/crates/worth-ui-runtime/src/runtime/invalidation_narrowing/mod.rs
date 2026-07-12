mod active_set;
mod authority;
mod authority_context;
mod authority_index;
#[cfg(test)]
mod authority_tests;
mod catalog_transition;
mod committed_portal_source;
mod committed_scroll_sources;
mod consumer;
mod consumer_support;
mod denial;
mod host_measurement_narrowing;
mod narrowed_frame_plan;
mod narrowed_invalidation;
mod portal_binding;
mod portal_binding_index;
mod portal_binding_succession;
mod scroll_authority;
mod scroll_binding;
mod scroll_binding_index;
mod scroll_binding_key_index;
mod scroll_catalog_evidence;
mod scroll_owner_acquisition;

pub(crate) use crate::graph::UiAdmittedAllocationPlanReference;
pub(crate) use active_set::UiAllocationNeighborhoodCatalogTransition;
pub(crate) use active_set::{UiAllocationActivationCatalog, UiAllocationActivationCatalogDenial};
pub(crate) use authority::{
    UiAllocationInvalidationAdmissionContext, UiAllocationInvalidationAuthority,
    UiCommittedAllocationInvalidationContext,
};
pub(crate) use catalog_transition::{
    UiAllocationNeighborhoodActivationDenial, UiPreparedInvalidationCatalogTransition,
};
pub(crate) use consumer::{narrow_resolved_frame, UiAllocationInvalidationNarrowingDisposition};
pub use denial::{
    UiAllocationInvalidationNarrowingDenial, UiAllocationInvalidationNarrowingRejection,
};
pub use narrowed_frame_plan::{
    UiAllocationInvalidationNarrowingCounters, UiNarrowedAllocationFramePlan,
};
pub use narrowed_invalidation::{UiAllocationInvalidationTarget, UiNarrowedAllocationInvalidation};
pub(crate) use portal_binding::UiAdmittedPortalInvalidationBinding;
pub use portal_binding::UiAdmittedPortalMovement;
pub(crate) use portal_binding_index::UiPortalInvalidationBindingIndex;
#[cfg(test)]
pub(crate) use portal_binding_index::UiPortalMovementLookupDenial;
pub(crate) use portal_binding_succession::UiPreparedPortalBindingSuccession;
pub use portal_binding_succession::{
    UiPortalBindingSuccessionCounters, UiPortalBindingSuccessionDenial,
    UiPortalBindingSuccessionLineage, UiPortalBindingSuccessionReceipt,
};
pub(crate) use scroll_binding::UiAdmittedScrollInvalidationBinding;
pub use scroll_binding::{UiScrollInvalidationBindingDenial, UiScrollOwnerAcquisitionDenial};
pub(crate) use scroll_binding_index::UiScrollInvalidationBindingIndex;
pub use scroll_catalog_evidence::{
    UiScrollBindingCatalogCounters, UiScrollCatalogSwapEvidence, UiScrollOwnerCatalogDenialReport,
    UiScrollOwnerCatalogReceipt,
};
