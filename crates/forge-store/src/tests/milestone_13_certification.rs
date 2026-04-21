use crate::{ForgeStore, ForgeStoreBuilder};

use super::harness::{
    certification::{
        assertions::{assert_all_equal, assert_any_not_equal},
        core::{AssertionClass, CanonicalRow, CertificationSuite, LaneResult},
        requirements::{evaluate_completeness, TIERING_AND_WORKING_SET_NON_AUTHORITY_TEST},
    },
    fixtures::{
        runtime::{create_entity, latest_envelope, runtime_with_demo_schema},
        stores::{unique_test_sqlite_path, unique_test_store_path},
    },
};

use crate::{
    AspectLayoutReadRequest, AspectLayoutTarget, AspectProjectionSet, AspectScopeClass,
    ColdDerivedFamilyPolicy, ConservativePlacementPolicy, ContinuationBatchBudget,
    ContinuationRetentionStatus, CursorContinuationRequest, FetchWidth, MaxBatchItems,
    MaxCoveredCommits, MaxMaterializedBytes, MaxSupportRowsPerBatch, PlacementBoundArtifactRef,
    PlacementExecutionOrigin, PlacementObservationScopeClass, PlacementPolicyClass,
    PlacementRaceOutcome, SingleEntityAspectScope, SnapshotCaptureRequest,
};

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

fn layout_request(
    branch_id: forge_relational::facade::history::BranchId,
    commit_id: forge_relational::facade::history::CommitId,
) -> AspectLayoutReadRequest {
    AspectLayoutReadRequest::new(
        AspectLayoutTarget::new(branch_id, commit_id),
        AspectScopeClass::SingleEntity(SingleEntityAspectScope::new("entity-alpha")),
        AspectProjectionSet::new(vec!["profile".to_string()]),
    )
}

fn build_store(builder: ForgeStoreBuilder) -> (ForgeStore, u64) {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let envelope = latest_envelope(&runtime);
    let branch_id = envelope.branch_context.clone();
    let commit_id = envelope.commit.commit_id;

    let mut store = builder.build().unwrap();
    store.append_canonical_commit(envelope).unwrap();
    let snapshot = store
        .capture_snapshot(SnapshotCaptureRequest::new(branch_id.clone(), commit_id))
        .unwrap();
    store
        .materialize_milestone_6_layout_support(layout_request(branch_id, commit_id))
        .unwrap();
    (store, snapshot.snapshot_id.0)
}

fn execute_tiering_batch(store: &mut ForgeStore, snapshot_id: u64) {
    let snapshot_basis_label = format!("snapshot:{snapshot_id}");
    let authoritative = store
        .plan_authoritative_tier_move(
            conservative_policy(),
            PlacementObservationScopeClass::RetainedBasis,
            &snapshot_basis_label,
            PlacementExecutionOrigin::Background,
        )
        .unwrap()
        .tier_move_plan()
        .cloned()
        .unwrap();
    let authoritative_intent = store
        .prepare_authoritative_tier_move(authoritative)
        .unwrap();
    let authoritative_transferred = store.transfer_tier_replica(authoritative_intent).unwrap();
    let authoritative_verified = store
        .verify_tier_replica(authoritative_transferred)
        .unwrap();
    let authoritative_cutover = store.cutover_tier_replica(authoritative_verified).unwrap();
    store.retire_tier_replica(authoritative_cutover).unwrap();

    let derived = store
        .plan_derived_tier_move(
            conservative_policy(),
            ColdDerivedFamilyPolicy::SnapshotFamily,
            &snapshot_id.to_string(),
            PlacementExecutionOrigin::Background,
        )
        .unwrap()
        .tier_move_plan()
        .cloned()
        .unwrap();
    let derived_intent = store.prepare_derived_tier_move(derived).unwrap();
    let derived_transferred = store.transfer_tier_replica(derived_intent).unwrap();
    let derived_verified = store.verify_tier_replica(derived_transferred).unwrap();
    let derived_cutover = store.cutover_tier_replica(derived_verified).unwrap();
    store.retire_tier_replica(derived_cutover).unwrap();

    let cold = store
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
    let second = store
        .execute_cold_recall(
            cold.cold_recall_lease().cloned().unwrap(),
            cold.recall_witness().cloned().unwrap(),
        )
        .unwrap();
    assert_eq!(
        second.disposition(),
        crate::RecallExecutionDisposition::CoalescedJoin
    );
    assert!(second.completion_witness().is_none());
}

