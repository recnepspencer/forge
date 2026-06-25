use super::admission_test_support::{
    complete_ledger_from_rows, split_request_subject, SpatialReceiptAdmissionSubject,
};
use super::SpatialGeometryEvidenceTouchRequest;
use crate::facade::query_adoption::lower_spatial_touch_authority_to_query_descriptor;
use crate::workload_platform::evidence_ledger::{
    WorkloadEvidenceRow, WorkloadEvidenceStage, WorkloadEvidenceStageCounters,
};
use crate::workload_platform::planar_boolean_edge_splitting::PlanarBooleanSplitEdgeChainLedgerReceipt;
use crate::workload_platform::planar_boolean_loop_reconstruction::test_support::LoopFixtureEntryOrder;

#[test]
fn query_descriptor_parity_preserves_digest_across_replay_and_public_facade_paths() {
    let canonical = split_subject_with_unrelated_boolean_row();
    let replayed = split_subject_with_unrelated_boolean_row();

    let canonical_authority =
        SpatialGeometryEvidenceTouchRequest::from_boolean_receipt(&canonical.receipt)
            .with_complete_ledger(&canonical.complete)
            .admit()
            .expect("canonical split receipt should admit");
    let replayed_authority =
        SpatialGeometryEvidenceTouchRequest::from_boolean_receipt(&replayed.receipt)
            .with_complete_ledger(&replayed.complete)
            .admit()
            .expect("replayed split receipt should admit");

    let canonical_lookup = canonical_authority
        .spatial_evidence_lookup(&canonical.complete)
        .expect("canonical lookup should derive");
    let replayed_lookup = replayed_authority
        .spatial_evidence_lookup(&replayed.complete)
        .expect("replayed lookup should derive");

    let direct = canonical_authority
        .query_touch_descriptor(&canonical_lookup)
        .expect("authority should lower directly to Query descriptor product");
    let public =
        lower_spatial_touch_authority_to_query_descriptor(&replayed_authority, &replayed_lookup)
            .expect("facade lowering should preserve the same descriptor product");

    assert_eq!(direct.product_digest(), public.product_digest());
    assert_eq!(
        direct.touch_descriptor().descriptor_digest(),
        public.touch_descriptor().descriptor_digest()
    );
    assert_eq!(
        direct.operating_world().descriptor_digest(),
        public.operating_world().descriptor_digest()
    );
    assert_eq!(direct.spatial_touch_digest(), public.spatial_touch_digest());
    assert_eq!(
        direct.lookup_product_digest(),
        public.lookup_product_digest()
    );
    assert_eq!(direct.gap_rows(), public.gap_rows());
    assert_eq!(direct.counters(), public.counters());
}

fn split_subject_with_unrelated_boolean_row(
) -> SpatialReceiptAdmissionSubject<PlanarBooleanSplitEdgeChainLedgerReceipt> {
    let subject = split_request_subject(LoopFixtureEntryOrder::Canonical);
    let mut rows = subject.complete.rows().to_vec();
    rows.push(WorkloadEvidenceRow::receipt_backed(
        WorkloadEvidenceStage::BooleanClassify,
        "phase7 unrelated boolean classify evidence",
        WorkloadEvidenceStageCounters::boolean_classify(),
    ));
    SpatialReceiptAdmissionSubject {
        complete: complete_ledger_from_rows(rows),
        receipt: subject.receipt,
    }
}
