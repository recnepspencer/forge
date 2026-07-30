//! SUPPORT AUTHORITY — certification-consumer fixtures.
//!
//! Owned here (not `include!` from `runtime/tests`). External crates must consume fixtures only
//! through `worth-ui-test-support` (feature `certification-support`). Production law is not defined here.

mod activation_interruption;
mod active_session_observation;
mod application_builder;
mod application_graph;
mod application_replacement;
mod framework_turn_execution;
mod identity_overlay_projection;
mod layout_admission;
mod mounted_frame_execution;
mod planning;
mod rebind_identity_lifecycle;
mod runtime_launch;
mod semantic_text_projection;
mod touch_origin;
mod touch_origin_source;

pub use crate::admission::{
    UiMeasurementAdmissionPosture, UiMeasurementCapabilityGateReason,
    UiMeasurementUnsupportedReason, UiQueryMeasurementEligibilityPosture,
    UiQueryMeasurementSourceIdentity, UiQueryMeasurementUnsupportedQueryReason,
};
pub use crate::declaration::{UiDeclaredMeasurementBasisSource, UiDeclaredMeasurementMode};
pub use crate::facade::entry::{
    WorthUiMountedAllocationCertificationExt, WorthUiMountedAllocationInspectionCertificationExt,
    WorthUiMountedIdentityCertificationExt, WorthUiMountedInteractionLifecycleCertificationExt,
};
pub use crate::graph::{
    UiGraphFactConsumerIdentity, UiGraphFactIndexBasis, UiGraphFactIndexEntry,
    UiGraphFactLookupCost, UiGraphFactLookupDenial, UiGraphFactLookupReceipt,
};
pub(crate) use activation_interruption::interrupt_if_armed;
pub use activation_interruption::{
    with_activation_precommit_interruption, WorthUiActivationPrecommitStage,
};
pub use active_session_observation::WorthUiActiveSessionCertificationExt;
pub use application_builder::WorthUiApplicationBuilderCertificationExt;
pub use application_graph::{
    UiRepeatedInstanceIdentityCertificationRow, WorthUiApplicationGraphCertificationExt,
};
pub use application_replacement::WorthUiApplicationReplacementCertificationExt;
pub use framework_turn_execution::WorthUiFrameworkTurnCertificationExt;
pub use identity_overlay_projection::{
    identity_overlay_projection_for_certification, UiIdentityOverlayProjectionCertificationMutation,
};
#[cfg(test)]
pub(crate) use layout_admission::snapshot_after_layout_admission_support;
pub use mounted_frame_execution::{
    UiMountedVisualOverlayLeaseCertificationReceipt, WorthUiMountedFrameExecutionCertificationExt,
    WorthUiMountedPublicationCertificationExt,
};
pub use planning::planning_pair_for_certification_suite;
pub use rebind_identity_lifecycle::{
    identity_lifecycle_decision_for_certification, UiIdentityLifecyclePresence,
    UiRebindPlanningBasisMutation, UiResolvedIdentityLifecycleCertificationExt,
    WorthUiNodeLifecycleTransition,
};
pub use runtime_launch::launch_empty_runtime_for_certification;
pub use semantic_text_projection::{
    semantic_text_projection_for_certification,
    semantic_text_projection_for_certification_with_capability,
    UiSemanticTextProjectionCertificationMutation,
};
pub use touch_origin::{
    runtime_origin_fixture, WorthUiTouchOriginCertificationFixture,
    WorthUiTouchOriginFixtureVariant,
};
