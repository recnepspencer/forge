use crate::{
    AspectLayoutReadRequest, AspectLayoutTarget, AspectProjectionSet, AspectScopeClass,
    ConservativeRetentionPolicy, ContinuationRetentionStatus, DerivedFamilyRetentionPolicy,
    ForgeStore, ForgeStoreBuilder, ReclaimEligibilityWitness, RetentionPolicyClass,
    SingleEntityAspectScope, StableBasisLayoutPosture, StableBasisReadRequest,
    StableBasisReadScope, StoreErrorKind,
};
use forge_relational::facade::history::{BranchId, CommitId};
use serde::Serialize;
use sha2::{Digest, Sha256};

use super::harness::fixtures::runtime::{create_entity, latest_envelope, runtime_with_demo_schema};

fn stable_digest<T: Serialize>(value: &T) -> String {
    let normalized = serde_json::to_value(value).expect("retention phase 4 normalization");
    let json = serde_json::to_vec(&normalized).expect("retention phase 4 serialization");
    let mut hasher = Sha256::new();
    hasher.update(json);
    format!("{:x}", hasher.finalize())
}

fn stable_basis_request(
    store: &ForgeStore,
    branch_id: BranchId,
    commit_id: CommitId,
) -> StableBasisReadRequest {
    let export = store.export_authoritative_records().into_canonicalized();
    let support_summary = export
        .commit_support_summaries
        .iter()
        .find(|summary| summary.commit_id == commit_id)
        .expect("retention stable-basis fixture requires a commit support summary")
        .clone();
    let commit = export
        .commit_envelopes
        .iter()
        .find(|envelope| envelope.envelope.commit.commit_id == commit_id)
        .expect("retention stable-basis fixture requires a frontier commit");
    StableBasisReadRequest::new(
        branch_id,
        commit_id,
        StableBasisReadScope::SingleEntity(SingleEntityAspectScope::new("entity-alpha")),
        stable_digest(&support_summary),
        support_summary
            .schema_support_artifact_id
            .clone()
            .unwrap_or_else(|| "schema-support:v1".to_string()),
        StableBasisLayoutPosture::ProofOnly,
        commit.envelope_digest.clone(),
        ContinuationRetentionStatus::Retained,
    )
}

fn store_with_materialized_layout() -> (
    ForgeStore,
    AspectLayoutReadRequest,
    String,
    BranchId,
    CommitId,
) {
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
        .materialize_milestone_6_layout_support(request.clone())
        .unwrap();
    (
        store,
        request,
        materialization.artifact_id().to_string(),
        branch_id,
        commit_id,
    )
}

#[test]
fn derived_reclaim_publishes_rebuild_debt_and_rebuild_restores_layout_family() {
    let (mut store, request, artifact_id, _branch_id, _commit_id) =
        store_with_materialized_layout();
    let policy = ConservativeRetentionPolicy::new(
        Vec::new(),
        Vec::new(),
        vec![DerivedFamilyRetentionPolicy::Milestone6LayoutMaterialization],
    );
    let planning = store
        .plan_retention_candidates(RetentionPolicyClass::Conservative(policy))
        .unwrap();
    let witness = planning
        .reclaim_candidates()
        .iter()
        .find(|witness| witness.artifact_family() == "milestone_6_layout_materialization")
        .cloned()
        .expect("layout reclaim witness");

    let reclaim = store.execute_derived_reclaim(witness).unwrap();
    assert_eq!(
        reclaim.reclaim_unit().family_label(),
        "milestone_6_layout_materialization"
    );
    assert_eq!(reclaim.rebuild_unit().rebuild_target_id(), artifact_id);
    assert!(reclaim.deleted_artifact_count() >= 1);
    assert_eq!(
        reclaim.cost_surface().reclaim_deletion_count(),
        reclaim.deleted_artifact_count()
    );
    assert!(store
        .fetch_milestone_6_layout_support(request.clone())
        .is_err());

    let rebuild = store
        .rebuild_reclaimed_derived_family(reclaim.rebuild_unit().clone())
        .unwrap();
    assert!(rebuild.rebuilt_artifact_count() >= 1);
    assert_eq!(rebuild.cost_surface().rebuild_debt_delta(), -1);
    let restored = store.fetch_milestone_6_layout_support(request).unwrap();
    assert_eq!(restored.artifact_id(), artifact_id);

    let counters = store.counters();
    assert_eq!(
        counters.reclaimed_derived_artifact_count,
        reclaim.deleted_artifact_count()
    );
    assert_eq!(counters.retained_range_rebuild_count, 1);
}

#[test]
fn derived_reclaim_rejects_live_stable_basis_conflicts() {
    let (mut store, _request, artifact_id, branch_id, commit_id) = store_with_materialized_layout();
    store
        .read_stable_basis(stable_basis_request(&store, branch_id.clone(), commit_id))
        .unwrap();

    let error = store
        .execute_derived_reclaim(ReclaimEligibilityWitness::new(
            "milestone_6_layout_materialization",
            artifact_id,
            format!("branch:{}@{}", branch_id.0, commit_id.0),
        ))
        .unwrap_err();

    assert_eq!(error.kind(), &StoreErrorKind::ReclaimLiveBasisConflict);
    assert_eq!(store.counters().reclaim_rejected_live_basis_count, 1);
}
