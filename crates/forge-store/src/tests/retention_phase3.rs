use crate::{
    AspectLayoutReadRequest, AspectLayoutTarget, AspectProjectionSet, AspectScopeClass,
    ConservativeRetentionPolicy, DerivedFamilyRetentionPolicy, ForgeStore, ForgeStoreBuilder,
    RetainedReadPath, RetentionPolicyClass, SingleEntityAspectScope, StoreErrorKind,
};
use forge_relational::facade::history::{BranchId, CommitId};

use super::harness::fixtures::runtime::{create_entity, latest_envelope, runtime_with_demo_schema};

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
fn compaction_publication_verification_and_cutover_stay_typed() {
    let (mut store, _branch_id, _commit_id, artifact_id) = store_with_materialized_layout();
    let policy = ConservativeRetentionPolicy::new(
        Vec::new(),
        Vec::new(),
        vec![DerivedFamilyRetentionPolicy::Milestone6LayoutMaterialization],
    );
    let planning = store
        .plan_retention_candidates(RetentionPolicyClass::Conservative(policy))
        .unwrap();

    let plan = planning
        .compaction_plans()
        .iter()
        .find(|plan| {
            plan.family_labels()
                .iter()
                .any(|label| label == "milestone_6_layout_materialization")
        })
        .cloned()
        .expect("layout compaction plan");
    let publication = store.publish_compaction_product(plan.clone()).unwrap();
    assert_eq!(
        publication.product().retained_basis_label(),
        plan.retained_basis_label()
    );
    assert_eq!(
        publication.cost_surface().read_path(),
        RetainedReadPath::CompactionDerived
    );
    assert_eq!(publication.superseded_families().len(), 1);
    assert_eq!(
        publication.superseded_families()[0].artifact_id(),
        artifact_id.as_str()
    );

    let verification = store
        .verify_compaction_product(publication.product().clone())
        .unwrap();
    assert_eq!(
        verification.read_path(),
        RetainedReadPath::CompactionDerived
    );
    assert_eq!(verification.rewritten_range_count(), 1);

    let cutover = store
        .cutover_compaction_product(publication.product().clone())
        .unwrap();
    assert_eq!(
        cutover.witness().retained_basis_label(),
        plan.retained_basis_label()
    );
    assert_eq!(
        cutover.witness().compaction_product_id(),
        publication.product().product_id()
    );
    assert_eq!(cutover.superseded_families().len(), 1);
    assert_eq!(
        cutover.cost_surface().read_path(),
        RetainedReadPath::CompactionDerived
    );

    let counters = store.counters();
    assert_eq!(counters.compacted_layout_family_count, 1);
    assert_eq!(counters.compaction_cutover_count, 1);
    assert_eq!(counters.compaction_cutover_rejection_count, 0);
}

#[test]
fn cutover_rejects_unverified_products() {
    let (mut store, _branch_id, _commit_id, _artifact_id) = store_with_materialized_layout();
    let policy = ConservativeRetentionPolicy::new(
        Vec::new(),
        Vec::new(),
        vec![DerivedFamilyRetentionPolicy::Milestone6LayoutMaterialization],
    );
    let planning = store
        .plan_retention_candidates(RetentionPolicyClass::Conservative(policy))
        .unwrap();

    let plan = planning
        .compaction_plans()
        .iter()
        .find(|plan| {
            plan.family_labels()
                .iter()
                .any(|label| label == "milestone_6_layout_materialization")
        })
        .cloned()
        .expect("layout compaction plan");
    let publication = store.publish_compaction_product(plan).unwrap();
    let error = store
        .cutover_compaction_product(publication.product().clone())
        .unwrap_err();

    assert_eq!(error.kind(), &StoreErrorKind::CompactionCutoverViolation);
    assert_eq!(store.counters().compaction_cutover_rejection_count, 1);
}
