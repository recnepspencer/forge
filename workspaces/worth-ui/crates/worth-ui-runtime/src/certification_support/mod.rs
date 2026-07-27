//! SUPPORT AUTHORITY — certification-consumer fixtures.
//!
//! Owned here (not `include!` from `runtime/tests`). External crates must consume fixtures only
//! through `worth-ui-test-support` (feature `certification-support`). Production law is not defined here.

mod activation_interruption;
mod active_session_observation;
mod application_builder;
mod application_replacement;
mod framework_turn_execution;
mod layout_admission;
mod mounted_frame_execution;
mod planning;
mod runtime_launch;
mod touch_origin;
mod touch_origin_source;

pub use crate::admission::{
    UiMeasurementAdmissionPosture, UiMeasurementCapabilityGateReason,
    UiMeasurementUnsupportedReason, UiQueryMeasurementEligibilityPosture,
    UiQueryMeasurementSourceIdentity, UiQueryMeasurementUnsupportedQueryReason,
};
pub use crate::declaration::UiDeclaredMeasurementMode;
pub use crate::facade::entry::{
    WorthUiMountedAllocationCertificationExt, WorthUiMountedAllocationInspectionCertificationExt,
    WorthUiMountedIdentityCertificationExt,
};
pub(crate) use activation_interruption::interrupt_if_armed;
pub use activation_interruption::{
    with_activation_precommit_interruption, WorthUiActivationPrecommitStage,
};
pub use active_session_observation::WorthUiActiveSessionCertificationExt;
pub use application_builder::WorthUiApplicationBuilderCertificationExt;
pub use application_replacement::WorthUiApplicationReplacementCertificationExt;
pub use framework_turn_execution::WorthUiFrameworkTurnCertificationExt;
#[cfg(test)]
pub(crate) use layout_admission::snapshot_after_layout_admission_support;
pub use mounted_frame_execution::{
    WorthUiMountedFrameExecutionCertificationExt, WorthUiMountedPublicationCertificationExt,
};
pub use planning::planning_pair_for_certification_suite;
pub use runtime_launch::launch_empty_runtime_for_certification;
pub use touch_origin::{
    runtime_origin_fixture, WorthUiTouchOriginCertificationFixture,
    WorthUiTouchOriginFixtureVariant,
};
