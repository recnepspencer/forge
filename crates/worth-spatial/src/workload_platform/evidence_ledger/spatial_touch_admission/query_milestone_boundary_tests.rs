use super::admission_test_support::split_request_subject;
use super::SpatialGeometryEvidenceTouchRequest;
use crate::workload_platform::planar_boolean_loop_reconstruction::test_support::LoopFixtureEntryOrder;

#[test]
fn query_milestone_boundary_does_not_claim_full_obligation_selection_closeout() {
    let subject = split_request_subject(LoopFixtureEntryOrder::Canonical);
    let authority = SpatialGeometryEvidenceTouchRequest::from_boolean_receipt(&subject.receipt)
        .with_complete_ledger(&subject.complete)
        .admit()
        .expect("split receipt should admit");
    let lookup = authority
        .spatial_evidence_lookup(&subject.complete)
        .expect("lookup should derive");
    let lowered = authority
        .query_touch_descriptor(&lookup)
        .expect("query lowering should derive");

    assert!(!lowered.claims_milestone_five_selection_closeout());
    assert_eq!(lowered.counters().query_descriptor_count(), 1);
    assert_eq!(lowered.counters().operating_world_descriptor_count(), 1);
    assert_eq!(lowered.counters().broad_ledger_scan_count(), 0);
    assert_eq!(lowered.touch_descriptor().declared_collection_count(), 1);
    assert_eq!(lowered.touch_descriptor().update_command_count(), 0);
    assert_eq!(lowered.touch_descriptor().delete_command_count(), 0);
}
