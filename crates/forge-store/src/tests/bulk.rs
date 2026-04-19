use super::harness::{
    corruption::{
        local_file::{
            force_bulk_checkpoint_completed_chunk_regression, force_bulk_checkpoint_gap,
            force_bulk_plan_payload_chunk_width_drift,
            force_bulk_witness_index_highest_ordinal_regression,
            force_bulk_witness_index_witness_count_drift, force_bulk_witness_missing_commit,
            force_frozen_transform_basis_payload_scope_drift,
        },
        sqlite::{
            delete_sqlite_bulk_checkpoint, drift_sqlite_bulk_witness_index_witness_count,
            drift_sqlite_frozen_transform_partition_payload_member_width,
            regress_sqlite_bulk_checkpoint_completed_chunk,
            regress_sqlite_bulk_witness_index_highest_ordinal,
        },
    },
    fixtures::{
        runtime::{create_entity, latest_envelope, runtime_with_demo_schema},
        stores::{unique_test_sqlite_path, unique_test_store_path},
    },
};
use crate::{
    BulkCheckpointPolicy, BulkIngestSourceRequest, BulkSourceMember, BulkTransformRequest,
    ChunkOrdinal, ChunkWidthBudget, DurableRetryResolution, ForgeStoreBuilder, StoreErrorKind,
};
use forge_relational::facade::history::{BranchId, CommitId};

fn persist_two_checkpoint_bulk_family(
    store: &mut crate::ForgeStore,
    program_id: &str,
    source_identity: &str,
) -> (
    crate::FrozenBulkSourceManifest,
    crate::DeterministicChunkPlan,
) {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, &format!("{program_id}-entity-a"));
    let first_envelope = latest_envelope(&runtime);
    create_entity(&mut runtime, &format!("{program_id}-entity-b"));
    let second_envelope = latest_envelope(&runtime);
    let manifest = store
        .freeze_bulk_ingest_source(BulkIngestSourceRequest::new(
            program_id,
            source_identity,
            first_envelope.branch_context.clone(),
            vec![
                BulkSourceMember::new("a", 1),
                BulkSourceMember::new("b", 2),
                BulkSourceMember::new("c", 1),
                BulkSourceMember::new("d", 1),
            ],
        ))
        .unwrap();
    let plan = store
        .plan_bulk_ingest(manifest.clone(), ChunkWidthBudget::new(3))
        .unwrap();
    let admitted = store
        .admit_bulk_ingest_chunk(&plan, ChunkOrdinal::new(0), 3)
        .unwrap();
    let request = store
        .admit_bulk_canonical_chunk_execution(admitted, first_envelope)
        .unwrap();
    store
        .execute_bulk_canonical_chunk(request, BulkCheckpointPolicy::Publish)
        .unwrap();

    let resumed = store
        .admit_bulk_ingest_resume(program_id, plan.plan_id(), manifest.manifest_digest())
        .unwrap();
    store
        .execute_next_resumed_bulk_chunk(
            &resumed,
            3,
            second_envelope,
            BulkCheckpointPolicy::Publish,
        )
        .unwrap()
        .expect("second chunk should execute");

    (manifest, plan)
}

fn persist_transform_artifacts(
    store: &mut crate::ForgeStore,
    program_id: &str,
    transform_identity: &str,
) -> (
    crate::FrozenTransformBasis,
    crate::FrozenTransformTargetPartition,
    crate::DeterministicChunkPlan,
) {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, &format!("{program_id}-transform-authority"));
    let envelope = latest_envelope(&runtime);
    let request = BulkTransformRequest::new(
        program_id,
        transform_identity,
        envelope.branch_context,
        envelope.commit.commit_id,
        vec![
            BulkSourceMember::new("alpha", 1),
            BulkSourceMember::new("beta", 2),
            BulkSourceMember::new("gamma", 1),
        ],
    );
    let basis = store.freeze_bulk_transform_basis(request.clone()).unwrap();
    let partition = store
        .freeze_bulk_transform_target_partition(request, &basis)
        .unwrap();
    let plan = store
        .plan_bulk_transform(&basis, &partition, ChunkWidthBudget::new(3))
        .unwrap();
    (basis, partition, plan)
}

#[test]
fn bulk_ingest_source_freezing_is_deterministic_across_input_order() {
    let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    let branch = BranchId("bulk-main".to_string());

    let left = store
        .freeze_bulk_ingest_source(BulkIngestSourceRequest::new(
            "program-alpha",
            "source-demo",
            branch.clone(),
            vec![
                BulkSourceMember::new("gamma.json", 2),
                BulkSourceMember::new("alpha.json", 1),
                BulkSourceMember::new("beta.json", 3),
            ],
        ))
        .unwrap();
    let right = store
        .freeze_bulk_ingest_source(BulkIngestSourceRequest::new(
            "program-alpha",
            "source-demo",
            branch,
            vec![
                BulkSourceMember::new("beta.json", 3),
                BulkSourceMember::new("gamma.json", 2),
                BulkSourceMember::new("alpha.json", 1),
            ],
        ))
        .unwrap();

    assert_eq!(left.manifest_digest(), right.manifest_digest());
    assert_eq!(
        left.ordered_members()
            .iter()
            .map(|member| member.member_id())
            .collect::<Vec<_>>(),
        vec!["alpha.json", "beta.json", "gamma.json"]
    );

    let counters = store.counters();
    assert_eq!(counters.bulk_program_plan_count, 2);
    assert_eq!(counters.bulk_source_manifest_member_count, 6);
    assert_eq!(counters.bulk_source_manifest_stream_pass_count, 2);
}

