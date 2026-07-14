use super::*;

#[test]
fn duplicate_continuation_acknowledgment_fails_typed() {
    let mut runtime = runtime_with_demo_schema();
    let entity_id: EntityId = create_entity(&mut runtime, "alpha");

    let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
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
    let plan = store
        .plan_cursor_continuation(CursorContinuationRequest::new(
            "cursor-main",
            "subscriber-a",
            latest.branch_context.clone(),
            "demo-feed",
            "schema:v1",
            1,
            basis,
            demo_budget(),
        ))
        .unwrap();
    let receipt = match store.execute_cursor_continuation(plan).unwrap() {
        ContinuationBatchResult::AdmittedNarrow(receipt) => receipt,
        other => panic!("unexpected continuation result: {other:?}"),
    };
    let duplicate = receipt.clone().into_advance_receipt();

    store
        .acknowledge_cursor_continuation(receipt.into_advance_receipt())
        .unwrap();
    let error = store
        .acknowledge_cursor_continuation(duplicate)
        .expect_err("duplicate acknowledgment must fail typed");

    assert_eq!(
        error.kind(),
        &crate::StoreErrorKind::ContinuationBatchDuplicate
    );
    let counters = store.counters();
    assert_eq!(counters.continuation_batch_duplicate_count, 1);
    assert_eq!(counters.continuation_illegal_acknowledgment_count, 1);
}

#[test]
fn gap_continuation_acknowledgment_fails_typed() {
    let mut runtime = runtime_with_demo_schema();
    let entity_id: EntityId = create_entity(&mut runtime, "alpha");

    let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
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

    let basis = planned_basis_handle(
        &store,
        third.branch_context.clone(),
        initial.commit.commit_id,
        ContinuationRetentionStatus::Retained,
    );
    let WORTHd = AdmittedNarrowBatchReceipt::new(
        ContinuationBatchId::from_parts(
            &basis,
            "cursor-main",
            "subscriber-a",
            (second.commit.commit_id, third.commit.commit_id),
            basis.read_scope(),
            1,
        ),
        basis.stable_basis_id().clone(),
        "cursor-main",
        "subscriber-a",
        third.branch_context.clone(),
        "demo-feed",
        "schema:v1",
        1,
        basis.schema_boundary_artifact_id().to_string(),
        (second.commit.commit_id, third.commit.commit_id),
        vec![second.commit.commit_id, third.commit.commit_id],
        second.commit.commit_id,
        third.commit.commit_id,
        basis.read_scope().clone(),
        1,
        2,
        2,
        2,
        1,
    );

    let error = store
        .acknowledge_cursor_continuation(WORTHd.into_advance_receipt())
        .expect_err("skipping the immediately next frontier must fail typed");

    assert_eq!(error.kind(), &crate::StoreErrorKind::ContinuationBatchGap);
    let counters = store.counters();
    assert_eq!(counters.continuation_batch_gap_count, 1);
    assert_eq!(counters.continuation_illegal_acknowledgment_count, 1);
}
