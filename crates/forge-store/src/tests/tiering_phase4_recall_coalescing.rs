use std::path::PathBuf;

use crate::{
    AspectLayoutReadRequest, AspectLayoutTarget, AspectProjectionSet, AspectScopeClass,
    ColdDerivedFamilyPolicy, ComplexityStatus, ConservativePlacementPolicy, ForgeStore,
    ForgeStoreBuilder, PlacementBoundArtifactRef, PlacementExecutionOrigin,
    PlacementObservationScopeClass, PlacementPolicyClass, SingleEntityAspectScope,
    SnapshotCaptureRequest,
};
use forge_relational::facade::history::{BranchId, CommitId};
use rusqlite::{params, Connection};

use super::harness::fixtures::{
    runtime::{create_entity, latest_envelope, runtime_with_demo_schema},
    stores::unique_test_sqlite_path,
};

fn conservative_policy() -> PlacementPolicyClass {
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

fn tiering_phase4_fixture() -> (ForgeStore, u64) {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let envelope = latest_envelope(&runtime);
    let branch_id = envelope.branch_context.clone();
    let commit_id = envelope.commit.commit_id;

    let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    store.append_canonical_commit(envelope).unwrap();
    let snapshot = store
        .capture_snapshot(SnapshotCaptureRequest::new(branch_id.clone(), commit_id))
        .unwrap();
    store
        .materialize_milestone_6_layout_support(layout_request(branch_id, commit_id))
        .unwrap();

    let derived = store
        .plan_derived_tier_move(
            conservative_policy(),
            ColdDerivedFamilyPolicy::SnapshotFamily,
            &snapshot.snapshot_id.0.to_string(),
            PlacementExecutionOrigin::Background,
        )
        .unwrap()
        .tier_move_plan()
        .cloned()
        .unwrap();
    let intent = store.prepare_derived_tier_move(derived).unwrap();
    let transferred = store.transfer_tier_replica(intent).unwrap();
    let verified = store.verify_tier_replica(transferred).unwrap();
    let cutover = store.cutover_tier_replica(verified).unwrap();
    store.retire_tier_replica(cutover).unwrap();

    (store, snapshot.snapshot_id.0)
}

fn tiering_phase4_sqlite_fixture(path: PathBuf) -> (ForgeStore, u64) {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let envelope = latest_envelope(&runtime);
    let branch_id = envelope.branch_context.clone();
    let commit_id = envelope.commit.commit_id;

    let mut store = ForgeStoreBuilder::new().sqlite_file(path).build().unwrap();
    store.append_canonical_commit(envelope).unwrap();
    let snapshot = store
        .capture_snapshot(SnapshotCaptureRequest::new(branch_id.clone(), commit_id))
        .unwrap();
    store
        .materialize_milestone_6_layout_support(layout_request(branch_id, commit_id))
        .unwrap();

    (store, snapshot.snapshot_id.0)
}

#[test]
fn duplicate_snapshot_recall_coalesces_and_counts_exactly() {
    let (mut store, snapshot_id) = tiering_phase4_fixture();
    store
        .admit_inflight_cold_recall(
            PlacementBoundArtifactRef::snapshot_family(snapshot_id.to_string()),
            PlacementExecutionOrigin::Background,
        )
        .unwrap();
    let report = store
        .plan_cold_recall_lease(
            PlacementBoundArtifactRef::snapshot_family(snapshot_id.to_string()),
            PlacementExecutionOrigin::Foreground,
        )
        .unwrap();

    let joined = store
        .execute_cold_recall(
            report.cold_recall_lease().cloned().unwrap(),
            report.recall_witness().cloned().unwrap(),
        )
        .unwrap();

    assert_eq!(
        joined.disposition(),
        crate::RecallExecutionDisposition::CoalescedJoin
    );
    assert!(joined.completion_witness().is_none());
    assert_eq!(joined.artifact_key(), format!("snapshot:{snapshot_id}"));

    let counters = store.milestone_13_counter_contract();
    assert_eq!(counters.recall_coalesced_request_count, 1);
    assert_eq!(counters.recall_duplicate_suppression_count, 1);
    assert_eq!(counters.cold_tier_recall_count, 0);
    assert_eq!(counters.foreground_cold_recall_count, 0);
    assert_eq!(counters.tier_miss_count, 0);

    let surface = store.milestone_13_complexity_surface();
    assert_eq!(surface.recall_coalescing.status, ComplexityStatus::Verified);
}

#[test]
fn broadened_recall_plan_does_not_replace_family_local_coalescing_identity() {
    let (mut store, snapshot_id) = tiering_phase4_fixture();
    let broadened = store
        .plan_broadened_recall(
            ColdDerivedFamilyPolicy::SnapshotFamily,
            PlacementObservationScopeClass::ArtifactFamily,
            &snapshot_id.to_string(),
            vec![format!("snapshot:{snapshot_id}")],
            PlacementExecutionOrigin::Background,
        )
        .unwrap();
    assert_eq!(broadened.scope_key(), &snapshot_id.to_string());

    let report = store
        .plan_cold_recall_lease(
            PlacementBoundArtifactRef::snapshot_family(snapshot_id.to_string()),
            PlacementExecutionOrigin::Foreground,
        )
        .unwrap();
    store
        .admit_inflight_cold_recall(
            PlacementBoundArtifactRef::snapshot_family(snapshot_id.to_string()),
            PlacementExecutionOrigin::Background,
        )
        .unwrap();
    let joined = store
        .execute_cold_recall(
            report.cold_recall_lease().cloned().unwrap(),
            report.recall_witness().cloned().unwrap(),
        )
        .unwrap();

    assert_eq!(
        joined.coalescing_key().scope_class(),
        PlacementObservationScopeClass::ArtifactFamily
    );
}

#[test]
fn sqlite_in_flight_recall_state_resumes_after_reopen() {
    let path = unique_test_sqlite_path("forge-store-tiering-phase4-recall");
    let (store, snapshot_id) = tiering_phase4_sqlite_fixture(path.clone());
    drop(store);

    let connection = Connection::open(&path).unwrap();
    connection
        .execute(
            "
            INSERT INTO tier_recall_records(
                coalescing_key,
                artifact_family,
                scope_class,
                scope_key,
                execution_origin,
                artifact_key,
                recall_cost_class,
                amplification_budget,
                completion_state
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
            ",
            params![
                format!("snapshot_family|artifact_family|{snapshot_id}"),
                "snapshot_family",
                "artifact_family",
                snapshot_id.to_string(),
                "restart_recovery",
                format!("snapshot:{snapshot_id}"),
                "deferred",
                "single_family_local_unit",
                "in_flight",
            ],
        )
        .unwrap();
    drop(connection);

    let mut reopened = ForgeStoreBuilder::new().sqlite_file(path).build().unwrap();
    let report = reopened
        .plan_cold_recall_lease(
            PlacementBoundArtifactRef::snapshot_family(snapshot_id.to_string()),
            PlacementExecutionOrigin::RestartRecovery,
        )
        .unwrap();
    let resumed = reopened
        .execute_cold_recall(
            report.cold_recall_lease().cloned().unwrap(),
            report.recall_witness().cloned().unwrap(),
        )
        .unwrap();

    assert_eq!(
        resumed.disposition(),
        crate::RecallExecutionDisposition::Executed
    );
    assert!(resumed.completion_witness().is_some());
    assert_eq!(resumed.artifact_key(), format!("snapshot:{snapshot_id}"));
    let counters = reopened.milestone_13_counter_contract();
    assert_eq!(counters.restart_recall_count, 1);
    assert_eq!(counters.cold_tier_recall_count, 1);
    assert_eq!(counters.recall_duplicate_suppression_count, 0);
}

#[test]
fn sqlite_completed_recall_state_fails_on_open() {
    let path = unique_test_sqlite_path("forge-store-tiering-phase4-completed-recall");
    let (store, snapshot_id) = tiering_phase4_sqlite_fixture(path.clone());
    drop(store);

    let connection = Connection::open(&path).unwrap();
    connection
        .execute(
            "
            INSERT INTO tier_recall_records(
                coalescing_key,
                artifact_family,
                scope_class,
                scope_key,
                execution_origin,
                artifact_key,
                recall_cost_class,
                amplification_budget,
                completion_state
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
            ",
            params![
                format!("snapshot_family|artifact_family|{snapshot_id}"),
                "snapshot_family",
                "artifact_family",
                snapshot_id.to_string(),
                "background",
                format!("snapshot:{snapshot_id}"),
                "deferred",
                "single_family_local_unit",
                "completed",
            ],
        )
        .unwrap();
    drop(connection);

    let error = ForgeStoreBuilder::new()
        .sqlite_file(path)
        .build()
        .expect_err("open should fail on persisted completed recall state");
    assert_eq!(
        error.kind(),
        &crate::StoreErrorKind::BackendIntegrityViolation
    );
}
