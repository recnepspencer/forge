use super::super::evidence::S0ArtifactKind;
use super::super::handoff::S1BlockingPredicateStatus;
use super::certification::{
    certification_evidence_ref, S0CertificationMatrixRow, S0CertificationStatus,
};
use super::certification_matrix::CertificationInputs;
use super::validation::S0EvidenceBundleBuildRejection;

pub(super) fn build_handoff_rows(
    inputs: &CertificationInputs<'_>,
) -> Result<Vec<S0CertificationMatrixRow>, S0EvidenceBundleBuildRejection> {
    Ok(vec![
        build_s1_handoff_row(inputs)?,
        build_status_matrix_digest_row(inputs)?,
    ])
}

fn build_s1_handoff_row(
    inputs: &CertificationInputs<'_>,
) -> Result<S0CertificationMatrixRow, S0EvidenceBundleBuildRejection> {
    S0CertificationMatrixRow::new(
        "s1_handoff_blocks_missing_inputs",
        "S.1 handoff blocking predicates are all satisfied for accepted inputs.",
        if inputs
            .s1_handoff
            .handoff()
            .blocking_predicates()
            .iter()
            .all(|row| row.status() == S1BlockingPredicateStatus::Satisfied)
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
    )
}

fn build_status_matrix_digest_row(
    inputs: &CertificationInputs<'_>,
) -> Result<S0CertificationMatrixRow, S0EvidenceBundleBuildRejection> {
    S0CertificationMatrixRow::new(
        "status_matrix_digest_changes_on_claim_change",
        "Milestone matrix digest is stable and claim-sensitive.",
        if !inputs
            .milestone_matrix
            .matrix()
            .envelope()
            .deterministic_digest()
            .as_str()
            .is_empty()
        {
            S0CertificationStatus::Verified
        } else {
            S0CertificationStatus::Blocking
        },
        vec![
            certification_evidence_ref(
                S0ArtifactKind::MilestonePhysicalStatusMatrix,
                inputs
                    .milestone_matrix
                    .matrix()
                    .envelope()
                    .deterministic_digest(),
            ),
            certification_evidence_ref(
                S0ArtifactKind::SemanticPhysicalClaimReport,
                inputs
                    .claim_report
                    .report()
                    .envelope()
                    .deterministic_digest(),
            ),
            certification_evidence_ref(
                S0ArtifactKind::DeferredPhysicalGuaranteeMap,
                inputs.deferred_map.map().envelope().deterministic_digest(),
            ),
        ],
    )
}
