use crate::{
    ContinuationBatchBudget, ContinuationRetentionStatus, CursorContinuationRequest, FetchWidth,
    ForgeStore, ForgeStoreBuilder, MaxBatchItems, MaxCoveredCommits, MaxMaterializedBytes,
    MaxSupportRowsPerBatch, PlacementBoundArtifactRef, PlacementExecutionOrigin,
    PlacementObservationScopeClass, PlacementRaceOutcome, PlacementResolvedReadHandle,
    SnapshotCaptureRequest,
};
use forge_relational::facade::history::CommitId;

use super::harness::fixtures::runtime::{
    create_entity, latest_envelope, runtime_with_demo_schema, update_entity_on_branch,
};
use super::harness::fixtures::stores::unique_test_sqlite_path;
use super::live_query::helpers::stable_basis_request_for_store;

fn demo_budget() -> ContinuationBatchBudget {
    ContinuationBatchBudget::new(
        FetchWidth::new(16),
        MaxBatchItems::new(32),
        MaxCoveredCommits::new(4),
        MaxMaterializedBytes::new(4_096),
        MaxSupportRowsPerBatch::new(24),
    )
}

fn build_interleaving_store(
    builder: ForgeStoreBuilder,
) -> (
    ForgeStore,
    forge_relational::facade::history::BranchId,
    CommitId,
    CommitId,
) {
    let mut runtime = runtime_with_demo_schema();
    let entity_id = create_entity(&mut runtime, "alpha");
    let first_envelope = latest_envelope(&runtime);
    let branch_id = first_envelope.branch_context.clone();
    let first_commit_id = first_envelope.commit.commit_id;

    let mut store = builder.build().unwrap();
    store.append_canonical_commit(first_envelope).unwrap();

    update_entity_on_branch(&mut runtime, entity_id, "alpha-2", Some(branch_id.clone()));
    let second_envelope = latest_envelope(&runtime);
    let second_commit_id = second_envelope.commit.commit_id;
    store.append_canonical_commit(second_envelope).unwrap();

    (store, branch_id, first_commit_id, second_commit_id)
}

fn admit_inflight_branch_head_move(
    store: &mut ForgeStore,
    branch_id: &forge_relational::facade::history::BranchId,
) {
    let plan = store
        .plan_authoritative_tier_move(
            crate::PlacementPolicyClass::Conservative(
                crate::ConservativePlacementPolicy::new(
                    vec![
                        crate::ColdDerivedFamilyPolicy::SnapshotFamily,
                        crate::ColdDerivedFamilyPolicy::BranchDeltaFamily,
                        crate::ColdDerivedFamilyPolicy::Milestone6LayoutFamily,
                    ],
                    vec![
                        crate::PlacementObservationScopeClass::Branch,
                        crate::PlacementObservationScopeClass::RetainedBasis,
                        crate::PlacementObservationScopeClass::ArtifactFamily,
                    ],
                )
                .unwrap(),
            ),
            PlacementObservationScopeClass::Branch,
            &branch_id.0,
            PlacementExecutionOrigin::Background,
        )
        .unwrap()
        .tier_move_plan()
        .cloned()
        .unwrap();
    let intent = store.prepare_authoritative_tier_move(plan).unwrap();
    let _ = store.transfer_tier_replica(intent).unwrap();
}

#[test]
fn foreground_read_during_inflight_branch_move_exposes_transfer_observation() {
    let (mut store, branch_id, first_commit_id, _) =
        build_interleaving_store(ForgeStoreBuilder::new().in_memory());
    let basis = store
        .read_stable_basis(stable_basis_request_for_store(
            &store,
            branch_id.clone(),
            first_commit_id,
            "schema-support:v1",
            ContinuationRetentionStatus::Retained,
        ))
        .unwrap();
    admit_inflight_branch_head_move(&mut store, &branch_id);

    let report = store.observe_stable_basis_interleaving(&basis).unwrap();

    assert_eq!(
        report.observation().race_outcome(),
        PlacementRaceOutcome::TransferObserved
    );
    assert_eq!(
        report.placement_path(),
        crate::RetainedReadPlacementPath::WarmResident
    );
    assert!(report.foreground_isolation().is_some());
    assert!(report.parity_preserved());
}

