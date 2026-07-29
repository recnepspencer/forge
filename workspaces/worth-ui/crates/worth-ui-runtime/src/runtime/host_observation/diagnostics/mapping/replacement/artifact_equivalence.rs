use super::digest::{fold, phase_denial_diagnostic};
use crate::runtime::host_observation::diagnostics::{
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
        WorthUiRuntimeArtifactComparisonDenial::EquivalenceBasisMismatch {
            candidate_basis,
            counters,
            ..
        } => fold(
            fold(0xA1_00_00_02, candidate_basis.artifact_digest().raw()),
            counters.artifact_comparisons() as u64,
        ),
        WorthUiRuntimeArtifactComparisonDenial::StructuralCapacityExceeded {
            limit,
            observed,
            counters,
        } => fold(
            fold(fold(0xA1_00_00_03, *limit as u64), *observed as u64),
            counters.artifact_comparisons() as u64,
        ),
    }
}
