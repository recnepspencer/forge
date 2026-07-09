//! SUPPORT AUTHORITY — certification-consumer fixtures.
//!
//! Owned here (not `include!` from `runtime/tests`). External crates must consume fixtures only
//! through `worth-ui-test-support` (feature `certification-support`). Production law is not defined here.

mod layout_admission;
mod planning;
mod touch_origin;
mod touch_origin_source;

pub use crate::admission::{
    UiMeasurementAdmissionPosture, UiMeasurementCapabilityGateReason,
    UiMeasurementUnsupportedReason, UiQueryMeasurementBasisAuthority,
    UiQueryMeasurementEligibilityPosture, UiQueryMeasurementUnsupportedQueryReason,
};
pub use crate::declaration::UiDeclaredMeasurementMode;
#[cfg(test)]
pub(crate) use layout_admission::snapshot_after_layout_admission_support;
pub use planning::planning_pair_for_certification_suite;
pub use touch_origin::{
    runtime_origin_fixture, WorthUiTouchOriginCertificationFixture,
    WorthUiTouchOriginFixtureVariant,
};
