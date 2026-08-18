use crate::facade::history::BranchId;
use crate::facade::lineage::{
    CorrespondencePromotionExecutionFailureClass, LineageDecisionKind, LineageGraphRequest,
    LineageGraphTraversalBasis, LineageResolutionStatus,
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
    let branch_id = BranchId("main".to_string());
    let before_metadata = runtime
        .history
        .branch_cell(&branch_id)
        .expect("main branch cell")
        .clone();
    let graph_before = runtime.lineage_access().graph(LineageGraphRequest {
        branch_id: BranchId("main".to_string()),
        traversal_basis: LineageGraphTraversalBasis::FullBranchGraphMaterialization,
    });
    let resolution = runtime
        .lineage_authority()
        .promote_correspondence(candidate.candidate_id, second.commit.clone())
        .unwrap();
    let graph_after = runtime.lineage_access().graph(LineageGraphRequest {
        branch_id: BranchId("main".to_string()),
        traversal_basis: LineageGraphTraversalBasis::FullBranchGraphMaterialization,
    });

    assert_eq!(graph_before.events.len(), 2);
    assert_eq!(graph_before.correspondence_candidates.len(), 1);
    assert_eq!(resolution.status(), LineageResolutionStatus::Promoted);
    assert_eq!(graph_after.events.len(), 3);
    let promotion_commit_id = resolution
        .promoted_commit_id()
        .expect("promotion commit id");
    let replay = runtime.replay();
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
            && decision.event_id == resolution.promoted_event_id()
    }));
    assert!(envelope
        .lineage_events()
        .iter()
        .any(|event| event.kind == crate::facade::lineage::LineageEventKind::Correspond));
    let after_metadata = runtime
        .history
        .branch_cell(&branch_id)
        .expect("main branch cell after promotion");
    assert_eq!(
        after_metadata.truth_version(),
        before_metadata.truth_version(),
        "metadata publication must not advance branch truth"
    );
    assert_eq!(
        after_metadata.observation().target(),
        before_metadata.observation().target(),
        "metadata publication must preserve the branch target"
    );
    assert_eq!(
        after_metadata.observation().generation().get(),
        before_metadata.observation().generation().get() + 1,
        "metadata publication advances only the reference generation"
    );
    assert_eq!(
        runtime.history().branch_head(&branch_id),
        Some(&second.commit),
        "metadata publication must not replace the branch-cell truth head"
    );
}

#[test]
fn lineage_promotion_execution_reports_operational_anchor_drift_after_plan_lowering() {
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

    let resolution = runtime
        .lineage_authority()
        .promote_correspondence_with_post_plan_hook_for_test(
            candidate.candidate_id,
            second.commit.clone(),
            |runtime| {
                let _advanced_head = create_entity_outcome(runtime, "branch-head-advanced");
            },
        )
        .unwrap();

    assert_eq!(
        resolution.status(),
        LineageResolutionStatus::ExecutionFailed
    );
    assert_eq!(
        resolution.execution_failure_class(),
        Some(CorrespondencePromotionExecutionFailureClass::AnchorDriftedFromBranchHead)
    );
    assert_eq!(resolution.rejection_class(), None);
    assert!(resolution.promoted_event_id().is_some());
    assert_eq!(resolution.promoted_commit_id(), None);
}
