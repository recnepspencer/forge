use super::admission_test_support::{assert_indexed_single_receipt_lookup, split_request_subject};
use super::*;
use crate::workload_platform::planar_boolean_loop_reconstruction::test_support::LoopFixtureEntryOrder;

#[test]
fn request_boundary_replay_equivalence_preserves_spatial_touch_authority_identity() {
    let canonical_subject = split_request_subject(LoopFixtureEntryOrder::Canonical);
    let replayed_subject = split_request_subject(LoopFixtureEntryOrder::Replayed);

    let canonical_authority =
        SpatialGeometryEvidenceTouchRequest::from_boolean_receipt(&canonical_subject.receipt)
            .with_complete_ledger(&canonical_subject.complete)
            .admit()
            .expect("canonical split receipt should admit through complete-ledger request route");
    let replayed_authority =
        SpatialGeometryEvidenceTouchRequest::from_boolean_receipt(&replayed_subject.receipt)
            .with_complete_ledger(&replayed_subject.complete)
            .admit()
            .expect("replayed split receipt should admit through complete-ledger request route");

    assert_eq!(canonical_subject.receipt, replayed_subject.receipt);
    assert_eq!(canonical_authority.digest(), replayed_authority.digest());
    assert_eq!(
        canonical_authority.stage_index_identity(),
        replayed_authority.stage_index_identity()
    );
    assert_eq!(
        canonical_authority.stage_link_set_identity(),
        replayed_authority.stage_link_set_identity()
    );
    assert_eq!(
        canonical_authority.evidence_counters(),
        replayed_authority.evidence_counters()
    );
    assert_eq!(canonical_authority.support(), replayed_authority.support());
    assert_indexed_single_receipt_lookup(&canonical_authority);
    assert_indexed_single_receipt_lookup(&replayed_authority);
}
