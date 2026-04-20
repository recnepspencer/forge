use crate::{
    AggressiveRetentionDebtMarker, AspectLayoutReadRequest, AspectLayoutTarget,
    AspectProjectionSet, AspectScopeClass, ComplexityStatus, ConservativeRetentionPolicy,
    DerivedFamilyRetentionPolicy, ForgeStore, ForgeStoreBuilder, RetentionPolicyClass,
    SingleEntityAspectScope,
};

use super::harness::fixtures::runtime::{create_entity, latest_envelope, runtime_with_demo_schema};

fn store_with_materialized_layout() -> (ForgeStore, AspectLayoutReadRequest) {
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
        .materialize_milestone_6_layout_support(request.clone())
        .unwrap();
    (store, request)
}

#[test]
fn milestone_10_evidence_surface_reports_verified_after_clean_loop() {
    let (mut store, request) = store_with_materialized_layout();
    let policy = ConservativeRetentionPolicy::new(
        Vec::new(),
        Vec::new(),
        vec![DerivedFamilyRetentionPolicy::Milestone6LayoutMaterialization],
    );
    let planning = store
        .plan_retention_candidates(RetentionPolicyClass::Conservative(policy))
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
    assert!(store.fetch_milestone_6_layout_support(request).is_ok());

    let counters = store.milestone_10_counter_contract();
    assert!(counters.compaction_plan_count >= 1);
    assert!(counters.compaction_cutover_count >= 1);
    assert!(counters.reclaimed_derived_artifact_count >= 1);
    assert!(counters.retained_range_rebuild_count >= 1);
    assert_eq!(counters.retention_truth_parity_failure_count, 0);
    assert_eq!(counters.retention_restore_parity_failure_count, 0);

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
fn aggressive_retention_policy_surfaces_compaction_debt() {
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
    assert_eq!(
        store.milestone_10_counter_contract().compaction_debt_count,
        1
    );
    assert_eq!(
        store
            .milestone_10_complexity_surface()
            .compaction_publication
            .status,
        ComplexityStatus::Debt
    );
}
