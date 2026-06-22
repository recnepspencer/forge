use super::admission_test_support::split_request_subject;
use super::{SpatialEvidenceQueryGapKind, SpatialGeometryEvidenceTouchRequest};
use crate::workload_platform::evidence_ledger::SpatialEvidenceSurfaceOwner;
use crate::workload_platform::planar_boolean_loop_reconstruction::test_support::LoopFixtureEntryOrder;

#[test]
fn query_gap_records_owned_capped_gap_when_query_lacks_declared_mutation_expressiveness() {
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
        .expect("query lowering should produce a descriptor plus gap posture");

    assert_eq!(lowered.gap_rows().len(), 1);
    let gap = &lowered.gap_rows()[0];
    assert_eq!(
        gap.kind(),
        SpatialEvidenceQueryGapKind::DeclaredMutationCollectionNotExpressed
    );
    assert_eq!(gap.owner(), SpatialEvidenceSurfaceOwner::WorthSpatial);
    assert!(gap.cap().contains("read-family touch only"));
    assert!(gap.blocker().contains("not graph mutation meaning"));
    assert!(gap.removal_trigger().contains("Milestone 5"));
    assert_eq!(lowered.counters().query_gap_count(), 1);
    assert_eq!(gap.gap_digest(), lowered.gap_rows()[0].gap_digest());
}
