use super::admission_test_support::{complete_ledger_from_rows, split_request_subject};
use super::SpatialGeometryEvidenceTouchRequest;
use crate::workload_platform::evidence_ledger::{
    WorkloadEvidenceRow, WorkloadEvidenceStage, WorkloadEvidenceStageCounters,
};
use crate::workload_platform::planar_boolean_loop_reconstruction::test_support::LoopFixtureEntryOrder;

#[test]
fn lookup_locality_counters_stay_bound_to_required_stage_index_slot() {
    let subject = split_request_subject(LoopFixtureEntryOrder::Canonical);
    let mut rows = subject.complete.rows().to_vec();
    rows.push(WorkloadEvidenceRow::receipt_backed(
        WorkloadEvidenceStage::BooleanClassify,
        "unrelated boolean classify evidence",
        WorkloadEvidenceStageCounters::boolean_classify(),
    ));
    rows.push(WorkloadEvidenceRow::receipt_backed(
        WorkloadEvidenceStage::BooleanAssemble,
        "unrelated boolean assemble evidence",
        WorkloadEvidenceStageCounters::boolean_assemble(),
    ));
    let complete = complete_ledger_from_rows(rows);

    let authority = SpatialGeometryEvidenceTouchRequest::from_boolean_receipt(&subject.receipt)
        .with_complete_ledger(&complete)
        .admit()
        .expect("split receipt should admit through complete ledger with unrelated rows");
    let lookup = authority
        .spatial_evidence_lookup(&complete)
        .expect("authority should derive lookup product");

    assert_eq!(complete.stage_index().counters().boolean_row_count(), 3);
    assert_eq!(lookup.lookup_counters().required_stage_count(), 1);
    assert_eq!(lookup.lookup_counters().indexed_lookup_count(), 1);
    assert_eq!(lookup.lookup_counters().raw_row_scan_count(), 0);
    assert_eq!(lookup.lookup_counters().rejected_raw_row_scan_count(), 0);
    assert_eq!(
        lookup
            .lookup_counters()
            .rejected_string_prefix_stage_link_count(),
        0
    );
}
