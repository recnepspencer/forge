mod bundle;
mod pressure;
mod report;
mod suite;

#[cfg(test)]
mod bundle_tests;
#[cfg(test)]
mod pressure_bundle_tests;
#[cfg(test)]
mod pressure_delta_tests;
#[cfg(test)]
mod pressure_tests;

pub use bundle::{
    prepare_primitive_construction_policy_profile_bundle_from_combined_hostility_suite,
    prepare_primitive_construction_policy_profile_bundle_from_hostility_suites,
    prepare_primitive_construction_policy_profile_report_bundle,
    PrimitiveConstructionPolicyProfileReportBundle,
    PrimitiveConstructionPolicyProfileReportBundleError,
};
pub use pressure::{
    prepare_primitive_construction_policy_pressure_delta_report,
    prepare_primitive_construction_policy_pressure_report,
    prepare_primitive_construction_policy_pressure_report_bundle,
    PrimitiveConstructionPolicyPressureBundleVerificationFailure,
    PrimitiveConstructionPolicyPressureBundleVerificationMismatch,
    PrimitiveConstructionPolicyPressureCanonicalTruth, PrimitiveConstructionPolicyPressureCase,
    PrimitiveConstructionPolicyPressureDeltaCase, PrimitiveConstructionPolicyPressureDeltaReport,
    PrimitiveConstructionPolicyPressureDeltaReportError,
    PrimitiveConstructionPolicyPressureDeltaRow, PrimitiveConstructionPolicyPressureReportBundle,
    PrimitiveConstructionPolicyPressureReportBundleError, PrimitiveConstructionPolicyPressureRow,
    PrimitiveConstructionPolicyPressureSetup, PrimitiveConstructionPolicyPressureSurfaceReport,
    PrimitiveConstructionPolicyPressureSurfaceReportError,
};
pub(crate) use report::prepare_primitive_construction_policy_profile_row;
pub use report::{
    prepare_primitive_construction_policy_profile_report, PrimitiveConstructionPolicyProfileCase,
    PrimitiveConstructionPolicyProfileRow, PrimitiveConstructionPolicyProfileSurfaceReport,
};
pub use suite::{
    prepare_primitive_construction_preview_continuity_hostility_suite_report,
    PrimitiveConstructionPreviewContinuityHostilityCase,
    PrimitiveConstructionPreviewContinuityHostilityRow,
    PrimitiveConstructionPreviewContinuityHostilitySuiteError,
    PrimitiveConstructionPreviewContinuityHostilitySuiteReport,
};
