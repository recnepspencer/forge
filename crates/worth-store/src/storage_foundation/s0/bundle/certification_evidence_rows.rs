use super::super::evidence::S0ArtifactKind;
use super::certification::{
    certification_evidence_ref, S0CertificationMatrixRow, S0CertificationStatus,
};
use super::certification_matrix::CertificationInputs;
use super::validation::S0EvidenceBundleBuildRejection;

pub(super) fn build_evidence_rows(
    inputs: &CertificationInputs<'_>,
) -> Result<Vec<S0CertificationMatrixRow>, S0EvidenceBundleBuildRejection> {
    Ok(vec![
        S0CertificationMatrixRow::new(
            "release_claim_gate_rejects_overclaim",
            "Release/public claim surfaces remain qualified.",
            if inputs.counters.unqualified_release_claim_count() == 0 {
                S0CertificationStatus::Verified
            } else {
                S0CertificationStatus::Blocking
            },
            vec![certification_evidence_ref(
                S0ArtifactKind::TerminologyRiskReport,
                inputs
                    .terminology_report
                    .report()
                    .envelope()
                    .deterministic_digest(),
            )],
        )?,
        S0CertificationMatrixRow::new(
            "test_evidence_scope_declared",
            "Existing named suites carry explicit semantic-versus-physical scope.",
            if !inputs.migration_notes.report().rows().is_empty() {
                S0CertificationStatus::Verified
            } else {
                S0CertificationStatus::Blocking
            },
            vec![certification_evidence_ref(
                S0ArtifactKind::TestMigrationNotes,
                inputs
                    .migration_notes
                    .report()
                    .envelope()
                    .deterministic_digest(),
            )],
        )?,
        S0CertificationMatrixRow::new(
            "harness_maturity_rows_present",
            "Required harness maturity rows are visible before S.1 closeout.",
            if !inputs.harness_report.report().rows().is_empty() {
                S0CertificationStatus::Verified
            } else {
                S0CertificationStatus::Blocking
            },
            vec![certification_evidence_ref(
                S0ArtifactKind::HarnessMaturityReport,
                inputs
                    .harness_report
                    .report()
                    .envelope()
                    .deterministic_digest(),
            )],
        )?,
    ])
}
