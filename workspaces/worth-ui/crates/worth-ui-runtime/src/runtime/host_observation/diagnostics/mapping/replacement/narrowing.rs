use super::admission::candidate_admission_digest;
use super::digest::{fold, phase_denial_diagnostic};
use crate::runtime::host_observation::diagnostics::{
    WorthUiRuntimeDiagnostic, WorthUiRuntimeDiagnosticCode, WorthUiRuntimeDiagnosticFamily,
};
use crate::runtime::WorthUiRuntimeImpactNarrowingDenial;

pub(crate) fn diagnostic_for_impact_narrowing(
    denial: &WorthUiRuntimeImpactNarrowingDenial,
) -> WorthUiRuntimeDiagnostic {
    phase_denial_diagnostic(
        WorthUiRuntimeDiagnosticFamily::ImpactNarrowing,
        WorthUiRuntimeDiagnosticCode::ImpactNarrowingDenied,
        impact_narrowing_digest(denial),
    )
}

fn impact_narrowing_digest(denial: &WorthUiRuntimeImpactNarrowingDenial) -> u64 {
    match denial {
        WorthUiRuntimeImpactNarrowingDenial::ClassificationActiveBasisMismatch {
            classification_active_artifact_digest,
            admitted_active_artifact_digest,
            counters,
        } => fold(
            fold(
                fold(0xA3_00_00_01, *classification_active_artifact_digest),
                *admitted_active_artifact_digest,
            ),
            counters.dependency_metadata_reads() as u64,
        ),
        WorthUiRuntimeImpactNarrowingDenial::ClassificationCandidateMismatch {
            classification_candidate_artifact_digest,
            admitted_candidate_artifact_digest,
            counters,
        } => fold(
            fold(
                fold(0xA3_00_00_02, *classification_candidate_artifact_digest),
                *admitted_candidate_artifact_digest,
            ),
            counters.dependency_metadata_reads() as u64,
        ),
        WorthUiRuntimeImpactNarrowingDenial::AdmissionReceiptChanged { denial, counters } => fold(
            fold(0xA3_00_00_03, candidate_admission_digest(denial)),
            counters.dependency_metadata_reads() as u64,
        ),
        WorthUiRuntimeImpactNarrowingDenial::QueryDependencyMetadataReceiptMismatch {
            receipt_runtime_hook_count,
            metadata_runtime_hook_count,
            counters,
        } => fold(
            fold(
                fold(0xA3_00_00_04, *receipt_runtime_hook_count as u64),
                *metadata_runtime_hook_count as u64,
            ),
            counters.runtime_hook_lookups() as u64,
        ),
        WorthUiRuntimeImpactNarrowingDenial::QueryDependencyPostureMissing {
            expected_runtime_hook_count,
            observed_runtime_hook_count,
            counters,
        } => fold(
            fold(
                fold(0xA3_00_00_05, *expected_runtime_hook_count as u64),
                *observed_runtime_hook_count as u64,
            ),
            counters.runtime_hook_lookups() as u64,
        ),
    }
}
