use crate::runtime::host_observation::diagnostics::{
    WorthUiDiagnosticSource, WorthUiRuntimeDiagnostic, WorthUiRuntimeDiagnosticCode,
    WorthUiRuntimeDiagnosticFamily,
};
use crate::runtime::UiCommittedAllocationActivationDenial;

pub(crate) fn diagnostic_for_committed_allocation_denial(
    denial: &UiCommittedAllocationActivationDenial,
) -> WorthUiRuntimeDiagnostic {
    let evidence_digest = denial.attempt_identity_digest()
        ^ (denial.evidence().committed_row_count() as u64).rotate_left(17)
        ^ u64::from(denial.evidence().counters().denial_count()).rotate_left(31);
    WorthUiRuntimeDiagnostic::new(
        WorthUiRuntimeDiagnosticFamily::CommittedAllocationActivation,
        WorthUiRuntimeDiagnosticCode::CommittedAllocationActivationDenied,
        WorthUiDiagnosticSource::PhaseDenial { evidence_digest },
        Some(evidence_digest),
    )
}
