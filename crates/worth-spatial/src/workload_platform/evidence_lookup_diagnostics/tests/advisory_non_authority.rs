use crate::workload_platform::evidence_ledger::WorkloadEvidenceStage;
use crate::workload_platform::evidence_lookup_diagnostics::{
    derive_evidence_lookup_diagnostics, EvidenceLookupDiagnosticAdvisoryReason,
    EvidenceLookupDiagnosticWitness,
};
use crate::workload_platform::evidence_lookup_family_catalog::current_evidence_lookup_family_catalog;
use crate::workload_platform::evidence_lookup_stage_cutover::current_path::admit_current_family_stage_cutover_path;

#[test]
fn advisory_lookup_posture_stays_non_authoritative() {
    let catalog = current_evidence_lookup_family_catalog().expect("catalog closes");
    let family = catalog
        .family_by_identity("spatial-touch.boolean.event-ledger-evidence.v1")
        .expect("event ledger family");
    let witness = admit_current_family_stage_cutover_path(
        &catalog,
        family,
        WorkloadEvidenceStage::BooleanEventLedger,
    )
    .expect("current path");

    let diagnostics =
        derive_evidence_lookup_diagnostics(witness.selected_plan(), witness.execution_receipt())
            .expect("diagnostics derive");
    let overlap_row = diagnostics
        .require_family_stage_witness(
            "spatial-touch.boolean.overlap-evidence.v1",
            WorkloadEvidenceStage::BooleanEventLedger,
        )
        .expect("unaffected overlap row");

    assert!(matches!(
        overlap_row.witness(),
        EvidenceLookupDiagnosticWitness::Advisory(
            EvidenceLookupDiagnosticAdvisoryReason::UnaffectedFamily
        )
    ));
    assert!(!overlap_row.claims_lookup_execution_authority());
    assert!(!overlap_row.claims_query_descriptor_authority());
}
