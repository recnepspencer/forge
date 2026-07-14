use crate::{
    AspectLayoutReadRequest, AspectLayoutTarget, AspectProjectionSet, AspectScopeClass,
    ConservativeRetentionPolicy, DerivedFamilyRetentionPolicy, WORTHStore, WORTHStoreBuilder,
    RetentionPolicyClass, SingleEntityAspectScope,
};
use worth_relational::facade::history::{BranchId, CommitId};

use super::harness::fixtures::runtime::{create_entity, latest_envelope, runtime_with_demo_schema};

fn store_with_materialized_layout() -> (WORTHStore, AspectLayoutReadRequest, BranchId, CommitId) {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let envelope = latest_envelope(&runtime);
    let branch_id = envelope.branch_context.clone();
    let commit_id = envelope.commit.commit_id;

    let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
    store.append_canonical_commit(envelope).unwrap();
    let request = AspectLayoutReadRequest::new(
        AspectLayoutTarget::new(branch_id.clone(), commit_id),
        AspectScopeClass::SingleEntity(SingleEntityAspectScope::new("entity-alpha")),
        AspectProjectionSet::new(vec!["profile".to_string()]),
    );
    store
        .materialize_milestone_6_layout_support(request.clone())
        .unwrap();
    (store, request, branch_id, commit_id)
}

#[test]
fn retention_compaction_reclaim_rebuild_preserves_authoritative_parity() {
    let (mut store, request, _branch_id, _commit_id) = store_with_materialized_layout();
    let before = store.export_authoritative_records().canonical_json();

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

    let reclaim_witness = planning
        .reclaim_candidates()
        .iter()
        .find(|witness| witness.artifact_family() == "milestone_6_layout_materialization")
        .cloned()
        .expect("layout reclaim witness");
    let reclaim = store.execute_derived_reclaim(reclaim_witness).unwrap();
    assert!(store
        .fetch_milestone_6_layout_support(request.clone())
        .is_err());

    store
        .rebuild_reclaimed_derived_family(reclaim.rebuild_unit().clone())
        .unwrap();

    let after = store.export_authoritative_records().canonical_json();
    assert_eq!(before, after);
    assert!(store.fetch_milestone_6_layout_support(request).is_ok());
}