#[test]
fn bulk_ingest_plan_chunks_deterministically_and_tracks_chunk_count() {
    let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    let manifest = store
        .freeze_bulk_ingest_source(BulkIngestSourceRequest::new(
            "program-plan",
            "source-plan",
            BranchId("bulk-main".to_string()),
            vec![
                BulkSourceMember::new("a", 2),
                BulkSourceMember::new("b", 2),
                BulkSourceMember::new("c", 1),
                BulkSourceMember::new("d", 3),
            ],
        ))
        .unwrap();

    let plan = store
        .plan_bulk_ingest(manifest, ChunkWidthBudget::new(4))
        .unwrap();

    assert_eq!(plan.chunk_count(), 2);
    assert_eq!(
        plan.chunks()[0].member_ids(),
        &["a".to_string(), "b".to_string()]
    );
    assert_eq!(
        plan.chunks()[1].member_ids(),
        &["c".to_string(), "d".to_string()]
    );

    let counters = store.counters();
    assert_eq!(counters.bulk_chunk_plan_count, 2);
}

#[test]
fn bulk_ingest_chunk_admission_rejects_before_materialization_when_budget_is_too_small() {
    let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    let manifest = store
        .freeze_bulk_ingest_source(BulkIngestSourceRequest::new(
            "program-budget",
            "source-budget",
            BranchId("bulk-main".to_string()),
            vec![
                BulkSourceMember::new("wide-a", 3),
                BulkSourceMember::new("wide-b", 1),
            ],
        ))
        .unwrap();
    let plan = store
        .plan_bulk_ingest(manifest, ChunkWidthBudget::new(4))
        .unwrap();

    let error = store
        .admit_bulk_ingest_chunk(&plan, ChunkOrdinal::new(0), 2)
        .expect_err("undersized memory budget must reject before materialization");

    assert_eq!(error.kind(), &StoreErrorKind::BulkChunkWidthBudgetExceeded);
    assert_eq!(store.counters().bulk_chunk_execute_count, 0);
}

#[test]
fn bulk_ingest_materialization_emits_cost_carrying_receipt_and_counters() {
    let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    let manifest = store
        .freeze_bulk_ingest_source(BulkIngestSourceRequest::new(
            "program-receipt",
            "source-receipt",
            BranchId("bulk-main".to_string()),
            vec![
                BulkSourceMember::new("a", 1),
                BulkSourceMember::new("b", 2),
                BulkSourceMember::new("c", 1),
            ],
        ))
        .unwrap();
    let plan = store
        .plan_bulk_ingest(manifest, ChunkWidthBudget::new(3))
        .unwrap();
    let admitted = store
        .admit_bulk_ingest_chunk(&plan, ChunkOrdinal::new(0), 3)
        .unwrap();

    let receipt = store.materialize_bulk_ingest_chunk(&admitted).unwrap();

    assert_eq!(receipt.program_id(), "program-receipt");
    assert_eq!(receipt.plan_id(), plan.plan_id());
    assert_eq!(receipt.chunk_ordinal(), ChunkOrdinal::new(0));
    assert_eq!(receipt.admitted_width_units(), 3);
    assert_eq!(receipt.materialized_member_count(), 2);
    assert_eq!(receipt.materialization_breadth_units(), 2);
    assert_eq!(receipt.memory_units(), 3);

    let counters = store.counters();
    assert_eq!(counters.bulk_chunk_execute_count, 1);
    assert_eq!(counters.bulk_chunk_width_units, 3);
    assert_eq!(counters.bulk_peak_in_flight_memory_units, 3);
    assert_eq!(counters.bulk_fallback_path_count, 0);
}

#[test]
fn bulk_transform_artifacts_are_fetchable_and_chunk_admission_is_symmetric() {
    let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    let request = BulkTransformRequest::new(
        "program-transform",
        "transform-demo",
        BranchId("bulk-main".to_string()),
        CommitId(41),
        vec![
            BulkSourceMember::new("gamma", 2),
            BulkSourceMember::new("alpha", 1),
            BulkSourceMember::new("beta", 2),
        ],
    );

    let basis = store.freeze_bulk_transform_basis(request.clone()).unwrap();
    let partition = store
        .freeze_bulk_transform_target_partition(request, &basis)
        .unwrap();
    let plan = store
        .plan_bulk_transform(&basis, &partition, ChunkWidthBudget::new(3))
        .unwrap();

    let fetched_basis = store
        .fetch_frozen_transform_basis("program-transform", basis.basis_digest())
        .unwrap();
    let fetched_partition = store
        .fetch_frozen_transform_partition("program-transform", partition.partition_digest())
        .unwrap();
    let admitted = store
        .admit_bulk_transform_chunk(&plan, ChunkOrdinal::new(0), 3)
        .unwrap();

    assert_eq!(fetched_basis.basis_digest(), basis.basis_digest());
    assert_eq!(
        fetched_partition.partition_digest(),
        partition.partition_digest()
    );
    assert_eq!(admitted.chunk().ordinal(), ChunkOrdinal::new(0));
}

