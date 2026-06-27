use crate::workload_platform::evidence_ledger::WorkloadEvidenceStage;
use crate::workload_platform::evidence_lookup_diagnostics::{
    derive_evidence_lookup_diagnostics, EvidenceLookupDiagnosticDenialReason,
    EvidenceLookupDiagnosticWitness,
};

use super::fixtures::{
    alternate_spatial_projection_path, event_path, missing_projection_fact_path,
    product_swap_projection_receipt, required_support_projection_path, supported_projection_path,
};

#[test]
fn denial_witness_preserves_query_and_spatial_posture() {
    let supported = supported_projection_path();
    let required_support = required_support_projection_path();
    let missing_projection_fact = missing_projection_fact_path();
    let wrong_stage = event_path();
    let wrong_spatial = alternate_spatial_projection_path();
    let product_swap = product_swap_projection_receipt();

    assert_denial_reason(
        required_support.selected_plan(),
        required_support.execution_receipt(),
        EvidenceLookupDiagnosticDenialReason::RequiredQuerySupport,
    );
    assert_denial_reason(
        missing_projection_fact.selected_plan(),
        missing_projection_fact.execution_receipt(),
        EvidenceLookupDiagnosticDenialReason::MissingProjectionConsumptionFact,
    );
    assert_denial_reason(
        supported.selected_plan(),
        wrong_stage.execution_receipt(),
        EvidenceLookupDiagnosticDenialReason::WrongStageReceiptIdentity,
    );
    assert_denial_reason(
        supported.selected_plan(),
        wrong_spatial.execution_receipt(),
        EvidenceLookupDiagnosticDenialReason::WrongSpatialTouchDigest,
    );
    assert_denial_reason(
        supported.selected_plan(),
        &product_swap,
        EvidenceLookupDiagnosticDenialReason::ProductSwapDetected,
    );
}

fn assert_denial_reason(
    selected_plan: &crate::workload_platform::evidence_lookup_plan_selection::EvidenceLookupSelectedPlan,
    execution_receipt: &crate::workload_platform::evidence_lookup_execution::EvidenceLookupExecutionReceipt,
    expected_reason: EvidenceLookupDiagnosticDenialReason,
) {
    let diagnostics =
        derive_evidence_lookup_diagnostics(selected_plan, execution_receipt).expect("diagnostics");
    let row = diagnostics
        .require_family_stage_witness(
            "spatial-touch.boolean.projection-consumption-evidence.v1",
            WorkloadEvidenceStage::BooleanOperandAProjectionConsumption,
        )
        .expect("projection diagnostic row");
    assert_eq!(
        row.witness(),
        EvidenceLookupDiagnosticWitness::Denied(expected_reason)
    );
}
