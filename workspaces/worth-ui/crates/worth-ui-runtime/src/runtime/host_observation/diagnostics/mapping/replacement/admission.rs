use super::digest::{fold, phase_denial_diagnostic, runtime_posture_digest};
use crate::runtime::host_observation::diagnostics::{
    WorthUiRuntimeDiagnostic, WorthUiRuntimeDiagnosticCode, WorthUiRuntimeDiagnosticFamily,
};
use crate::runtime::WorthUiCandidateAdmissionDenial;

pub(crate) fn diagnostic_for_candidate_admission(
    denial: &WorthUiCandidateAdmissionDenial,
) -> WorthUiRuntimeDiagnostic {
    phase_denial_diagnostic(
        WorthUiRuntimeDiagnosticFamily::CandidateAdmission,
        WorthUiRuntimeDiagnosticCode::CandidateAdmissionDenied,
        candidate_admission_digest(denial),
    )
}

pub(super) fn candidate_admission_digest(denial: &WorthUiCandidateAdmissionDenial) -> u64 {
    match denial {
        WorthUiCandidateAdmissionDenial::SnapshotMismatch {
            candidate_snapshot_digest,
            active_snapshot_digest,
        } => fold(
            fold(0xA0_00_00_01, *candidate_snapshot_digest),
            *active_snapshot_digest,
        ),
        WorthUiCandidateAdmissionDenial::DeferredRuntimePosture { posture } => {
            fold(0xA0_00_00_02, runtime_posture_digest(*posture))
        }
        WorthUiCandidateAdmissionDenial::UnsupportedRuntimePosture { posture } => {
            fold(0xA0_00_00_03, runtime_posture_digest(*posture))
        }
    }
}
