//! SUPPORT AUTHORITY — synthetic fixtures for falsifying production surfaces.
//!
//! Sole public home for cross-crate certification fixtures. Do not import
//! `worth_ui_runtime::certification_support` from product code.

pub use worth_ui_runtime::certification_support::{
    launch_empty_runtime_for_certification, planning_pair_for_certification_suite,
    runtime_origin_fixture, UiDeclaredMeasurementMode, UiMeasurementAdmissionPosture,
    UiMeasurementCapabilityGateReason, UiMeasurementUnsupportedReason,
    UiQueryMeasurementBasisAuthority, UiQueryMeasurementEligibilityPosture,
    UiQueryMeasurementUnsupportedQueryReason, WorthUiTouchOriginCertificationFixture,
    WorthUiTouchOriginFixtureVariant,
};
