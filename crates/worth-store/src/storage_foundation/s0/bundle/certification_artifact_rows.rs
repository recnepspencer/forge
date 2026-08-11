use super::super::evidence::{S0ArtifactKind, S0EvidenceRef};
use super::certification::{
    certification_evidence_ref, S0CertificationMatrixRow, S0CertificationStatus,
};
use super::certification_matrix::CertificationInputs;
use super::validation::S0EvidenceBundleBuildRejection;

pub(super) fn build_artifact_rows(
    inputs: &CertificationInputs<'_>,
) -> Result<Vec<S0CertificationMatrixRow>, S0EvidenceBundleBuildRejection> {
    Ok(vec![
        build_backend_classification_row(inputs)?,
        canonical_artifact_set_row(inputs)?,
    ])
}

fn build_backend_classification_row(
    inputs: &CertificationInputs<'_>,
) -> Result<S0CertificationMatrixRow, S0EvidenceBundleBuildRejection> {
    S0CertificationMatrixRow::new(
        "all_existing_backends_classified",
        "Backend capability rows exist for the first audit baseline families.",
        if inputs.backend_matrix.matrix().rows().len() >= 10 {
            S0CertificationStatus::Verified
        } else {
            S0CertificationStatus::Blocking
        },
        vec![certification_evidence_ref(
            S0ArtifactKind::BackendCapabilityMatrix,
            inputs
                .backend_matrix
                .matrix()
                .envelope()
                .deterministic_digest(),
        )],
    )
}

fn canonical_artifact_set_row(
    inputs: &CertificationInputs<'_>,
) -> Result<S0CertificationMatrixRow, S0EvidenceBundleBuildRejection> {
    S0CertificationMatrixRow::new(
        "canonical_artifact_set_parseable",
        "Required canonical artifact set is present and schema-compatible.",
        if inputs.artifact_validation.is_complete() {
            S0CertificationStatus::Verified
        } else {
            S0CertificationStatus::Blocking
        },
        canonical_artifact_evidence_refs(inputs),
    )
}

fn canonical_artifact_evidence_refs(inputs: &CertificationInputs<'_>) -> Vec<S0EvidenceRef> {
    let mut refs = canonical_source_artifact_refs(inputs);
    refs.extend(canonical_audit_artifact_refs(inputs));
    refs
}

fn canonical_source_artifact_refs(inputs: &CertificationInputs<'_>) -> Vec<S0EvidenceRef> {
    vec![
        certification_evidence_ref(
            S0ArtifactKind::BackendCapabilityMatrix,
            inputs
                .backend_matrix
                .matrix()
                .envelope()
                .deterministic_digest(),
        ),
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
    ]
}

fn canonical_audit_artifact_refs(inputs: &CertificationInputs<'_>) -> Vec<S0EvidenceRef> {
    vec![
        certification_evidence_ref(
            S0ArtifactKind::TerminologyRiskReport,
            inputs
                .terminology_report
                .report()
                .envelope()
                .deterministic_digest(),
        ),
        certification_evidence_ref(
            S0ArtifactKind::TestMigrationNotes,
            inputs
                .migration_notes
                .report()
                .envelope()
                .deterministic_digest(),
        ),
        certification_evidence_ref(
            S0ArtifactKind::HarnessMaturityReport,
            inputs
                .harness_report
                .report()
                .envelope()
                .deterministic_digest(),
        ),
        certification_evidence_ref(
            S0ArtifactKind::S1HandoffReadiness,
            inputs
                .s1_handoff
                .handoff()
                .envelope()
                .deterministic_digest(),
        ),
    ]
}
