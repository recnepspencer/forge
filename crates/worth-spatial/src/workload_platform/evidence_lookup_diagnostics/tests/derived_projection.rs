use crate::workload_platform::evidence_ledger::WorkloadEvidenceStage;
use crate::workload_platform::evidence_lookup_diagnostics::{
    derive_evidence_lookup_diagnostics, EvidenceLookupDiagnosticQuerySurfaceProvenance,
    EvidenceLookupDiagnosticWitness, EvidenceLookupDiagnosticsErrorKind,
};

use super::fixtures::{
    supported_projection_path, supported_projection_path_with_extra_unrelated_receipts,
};

#[test]
fn lookup_diagnostics_are_derived_from_plan_and_receipt() {
    let baseline = supported_projection_path();
    let expanded = supported_projection_path_with_extra_unrelated_receipts(3);

    let baseline_diagnostics =
        derive_evidence_lookup_diagnostics(baseline.selected_plan(), baseline.execution_receipt())
            .expect("baseline diagnostics derive");
    let expanded_diagnostics =
        derive_evidence_lookup_diagnostics(expanded.selected_plan(), expanded.execution_receipt())
            .expect("expanded diagnostics derive");
    let baseline_row = baseline_diagnostics
        .require_family_stage_witness(
            "spatial-touch.boolean.projection-consumption-evidence.v1",
            WorkloadEvidenceStage::BooleanOperandAProjectionConsumption,
        )
        .expect("baseline projection diagnostic row");
    let expanded_row = expanded_diagnostics
        .require_family_stage_witness(
            "spatial-touch.boolean.projection-consumption-evidence.v1",
            WorkloadEvidenceStage::BooleanOperandAProjectionConsumption,
        )
        .expect("expanded projection diagnostic row");

    assert_eq!(
        baseline_row.selected_plan_digest(),
        baseline.selected_plan().selected_plan_digest()
    );
    assert_eq!(
        baseline_row.execution_receipt_digest(),
        baseline.execution_receipt().execution_receipt_digest()
    );
    assert_eq!(
        baseline_row.query_surface_provenance(),
        Some(EvidenceLookupDiagnosticQuerySurfaceProvenance::ProjectionConsumption)
    );
    assert!(matches!(
        baseline_row.witness(),
        EvidenceLookupDiagnosticWitness::Success
    ));
    assert_eq!(baseline_row.row_digest(), expanded_row.row_digest());
    assert_eq!(
        baseline_diagnostics.diagnostic_digest(),
        expanded_diagnostics.diagnostic_digest()
    );
    assert_eq!(
        baseline_diagnostics.counters(),
        expanded_diagnostics.counters()
    );

    let missing_row = baseline_diagnostics.require_family_stage_witness(
        "spatial-touch.boolean.absent-evidence.v1",
        WorkloadEvidenceStage::BooleanOperandAProjectionConsumption,
    );
    assert_eq!(
        missing_row.expect_err("missing witness").kind(),
        EvidenceLookupDiagnosticsErrorKind::MissingFamilyStageWitness
    );
}
