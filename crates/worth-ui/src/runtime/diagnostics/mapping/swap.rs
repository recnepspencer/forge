use crate::runtime::diagnostics::{
    WorthUiDiagnosticSource, WorthUiRuntimeDiagnostic, WorthUiRuntimeDiagnosticCode,
    WorthUiRuntimeDiagnosticFamily,
};
use crate::runtime::WorthUiPlanSwapRollback;

pub(crate) fn diagnostic_for_plan_swap(
    rollback: WorthUiPlanSwapRollback,
) -> WorthUiRuntimeDiagnostic {
    let evidence_digest = rollback.restored_active_artifact_digest()
        ^ rollback.restored_active_plan_digest().rotate_left(17)
        ^ rollback
            .attempted_next_artifact_digest()
            .unwrap_or(0)
            .rotate_left(31)
        ^ rollback
            .attempted_next_plan_digest()
            .unwrap_or(0)
            .rotate_left(43);
    WorthUiRuntimeDiagnostic::new(
        WorthUiRuntimeDiagnosticFamily::AtomicPlanSwap,
        WorthUiRuntimeDiagnosticCode::AtomicPlanSwapRolledBack,
        WorthUiDiagnosticSource::PhaseDenial { evidence_digest },
        Some(evidence_digest),
    )
}