#[test]
fn bulk_frozen_manifest_and_checkpoint_fetch_survive_sqlite_reopen() {
    let path = unique_test_sqlite_path("forge-store-bulk-fetch");
    let mut store = ForgeStoreBuilder::new()
        .sqlite_file(path.clone())
        .build()
        .unwrap();
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "bulk-fetch-authority");
    let envelope = latest_envelope(&runtime);
    let manifest = store
        .freeze_bulk_ingest_source(BulkIngestSourceRequest::new(
            "program-fetch",
            "source-fetch",
            envelope.branch_context.clone(),
            vec![
                BulkSourceMember::new("a", 1),
                BulkSourceMember::new("b", 2),
                BulkSourceMember::new("c", 1),
            ],
        ))
        .unwrap();
    let plan = store
        .plan_bulk_ingest(manifest.clone(), ChunkWidthBudget::new(3))
        .unwrap();
    let admitted = store
        .admit_bulk_ingest_chunk(&plan, ChunkOrdinal::new(0), 3)
        .unwrap();
    store.append_canonical_commit(envelope.clone()).unwrap();
    let witness = store
        .publish_bulk_chunk_witness(&admitted, envelope.commit.commit_id)
        .unwrap();
    store.publish_bulk_progress_checkpoint(&witness).unwrap();
    drop(store);

    let reopened = ForgeStoreBuilder::new().sqlite_file(path).build().unwrap();
    let fetched_manifest = reopened
        .fetch_frozen_bulk_manifest("program-fetch", manifest.manifest_digest())
        .unwrap();
    let fetched_checkpoint = reopened
        .fetch_bulk_progress_checkpoint("program-fetch", plan.plan_id())
        .unwrap();

    assert_eq!(
        fetched_manifest.manifest_digest(),
        manifest.manifest_digest()
    );
    assert_eq!(fetched_checkpoint.checkpoint_sequence(), 1);
}

#[test]
fn bulk_witness_with_missing_canonical_commit_fails_reopen_closed() {
    let path = unique_test_store_path("forge-store-bulk-witness-missing-commit");
    let mut store = ForgeStoreBuilder::new()
        .local_file(path.clone())
        .build()
        .unwrap();
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "bulk-witness-corruption");
    let envelope = latest_envelope(&runtime);
    let manifest = store
        .freeze_bulk_ingest_source(BulkIngestSourceRequest::new(
            "program-witness-integrity",
            "source-witness-integrity",
            envelope.branch_context.clone(),
            vec![BulkSourceMember::new("a", 1)],
        ))
        .unwrap();
    let plan = store
        .plan_bulk_ingest(manifest, ChunkWidthBudget::new(1))
        .unwrap();
    let admitted = store
        .admit_bulk_ingest_chunk(&plan, ChunkOrdinal::new(0), 1)
        .unwrap();
    let request = store
        .admit_bulk_canonical_chunk_execution(admitted, envelope.clone())
        .unwrap();
    store
        .execute_bulk_canonical_chunk(request, BulkCheckpointPolicy::Publish)
        .unwrap();
    drop(store);

    force_bulk_witness_missing_commit(&path, "program-witness-integrity", plan.plan_id(), 0);

    let error = ForgeStoreBuilder::new()
        .local_file(path)
        .build()
        .expect_err("reopen should fail when a bulk witness references a missing canonical commit");
    assert_eq!(error.kind(), &StoreErrorKind::BackendIntegrityViolation);
}

#[test]
fn bulk_checkpoint_gap_fails_local_file_reopen_closed() {
    let path = unique_test_store_path("forge-store-bulk-checkpoint-gap");
    let mut store = ForgeStoreBuilder::new()
        .local_file(path.clone())
        .build()
        .unwrap();
    let (_manifest, plan) = persist_two_checkpoint_bulk_family(
        &mut store,
        "program-checkpoint-gap",
        "source-checkpoint-gap",
    );
    drop(store);

    force_bulk_checkpoint_gap(&path, "program-checkpoint-gap", plan.plan_id(), 1);

    let error = ForgeStoreBuilder::new()
        .local_file(path)
        .build()
        .expect_err("reopen should fail when a bulk checkpoint family has a sequence gap");
    assert_eq!(error.kind(), &StoreErrorKind::BackendIntegrityViolation);
}

