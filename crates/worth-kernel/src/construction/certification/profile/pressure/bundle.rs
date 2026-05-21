use super::bundle_verified::{
    verify_bundle, PrimitiveConstructionPolicyPressureBundleVerificationFailure,
    PrimitiveConstructionPolicyPressureReportBundle,
    PrimitiveConstructionPolicyPressureUnverifiedBundle,
};
use super::delta::{
    prepare_primitive_construction_policy_pressure_delta_report_from_direct_report,
    PrimitiveConstructionPolicyPressureDeltaReportError,
};
use super::report::{
    prepare_primitive_construction_policy_pressure_report,
    PrimitiveConstructionPolicyPressureSurfaceReportError,
};

#[derive(Debug)]
pub enum PrimitiveConstructionPolicyPressureReportBundleError {
    Direct(PrimitiveConstructionPolicyPressureSurfaceReportError),
    Delta(PrimitiveConstructionPolicyPressureDeltaReportError),
    Verification(PrimitiveConstructionPolicyPressureBundleVerificationFailure),
}

impl std::fmt::Display for PrimitiveConstructionPolicyPressureReportBundleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Direct(error) => write!(f, "{error}"),
            Self::Delta(error) => write!(f, "{error}"),
            Self::Verification(failure) => write!(
                f,
                "policy pressure bundle failed coherence verification: {:?}",
                failure.mismatches()
            ),
        }
    }
}

impl std::error::Error for PrimitiveConstructionPolicyPressureReportBundleError {}

pub fn prepare_primitive_construction_policy_pressure_report_bundle() -> Result<
    PrimitiveConstructionPolicyPressureReportBundle,
    PrimitiveConstructionPolicyPressureReportBundleError,
> {
    let direct_report = prepare_primitive_construction_policy_pressure_report()
        .map_err(PrimitiveConstructionPolicyPressureReportBundleError::Direct)?;
    let delta_report =
        prepare_primitive_construction_policy_pressure_delta_report_from_direct_report(
            direct_report.clone(),
        )
        .map_err(PrimitiveConstructionPolicyPressureReportBundleError::Delta)?;
    verify_bundle(PrimitiveConstructionPolicyPressureUnverifiedBundle::new(
        direct_report,
        delta_report,
    ))
    .map_err(PrimitiveConstructionPolicyPressureReportBundleError::Verification)
}
