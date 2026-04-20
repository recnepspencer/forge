use crate::{
    AggressiveRetentionDebtMarker, AspectLayoutReadRequest, AspectLayoutTarget,
    AspectProjectionSet, AspectScopeClass, ComplexityStatus, ConservativeRetentionPolicy,
    DerivedFamilyRetentionPolicy, ForgeStore, ForgeStoreBuilder, RetentionPolicyClass,
    SingleEntityAspectScope,
};

use super::harness::fixtures::runtime::{create_entity, latest_envelope, runtime_with_demo_schema};

fn store_with_materialized_layout() -> ForgeStore {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let envelope = latest_envelope(&runtime);
    let branch_id = envelope.branch_context.clone();
    let commit_id = envelope.commit.commit_id;

    let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    store.append_canonical_commit(envelope).unwrap();
    let request = AspectLayoutReadRequest::new(
        AspectLayoutTarget::new(branch_id, commit_id),
        AspectScopeClass::SingleEntity(SingleEntityAspectScope::new("entity-alpha")),
        AspectProjectionSet::new(vec!["profile".to_string()]),
    );
    store
        .materialize_milestone_6_layout_support(request)
        .unwrap();
    store
}

#[test]
fn clean_retention_loop_publishes_exact_counter_contract() {
    let mut store = store_with_materialized_layout();
    let planning = store
        .plan_retention_candidates(RetentionPolicyClass::Conservative(
            ConservativeRetentionPolicy::new(
                Vec::new(),
                Vec::new(),
                vec![DerivedFamilyRetentionPolicy::Milestone6LayoutMaterialization],
            ),
        ))
        .unwrap();

    let compaction_plan = planning
        .compaction_plans()
        .iter()
        .find(|plan| {
            plan.family_labels()
                .iter()
                .any(|label| label == "milestone_6_layout_materialization")
        })
        .cloned()
        .expect("layout compaction plan");
    let publication = store.publish_compaction_product(compaction_plan).unwrap();
    store
        .verify_compaction_product(publication.product().clone())
        .unwrap();
    store
        .cutover_compaction_product(publication.product().clone())
        .unwrap();
    let reclaim = store
        .execute_derived_reclaim(
            planning
                .reclaim_candidates()
                .iter()
                .find(|witness| witness.artifact_family() == "milestone_6_layout_materialization")
                .cloned()
                .expect("layout reclaim witness"),
        )
        .unwrap();
    store
        .rebuild_reclaimed_derived_family(reclaim.rebuild_unit().clone())
        .unwrap();

    let counters = store.milestone_10_counter_contract();
    assert_eq!(counters.retention_policy_evaluation_count, 1);
    assert_eq!(
        counters.retained_authoritative_range_count,
        planning.retained_ranges().len() as u64
    );
    assert_eq!(
        counters.expired_authoritative_range_count,
        planning.expired_ranges().len() as u64
    );
    assert_eq!(
        counters.compaction_plan_count,
        planning.compaction_plans().len() as u64
    );
    assert_eq!(counters.compacted_delta_layer_count, 0);
    assert_eq!(counters.compacted_snapshot_family_count, 0);
    assert_eq!(counters.compacted_layout_family_count, 1);
    assert_eq!(counters.compaction_cutover_count, 1);
    assert_eq!(counters.compaction_cutover_rejection_count, 0);
    assert_eq!(
        counters.reclaim_candidate_count,
        planning.reclaim_candidates().len() as u64
    );
    assert_eq!(
        counters.reclaimed_derived_artifact_count,
        reclaim.deleted_artifact_count()
    );
    assert_eq!(counters.reclaimed_authoritative_artifact_count, 0);
    assert_eq!(counters.reclaim_rejected_live_basis_count, 0);
    assert_eq!(
        counters.retention_closure_ancestor_count,
        planning.closure_witness().closure_commit_ids().len() as u64
    );
    assert_eq!(counters.retention_closure_failure_count, 0);
    assert_eq!(counters.retained_range_rebuild_count, 1);
    assert_eq!(
        counters.rebuild_debt_count,
        planning.rebuild_debts().len() as u64 + 1
    );
    assert_eq!(
        counters.compaction_debt_count,
        planning.compaction_rejections().len() as u64
    );
    assert_eq!(counters.retention_truth_parity_failure_count, 0);
    assert_eq!(counters.retention_restore_parity_failure_count, 0);
    assert_eq!(counters.retention_artifact_rebuild_failure_count, 0);

    let surface = store.milestone_10_complexity_surface();
    assert_eq!(
        surface.retention_candidate_planning.status,
        ComplexityStatus::Verified
    );
    assert_eq!(
        surface.compaction_publication.status,
        ComplexityStatus::Verified
    );
    assert_eq!(surface.reclaim_execution.status, ComplexityStatus::Verified);
    assert_eq!(
        surface.retained_range_rebuild.status,
        ComplexityStatus::Verified
    );
}

#[test]
fn unsupported_aggressive_policy_publishes_exact_debt_contract() {
    let store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    let error = store
        .plan_retention_candidates(RetentionPolicyClass::AggressiveDebt(
            AggressiveRetentionDebtMarker::PressureReactivePolicySwitching,
        ))
        .unwrap_err();

    assert_eq!(
        error.kind(),
        &crate::StoreErrorKind::RetentionPolicyUnsupported
    );

    let counters = store.milestone_10_counter_contract();
    assert_eq!(counters.retention_policy_evaluation_count, 0);
    assert_eq!(counters.compaction_plan_count, 0);
    assert_eq!(counters.compaction_cutover_count, 0);
    assert_eq!(counters.reclaim_candidate_count, 0);
    assert_eq!(counters.rebuild_debt_count, 0);
    assert_eq!(counters.compaction_debt_count, 1);
    assert_eq!(counters.retention_truth_parity_failure_count, 0);
    assert_eq!(counters.retention_restore_parity_failure_count, 0);
    assert_eq!(counters.retention_artifact_rebuild_failure_count, 0);

    let surface = store.milestone_10_complexity_surface();
    assert_eq!(
        surface.retention_candidate_planning.status,
        ComplexityStatus::Verified
    );
    assert_eq!(
        surface.compaction_publication.status,
        ComplexityStatus::Debt
    );
    assert_eq!(surface.reclaim_execution.status, ComplexityStatus::Verified);
    assert_eq!(
        surface.retained_range_rebuild.status,
        ComplexityStatus::Verified
    );
}
