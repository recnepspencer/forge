use std::path::PathBuf;

use crate::{
    AspectLayoutReadRequest, AspectLayoutTarget, AspectProjectionSet, AspectScopeClass,
    ColdDerivedFamilyPolicy, ConservativePlacementPolicy, PlacementObservationScopeClass,
    PlacementPolicyClass, SingleEntityAspectScope, SnapshotCaptureRequest, WORTHStore,
    WORTHStoreBuilder,
};
use worth_relational::facade::history::{BranchId, CommitId};

use super::super::harness::fixtures::runtime::{
    create_entity, latest_envelope, runtime_with_demo_schema,
};

pub(super) fn conservative_policy() -> PlacementPolicyClass {
    PlacementPolicyClass::Conservative(
        ConservativePlacementPolicy::new(
            vec![
                ColdDerivedFamilyPolicy::SnapshotFamily,
                ColdDerivedFamilyPolicy::BranchDeltaFamily,
                ColdDerivedFamilyPolicy::Milestone6LayoutFamily,
            ],
            vec![
                PlacementObservationScopeClass::Branch,
                PlacementObservationScopeClass::RetainedBasis,
                PlacementObservationScopeClass::ArtifactFamily,
            ],
        )
        .unwrap(),
    )
}

fn layout_request(branch_id: BranchId, commit_id: CommitId) -> AspectLayoutReadRequest {
    AspectLayoutReadRequest::new(
        AspectLayoutTarget::new(branch_id, commit_id),
        AspectScopeClass::SingleEntity(SingleEntityAspectScope::new("entity-alpha")),
        AspectProjectionSet::new(vec!["profile".to_string()]),
    )
}

pub(super) fn tiering_phase3_fixture() -> (WORTHStore, BranchId, CommitId, u64, String) {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let envelope = latest_envelope(&runtime);
    let branch_id = envelope.branch_context.clone();
    let commit_id = envelope.commit.commit_id;

    let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
    store.append_canonical_commit(envelope).unwrap();
    let snapshot = store
        .capture_snapshot(SnapshotCaptureRequest::new(branch_id.clone(), commit_id))
        .unwrap();
    let materialization = store
        .materialize_milestone_6_layout_support(layout_request(branch_id.clone(), commit_id))
        .unwrap();

    (
        store,
        branch_id,
        commit_id,
        snapshot.snapshot_id.0,
        materialization.artifact_id().to_string(),
    )
}

pub(super) fn tiering_phase3_local_fixture(path: PathBuf) -> (WORTHStore, BranchId, CommitId, u64) {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let envelope = latest_envelope(&runtime);
    let branch_id = envelope.branch_context.clone();
    let commit_id = envelope.commit.commit_id;

    let mut store = WORTHStoreBuilder::new().local_file(path).build().unwrap();
    store.append_canonical_commit(envelope).unwrap();
    let snapshot = store
        .capture_snapshot(SnapshotCaptureRequest::new(branch_id.clone(), commit_id))
        .unwrap();

    (store, branch_id, commit_id, snapshot.snapshot_id.0)
}

pub(super) fn tiering_phase3_sqlite_fixture(
    path: PathBuf,
) -> (WORTHStore, BranchId, CommitId, u64, String) {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let envelope = latest_envelope(&runtime);
    let branch_id = envelope.branch_context.clone();
    let commit_id = envelope.commit.commit_id;

    let mut store = WORTHStoreBuilder::new().sqlite_file(path).build().unwrap();
    store.append_canonical_commit(envelope).unwrap();
    let snapshot = store
        .capture_snapshot(SnapshotCaptureRequest::new(branch_id.clone(), commit_id))
        .unwrap();
    let materialization = store
        .materialize_milestone_6_layout_support(layout_request(branch_id.clone(), commit_id))
        .unwrap();

    (
        store,
        branch_id,
        commit_id,
        snapshot.snapshot_id.0,
        materialization.artifact_id().to_string(),
    )
}