#[test]
fn bulk_checkpoint_completed_chunk_regression_fails_local_file_reopen_closed() {
    let path = unique_test_store_path("forge-store-bulk-checkpoint-regression");
    let mut store = ForgeStoreBuilder::new()
        .local_file(path.clone())
        .build()
        .unwrap();
    let (_manifest, plan) = persist_two_checkpoint_bulk_family(
        &mut store,
        "program-checkpoint-regression",
        "source-checkpoint-regression",
    );
    drop(store);

    force_bulk_checkpoint_completed_chunk_regression(
        &path,
        "program-checkpoint-regression",
        plan.plan_id(),
        2,
        0,
    );

    let error = ForgeStoreBuilder::new()
        .local_file(path)
        .build()
        .expect_err(
            "reopen should fail when a bulk checkpoint regresses the completed chunk ordinal",
        );
    assert_eq!(error.kind(), &StoreErrorKind::BackendIntegrityViolation);
}

#[test]
fn bulk_checkpoint_index_reference_missing_checkpoint_fails_sqlite_reopen_closed() {
    let path = unique_test_sqlite_path("forge-store-bulk-checkpoint-index-gap");
    let mut store = ForgeStoreBuilder::new()
        .sqlite_file(path.clone())
        .build()
        .unwrap();
    let (_manifest, plan) = persist_two_checkpoint_bulk_family(
        &mut store,
        "program-checkpoint-index-gap",
        "source-checkpoint-index-gap",
    );
    drop(store);

    delete_sqlite_bulk_checkpoint(&path, "program-checkpoint-index-gap", plan.plan_id(), 2);

    let error = ForgeStoreBuilder::new()
        .sqlite_file(path)
        .build()
        .expect_err("reopen should fail when witness index points at a missing bulk checkpoint");
    assert_eq!(error.kind(), &StoreErrorKind::BackendIntegrityViolation);
}

#[test]
fn bulk_checkpoint_completed_chunk_regression_fails_sqlite_reopen_closed() {
    let path = unique_test_sqlite_path("forge-store-bulk-checkpoint-regression-sqlite");
    let mut store = ForgeStoreBuilder::new()
        .sqlite_file(path.clone())
        .build()
        .unwrap();
    let (_manifest, plan) = persist_two_checkpoint_bulk_family(
        &mut store,
        "program-checkpoint-regression-sqlite",
        "source-checkpoint-regression-sqlite",
    );
    drop(store);

    regress_sqlite_bulk_checkpoint_completed_chunk(
        &path,
        "program-checkpoint-regression-sqlite",
        plan.plan_id(),
        2,
        0,
    );

    let error = ForgeStoreBuilder::new()
        .sqlite_file(path)
        .build()
        .expect_err(
        "reopen should fail when a sqlite bulk checkpoint regresses the completed chunk ordinal",
    );
    assert_eq!(error.kind(), &StoreErrorKind::BackendIntegrityViolation);
}

#[test]
fn bulk_witness_index_highest_ordinal_regression_fails_local_file_reopen_closed() {
    let path = unique_test_store_path("forge-store-bulk-witness-index-regression");
    let mut store = ForgeStoreBuilder::new()
        .local_file(path.clone())
        .build()
        .unwrap();
    let (_manifest, plan) = persist_two_checkpoint_bulk_family(
        &mut store,
        "program-witness-index-regression",
        "source-witness-index-regression",
    );
    drop(store);

    force_bulk_witness_index_highest_ordinal_regression(
        &path,
        "program-witness-index-regression",
        plan.plan_id(),
        0,
    );

    let error = ForgeStoreBuilder::new()
        .local_file(path)
        .build()
        .expect_err(
            "reopen should fail when witness index regresses below the persisted witness set",
        );
    assert_eq!(error.kind(), &StoreErrorKind::BackendIntegrityViolation);
}

#[test]
fn bulk_witness_index_witness_count_drift_fails_local_file_reopen_closed() {
    let path = unique_test_store_path("forge-store-bulk-witness-index-count-drift");
    let mut store = ForgeStoreBuilder::new()
        .local_file(path.clone())
        .build()
        .unwrap();
    let (_manifest, plan) = persist_two_checkpoint_bulk_family(
        &mut store,
        "program-witness-index-count-drift",
        "source-witness-index-count-drift",
    );
    drop(store);

    force_bulk_witness_index_witness_count_drift(
        &path,
        "program-witness-index-count-drift",
        plan.plan_id(),
        1,
    );

    let error = ForgeStoreBuilder::new()
        .local_file(path)
        .build()
        .expect_err("reopen should fail when witness index witness count drifts below the persisted witness set");
    assert_eq!(error.kind(), &StoreErrorKind::BackendIntegrityViolation);
}

#[test]
fn bulk_witness_index_highest_ordinal_regression_fails_sqlite_reopen_closed() {
    let path = unique_test_sqlite_path("forge-store-bulk-witness-index-regression-sqlite");
    let mut store = ForgeStoreBuilder::new()
        .sqlite_file(path.clone())
        .build()
        .unwrap();
    let (_manifest, plan) = persist_two_checkpoint_bulk_family(
        &mut store,
        "program-witness-index-regression-sqlite",
        "source-witness-index-regression-sqlite",
    );
    drop(store);

    regress_sqlite_bulk_witness_index_highest_ordinal(
        &path,
        "program-witness-index-regression-sqlite",
        plan.plan_id(),
        0,
    );

    let error = ForgeStoreBuilder::new()
        .sqlite_file(path)
        .build()
        .expect_err(
        "reopen should fail when a sqlite witness index regresses below the persisted witness set",
    );
    assert_eq!(error.kind(), &StoreErrorKind::BackendIntegrityViolation);
}

