use super::super::evidence::S0ArtifactKind;
use super::certification::{
    certification_evidence_ref, S0CertificationMatrixRow, S0CertificationStatus,
};
use super::certification_matrix::CertificationInputs;
use super::validation::S0EvidenceBundleBuildRejection;

pub(super) fn build_contract_rows(
    inputs: &CertificationInputs<'_>,
) -> Result<Vec<S0CertificationMatrixRow>, S0EvidenceBundleBuildRejection> {
    Ok(vec![
        S0CertificationMatrixRow::new(
            "complexity_contracts_verified",
            "All required S.0 complexity contracts are present without debt.",
            if inputs.counters.complexity_debt_count() == 0
                && inputs.counters.missing_complexity_contract_count() == 0
                && inputs.counters.duplicate_complexity_contract_count() == 0
            {
                S0CertificationStatus::Verified
            } else {
                S0CertificationStatus::Blocking
            },
            vec![certification_evidence_ref(
                S0ArtifactKind::S1HandoffReadiness,
                inputs
                    .s1_handoff
                    .handoff()
                    .envelope()
                    .deterministic_digest(),
            )],
        )?,
        S0CertificationMatrixRow::new(
            "roadmap_sequence_consistency_verified",
            "Prior milestone sequence state is reconciled or explicitly waived.",
            if inputs.counters.sequence_inconsistency_count() == 0 {
                S0CertificationStatus::Verified
            } else {
                S0CertificationStatus::Blocking
            },
            vec![certification_evidence_ref(
                S0ArtifactKind::MilestonePhysicalStatusMatrix,
                inputs
                    .milestone_matrix
                    .matrix()
                    .envelope()
                    .deterministic_digest(),
            )],
        )?,
        S0CertificationMatrixRow::new(
            "milestones_1_through_13_3_status_rows_complete",
            "Every declared milestone has a physical-status row.",
            if inputs.counters.missing_milestone_status_row_count() == 0 {
                S0CertificationStatus::Verified
            } else {
                S0CertificationStatus::Blocking
            },
            vec![certification_evidence_ref(
                S0ArtifactKind::MilestonePhysicalStatusMatrix,
                inputs
                    .milestone_matrix
                    .matrix()
                    .envelope()
                    .deterministic_digest(),
            )],
        )?,
    ])
}
