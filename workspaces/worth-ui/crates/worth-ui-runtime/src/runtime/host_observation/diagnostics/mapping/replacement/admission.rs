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
        WorthUiCandidateAdmissionDenial::DeferredQuerySupport { receipt } => {
            fold(0xA0_00_00_04, receipt.contract_identity().as_u64())
        }
        WorthUiCandidateAdmissionDenial::UnsupportedQuerySupport { receipt } => {
            fold(0xA0_00_00_05, receipt.contract_identity().as_u64())
        }
        WorthUiCandidateAdmissionDenial::QuerySupportContractChanged {
            admitted_contract_identity,
            current_contract_identity,
        } => fold(
            fold(0xA0_00_00_06, admitted_contract_identity.as_u64()),
            current_contract_identity.as_u64(),
        ),
    }
}
