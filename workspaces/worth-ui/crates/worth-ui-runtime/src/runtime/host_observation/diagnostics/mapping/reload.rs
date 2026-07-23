use crate::runtime::host_observation::diagnostics::{
    WorthUiDiagnosticSource, WorthUiRuntimeDiagnostic, WorthUiRuntimeDiagnosticCode,
    WorthUiRuntimeDiagnosticFamily,
};
use crate::runtime::{WorthUiReloadCheckedStopPosture, WorthUiReloadFailure};

pub(crate) fn diagnostic_for_reload_failure(
    failure: &WorthUiReloadFailure,
) -> WorthUiRuntimeDiagnostic {
    let denial = failure.denial();
    let code = if denial.checked_stop_posture() == WorthUiReloadCheckedStopPosture::ordinary() {
        match denial.stage() {
            crate::runtime::WorthUiReloadFailureStage::InvalidCandidate => {
                WorthUiRuntimeDiagnosticCode::InvalidCandidateDenied
            }
            _ => WorthUiRuntimeDiagnosticCode::ReloadFailurePreserved,
        }
    } else if denial.checked_stop_posture()
        == WorthUiReloadCheckedStopPosture::query_recovery_preserved()
    {
        WorthUiRuntimeDiagnosticCode::QueryRecoveryPreserved
    } else {
        WorthUiRuntimeDiagnosticCode::QueryLiveRebindDenied
    };
    WorthUiRuntimeDiagnostic::new(
        WorthUiRuntimeDiagnosticFamily::Reload,
        code,
        WorthUiDiagnosticSource::ReloadFailure {
            stage: denial.stage(),
            checked_stop_posture: denial.checked_stop_posture(),
            upstream_evidence_digest: denial.upstream_evidence_digest(),
        },
        denial.upstream_evidence_digest(),
    )
}
