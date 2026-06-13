use worth_kernel::workload_composition::WorkloadCatalog;
use worth_spatial::facade::workload_vocabulary::{
    CompleteWorkloadEvidenceLedger, WorkloadEvidenceLedger, WorkloadEvidenceLedgerError,
    WorkloadEvidenceRow, WorkloadEvidenceStage,
};

#[test]
fn workload_catalog_blocks_static_fixture_substitution() {
    let built = WorkloadCatalog::cube()
        .declared("compile-time catalog boundary companion")
        .build()
        .expect("real catalog cube should build");

    assert_eq!(built.workload().evidence_ledger().counters().rows(), 8);
    assert!(built
        .workload()
        .evidence_ledger()
        .rows()
        .iter()
        .all(|row| row.is_receipt_backed()));

    let error = manually_substituted_topology_ledger(built.workload().evidence_ledger())
        .expect("static fixture ledger should still have valid row shape")
        .certify_complete()
        .expect_err("manual topology evidence must not certify as complete");

    assert_eq!(
        error,
        WorkloadEvidenceLedgerError::ManualAuthorityStage(WorkloadEvidenceStage::Topology)
    );
    assert_eq!(
        error.human_reason(),
        "workload evidence ledger has hand-filled topology evidence instead of a source receipt"
    );
}

fn manually_substituted_topology_ledger(
    ledger: &CompleteWorkloadEvidenceLedger,
) -> Result<WorkloadEvidenceLedger, WorkloadEvidenceLedgerError> {
    let rows = ledger
        .rows()
        .iter()
        .map(|row| {
            if row.stage() == WorkloadEvidenceStage::Topology {
                WorkloadEvidenceRow::new(WorkloadEvidenceStage::Topology, row.evidence_identity())
            } else {
                row.clone()
            }
        })
        .collect();
    WorkloadEvidenceLedger::from_rows(rows)
}
