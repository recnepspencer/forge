use worth_spatial::facade::workload_vocabulary::{
    WorkloadEvidenceLedger, WorkloadEvidenceRow, WorkloadEvidenceStage,
    WorkloadEvidenceStageIndexProduct,
};

use super::evidence_ledger_receipts::counter_backed_rows;

#[test]
fn boolean_evidence_rows_remain_inspectable_without_becoming_public_receipt_authority() {
    run_with_large_stack(|| {
        let mut rows = counter_backed_rows("boolean-public-contract-ledger");
        rows.push(WorkloadEvidenceRow::new(
            WorkloadEvidenceStage::BooleanDeclarationEntry,
            "boolean-declaration:manual-public-row",
        ));

        let ledger = WorkloadEvidenceLedger::from_rows(rows)
            .expect("manual boolean evidence rows remain ordinary inspectable rows");

        assert_eq!(ledger.counters().boolean_rows(), 0);
        let stage_index: &WorkloadEvidenceStageIndexProduct = ledger.stage_index();
        let _stage_index_counters = stage_index.counters();
        assert_eq!(ledger.stage_index().counters().manual_row_count(), 1);
    });
}

fn run_with_large_stack(body: impl FnOnce() + Send + 'static) {
    std::thread::Builder::new()
        .name("boolean-evidence-ledger".to_string())
        .stack_size(16 * 1024 * 1024)
        .spawn(body)
        .expect("boolean evidence ledger contract thread should spawn")
        .join()
        .expect("boolean evidence ledger contract thread should finish");
}
