use super::*;

#[test]
fn degraded_basis_continuation_executes_as_broadened_and_counts_breadth() {
    let mut runtime = runtime_with_demo_schema();
    let entity_id: EntityId = create_entity(&mut runtime, "alpha");
    let initial = latest_envelope(&runtime);

    let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    store.append_canonical_commit(initial.clone()).unwrap();
    store
        .acknowledge_cursor(crate::DurableCursorAcknowledgeRequest::new(
            "cursor-main",
            "subscriber-a",
            initial.branch_context.clone(),
            "demo-feed",
            "schema:v1",
            1,
            initial.commit.commit_id,
        ))
        .unwrap();
    update_entity_on_branch(&mut runtime, entity_id, "beta", None);
    let latest = latest_envelope(&runtime);
    store.append_canonical_commit(latest.clone()).unwrap();

    let degraded_basis = store
        .read_stable_basis(stable_basis_request_for_store(
            &store,
            latest.branch_context.clone(),
            initial.commit.commit_id,
            "schema-support:v1",
            ContinuationRetentionStatus::Degraded {
                fallback_class: "authority_replay".to_string(),
            },
        ))
        .unwrap();
    let plan = store
        .plan_cursor_continuation(CursorContinuationRequest::new(
            "cursor-main",
            "subscriber-a",
            latest.branch_context.clone(),
            "demo-feed",
            "schema:v1",
            1,
            degraded_basis,
            demo_budget(),
        ))
        .unwrap();

    let result = store.execute_cursor_continuation(plan).unwrap();
    let crate::ContinuationBatchResult::Broadened(receipt) = result.clone() else {
        panic!("expected broadened continuation result for degraded basis");
    };
    assert_eq!(receipt.fallback_class(), "authority_replay");
    assert_eq!(
        result.resolved_strategy(),
        ContinuationStrategy::ExplicitBroadened
    );
    assert_eq!(result.fallback_class(), Some("authority_replay"));
    assert_eq!(result.complexity_status(), LiveQueryComplexityStatus::Debt);
    assert_eq!(result.broadened_item_count(), 1);
    assert_eq!(result.support_rows_read(), 1);

    let counters = store.counters();
    assert_eq!(counters.continuation_degraded_basis_count, 1);
    assert_eq!(counters.continuation_broadening_count, 2);
    assert_eq!(counters.continuation_broadened_item_count, 1);
    assert_eq!(counters.continuation_support_rows_read, 1);
}

#[test]
fn admitted_continuation_executes_and_acknowledges_monotonically() {
    let mut runtime = runtime_with_demo_schema();
    let entity_id: EntityId = create_entity(&mut runtime, "alpha");

    let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    let initial = append_latest_commit(&mut store, &runtime);
    store
        .acknowledge_cursor(crate::DurableCursorAcknowledgeRequest::new(
            "cursor-main",
            "subscriber-a",
            initial.branch_context.clone(),
            "demo-feed",
            "schema:v1",
            1,
            initial.commit.commit_id,
        ))
        .unwrap();

    update_entity_on_branch(&mut runtime, entity_id, "beta", None);
    let second = append_latest_commit(&mut store, &runtime);
    update_entity_on_branch(&mut runtime, entity_id, "gamma", None);
    let third = append_latest_commit(&mut store, &runtime);

    let basis = store
        .read_stable_basis(stable_basis_request_for_store(
            &store,
            third.branch_context.clone(),
            initial.commit.commit_id,
            "schema-support:v1",
            ContinuationRetentionStatus::Retained,
        ))
        .unwrap();
    let plan = store
        .plan_cursor_continuation(CursorContinuationRequest::new(
            "cursor-main",
            "subscriber-a",
            third.branch_context.clone(),
            "demo-feed",
            "schema:v1",
            1,
            basis,
            ContinuationBatchBudget::new(
                FetchWidth::new(1),
                MaxBatchItems::new(4),
                MaxCoveredCommits::new(4),
                MaxMaterializedBytes::new(4_096),
                MaxSupportRowsPerBatch::new(24),
            ),
        ))
        .unwrap();

    let batch = store.execute_cursor_continuation(plan).unwrap();
    let ContinuationBatchResult::AdmittedNarrow(receipt) = batch.clone() else {
        panic!("expected admitted narrow continuation receipt");
    };
    assert_eq!(
        receipt.covered_commit_range(),
        (second.commit.commit_id, second.commit.commit_id)
    );
    assert_eq!(receipt.from_frontier_commit_id(), initial.commit.commit_id);
    assert_eq!(receipt.to_frontier_commit_id(), second.commit.commit_id);
    assert_eq!(
        batch.resolved_strategy(),
        ContinuationStrategy::AdmittedLayoutNarrow
    );
    assert_eq!(batch.fallback_class(), None);
    assert_eq!(
        batch.complexity_status(),
        LiveQueryComplexityStatus::Verified
    );
    assert_eq!(batch.covered_commit_count(), 1);
    assert_eq!(batch.narrowed_item_count(), 1);

    store
        .acknowledge_cursor_continuation(receipt.into_advance_receipt())
        .unwrap();

    let resumed = store
        .plan_cursor_resume(crate::DurableCursorResumeRequest::new(
            "cursor-main",
            "subscriber-a",
            third.branch_context.clone(),
            "demo-feed",
            "schema:v1",
            1,
        ))
        .unwrap();
    assert_eq!(
        resumed.latest_checkpoint().basis_commit_id,
        second.commit.commit_id
    );
    let counters = store.counters();
    assert_eq!(counters.continuation_cursor_identity_lookup_count, 1);
    assert_eq!(counters.continuation_checkpoint_lookup_count, 1);
    assert_eq!(counters.continuation_batch_count, 1);
    assert_eq!(counters.continuation_narrowed_item_count, 1);
    assert_eq!(counters.continuation_support_rows_read, 1);
    assert_eq!(counters.continuation_step_count, 1);
}