#[test]
fn bulk_witness_index_witness_count_drift_fails_sqlite_reopen_closed() {
    let path = unique_test_sqlite_path("forge-store-bulk-witness-index-count-drift-sqlite");
    let mut store = ForgeStoreBuilder::new()
        .sqlite_file(path.clone())
        .build()
        .unwrap();
    let (_manifest, plan) = persist_two_checkpoint_bulk_family(
        &mut store,
        "program-witness-index-count-drift-sqlite",
        "source-witness-index-count-drift-sqlite",
    );
    drop(store);

    drift_sqlite_bulk_witness_index_witness_count(
        &path,
        "program-witness-index-count-drift-sqlite",
        plan.plan_id(),
        1,
    );

    let error = ForgeStoreBuilder::new()
        .sqlite_file(path)
        .build()
        .expect_err("reopen should fail when a sqlite witness index witness count drifts below the persisted witness set");
    assert_eq!(error.kind(), &StoreErrorKind::BackendIntegrityViolation);
}

#[test]
fn frozen_transform_basis_payload_drift_fails_local_file_reopen_closed() {
    let path = unique_test_store_path("forge-store-transform-basis-drift");
    let mut store = ForgeStoreBuilder::new()
        .local_file(path.clone())
        .build()
        .unwrap();
    let (basis, _partition, _plan) =
        persist_transform_artifacts(&mut store, "program-transform-drift", "transform-drift");
    drop(store);

    force_frozen_transform_basis_payload_scope_drift(
        &path,
        "program-transform-drift",
        basis.basis_digest(),
    );

    let error = ForgeStoreBuilder::new()
        .local_file(path)
        .build()
        .expect_err(
            "reopen should fail when a frozen transform basis payload drifts under a stale digest",
        );
    assert_eq!(error.kind(), &StoreErrorKind::BackendIntegrityViolation);
}

#[test]
fn frozen_transform_partition_payload_drift_fails_sqlite_reopen_closed() {
    let path = unique_test_sqlite_path("forge-store-transform-partition-drift-sqlite");
    let mut store = ForgeStoreBuilder::new()
        .sqlite_file(path.clone())
        .build()
        .unwrap();
    let (_basis, partition, _plan) = persist_transform_artifacts(
        &mut store,
        "program-transform-partition-drift",
        "transform-partition-drift",
    );
    drop(store);

    drift_sqlite_frozen_transform_partition_payload_member_width(
        &path,
        "program-transform-partition-drift",
        partition.partition_digest(),
    );

    let error = ForgeStoreBuilder::new()
        .sqlite_file(path)
        .build()
        .expect_err(
        "reopen should fail when a frozen transform partition payload drifts under a stale digest",
    );
    assert_eq!(error.kind(), &StoreErrorKind::BackendIntegrityViolation);
}

#[test]
fn bulk_plan_payload_drift_fails_local_file_reopen_closed() {
    let path = unique_test_store_path("forge-store-bulk-plan-drift");
    let mut store = ForgeStoreBuilder::new()
        .local_file(path.clone())
        .build()
        .unwrap();
    let (_manifest, plan) =
        persist_two_checkpoint_bulk_family(&mut store, "program-plan-drift", "source-plan-drift");
    drop(store);

    force_bulk_plan_payload_chunk_width_drift(&path, "program-plan-drift", plan.plan_id());

    let error = ForgeStoreBuilder::new()
        .local_file(path)
        .build()
        .expect_err(
            "reopen should fail when a persisted bulk plan payload drifts under a stale plan id",
        );
    assert_eq!(error.kind(), &StoreErrorKind::BackendIntegrityViolation);
}

#[test]
fn bulk_checkpoint_publication_rejects_non_advancing_duplicate_witness() {
    let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "bulk-checkpoint-duplicate");
    let envelope = latest_envelope(&runtime);
    let manifest = store
        .freeze_bulk_ingest_source(BulkIngestSourceRequest::new(
            "program-checkpoint-duplicate",
            "source-checkpoint-duplicate",
            envelope.branch_context.clone(),
            vec![BulkSourceMember::new("a", 1), BulkSourceMember::new("b", 1)],
        ))
        .unwrap();
    let plan = store
        .plan_bulk_ingest(manifest, ChunkWidthBudget::new(2))
        .unwrap();
    let admitted = store
        .admit_bulk_ingest_chunk(&plan, ChunkOrdinal::new(0), 2)
        .unwrap();
    store.append_canonical_commit(envelope.clone()).unwrap();
    let witness = store
        .publish_bulk_chunk_witness(&admitted, envelope.commit.commit_id)
        .unwrap();

    let first_checkpoint = store.publish_bulk_progress_checkpoint(&witness).unwrap();
    let error = store
        .publish_bulk_progress_checkpoint(&witness)
        .expect_err("duplicate checkpoint publication for the same witness should fail");

    assert_eq!(first_checkpoint.checkpoint_sequence(), 1);
    assert_eq!(error.kind(), &StoreErrorKind::BulkCheckpointPublicationGap);
}