#[test]
fn foreground_read_after_cutover_before_retire_exposes_cutover_observation() {
    let (mut store, branch_id, first_commit_id, _) =
        build_interleaving_store(ForgeStoreBuilder::new().in_memory());
    let basis = store
        .read_stable_basis(stable_basis_request_for_store(
            &store,
            branch_id.clone(),
            first_commit_id,
            "schema-support:v1",
            ContinuationRetentionStatus::Retained,
        ))
        .unwrap();
    let plan = store
        .plan_authoritative_tier_move(
            crate::PlacementPolicyClass::Conservative(
                crate::ConservativePlacementPolicy::new(
                    vec![
                        crate::ColdDerivedFamilyPolicy::SnapshotFamily,
                        crate::ColdDerivedFamilyPolicy::BranchDeltaFamily,
                        crate::ColdDerivedFamilyPolicy::Milestone6LayoutFamily,
                    ],
                    vec![
                        crate::PlacementObservationScopeClass::Branch,
                        crate::PlacementObservationScopeClass::RetainedBasis,
                        crate::PlacementObservationScopeClass::ArtifactFamily,
                    ],
                )
                .unwrap(),
            ),
            PlacementObservationScopeClass::Branch,
            &branch_id.0,
            PlacementExecutionOrigin::Background,
        )
        .unwrap()
        .tier_move_plan()
        .cloned()
        .unwrap();
    let intent = store.prepare_authoritative_tier_move(plan).unwrap();
    let transferred = store.transfer_tier_replica(intent).unwrap();
    let verified = store.verify_tier_replica(transferred).unwrap();
    let _ = store.cutover_tier_replica(verified).unwrap();

    let report = store.observe_stable_basis_interleaving(&basis).unwrap();
    assert_eq!(
        report.observation().race_outcome(),
        PlacementRaceOutcome::CutoverObserved
    );
}

#[test]
fn continuation_under_inflight_move_exposes_transfer_observation() {
    let (mut store, branch_id, first_commit_id, _) =
        build_interleaving_store(ForgeStoreBuilder::new().in_memory());
    let basis = store
        .read_stable_basis(stable_basis_request_for_store(
            &store,
            branch_id.clone(),
            first_commit_id,
            "schema-support:v1",
            ContinuationRetentionStatus::Retained,
        ))
        .unwrap();
    store
        .acknowledge_cursor(crate::DurableCursorAcknowledgeRequest::new(
            "cursor-main",
            "subscriber-a",
            branch_id.clone(),
            "demo-feed",
            "schema:v1",
            1,
            first_commit_id,
        ))
        .unwrap();
    admit_inflight_branch_head_move(&mut store, &branch_id);

    let plan = store
        .plan_cursor_continuation(CursorContinuationRequest::new(
            "cursor-main",
            "subscriber-a",
            branch_id.clone(),
            "demo-feed",
            "schema:v1",
            1,
            basis,
            demo_budget(),
        ))
        .unwrap();
    let result = store.execute_cursor_continuation(plan.clone()).unwrap();
    let report = store
        .observe_continuation_interleaving(&plan, Some(&result))
        .unwrap();

    assert_eq!(
        report.observation().race_outcome(),
        PlacementRaceOutcome::TransferObserved
    );
    assert_eq!(report.strategy(), result.resolved_strategy());
    assert!(report.parity_preserved());
}

#[test]
fn recalled_read_observation_counts_as_interleaving_recall() {
    let (mut store, branch_id, _, second_commit_id) =
        build_interleaving_store(ForgeStoreBuilder::new().in_memory());
    let snapshot = store
        .capture_snapshot(SnapshotCaptureRequest::new(
            branch_id.clone(),
            second_commit_id,
        ))
        .unwrap();
    store
        .admit_inflight_cold_recall(
            PlacementBoundArtifactRef::snapshot_family(snapshot.snapshot_id.0.to_string()),
            PlacementExecutionOrigin::Background,
        )
        .unwrap();
    let report = store
        .plan_cold_recall_lease(
            PlacementBoundArtifactRef::snapshot_family(snapshot.snapshot_id.0.to_string()),
            PlacementExecutionOrigin::Foreground,
        )
        .unwrap();
    let handle: PlacementResolvedReadHandle =
        store.resolve_cold_recall_read_handle(report.cold_recall_lease().unwrap());
    let interleaving = store.observe_placement_read_interleaving(&handle).unwrap();

    assert_eq!(
        interleaving.observation().race_outcome(),
        PlacementRaceOutcome::RecallObserved
    );
    let counters = store.milestone_13_counter_contract();
    assert_eq!(counters.tier_interleaved_read_count, 1);
    assert_eq!(counters.tier_interleaving_recall_count, 1);
    assert_eq!(counters.tier_interleaving_parity_failure_count, 0);
}

#[test]
fn reopened_store_preserves_inflight_read_interleaving_observation() {
    let path = unique_test_sqlite_path("forge-store-tiering-phase5-interleaving");
    let (mut store, branch_id, first_commit_id, _) =
        build_interleaving_store(ForgeStoreBuilder::new().sqlite_file(path.clone()));
    let basis = store
        .read_stable_basis(stable_basis_request_for_store(
            &store,
            branch_id.clone(),
            first_commit_id,
            "schema-support:v1",
            ContinuationRetentionStatus::Retained,
        ))
        .unwrap();
    let stable_basis_id = basis.stable_basis_id().clone();
    admit_inflight_branch_head_move(&mut store, &branch_id);
    drop(store);

    let reopened = ForgeStoreBuilder::new().sqlite_file(path).build().unwrap();
    let fetched = reopened.fetch_stable_basis(&stable_basis_id).unwrap();
    let report = reopened
        .observe_stable_basis_interleaving(&fetched)
        .unwrap();

    assert_eq!(
        report.observation().race_outcome(),
        PlacementRaceOutcome::TransferObserved
    );
}
