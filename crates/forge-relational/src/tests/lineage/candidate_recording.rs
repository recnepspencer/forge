use crate::facade::history::BranchId;
use crate::facade::identity::LineageId;
use crate::facade::lineage::{
    CorrespondencePromotionRejectionClass, LineageResolutionStatus,
};
use crate::tests::support::*;

// CONTRACT: lineage_candidate_recording
// LANES: success, failure_boundary

#[test]
fn lineage_candidate_recording_try_promote_returns_rejected_resolution() {
    let mut runtime = runtime_with_test_schema();
    let commit = create_entity_outcome(&mut runtime, "anchor");
    let candidate = runtime.lineage_authority().record_correspondence_candidate(
        BranchId("main".to_string()),
        vec![LineageId(999)],
        vec![LineageId(1000)],
        "invalid",
    );

    let resolution = runtime
        .lineage_authority()
        .try_promote_correspondence(candidate.candidate_id, commit.commit.clone());

    assert_eq!(resolution.status, LineageResolutionStatus::Rejected);
    assert_eq!(
        resolution.rejection_class,
        Some(CorrespondencePromotionRejectionClass::MissingLineageReference)
    );
    assert!(resolution.promoted_event_id.is_none());
}