#[test]
fn bulk_ingest_resume_admission_handles_not_started_programs_explicitly() {
    let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    let manifest = store
        .freeze_bulk_ingest_source(BulkIngestSourceRequest::new(
            "program-resume-ingest",
            "source-resume-ingest",
            BranchId("bulk-main".to_string()),
            vec![BulkSourceMember::new("a", 1), BulkSourceMember::new("b", 2)],
        ))
        .unwrap();
    let plan = store
        .plan_bulk_ingest(manifest.clone(), ChunkWidthBudget::new(3))
        .unwrap();

    let resumed = store
        .admit_bulk_ingest_resume(
            "program-resume-ingest",
            plan.plan_id(),
            manifest.manifest_digest(),
        )
        .unwrap();

    assert_eq!(resumed.plan().plan_id(), plan.plan_id());
    assert!(resumed.witness_index().is_none());
    assert!(resumed.latest_checkpoint().is_none());
    assert_eq!(
        resumed.resume_boundary().latest_committed_chunk_ordinal(),
        None
    );
    assert_eq!(
        resumed.resume_boundary().next_chunk_ordinal(),
        ChunkOrdinal::new(0)
    );
    let admitted = resumed
        .admit_next_chunk(3)
        .expect("next chunk admission should succeed")
        .expect("not-started program should admit first chunk");
    assert_eq!(admitted.chunk().ordinal(), ChunkOrdinal::new(0));
    assert_eq!(store.counters().bulk_chunk_resume_count, 1);
}

#[test]
fn bulk_transform_resume_admission_requires_locked_basis_and_resume_state() {
    let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "bulk-transform-resume-authority");
    let envelope = latest_envelope(&runtime);
    let request = BulkTransformRequest::new(
        "program-resume-transform",
        "transform-resume",
        envelope.branch_context.clone(),
        CommitId(44),
        vec![
            BulkSourceMember::new("alpha", 1),
            BulkSourceMember::new("beta", 2),
            BulkSourceMember::new("gamma", 1),
        ],
    );
    let basis = store.freeze_bulk_transform_basis(request.clone()).unwrap();
    let partition = store
        .freeze_bulk_transform_target_partition(request, &basis)
        .unwrap();
    let plan = store
        .plan_bulk_transform(&basis, &partition, ChunkWidthBudget::new(3))
        .unwrap();
    let admitted = store
        .admit_bulk_transform_chunk(&plan, ChunkOrdinal::new(0), 3)
        .unwrap();
    store.append_canonical_commit(envelope.clone()).unwrap();
    let witness = store
        .publish_bulk_chunk_witness(&admitted, envelope.commit.commit_id)
        .unwrap();
    let checkpoint = store.publish_bulk_progress_checkpoint(&witness).unwrap();

    let resumed = store
        .admit_bulk_transform_resume(
            "program-resume-transform",
            plan.plan_id(),
            basis.basis_digest(),
            partition.partition_digest(),
        )
        .unwrap();

    assert_eq!(resumed.plan().plan_id(), plan.plan_id());
    assert_eq!(
        resumed
            .witness_index()
            .expect("started program must expose witness index")
            .highest_committed_chunk_ordinal(),
        ChunkOrdinal::new(0)
    );
    assert_eq!(
        resumed
            .latest_checkpoint()
            .expect("started program must expose latest checkpoint")
            .checkpoint_sequence(),
        checkpoint.checkpoint_sequence()
    );
    assert_eq!(
        resumed.resume_boundary().latest_committed_chunk_ordinal(),
        Some(ChunkOrdinal::new(0))
    );
    assert_eq!(
        resumed.resume_boundary().next_chunk_ordinal(),
        ChunkOrdinal::new(1)
    );
    let admitted = resumed
        .admit_next_chunk(3)
        .expect("next chunk admission should succeed")
        .expect("partially completed transform should admit next chunk");
    assert_eq!(admitted.chunk().ordinal(), ChunkOrdinal::new(1));

    let error = store
        .admit_bulk_transform_resume(
            "program-resume-transform",
            plan.plan_id(),
            "wrong-basis",
            partition.partition_digest(),
        )
        .expect_err("resume admission must reject mismatched locked basis");
    assert_eq!(error.kind(), &StoreErrorKind::BulkTransformBasisDrift);
}

