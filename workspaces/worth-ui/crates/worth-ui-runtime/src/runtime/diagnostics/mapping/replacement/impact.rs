use super::admission::candidate_admission_digest;
use super::digest::{fold, phase_denial_diagnostic};
use crate::runtime::diagnostics::{
    WorthUiRuntimeDiagnostic, WorthUiRuntimeDiagnosticCode, WorthUiRuntimeDiagnosticFamily,
};
use crate::runtime::WorthUiReplacementImpactDenial;

pub(crate) fn diagnostic_for_replacement_impact(
    denial: &WorthUiReplacementImpactDenial,
) -> WorthUiRuntimeDiagnostic {
    phase_denial_diagnostic(
        WorthUiRuntimeDiagnosticFamily::ReplacementImpact,
        WorthUiRuntimeDiagnosticCode::ReplacementImpactDenied,
        replacement_impact_digest(denial),
    )
}

fn replacement_impact_digest(denial: &WorthUiReplacementImpactDenial) -> u64 {
    match denial {
        WorthUiReplacementImpactDenial::ComparisonActiveBasisMismatch {
            comparison_active_artifact_digest,
            admitted_active_artifact_digest,
            counters,
        } => fold(
            fold(
                fold(0xA2_00_00_01, *comparison_active_artifact_digest),
                *admitted_active_artifact_digest,
            ),
            counters.impact_classifications_attempted() as u64,
        ),
        WorthUiReplacementImpactDenial::ComparisonCandidateMismatch {
            comparison_candidate_artifact_digest,
            admitted_candidate_artifact_digest,
            counters,
        } => fold(
            fold(
                fold(0xA2_00_00_02, *comparison_candidate_artifact_digest),
                *admitted_candidate_artifact_digest,
            ),
            counters.impact_classifications_attempted() as u64,
        ),
        WorthUiReplacementImpactDenial::AdmissionReceiptChanged { denial, counters } => fold(
            fold(0xA2_00_00_03, candidate_admission_digest(denial)),
            counters.impact_classifications_attempted() as u64,
        ),
        WorthUiReplacementImpactDenial::UnsupportedImpact { counters, .. } => {
            fold(0xA2_00_00_04, counters.broad_replacement_denials() as u64)
        }
    }
}
