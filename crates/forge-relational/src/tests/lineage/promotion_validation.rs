use crate::facade::history::BranchId;
use crate::facade::identity::LineageId;
use crate::facade::lineage::{
    CorrespondencePromotionRejectionClass, LineageDecisionKind,
};
use crate::tests::support::*;

// CONTRACT: lineage_promotion_validation
// LANES: failure_boundary, locality

#[test]
fn lineage_promotion_validation_invalid_references_do_not_promote() {
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
        .promote_correspondence(candidate.candidate_id, commit.commit.clone());

    assert_eq!(
        resolution,
        Err(CorrespondencePromotionRejectionClass::MissingLineageReference)
    );
    let rejected = runtime.lineage_access().rejected_decisions_snapshot();
    assert!(rejected.iter().any(|decision| {
        decision.kind == LineageDecisionKind::CorrespondencePromotionRejected
            && decision.candidate_id == Some(candidate.candidate_id)
            && decision.rejection_class
                == Some(CorrespondencePromotionRejectionClass::MissingLineageReference)
    }));
}

#[test]
fn lineage_promotion_validation_rejects_commit_branch_mismatch() {
    let mut runtime = runtime_with_test_schema();
    let created = create_entity_outcome(&mut runtime, "source");
    let target = create_entity_outcome(&mut runtime, "target");
    let entity = changed_entities(&created)[0];
    let start_lineage = runtime
        .lineage_access()
        .for_record(entity)
        .unwrap()
        .lineage_id;
    let target_lineage = runtime
        .lineage_access()
        .for_record(changed_entities(&target)[0])
        .unwrap()
        .lineage_id;
    runtime
        .history_authority()
        .create_branch(
            BranchId("feature".to_string()),
            &BranchId("main".to_string()),
        )
        .unwrap();

    let candidate = runtime.lineage_authority().record_correspondence_candidate(
        BranchId("feature".to_string()),
        vec![start_lineage],
        vec![target_lineage],
        "feature-candidate",
    );

    let rejection = runtime
        .lineage_authority()
        .promote_correspondence(candidate.candidate_id, target.commit.clone());

    assert_eq!(
        rejection,
        Err(CorrespondencePromotionRejectionClass::CommitBranchMismatch)
    );
}

#[test]
fn lineage_promotion_validation_rejects_stale_commit_anchor() {
    let mut runtime = runtime_with_test_schema();
    let first = create_entity_outcome(&mut runtime, "source");
    let second = create_entity_outcome(&mut runtime, "target");
    let latest = create_entity_outcome(&mut runtime, "newer-head");
    let source_lineage = runtime
        .lineage_access()
        .for_record(changed_entities(&first)[0])
        .unwrap()
        .lineage_id;
    let target_lineage = runtime
        .lineage_access()
        .for_record(changed_entities(&second)[0])
        .unwrap()
        .lineage_id;
    let candidate = runtime.lineage_authority().record_correspondence_candidate(
        BranchId("main".to_string()),
        vec![source_lineage],
        vec![target_lineage],
        "stale-anchor",
    );

    let rejection = runtime
        .lineage_authority()
        .promote_correspondence(candidate.candidate_id, second.commit.clone());

    assert_eq!(
        rejection,
        Err(CorrespondencePromotionRejectionClass::CommitNotBranchHead)
    );
    assert_eq!(
        runtime
            .history_access()
            .branch_head(&BranchId("main".to_string()))
            .map(|head| head.commit_id),
        Some(latest.commit.commit_id)
    );
}

#[test]
fn lineage_promotion_validation_resolves_anchor_truth_from_history_not_caller_shape() {
    let mut runtime = runtime_with_test_schema();
    let first = create_entity_outcome(&mut runtime, "source");
    let second = create_entity_outcome(&mut runtime, "target");
    let source_lineage = runtime
        .lineage_access()
        .for_record(changed_entities(&first)[0])
        .unwrap()
        .lineage_id;
    let target_lineage = runtime
        .lineage_access()
        .for_record(changed_entities(&second)[0])
        .unwrap()
        .lineage_id;
    let candidate = runtime.lineage_authority().record_correspondence_candidate(
        BranchId("main".to_string()),
        vec![source_lineage],
        vec![target_lineage],
        "forged-anchor-shape",
    );
    let forged_anchor = crate::facade::history::CommitReference {
        commit_id: second.commit.commit_id,
        version_id: crate::facade::identity::VersionId(second.commit.version_id.0 + 999),
        branch_id: second.commit.branch_id.clone(),
        parents: Vec::new(),
    };

    let promoted = runtime
        .lineage_authority()
        .promote_correspondence(candidate.candidate_id, forged_anchor)
        .unwrap();
    let promoted_commit = runtime
        .history_access()
        .branch_head(&BranchId("main".to_string()))
        .cloned()
        .expect("promoted commit");

    assert_eq!(promoted.promoted_commit_id, Some(promoted_commit.commit_id));
    assert_eq!(promoted_commit.version_id, second.commit.version_id);
    assert_eq!(promoted_commit.parents, vec![second.commit.commit_id]);
}
