use super::*;

#[test]
fn continuation_reopens_through_sqlite_and_degrades_when_basis_support_is_not_retained() {
    let path = unique_test_sqlite_path("worth-store-live-query-continuation");
    let mut runtime = runtime_with_demo_schema();
    let entity_id: EntityId = create_entity(&mut runtime, "alpha");

    let initial = latest_envelope(&runtime);
    let mut store = WORTHStoreBuilder::new()
        .sqlite_file(path.clone())
        .build()
        .unwrap();
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
    let second = latest_envelope(&runtime);
    store.append_canonical_commit(second.clone()).unwrap();

    let basis = store
        .read_stable_basis(stable_basis_request_for_store(
            &store,
            second.branch_context.clone(),
            initial.commit.commit_id,
            "schema-support:v1",
            ContinuationRetentionStatus::Retained,
        ))
        .unwrap();
    let stable_basis_id = basis.stable_basis_id().clone();
    let plan = store
        .plan_cursor_continuation(CursorContinuationRequest::new(
            "cursor-main",
            "subscriber-a",
            second.branch_context.clone(),
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
    let receipt = match store.execute_cursor_continuation(plan).unwrap() {
        ContinuationBatchResult::AdmittedNarrow(receipt) => receipt,
        other => panic!("unexpected continuation result before restart: {other:?}"),
    };
    store
        .acknowledge_cursor_continuation(receipt.into_advance_receipt())
        .unwrap();
    drop(store);

    update_entity_on_branch(&mut runtime, entity_id, "gamma", None);
    let third = latest_envelope(&runtime);
    let mut reopened = WORTHStoreBuilder::new().sqlite_file(path).build().unwrap();
    reopened.append_canonical_commit(third.clone()).unwrap();
    let fetched_basis = reopened.fetch_stable_basis(&stable_basis_id).unwrap();
    let plan = reopened
        .plan_cursor_continuation(CursorContinuationRequest::new(
            "cursor-main",
            "subscriber-a",
            third.branch_context.clone(),
            "demo-feed",
            "schema:v1",
            1,
            fetched_basis,
            ContinuationBatchBudget::new(
                FetchWidth::new(1),
                MaxBatchItems::new(4),
                MaxCoveredCommits::new(4),
                MaxMaterializedBytes::new(4_096),
                MaxSupportRowsPerBatch::new(24),
            ),
        ))
        .unwrap();
    let result = reopened.execute_cursor_continuation(plan).unwrap();
    let ContinuationBatchResult::Broadened(receipt) = result else {
        panic!("expected broadened continuation result after restart-visible support loss");
    };
    assert_eq!(
        receipt.covered_commit_range(),
        (third.commit.commit_id, third.commit.commit_id)
    );
    assert_eq!(receipt.fallback_class(), "authority_replay");

    let resumed = reopened
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
}
