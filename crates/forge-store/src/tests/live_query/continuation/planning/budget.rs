use super::*;

#[test]
fn continuation_planning_rejects_batches_that_cannot_fit_materialized_byte_budget() {
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
    let latest = append_latest_commit(&mut store, &runtime);
    let basis = store
        .read_stable_basis(stable_basis_request_for_store(
            &store,
            latest.branch_context.clone(),
            initial.commit.commit_id,
            "schema-support:v1",
            ContinuationRetentionStatus::Retained,
        ))
        .unwrap();

    let error = store
        .plan_cursor_continuation(CursorContinuationRequest::new(
            "cursor-main",
            "subscriber-a",
            latest.branch_context.clone(),
            "demo-feed",
            "schema:v1",
            1,
            basis,
            ContinuationBatchBudget::new(
                FetchWidth::new(1),
                MaxBatchItems::new(8),
                MaxCoveredCommits::new(8),
                MaxMaterializedBytes::new(1),
                MaxSupportRowsPerBatch::new(24),
            ),
        ))
        .expect_err("materialized byte budget smaller than a single commit must fail typed");

    assert_eq!(error.kind(), &crate::StoreErrorKind::ContinuationBudgetExceeded);
}

