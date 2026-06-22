use super::admission_test_support::{
    assert_indexed_single_receipt_lookup, event_ledger_request_subject,
    loop_reconstruction_request_subject, segment_pair_request_subject, split_request_subject,
    SpatialReceiptAdmissionSubject,
};
use super::*;
use crate::workload_platform::evidence_ledger::{
    BooleanEvidenceReceipt, BooleanEvidenceRowAuthority, BooleanEvidenceStageKind,
};
use crate::workload_platform::planar_boolean_loop_reconstruction::test_support::LoopFixtureEntryOrder;

#[test]
fn request_boundary_admits_each_current_boolean_receipt_implementor_with_stage() {
    assert_receipt_admits_with_stage(
        segment_pair_request_subject(),
        BooleanEvidenceStageKind::SegmentPairEnumeration,
    );
    assert_receipt_admits_with_stage(
        event_ledger_request_subject(),
        BooleanEvidenceStageKind::EventLedger,
    );
    assert_receipt_admits_with_stage(
        split_request_subject(LoopFixtureEntryOrder::Canonical),
        BooleanEvidenceStageKind::Split,
    );
    assert_receipt_admits_with_stage(
        loop_reconstruction_request_subject(LoopFixtureEntryOrder::Canonical),
        BooleanEvidenceStageKind::LoopReconstruction,
    );
}

fn assert_receipt_admits_with_stage<T>(
    subject: SpatialReceiptAdmissionSubject<T>,
    expected_stage: BooleanEvidenceStageKind,
) where
    T: BooleanEvidenceReceipt + BooleanEvidenceRowAuthority + 'static,
{
    let authority = SpatialGeometryEvidenceTouchRequest::from_boolean_receipt(&subject.receipt)
        .with_complete_ledger(&subject.complete)
        .admit()
        .expect("sealed receipt plus request-boundary complete ledger should admit");

    assert_eq!(authority.boolean_stage(), expected_stage);
    assert_eq!(authority.evidence_stage(), expected_stage.evidence_stage());
    assert_eq!(
        authority.evidence_identity(),
        subject.receipt.evidence_identity()
    );
    assert_eq!(
        authority.evidence_counters(),
        subject.receipt.evidence_counters()
    );
    assert_eq!(authority.support(), subject.receipt.evidence_support());
    assert_indexed_single_receipt_lookup(&authority);
}
