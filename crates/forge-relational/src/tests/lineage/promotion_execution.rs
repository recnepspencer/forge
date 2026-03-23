use crate::facade::history::BranchId;
use crate::facade::lineage::{
    LineageDecisionKind, LineageGraphRequest, LineageResolutionStatus,
};
use crate::facade::replay::CanonicalCommitAuthorityKind;
use crate::tests::support::*;

// CONTRACT: lineage_promotion_execution
// LANES: success, publication

#[test]
fn lineage_promotion_execution_stays_advisory_until_promoted() {
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
    let candidate = record_lineage_candidate(
        &mut runtime,
        BranchId("main".to_string()),
        vec![left_lineage],
        vec![right_lineage],
        "candidate",
    );
    let graph_before = runtime
        .lineage_access()
        .graph(LineageGraphRequest {
            branch_id: BranchId("main".to_string()),
        });
    let resolution = runtime
        .lineage_authority()
        .promote_correspondence(candidate.candidate_id, second.commit.clone())
        .unwrap();
    let graph_after = runtime
        .lineage_access()
        .graph(LineageGraphRequest {
            branch_id: BranchId("main".to_string()),
        });

    assert_eq!(graph_before.events.len(), 2);
    assert_eq!(graph_before.correspondence_candidates.len(), 1);
    assert_eq!(resolution.status, LineageResolutionStatus::Promoted);
    assert_eq!(graph_after.events.len(), 3);
    let promotion_commit_id = resolution.promoted_commit_id.expect("promotion commit id");
    let replay = runtime.replay_access();
    let envelope = replay
        .canonical_commit_envelope(promotion_commit_id)
        .unwrap();
    assert_eq!(
        envelope.authority_kind(),
        CanonicalCommitAuthorityKind::MetadataOnlyLineage
    );
    assert!(envelope.lineage_decision_log().iter().any(|decision| {
        decision.kind == LineageDecisionKind::CorrespondencePromotionAccepted
            && decision.candidate_id == Some(candidate.candidate_id)
            && decision.event_id == resolution.promoted_event_id
    }));
    assert!(envelope
        .lineage_events()
        .iter()
        .any(|event| event.kind == crate::facade::lineage::LineageEventKind::Correspond));
}
