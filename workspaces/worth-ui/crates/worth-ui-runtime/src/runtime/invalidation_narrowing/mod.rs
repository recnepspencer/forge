#[path = "authority/active_set.rs"]
mod active_set;
#[path = "authority/authority.rs"]
mod authority;
#[path = "authority/authority_context.rs"]
mod authority_context;
#[path = "authority/authority_index.rs"]
mod authority_index;
#[cfg(test)]
#[path = "authority/authority_tests.rs"]
mod authority_tests;
#[path = "authority/catalog_transition.rs"]
mod catalog_transition;
#[path = "sources/committed_portal_source.rs"]
mod committed_portal_source;
#[path = "sources/committed_scroll_sources.rs"]
mod committed_scroll_sources;
#[path = "selection/consumer.rs"]
mod consumer;
#[path = "selection/consumer_support.rs"]
mod consumer_support;
#[path = "selection/denial.rs"]
mod denial;
#[path = "sources/host_measurement_narrowing.rs"]
mod host_measurement_narrowing;
#[path = "selection/narrowed_frame_plan.rs"]
mod narrowed_frame_plan;
#[path = "selection/narrowed_invalidation.rs"]
mod narrowed_invalidation;
#[path = "portal_authority/portal_binding.rs"]
mod portal_binding;
#[path = "portal_authority/portal_binding_index.rs"]
mod portal_binding_index;
#[path = "portal_authority/portal_binding_succession.rs"]
mod portal_binding_succession;
#[path = "authority/scroll_authority.rs"]
mod scroll_authority;
#[path = "scroll_authority/scroll_binding.rs"]
mod scroll_binding;
#[path = "scroll_authority/scroll_binding_index.rs"]
mod scroll_binding_index;
#[path = "scroll_authority/scroll_binding_key_index.rs"]
mod scroll_binding_key_index;
#[path = "scroll_authority/scroll_catalog_evidence.rs"]
mod scroll_catalog_evidence;
#[path = "sources/scroll_owner_acquisition.rs"]
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
