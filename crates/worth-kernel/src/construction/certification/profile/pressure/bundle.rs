use super::delta::{
    prepare_primitive_construction_policy_pressure_delta_report_from_direct_report,
    PrimitiveConstructionPolicyPressureDeltaCase, PrimitiveConstructionPolicyPressureDeltaReport,
    PrimitiveConstructionPolicyPressureDeltaReportError,
};
use super::report::{
    prepare_primitive_construction_policy_pressure_report, PrimitiveConstructionPolicyPressureCase,
    PrimitiveConstructionPolicyPressureSurfaceReport,
    PrimitiveConstructionPolicyPressureSurfaceReportError,
};
use crate::construction::digest::digest_owned_parts;

#[derive(Clone, Debug, PartialEq)]
pub struct PrimitiveConstructionPolicyPressureReportBundle {
    direct_report: PrimitiveConstructionPolicyPressureSurfaceReport,
    delta_report: PrimitiveConstructionPolicyPressureDeltaReport,
    direct_cases_present: bool,
    delta_cases_present: bool,
    parity_verified: bool,
    report_digest: String,
}

impl PrimitiveConstructionPolicyPressureReportBundle {
    fn new(
        direct_report: PrimitiveConstructionPolicyPressureSurfaceReport,
        delta_report: PrimitiveConstructionPolicyPressureDeltaReport,
    ) -> Self {
        let direct_cases_present = required_direct_cases()
            .iter()
            .all(|case| direct_report.row(*case).is_some());
        let delta_cases_present = required_delta_cases()
            .iter()
            .all(|case| delta_report.row(*case).is_some());
        let parity_verified = direct_report.pressure_verified()
            && delta_report.delta_verified()
            && direct_cases_present
            && delta_cases_present
            && delta_report.direct_report().report_digest() == direct_report.report_digest();
        let report_digest = digest_owned_parts(&[
            direct_report.report_digest().to_string(),
            delta_report.report_digest().to_string(),
            direct_cases_present.to_string(),
            delta_cases_present.to_string(),
            parity_verified.to_string(),
        ]);
        Self {
            direct_report,
            delta_report,
            direct_cases_present,
            delta_cases_present,
            parity_verified,
            report_digest,
        }
    }

    pub fn direct_report(&self) -> &PrimitiveConstructionPolicyPressureSurfaceReport {
        &self.direct_report
    }

    pub fn delta_report(&self) -> &PrimitiveConstructionPolicyPressureDeltaReport {
        &self.delta_report
    }

    pub fn required_direct_cases(&self) -> &[PrimitiveConstructionPolicyPressureCase] {
        required_direct_cases()
    }

    pub fn required_delta_cases(&self) -> &[PrimitiveConstructionPolicyPressureDeltaCase] {
        required_delta_cases()
    }

    pub fn direct_cases_present(&self) -> bool {
        self.direct_cases_present
    }

    pub fn delta_cases_present(&self) -> bool {
        self.delta_cases_present
    }

    pub fn parity_verified(&self) -> bool {
        self.parity_verified
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

#[derive(Debug)]
pub enum PrimitiveConstructionPolicyPressureReportBundleError {
    Direct(PrimitiveConstructionPolicyPressureSurfaceReportError),
    Delta(PrimitiveConstructionPolicyPressureDeltaReportError),
}

impl std::fmt::Display for PrimitiveConstructionPolicyPressureReportBundleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Direct(error) => write!(f, "{error}"),
            Self::Delta(error) => write!(f, "{error}"),
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
    Ok(PrimitiveConstructionPolicyPressureReportBundle::new(
        direct_report,
        delta_report,
    ))
}

fn required_direct_cases() -> &'static [PrimitiveConstructionPolicyPressureCase] {
    &[
        PrimitiveConstructionPolicyPressureCase::GrazingAskFirst,
        PrimitiveConstructionPolicyPressureCase::GrazingPreserveAmbiguity,
        PrimitiveConstructionPolicyPressureCase::GrazingAggressiveSnap,
        PrimitiveConstructionPolicyPressureCase::GrazingAggressiveSnapHighFidelity,
        PrimitiveConstructionPolicyPressureCase::HostFaceAskFirst,
        PrimitiveConstructionPolicyPressureCase::HostFaceBimHostFriendly,
        PrimitiveConstructionPolicyPressureCase::HostFaceBimHostHighFidelityAskFirst,
    ]
}

fn required_delta_cases() -> &'static [PrimitiveConstructionPolicyPressureDeltaCase] {
    &[
        PrimitiveConstructionPolicyPressureDeltaCase::GrazingClarificationVsPreservedAmbiguity,
        PrimitiveConstructionPolicyPressureDeltaCase::GrazingClarificationVsAggressiveSnap,
        PrimitiveConstructionPolicyPressureDeltaCase::GrazingAggressiveSnapVsHighFidelity,
        PrimitiveConstructionPolicyPressureDeltaCase::HostFaceAskFirstVsBimHostFriendly,
        PrimitiveConstructionPolicyPressureDeltaCase::HostFaceBimHostFriendlyVsHighFidelityAskFirst,
    ]
}
