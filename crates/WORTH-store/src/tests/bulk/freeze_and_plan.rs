use super::*;

#[test]
fn bulk_ingest_source_freezing_is_deterministic_across_input_order() {
    let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
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
    let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
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
    let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
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
    let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
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
    let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
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
