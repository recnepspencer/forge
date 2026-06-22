use worth_kernel::workload_composition::WorkloadCatalog;
use worth_spatial::certification::workload_evidence::{
    complete_ledger_stage_snapshots, ledger_with_manual_stage_substitution,
};
use worth_spatial::facade::workload_vocabulary::{
    WorkloadEvidenceLedgerError, WorkloadEvidenceStage,
};

#[test]
fn workload_catalog_blocks_static_fixture_substitution() {
    let built = WorkloadCatalog::cube()
        .declared("compile-time catalog boundary companion")
        .build()
        .expect("real catalog cube should build");

    assert_eq!(built.workload().evidence_ledger().counters().rows(), 8);
    assert!(
        complete_ledger_stage_snapshots(built.workload().evidence_ledger())
            .iter()
            .all(|row| row.is_receipt_backed())
    );

    let error = ledger_with_manual_stage_substitution(
        built.workload().evidence_ledger(),
        WorkloadEvidenceStage::Topology,
    )
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