#[test]
fn continuation_batch_partitioning_is_deterministic_across_fetch_widths() {
    let mut runtime = runtime_with_demo_schema();
    let entity_id: EntityId = create_entity(&mut runtime, "alpha");

    let mut narrow_store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    let initial = append_latest_commit(&mut narrow_store, &runtime);
    narrow_store
        .acknowledge_cursor(crate::DurableCursorAcknowledgeRequest::new(
            "cursor-main",
            "subscriber-a",
            initial.branch_context.clone(),
            "demo-feed",
            "schema:v1",
            1,
            initial.commit.commit_id,
        ))
        .unwrap();

    update_entity_on_branch(&mut runtime, entity_id, "beta", None);
    append_latest_commit(&mut narrow_store, &runtime);
    update_entity_on_branch(&mut runtime, entity_id, "gamma", None);
    let latest = append_latest_commit(&mut narrow_store, &runtime);

    let export = narrow_store.export_authoritative_records();
    let mut wide_store =
        crate::ForgeStore::restore_from_authoritative_export(export.admit_restore()).unwrap();

    for (store, width) in [(&mut narrow_store, 1_u32), (&mut wide_store, 2_u32)] {
        let basis = store
            .read_stable_basis(stable_basis_request_for_store(
                store,
                latest.branch_context.clone(),
                initial.commit.commit_id,
                "schema-support:v1",
                ContinuationRetentionStatus::Retained,
            ))
            .unwrap();

        loop {
            let plan = store
                .plan_cursor_continuation(CursorContinuationRequest::new(
                    "cursor-main",
                    "subscriber-a",
                    latest.branch_context.clone(),
                    "demo-feed",
                    "schema:v1",
                    1,
                    basis.clone(),
                    ContinuationBatchBudget::new(
                        FetchWidth::new(width),
                        MaxBatchItems::new(8),
                        MaxCoveredCommits::new(8),
                        MaxMaterializedBytes::new(4_096),
                        MaxSupportRowsPerBatch::new(24),
                    ),
                ))
                .unwrap();
            match store.execute_cursor_continuation(plan).unwrap() {
                ContinuationBatchResult::AdmittedNarrow(receipt) => {
                    store
                        .acknowledge_cursor_continuation(receipt.into_advance_receipt())
                        .unwrap();
                }
                ContinuationBatchResult::CaughtUp(_) => break,
                other => panic!("unexpected continuation result: {other:?}"),
            }
        }
    }

    let narrow_resume = narrow_store
        .plan_cursor_resume(crate::DurableCursorResumeRequest::new(
            "cursor-main",
            "subscriber-a",
            latest.branch_context.clone(),
            "demo-feed",
            "schema:v1",
            1,
        ))
        .unwrap();
    let wide_resume = wide_store
        .plan_cursor_resume(crate::DurableCursorResumeRequest::new(
            "cursor-main",
            "subscriber-a",
            latest.branch_context.clone(),
            "demo-feed",
            "schema:v1",
            1,
        ))
        .unwrap();

    assert_eq!(
        narrow_resume.latest_checkpoint().basis_commit_id,
        latest.commit.commit_id
    );
    assert_eq!(
        wide_resume.latest_checkpoint().basis_commit_id,
        latest.commit.commit_id
    );
}