#[test]
fn bulk_resume_ready_program_reports_completion_without_admitting_more_chunks() {
    let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "bulk-complete");
    let envelope = latest_envelope(&runtime);
    let manifest = store
        .freeze_bulk_ingest_source(BulkIngestSourceRequest::new(
            "program-complete",
            "source-complete",
            envelope.branch_context.clone(),
            vec![BulkSourceMember::new("a", 1), BulkSourceMember::new("b", 1)],
        ))
        .unwrap();
    let plan = store
        .plan_bulk_ingest(manifest.clone(), ChunkWidthBudget::new(2))
        .unwrap();
    let admitted = store
        .admit_bulk_ingest_chunk(&plan, ChunkOrdinal::new(0), 2)
        .unwrap();
    let request = store
        .admit_bulk_canonical_chunk_execution(admitted, envelope)
        .unwrap();
    store
        .execute_bulk_canonical_chunk(request, BulkCheckpointPolicy::Publish)
        .unwrap();

    let resumed = store
        .admit_bulk_ingest_resume(
            "program-complete",
            plan.plan_id(),
            manifest.manifest_digest(),
        )
        .unwrap();

    assert!(resumed.is_complete());
    assert_eq!(resumed.next_chunk_ordinal(), ChunkOrdinal::new(1));
    assert!(resumed
        .admit_next_chunk(2)
        .expect("completion check should succeed")
        .is_none());
}

#[test]
fn bulk_execute_next_resumed_chunk_finalizes_witness_and_checkpoint() {
    let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "bulk-execute-a");
    let envelope = latest_envelope(&runtime);
    let manifest = store
        .freeze_bulk_ingest_source(BulkIngestSourceRequest::new(
            "program-execute",
            "source-execute",
            envelope.branch_context.clone(),
            vec![
                BulkSourceMember::new("a", 1),
                BulkSourceMember::new("b", 2),
                BulkSourceMember::new("c", 1),
            ],
        ))
        .unwrap();
    let plan = store
        .plan_bulk_ingest(manifest.clone(), ChunkWidthBudget::new(3))
        .unwrap();
    let resumed = store
        .admit_bulk_ingest_resume(
            "program-execute",
            plan.plan_id(),
            manifest.manifest_digest(),
        )
        .unwrap();

    let outcome = store
        .execute_next_resumed_bulk_chunk(
            &resumed,
            3,
            envelope.clone(),
            BulkCheckpointPolicy::Publish,
        )
        .unwrap()
        .expect("not-started program should execute its first chunk");

    assert_eq!(
        outcome.materialization_receipt().chunk_ordinal(),
        ChunkOrdinal::new(0)
    );
    assert_eq!(
        outcome.chunk_commit_witness().canonical_commit_id(),
        envelope.commit.commit_id
    );
    assert_eq!(
        outcome
            .published_checkpoint()
            .expect("checkpoint should be published")
            .checkpoint_sequence(),
        1
    );
    let counters = store.counters();
    assert_eq!(counters.bulk_chunk_commit_count, 1);
    assert_eq!(counters.bulk_chunk_witness_write_count, 1);
    assert_eq!(counters.bulk_checkpoint_write_count, 1);
}

#[test]
fn bulk_execute_next_resumed_chunk_advances_checkpoint_sequence_for_started_programs() {
    let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "bulk-execute-b");
    let first_envelope = latest_envelope(&runtime);
    create_entity(&mut runtime, "bulk-execute-c");
    let second_envelope = latest_envelope(&runtime);
    let manifest = store
        .freeze_bulk_ingest_source(BulkIngestSourceRequest::new(
            "program-execute-continued",
            "source-execute-continued",
            first_envelope.branch_context.clone(),
            vec![
                BulkSourceMember::new("a", 1),
                BulkSourceMember::new("b", 2),
                BulkSourceMember::new("c", 1),
                BulkSourceMember::new("d", 1),
            ],
        ))
        .unwrap();
    let plan = store
        .plan_bulk_ingest(manifest.clone(), ChunkWidthBudget::new(3))
        .unwrap();
    let admitted = store
        .admit_bulk_ingest_chunk(&plan, ChunkOrdinal::new(0), 3)
        .unwrap();
    let request = store
        .admit_bulk_canonical_chunk_execution(admitted, first_envelope)
        .unwrap();
    store
        .execute_bulk_canonical_chunk(request, BulkCheckpointPolicy::Publish)
        .unwrap();

    let resumed = store
        .admit_bulk_ingest_resume(
            "program-execute-continued",
            plan.plan_id(),
            manifest.manifest_digest(),
        )
        .unwrap();
    let outcome = store
        .execute_next_resumed_bulk_chunk(
            &resumed,
            3,
            second_envelope,
            BulkCheckpointPolicy::Publish,
        )
        .unwrap()
        .expect("partially completed program should execute next chunk");

    assert_eq!(
        outcome.materialization_receipt().chunk_ordinal(),
        ChunkOrdinal::new(1)
    );
    assert_eq!(
        outcome
            .published_checkpoint()
            .expect("checkpoint should be published")
            .checkpoint_sequence(),
        2
    );
}

