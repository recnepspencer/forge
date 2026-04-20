use crate::{
    AspectLayoutReadRequest, AspectLayoutTarget, AspectProjectionSet, AspectScopeClass,
    BranchHistoryWindowPolicy, ConservativeRetentionPolicy, DerivedFamilyRetentionPolicy,
    DurableCursorAcknowledgeRequest, ForgeStore, ForgeStoreBuilder, RetentionPolicyClass,
    SingleEntityAspectScope,
};
use forge_relational::facade::history::{BranchId, CommitId};

use super::harness::fixtures::runtime::{create_entity, latest_envelope, runtime_with_demo_schema};

fn store_with_two_commits() -> (ForgeStore, BranchId, CommitId, CommitId) {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let first = latest_envelope(&runtime);
    create_entity(&mut runtime, "beta");
    let second = latest_envelope(&runtime);

    let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    store.append_canonical_commit(first.clone()).unwrap();
    store.append_canonical_commit(second.clone()).unwrap();

    (
        store,
        second.branch_context.clone(),
        first.commit.commit_id,
        second.commit.commit_id,
    )
}

fn store_with_materialized_layout() -> (ForgeStore, BranchId, CommitId, String) {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let envelope = latest_envelope(&runtime);
    let branch_id = envelope.branch_context.clone();
    let commit_id = envelope.commit.commit_id;

    let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    store.append_canonical_commit(envelope).unwrap();
    let request = AspectLayoutReadRequest::new(
        AspectLayoutTarget::new(branch_id.clone(), commit_id),
        AspectScopeClass::SingleEntity(SingleEntityAspectScope::new("entity-alpha")),
        AspectProjectionSet::new(vec!["profile".to_string()]),
    );
    let materialization = store
        .materialize_milestone_6_layout_support(request)
        .unwrap();
    (
        store,
        branch_id,
        commit_id,
        materialization.artifact_id().to_string(),
    )
}

#[test]
fn conservative_retention_planning_emits_closure_and_retained_ranges() {
    let (store, branch_id, first_commit_id, second_commit_id) = store_with_two_commits();
    let policy = ConservativeRetentionPolicy::new(
        vec![BranchHistoryWindowPolicy::new(branch_id.clone(), 1).unwrap()],
        Vec::new(),
        Vec::new(),
    );

    let report = store
        .plan_retention_candidates(RetentionPolicyClass::Conservative(policy))
        .unwrap();

    assert_eq!(
        report
            .conservative_plan()
            .policy()
            .branch_history_windows()
            .len(),
        1
    );
    assert_eq!(report.retained_ranges().len(), 1);
    assert_eq!(report.retained_ranges()[0].branch_id(), &branch_id);
    assert!(report
        .closure_witness()
        .closure_commit_ids()
        .contains(&first_commit_id));
    assert!(report
        .closure_witness()
        .closure_commit_ids()
        .contains(&second_commit_id));
    assert!(report.expired_ranges().is_empty());

    let counters = store.counters();
    assert_eq!(counters.retention_policy_evaluation_count, 1);
    assert_eq!(counters.retained_authoritative_range_count, 1);
    assert_eq!(counters.retention_closure_failure_count, 0);
}

#[test]
fn reclaimable_layout_candidates_publish_rebuild_debt() {
    let (store, _branch_id, _commit_id, artifact_id) = store_with_materialized_layout();
    let policy = ConservativeRetentionPolicy::new(
        Vec::new(),
        Vec::new(),
        vec![DerivedFamilyRetentionPolicy::Milestone6LayoutMaterialization],
    );

    let report = store
        .plan_retention_candidates(RetentionPolicyClass::Conservative(policy))
        .unwrap();

    assert_eq!(report.reclaim_candidates().len(), 1);
    assert_eq!(report.rebuild_debts().len(), 1);
    assert_eq!(report.reclaim_candidates()[0].artifact_id(), artifact_id);
    assert_eq!(
        report.rebuild_debts()[0].family_label(),
        "milestone_6_layout_materialization"
    );

    let counters = store.counters();
    assert_eq!(counters.reclaim_candidate_count, 1);
    assert_eq!(counters.rebuild_debt_count, 1);
    assert_eq!(counters.reclaim_rejected_live_basis_count, 0);
}

#[test]
fn live_cursor_basis_blocks_layout_reclaim_candidates() {
    let (mut store, branch_id, commit_id, _artifact_id) = store_with_materialized_layout();
    store
        .acknowledge_cursor(DurableCursorAcknowledgeRequest::new(
            "cursor-main",
            "subscriber-a",
            branch_id,
            "demo-feed",
            "schema:v1",
            1,
            commit_id,
        ))
        .unwrap();

    let policy = ConservativeRetentionPolicy::new(
        Vec::new(),
        Vec::new(),
        vec![DerivedFamilyRetentionPolicy::Milestone6LayoutMaterialization],
    );
    let report = store
        .plan_retention_candidates(RetentionPolicyClass::Conservative(policy))
        .unwrap();

    assert!(report.reclaim_candidates().is_empty());
    assert!(report.rebuild_debts().is_empty());
    assert!(report
        .basis_survival_verdicts()
        .iter()
        .any(|verdict| verdict.basis_label().starts_with("cursor:") && verdict.survives_basis()));

    let counters = store.counters();
    assert_eq!(counters.reclaim_candidate_count, 0);
    assert_eq!(counters.rebuild_debt_count, 0);
    assert_eq!(counters.reclaim_rejected_live_basis_count, 1);
}
