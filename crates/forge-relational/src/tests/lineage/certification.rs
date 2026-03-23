use crate::facade::history::BranchId;
use crate::facade::identity::LineageId;
use crate::facade::lineage::{
    CorrespondencePromotionRejectionClass, LineageDecisionKind, LineageResolutionStatus,
};
use crate::tests::support::*;

// CONTRACT: lineage_correspondence_hardening
// LANES: certification, replay

#[test]
fn lineage_correspondence_hardening_tracks_advisory_promotion_and_rejection_artifacts() {
    let mut runtime = runtime_with_test_schema();
    let first = create_entity_outcome(&mut runtime, "left");
    let second = create_entity_outcome(&mut runtime, "right");
    let left_lineage = runtime
        .lineage_access()
        .for_record(changed_entities(&first)[0])
        .unwrap()
        .lineage_id;
    let right_lineage = runtime
        .lineage_access()
        .for_record(changed_entities(&second)[0])
        .unwrap()
        .lineage_id;

    let advisory = record_lineage_candidate(
        &mut runtime,
        BranchId("main".to_string()),
        vec![left_lineage],
        vec![right_lineage],
        "candidate",
    );
    let invalid = record_lineage_candidate(
        &mut runtime,
        BranchId("main".to_string()),
        vec![LineageId(999)],
        vec![LineageId(1000)],
        "invalid",
    );

    let promoted = runtime
        .lineage_authority()
        .promote_correspondence(advisory.candidate_id, second.commit.clone())
        .unwrap();
    let rejected = runtime
        .lineage_authority()
        .promote_correspondence(invalid.candidate_id, second.commit.clone());

    assert_eq!(promoted.status, LineageResolutionStatus::Promoted);
    assert_eq!(
        rejected,
        Err(CorrespondencePromotionRejectionClass::MissingLineageReference)
    );
    let rejected_decisions = runtime.lineage_access().rejected_decisions_snapshot();
    assert!(rejected_decisions.iter().any(|decision| {
        decision.kind == LineageDecisionKind::CorrespondencePromotionRejected
            && decision.candidate_id == Some(invalid.candidate_id)
    }));
    let promotion_commit_id = promoted.promoted_commit_id.expect("promotion commit id");
    let replay = runtime.replay_access();
    let envelope = replay
        .canonical_commit_envelope(promotion_commit_id)
        .unwrap();
    assert!(envelope.lineage_decision_log().iter().any(|decision| {
        decision.kind == LineageDecisionKind::CorrespondencePromotionAccepted
            && decision.candidate_id == Some(advisory.candidate_id)
    }));
}