#[test]
fn bulk_execute_canonical_chunk_durably_records_wal_lifecycle_and_acknowledgment() {
    let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "bulk-durable-a");
    let envelope = latest_envelope(&runtime);
    let manifest = store
        .freeze_bulk_ingest_source(BulkIngestSourceRequest::new(
            "program-durable",
            "source-durable",
            envelope.branch_context.clone(),
            vec![BulkSourceMember::new("a", 1), BulkSourceMember::new("b", 2)],
        ))
        .unwrap();
    let plan = store
        .plan_bulk_ingest(manifest, ChunkWidthBudget::new(3))
        .unwrap();
    let admitted = store
        .admit_bulk_ingest_chunk(&plan, ChunkOrdinal::new(0), 3)
        .unwrap();
    let request = store
        .admit_bulk_canonical_chunk_execution(admitted, envelope.clone())
        .unwrap();

    let durable = store
        .execute_bulk_canonical_chunk_durably(request, BulkCheckpointPolicy::Publish)
        .unwrap();

    assert_eq!(
        durable
            .execution_outcome()
            .chunk_commit_witness()
            .canonical_commit_id(),
        envelope.commit.commit_id
    );
    assert_eq!(
        durable
            .execution_outcome()
            .published_checkpoint()
            .expect("durable execution should publish a checkpoint")
            .checkpoint_sequence(),
        1
    );
    assert_eq!(
        store
            .resolve_durable_retry(durable.durable_mutation_id())
            .unwrap(),
        DurableRetryResolution::PreviouslyAcknowledgedEquivalentCommit {
            commit_id: envelope.commit.commit_id
        }
    );
    let counters = store.counters();
    assert_eq!(counters.durable_mutation_admit_count, 1);
    assert_eq!(counters.durable_commit_acknowledged_count, 1);
    assert_eq!(counters.wal_record_append_count, 6);
}

#[test]
fn bulk_execute_next_resumed_chunk_durably_advances_checkpoint_sequence() {
    let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "bulk-durable-b");
    let first_envelope = latest_envelope(&runtime);
    create_entity(&mut runtime, "bulk-durable-c");
    let second_envelope = latest_envelope(&runtime);
    let manifest = store
        .freeze_bulk_ingest_source(BulkIngestSourceRequest::new(
            "program-durable-resume",
            "source-durable-resume",
            first_envelope.branch_context.clone(),
            vec![
                BulkSourceMember::new("a", 1),
                BulkSourceMember::new("b", 2),
                BulkSourceMember::new("c", 1),
                BulkSourceMember::new("d", 1),
            ],
        ))
        .unwrap();
    let plan = store
        .plan_bulk_ingest(manifest.clone(), ChunkWidthBudget::new(3))
        .unwrap();
    let admitted = store
        .admit_bulk_ingest_chunk(&plan, ChunkOrdinal::new(0), 3)
        .unwrap();
    let request = store
        .admit_bulk_canonical_chunk_execution(admitted, first_envelope)
        .unwrap();
    store
        .execute_bulk_canonical_chunk(request, BulkCheckpointPolicy::Publish)
        .unwrap();

    let resumed = store
        .admit_bulk_ingest_resume(
            "program-durable-resume",
            plan.plan_id(),
            manifest.manifest_digest(),
        )
        .unwrap();
    let durable = store
        .execute_next_resumed_bulk_chunk_durably(
            &resumed,
            3,
            second_envelope.clone(),
            BulkCheckpointPolicy::Publish,
        )
        .unwrap()
        .expect("partially completed program should durably execute next chunk");

    assert_eq!(
        durable
            .execution_outcome()
            .materialization_receipt()
            .chunk_ordinal(),
        ChunkOrdinal::new(1)
    );
    assert_eq!(
        durable
            .execution_outcome()
            .published_checkpoint()
            .expect("durable resumed execution should publish a checkpoint")
            .checkpoint_sequence(),
        2
    );
    assert_eq!(
        store
            .resolve_durable_retry(durable.durable_mutation_id())
            .unwrap(),
        DurableRetryResolution::PreviouslyAcknowledgedEquivalentCommit {
            commit_id: second_envelope.commit.commit_id
        }
    );
}

#[test]
fn bulk_canonical_chunk_execution_request_rejects_branch_mismatch() {
    let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    let manifest = store
        .freeze_bulk_ingest_source(BulkIngestSourceRequest::new(
            "program-branch-mismatch",
            "source-branch-mismatch",
            BranchId("bulk-main".to_string()),
            vec![BulkSourceMember::new("a", 1)],
        ))
        .unwrap();
    let plan = store
        .plan_bulk_ingest(manifest, ChunkWidthBudget::new(1))
        .unwrap();
    let admitted = store
        .admit_bulk_ingest_chunk(&plan, ChunkOrdinal::new(0), 1)
        .unwrap();

    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "bulk-wrong-branch");
    let mut wrong_envelope = latest_envelope(&runtime);
    wrong_envelope.branch_context = BranchId("some-other-branch".to_string());

    let error = store
        .admit_bulk_canonical_chunk_execution(admitted, wrong_envelope)
        .expect_err("branch mismatch must reject before append");

    assert_eq!(
        error.kind(),
        &StoreErrorKind::ConcurrentBulkBoundaryViolation
    );
}
