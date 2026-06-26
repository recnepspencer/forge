use super::admission_test_support::{
    complete_ledger_from_rows, split_request_subject, SpatialReceiptAdmissionSubject,
};
use super::SpatialGeometryEvidenceTouchRequest;
use crate::workload_platform::evidence_ledger::{
    WorkloadEvidenceRow, WorkloadEvidenceStage, WorkloadEvidenceStageCounters,
};
use crate::workload_platform::planar_boolean_loop_reconstruction::test_support::LoopFixtureEntryOrder;

#[test]
fn lookup_equivalence_preserves_key_and_digest_with_unrelated_rows_present() {
    let canonical = split_subject_with_unrelated_boolean_row();
    let replayed = split_subject_with_unrelated_boolean_row();

    let canonical_authority =
        SpatialGeometryEvidenceTouchRequest::from_boolean_receipt(&canonical.receipt)
            .with_complete_ledger(&canonical.complete)
            .admit()
            .expect("canonical receipt should admit through complete ledger");
    let replayed_authority =
        SpatialGeometryEvidenceTouchRequest::from_boolean_receipt(&replayed.receipt)
            .with_complete_ledger(&replayed.complete)
            .admit()
            .expect("replayed receipt should admit through complete ledger");

    let canonical_lookup = canonical_authority
        .spatial_evidence_lookup(&canonical.complete)
        .expect("canonical authority should derive lookup product");
    let replayed_lookup = replayed_authority
        .spatial_evidence_lookup(&replayed.complete)
        .expect("replayed authority should derive lookup product");

    assert_eq!(canonical_lookup.lookup_key(), replayed_lookup.lookup_key());
    assert_eq!(
        canonical_lookup.product_digest(),
        replayed_lookup.product_digest()
    );
    assert_eq!(canonical_authority.digest(), replayed_authority.digest());
    assert_eq!(
        canonical_lookup.lookup_key().stage_index_identity(),
        canonical_authority.stage_index_identity()
    );
    assert_eq!(canonical_lookup.support(), replayed_lookup.support());
    assert_eq!(canonical_lookup.counters(), replayed_lookup.counters());
    assert_eq!(
        canonical_lookup.lookup_counters(),
        replayed_lookup.lookup_counters()
    );
}

fn split_subject_with_unrelated_boolean_row(
) -> SpatialReceiptAdmissionSubject<
    crate::workload_platform::planar_boolean_edge_splitting::PlanarBooleanSplitEdgeChainLedgerReceipt,
>{
    let subject = split_request_subject(LoopFixtureEntryOrder::Canonical);
    let mut rows = subject.complete.rows().to_vec();
    rows.push(WorkloadEvidenceRow::receipt_backed(
        WorkloadEvidenceStage::BooleanClassify,
        "unrelated boolean classify evidence",
        WorkloadEvidenceStageCounters::boolean_classify(),
    ));
    SpatialReceiptAdmissionSubject {
        complete: complete_ledger_from_rows(rows),
        receipt: subject.receipt,
    }
}
