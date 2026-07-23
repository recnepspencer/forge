use crate::runtime::host_observation::diagnostics::{
    WorthUiDiagnosticSource, WorthUiRuntimeDiagnostic, WorthUiRuntimeDiagnosticCode,
    WorthUiRuntimeDiagnosticFamily,
};
use crate::runtime::{WorthUiPlanInspectionDenial, WorthUiPlanLoweringDenial};

pub(crate) fn diagnostic_for_plan_lowering(
    denial: &WorthUiPlanLoweringDenial,
) -> WorthUiRuntimeDiagnostic {
    let evidence_digest = denial.active_artifact_digest()
        ^ denial.candidate_artifact_digest().rotate_left(11)
        ^ denial.pending_frame_epoch().as_u64().rotate_left(23)
        ^ denial.active_frame_epoch().as_u64().rotate_left(37);
    WorthUiRuntimeDiagnostic::new(
        WorthUiRuntimeDiagnosticFamily::PlanLowering,
        WorthUiRuntimeDiagnosticCode::PlanLoweringDenied,
        WorthUiDiagnosticSource::PhaseDenial { evidence_digest },
        Some(evidence_digest),
    )
}

pub(crate) fn diagnostic_for_plan_inspection(
    denial: &WorthUiPlanInspectionDenial,
) -> WorthUiRuntimeDiagnostic {
    let evidence_digest = fold(
        0xB3_00_00_01,
        plan_inspection_reason_digest(denial.reason()),
    );
    WorthUiRuntimeDiagnostic::new(
        WorthUiRuntimeDiagnosticFamily::PlanInspection,
        WorthUiRuntimeDiagnosticCode::PlanInspectionDenied,
        WorthUiDiagnosticSource::PhaseDenial { evidence_digest },
        Some(evidence_digest),
    )
}

fn plan_inspection_reason_digest(
    reason: crate::runtime::planning::plan_inspection::WorthUiPlanInspectionDenialReason,
) -> u64 {
    match reason {
        crate::runtime::planning::plan_inspection::WorthUiPlanInspectionDenialReason::ForeignLoweringAuthority => 1,
        crate::runtime::planning::plan_inspection::WorthUiPlanInspectionDenialReason::PlanInputReceiptMismatch => 2,
        crate::runtime::planning::plan_inspection::WorthUiPlanInspectionDenialReason::PlanInputNodeCountMismatch => 3,
        crate::runtime::planning::plan_inspection::WorthUiPlanInspectionDenialReason::PlanNodeFamilyMismatch => 4,
        crate::runtime::planning::plan_inspection::WorthUiPlanInspectionDenialReason::RuntimeHandlePlanIndexMismatch => 5,
    }
}

fn fold(mut digest: u64, value: u64) -> u64 {
    digest ^= value;
    digest.wrapping_mul(0x100000001b3)
}
