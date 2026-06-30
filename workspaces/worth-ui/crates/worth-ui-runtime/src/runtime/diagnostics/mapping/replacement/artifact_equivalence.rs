use super::admission::candidate_admission_digest;
use super::digest::{fold, phase_denial_diagnostic, query_support_status_digest};
use crate::runtime::diagnostics::{
    WorthUiRuntimeDiagnostic, WorthUiRuntimeDiagnosticCode, WorthUiRuntimeDiagnosticFamily,
};
use crate::runtime::WorthUiRuntimeArtifactComparisonDenial;

pub(crate) fn diagnostic_for_artifact_equivalence(
    denial: &WorthUiRuntimeArtifactComparisonDenial,
) -> WorthUiRuntimeDiagnostic {
    phase_denial_diagnostic(
        WorthUiRuntimeDiagnosticFamily::ArtifactEquivalence,
        WorthUiRuntimeDiagnosticCode::ArtifactEquivalenceDenied,
        artifact_equivalence_digest(denial),
    )
}

fn artifact_equivalence_digest(denial: &WorthUiRuntimeArtifactComparisonDenial) -> u64 {
    match denial {
        WorthUiRuntimeArtifactComparisonDenial::AdmissionReceiptChanged { denial, counters } => {
            fold(
                fold(0xA1_00_00_01, candidate_admission_digest(denial)),
                counters.artifact_comparisons() as u64,
            )
        }
        WorthUiRuntimeArtifactComparisonDenial::EquivalenceBasisMismatch {
            candidate_basis,
            candidate_query_support_status,
            counters,
            ..
        } => fold(
            fold(
                fold(0xA1_00_00_02, candidate_basis.artifact_digest().raw()),
                query_support_status_digest(*candidate_query_support_status),
            ),
            counters.artifact_comparisons() as u64,
        ),
    }
}
