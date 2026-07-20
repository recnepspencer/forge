//! SUPPORT AUTHORITY — certification-consumer fixtures.
//!
//! Owned here (not `include!` from `runtime/tests`). External crates must consume fixtures only
//! through `worth-ui-test-support` (feature `certification-support`). Production law is not defined here.

mod activation_interruption;
mod layout_admission;
mod planning;
mod runtime_launch;
mod touch_origin;
mod touch_origin_source;

pub use crate::admission::{
    UiMeasurementAdmissionPosture, UiMeasurementCapabilityGateReason,
    UiMeasurementUnsupportedReason, UiQueryMeasurementBasisAuthority,
    UiQueryMeasurementEligibilityPosture, UiQueryMeasurementUnsupportedQueryReason,
};
pub use crate::declaration::UiDeclaredMeasurementMode;
pub(crate) use activation_interruption::interrupt_if_armed;
pub use activation_interruption::{
    with_activation_precommit_interruption, WorthUiActivationPrecommitStage,
};
#[cfg(test)]
pub(crate) use layout_admission::snapshot_after_layout_admission_support;
pub use planning::planning_pair_for_certification_suite;
pub use runtime_launch::launch_empty_runtime_for_certification;
pub use touch_origin::{
    runtime_origin_fixture, WorthUiTouchOriginCertificationFixture,
    WorthUiTouchOriginFixtureVariant,
};