fn interleaved_tiering_lane(builder: ForgeStoreBuilder) -> (ForgeStore, u64) {
    let (mut store, snapshot_id) = build_store(builder);
    let derived = store
        .plan_derived_tier_move(
            conservative_policy(),
            ColdDerivedFamilyPolicy::SnapshotFamily,
            &snapshot_id.to_string(),
            PlacementExecutionOrigin::Background,
        )
        .unwrap()
        .tier_move_plan()
        .cloned()
        .unwrap();
    let intent = store.prepare_derived_tier_move(derived).unwrap();
    let _ = store.transfer_tier_replica(intent).unwrap();

    let cold = store
        .plan_cold_recall_lease(
            PlacementBoundArtifactRef::snapshot_family(snapshot_id.to_string()),
            PlacementExecutionOrigin::Foreground,
        )
        .unwrap();
    let joined = store
        .execute_cold_recall(
            cold.cold_recall_lease().cloned().unwrap(),
            cold.recall_witness().cloned().unwrap(),
        )
        .unwrap();
    assert_eq!(joined.artifact_key(), format!("snapshot:{snapshot_id}"));
    (store, snapshot_id)
}

fn recalled_tiering_lane(builder: ForgeStoreBuilder) -> (ForgeStore, u64) {
    let (mut store, snapshot_id) = build_store(builder);
    let derived = store
        .plan_derived_tier_move(
            conservative_policy(),
            ColdDerivedFamilyPolicy::SnapshotFamily,
            &snapshot_id.to_string(),
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

    let cold = store
        .plan_cold_recall_lease(
            PlacementBoundArtifactRef::snapshot_family(snapshot_id.to_string()),
            PlacementExecutionOrigin::Foreground,
        )
        .unwrap();
    let recalled = store
        .execute_cold_recall(
            cold.cold_recall_lease().cloned().unwrap(),
            cold.recall_witness().cloned().unwrap(),
        )
        .unwrap();
    assert_eq!(
        recalled.disposition(),
        crate::RecallExecutionDisposition::Executed
    );
    assert!(recalled.completion_witness().is_some());
    (store, snapshot_id)
}

fn admit_branch_head_transfer(
    store: &mut ForgeStore,
    branch_id: &forge_relational::facade::history::BranchId,
) {
    let plan = store
        .plan_authoritative_tier_move(
            conservative_policy(),
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

fn foreground_read_interleaving_lane(builder: ForgeStoreBuilder) -> ForgeStore {
    let (mut store, _) = build_store(builder);
    let export = store.export_authoritative_records().into_canonicalized();
    let envelope = export.commit_envelopes.first().unwrap().envelope.clone();
    let branch_id = envelope.branch_context.clone();
    let basis = store
        .read_stable_basis(stable_basis_request_for_store(
            &store,
            branch_id.clone(),
            envelope.commit.commit_id,
            "schema-support:v1",
            ContinuationRetentionStatus::Retained,
        ))
        .unwrap();
    admit_branch_head_transfer(&mut store, &branch_id);
    let report = store.observe_stable_basis_interleaving(&basis).unwrap();
    assert_eq!(
        report.observation().race_outcome(),
        PlacementRaceOutcome::TransferObserved
    );
    store
}

fn continuation_interleaving_lane(builder: ForgeStoreBuilder) -> ForgeStore {
    let mut runtime = runtime_with_demo_schema();
    let entity_id = create_entity(&mut runtime, "alpha");
    let first_envelope = latest_envelope(&runtime);
    let branch_id = first_envelope.branch_context.clone();
    let first_commit_id = first_envelope.commit.commit_id;
    let mut store = builder.build().unwrap();
    store.append_canonical_commit(first_envelope).unwrap();
    super::harness::fixtures::runtime::update_entity_on_branch(
        &mut runtime,
        entity_id,
        "alpha-2",
        Some(branch_id.clone()),
    );
    let second_envelope = latest_envelope(&runtime);
    store.append_canonical_commit(second_envelope).unwrap();
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
    admit_branch_head_transfer(&mut store, &branch_id);
    let plan = store
        .plan_cursor_continuation(CursorContinuationRequest::new(
            "cursor-main",
            "subscriber-a",
            branch_id,
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
    store
}

fn interleaving_counter_lane(builder: ForgeStoreBuilder) -> ForgeStore {
    let mut store = continuation_interleaving_lane(builder);
    let export = store.export_authoritative_records().into_canonicalized();
    let envelope = export.commit_envelopes.first().unwrap().envelope.clone();
    let basis = store
        .read_stable_basis(stable_basis_request_for_store(
            &store,
            envelope.branch_context.clone(),
            envelope.commit.commit_id,
            "schema-support:v1",
            ContinuationRetentionStatus::Retained,
        ))
        .unwrap();
    let foreground = store.observe_stable_basis_interleaving(&basis).unwrap();
    assert_eq!(
        foreground.observation().race_outcome(),
        PlacementRaceOutcome::TransferObserved
    );

    let snapshot = store
        .capture_snapshot(SnapshotCaptureRequest::new(
            envelope.branch_context.clone(),
            envelope.commit.commit_id,
        ))
        .unwrap();
    store
        .admit_inflight_cold_recall(
            PlacementBoundArtifactRef::snapshot_family(snapshot.snapshot_id.0.to_string()),
            PlacementExecutionOrigin::Background,
        )
        .unwrap();
    let cold = store
        .plan_cold_recall_lease(
            PlacementBoundArtifactRef::snapshot_family(snapshot.snapshot_id.0.to_string()),
            PlacementExecutionOrigin::Foreground,
        )
        .unwrap();
    let handle = store.resolve_cold_recall_read_handle(cold.cold_recall_lease().unwrap());
    let report = store.observe_placement_read_interleaving(&handle).unwrap();
    assert_eq!(
        report.observation().race_outcome(),
        PlacementRaceOutcome::RecallObserved
    );
    store
}

fn milestone_13_suite() -> CertificationSuite<String, String> {
    let (control_store, control_snapshot_id) = build_store(ForgeStoreBuilder::new().in_memory());
    let control_export = control_store.export_authoritative_records();
    let control_bundle = control_store
        .milestone_13_certification_bundle(&control_export)
        .unwrap();

    let (mut moved_store, moved_snapshot_id) = build_store(ForgeStoreBuilder::new().in_memory());
    execute_tiering_batch(&mut moved_store, moved_snapshot_id);
    let moved_bundle = moved_store
        .milestone_13_certification_bundle(&control_export)
        .unwrap();

    let sqlite_path = unique_test_sqlite_path("forge-store-m13-certification");
    let (mut sqlite_store, sqlite_snapshot_id) =
        build_store(ForgeStoreBuilder::new().sqlite_file(sqlite_path.clone()));
    execute_tiering_batch(&mut sqlite_store, sqlite_snapshot_id);
    let sqlite_moved_bundle = sqlite_store
        .milestone_13_certification_bundle(&control_export)
        .unwrap();
    let sqlite_manifest =
        serde_json::to_string(&sqlite_store.recover_tiering_state().unwrap()).unwrap();
    drop(sqlite_store);

    let reopened_store = ForgeStoreBuilder::new()
        .sqlite_file(sqlite_path)
        .build()
        .unwrap();
    let reopened_bundle = reopened_store
        .milestone_13_certification_bundle(&control_export)
        .unwrap();
    let reopened_manifest =
        serde_json::to_string(&reopened_store.recover_tiering_state().unwrap()).unwrap();

    let local_path = unique_test_store_path("forge-store-m13-certification-local");
    let (mut local_store, local_snapshot_id) =
        build_store(ForgeStoreBuilder::new().local_file(local_path.clone()));
    execute_tiering_batch(&mut local_store, local_snapshot_id);
    let local_moved_bundle = local_store
        .milestone_13_certification_bundle(&control_export)
        .unwrap();
    let local_manifest =
        serde_json::to_string(&local_store.recover_tiering_state().unwrap()).unwrap();
    drop(local_store);

    let reopened_local_store = ForgeStoreBuilder::new()
        .local_file(local_path)
        .build()
        .unwrap();
    let reopened_local_bundle = reopened_local_store
        .milestone_13_certification_bundle(&control_export)
        .unwrap();
    let reopened_local_manifest =
        serde_json::to_string(&reopened_local_store.recover_tiering_state().unwrap()).unwrap();

    let (recalled_store, _) = recalled_tiering_lane(ForgeStoreBuilder::new().in_memory());
    let recalled_bundle = recalled_store
        .milestone_13_certification_bundle(&control_export)
        .unwrap();

    let (interleaved_store, _) = interleaved_tiering_lane(ForgeStoreBuilder::new().in_memory());
    let interleaved_bundle = interleaved_store
        .milestone_13_certification_bundle(&control_export)
        .unwrap();

    let foreground_interleaved_store =
        foreground_read_interleaving_lane(ForgeStoreBuilder::new().in_memory());
    let foreground_interleaved_bundle = foreground_interleaved_store
        .milestone_13_certification_bundle(&control_export)
        .unwrap();

    let continuation_interleaved_store =
        continuation_interleaving_lane(ForgeStoreBuilder::new().in_memory());
    let continuation_interleaved_bundle = continuation_interleaved_store
        .milestone_13_certification_bundle(&control_export)
        .unwrap();

    let interleaving_counter_store =
        interleaving_counter_lane(ForgeStoreBuilder::new().in_memory());
    let interleaving_counter_contract = interleaving_counter_store.milestone_13_counter_contract();

    let expected_counter_contract =
        serde_json::to_string(&moved_store.milestone_13_counter_contract()).unwrap();

    let _ = control_snapshot_id;

    CertificationSuite::new(TIERING_AND_WORKING_SET_NON_AUTHORITY_TEST.suite_name)
        .with_canonical_row(CanonicalRow::new(
            "truth_digest_parity",
            vec![
                LaneResult::new("control", control_bundle.truth_digest.clone()),
                LaneResult::new("moved", moved_bundle.truth_digest.clone()),
                LaneResult::new("local_reopened", reopened_local_bundle.truth_digest.clone()),
                LaneResult::new("sqlite_reopened", reopened_bundle.truth_digest.clone()),
            ],
            &[AssertionClass::Equality],
        ))
        .with_canonical_row(CanonicalRow::new(
            "artifact_digest_parity",
            vec![
                LaneResult::new("control", control_bundle.artifact_digest.clone()),
                LaneResult::new("moved", moved_bundle.artifact_digest.clone()),
                LaneResult::new(
                    "local_reopened",
                    reopened_local_bundle.artifact_digest.clone(),
                ),
                LaneResult::new("sqlite_reopened", reopened_bundle.artifact_digest.clone()),
            ],
            &[AssertionClass::Equality],
        ))
        .with_canonical_row(CanonicalRow::new(
            "diagnostics_digest_divergence",
            vec![
                LaneResult::new("control", control_bundle.diagnostics_digest.clone()),
                LaneResult::new("moved", moved_bundle.diagnostics_digest.clone()),
                LaneResult::new(
                    "local_reopened",
                    reopened_local_bundle.diagnostics_digest.clone(),
                ),
                LaneResult::new(
                    "sqlite_reopened",
                    reopened_bundle.diagnostics_digest.clone(),
                ),
            ],
            &[AssertionClass::Inequality],
        ))
        .with_canonical_row(CanonicalRow::new(
            "counter_snapshot_exactness",
            vec![
                LaneResult::new(
                    "moved",
                    serde_json::to_string(&moved_bundle.counter_contract).unwrap(),
                ),
                LaneResult::new(
                    "local_moved",
                    serde_json::to_string(&local_moved_bundle.counter_contract).unwrap(),
                ),
                LaneResult::new(
                    "sqlite_moved",
                    serde_json::to_string(&sqlite_moved_bundle.counter_contract).unwrap(),
                ),
                LaneResult::new("expected", expected_counter_contract),
            ],
            &[AssertionClass::Equality, AssertionClass::ExactCounter],
        ))
        .with_canonical_row(CanonicalRow::new(
            "recalled_lane_truth_parity",
            vec![
                LaneResult::new("control", control_bundle.truth_digest.clone()),
                LaneResult::new("recalled", recalled_bundle.truth_digest.clone()),
                LaneResult::new("sqlite_reopened", reopened_bundle.truth_digest.clone()),
            ],
            &[AssertionClass::Equality],
        ))
        .with_canonical_row(CanonicalRow::new(
            "coalesced_duplicate_suppression_exactness",
            vec![
                LaneResult::new(
                    "moved",
                    format!(
                        "{}:{}",
                        moved_bundle.counter_contract.recall_coalesced_request_count,
                        moved_bundle
                            .counter_contract
                            .recall_duplicate_suppression_count
                    ),
                ),
                LaneResult::new(
                    "sqlite_moved",
                    format!(
                        "{}:{}",
                        sqlite_moved_bundle
                            .counter_contract
                            .recall_coalesced_request_count,
                        sqlite_moved_bundle
                            .counter_contract
                            .recall_duplicate_suppression_count
                    ),
                ),
                LaneResult::new("expected", "1:1".to_string()),
            ],
            &[AssertionClass::Equality, AssertionClass::ExactCounter],
        ))
        .with_canonical_row(CanonicalRow::new(
            "restart_manifest_bounded_reconstruction",
            vec![
                LaneResult::new("sqlite_before_reopen", sqlite_manifest),
                LaneResult::new("sqlite_after_reopen", reopened_manifest),
                LaneResult::new("local_before_reopen", local_manifest),
                LaneResult::new("local_after_reopen", reopened_local_manifest),
            ],
            &[AssertionClass::Equality],
        ))
        .with_canonical_row(CanonicalRow::new(
            "movement_read_interleaving_truth_parity",
            vec![
                LaneResult::new("control", control_bundle.truth_digest.clone()),
                LaneResult::new("interleaved", interleaved_bundle.truth_digest.clone()),
            ],
            &[AssertionClass::Equality],
        ))
        .with_canonical_row(CanonicalRow::new(
            "foreground_read_move_interleaving_truth_parity",
            vec![
                LaneResult::new("control", control_bundle.truth_digest.clone()),
                LaneResult::new(
                    "foreground_interleaved",
                    foreground_interleaved_bundle.truth_digest.clone(),
                ),
            ],
            &[AssertionClass::Equality],
        ))
        .with_canonical_row(CanonicalRow::new(
            "continuation_move_interleaving_truth_parity",
            vec![
                LaneResult::new("control", control_bundle.truth_digest.clone()),
                LaneResult::new(
                    "continuation_interleaved",
                    continuation_interleaved_bundle.truth_digest.clone(),
                ),
            ],
            &[AssertionClass::Equality],
        ))
        .with_canonical_row(CanonicalRow::new(
            "interleaving_counter_exactness",
            vec![
                LaneResult::new(
                    "observed",
                    format!(
                        "{}:{}:{}:{}",
                        interleaving_counter_contract.tier_interleaved_read_count,
                        interleaving_counter_contract.tier_interleaved_continuation_count,
                        interleaving_counter_contract.tier_interleaving_recall_count,
                        interleaving_counter_contract.tier_interleaving_parity_failure_count
                    ),
                ),
                LaneResult::new("expected", "2:1:1:0".to_string()),
            ],
            &[AssertionClass::Equality, AssertionClass::ExactCounter],
        ))
}

#[test]
fn milestone_13_certification_suite_is_complete_and_truth_equal() {
    let suite = milestone_13_suite();
    assert_all_equal(&suite.canonical_rows()[0]);
    assert_all_equal(&suite.canonical_rows()[1]);
    let completeness = evaluate_completeness(&suite, &TIERING_AND_WORKING_SET_NON_AUTHORITY_TEST);
    assert!(completeness.missing_rows().is_empty());
    assert!(completeness.missing_assertion_classes().is_empty());
}

#[test]
fn milestone_13_certification_diagnostics_diverge_while_truth_stays_equal() {
    let suite = milestone_13_suite();
    assert_all_equal(&suite.canonical_rows()[0]);
    assert_all_equal(&suite.canonical_rows()[1]);
    assert_any_not_equal(&suite.canonical_rows()[2]);
}

#[test]
fn milestone_13_certification_counters_match_expected_batch() {
    let suite = milestone_13_suite();
    assert_all_equal(&suite.canonical_rows()[3]);
}

#[test]
fn milestone_13_certification_bundle_summary_flags_are_adversarially_meaningful() {
    let (control_store, _) = build_store(ForgeStoreBuilder::new().in_memory());
    let control_export = control_store.export_authoritative_records();
    let control_bundle = control_store
        .milestone_13_certification_bundle(&control_export)
        .unwrap();

    let (mut moved_store, moved_snapshot_id) = build_store(ForgeStoreBuilder::new().in_memory());
    execute_tiering_batch(&mut moved_store, moved_snapshot_id);
    let moved_bundle = moved_store
        .milestone_13_certification_bundle(&control_export)
        .unwrap();

    assert!(
        moved_bundle
            .certification_summary
            .truth_matches_control_lane
    );
    assert!(
        moved_bundle
            .certification_summary
            .no_tier_truth_parity_failures
    );
    assert!(
        moved_bundle
            .certification_summary
            .no_tier_restore_parity_failures
    );
    assert!(moved_bundle.certification_summary.no_tier_recall_failures);
    assert!(
        moved_bundle
            .certification_summary
            .no_residual_residency_ambiguity
    );
    assert_eq!(
        moved_bundle
            .artifact_report
            .residual_residency_ambiguity_count,
        0
    );
    assert_eq!(moved_bundle.truth_digest, control_bundle.truth_digest);
    assert_eq!(moved_bundle.artifact_digest, control_bundle.artifact_digest);
    assert_ne!(
        moved_bundle.diagnostics_digest,
        control_bundle.diagnostics_digest
    );
    assert!(
        moved_bundle.certification_summary.verified_path_count > 0,
        "certification summary should count verified paths"
    );
    assert!(
        moved_bundle.certification_summary.debt_path_count > 0,
        "coalesced-only moved lane should keep unexercised recall execution explicit as debt"
    );
}
