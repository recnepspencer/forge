use crate::construction::certification::profile::{
    prepare_primitive_construction_policy_pressure_delta_report,
    required_policy_pressure_delta_cases, required_policy_pressure_direct_cases,
    PrimitiveConstructionPolicyPressureCase, PrimitiveConstructionPolicyPressureDeltaCase,
    PrimitiveConstructionPolicyPressureDeltaReport,
    PrimitiveConstructionPolicyPressureDeltaReportError,
    PrimitiveConstructionPolicyPressureSurfaceReport,
};
use crate::construction::digest::{digest_owned_parts_with_scope, ConstructionDigestScope};

#[derive(Clone, Debug, PartialEq)]
pub struct PrimitiveConstructionPolicyPressureRepresentativeEvidence {
    direct_report: PrimitiveConstructionPolicyPressureSurfaceReport,
    delta_report: PrimitiveConstructionPolicyPressureDeltaReport,
    parity_verified: bool,
    report_digest: String,
}

impl PrimitiveConstructionPolicyPressureRepresentativeEvidence {
    fn new(
        direct_report: PrimitiveConstructionPolicyPressureSurfaceReport,
        delta_report: PrimitiveConstructionPolicyPressureDeltaReport,
    ) -> Result<Self, PrimitiveConstructionPolicyPressureRepresentativeEvidenceError> {
        let direct_rows_match = direct_report
            .rows()
            .iter()
            .map(|row| row.case())
            .eq(required_policy_pressure_direct_cases().iter().copied());
        if !direct_rows_match
            || required_policy_pressure_direct_cases()
                .iter()
                .any(|case| direct_report.row(*case).is_none())
        {
            return Err(
                PrimitiveConstructionPolicyPressureRepresentativeEvidenceError::DirectCoverageDrift,
            );
        }

        let delta_rows_match = delta_report
            .rows()
            .iter()
            .map(|row| row.case())
            .eq(required_policy_pressure_delta_cases().iter().copied());
        if !delta_rows_match
            || required_policy_pressure_delta_cases()
                .iter()
                .any(|case| delta_report.row(*case).is_none())
        {
            return Err(
                PrimitiveConstructionPolicyPressureRepresentativeEvidenceError::DeltaCoverageDrift,
            );
        }

        let parity_verified = direct_report.pressure_verified()
            && delta_report.delta_verified()
            && delta_report.direct_report() == &direct_report;
        if !parity_verified {
            return Err(
                PrimitiveConstructionPolicyPressureRepresentativeEvidenceError::ParityDrift,
            );
        }

        let report_digest = digest_owned_parts_with_scope(
            ConstructionDigestScope::ArtifactIdentity,
            &std::iter::once(direct_report.report_digest().to_string())
                .chain(std::iter::once(delta_report.report_digest().to_string()))
                .chain(
                    required_policy_pressure_direct_cases()
                        .iter()
                        .map(|case| format!("{case:?}")),
                )
                .chain(
                    required_policy_pressure_delta_cases()
                        .iter()
                        .map(|case| format!("{case:?}")),
                )
                .collect::<Vec<_>>(),
        );

        Ok(Self {
            direct_report,
            delta_report,
            parity_verified,
            report_digest,
        })
    }

    pub fn direct_report(&self) -> &PrimitiveConstructionPolicyPressureSurfaceReport {
        &self.direct_report
    }

    pub fn delta_report(&self) -> &PrimitiveConstructionPolicyPressureDeltaReport {
        &self.delta_report
    }

    pub fn required_direct_cases(&self) -> &'static [PrimitiveConstructionPolicyPressureCase] {
        required_policy_pressure_direct_cases()
    }

    pub fn required_delta_cases(&self) -> &'static [PrimitiveConstructionPolicyPressureDeltaCase] {
        required_policy_pressure_delta_cases()
    }

    pub fn parity_verified(&self) -> bool {
        self.parity_verified
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

pub(crate) fn prepare_primitive_construction_policy_pressure_representative_evidence() -> Result<
    PrimitiveConstructionPolicyPressureRepresentativeEvidence,
    PrimitiveConstructionPolicyPressureRepresentativeEvidenceError,
> {
    let delta_report = prepare_primitive_construction_policy_pressure_delta_report()
        .map_err(PrimitiveConstructionPolicyPressureRepresentativeEvidenceError::Delta)?;
    PrimitiveConstructionPolicyPressureRepresentativeEvidence::new(
        delta_report.direct_report().clone(),
        delta_report,
    )
}

#[derive(Debug)]
pub enum PrimitiveConstructionPolicyPressureRepresentativeEvidenceError {
    Delta(PrimitiveConstructionPolicyPressureDeltaReportError),
    DirectCoverageDrift,
    DeltaCoverageDrift,
    ParityDrift,
}

impl std::fmt::Display for PrimitiveConstructionPolicyPressureRepresentativeEvidenceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Delta(error) => write!(f, "{error}"),
            Self::DirectCoverageDrift => {
                write!(
                    f,
                    "policy pressure direct report drifted from required coverage"
                )
            }
            Self::DeltaCoverageDrift => {
                write!(
                    f,
                    "policy pressure delta report drifted from required coverage"
                )
            }
            Self::ParityDrift => write!(
                f,
                "policy pressure representative evidence lost direct/delta parity"
            ),
        }
    }
}

impl std::error::Error for PrimitiveConstructionPolicyPressureRepresentativeEvidenceError {}

#[cfg(test)]
mod tests {
    use super::prepare_primitive_construction_policy_pressure_representative_evidence;

    #[test]
    fn representative_evidence_replaces_policy_pressure_bundle() {
        let evidence = prepare_primitive_construction_policy_pressure_representative_evidence()
            .expect("representative evidence");

        assert!(evidence.parity_verified());
        assert_eq!(
            evidence.direct_report().rows().len(),
            evidence.required_direct_cases().len()
        );
        assert_eq!(
            evidence.delta_report().rows().len(),
            evidence.required_delta_cases().len()
        );
        assert_ne!(
            evidence.report_digest(),
            evidence.direct_report().report_digest()
        );
        assert_ne!(
            evidence.report_digest(),
            evidence.delta_report().report_digest()
        );
    }
}
