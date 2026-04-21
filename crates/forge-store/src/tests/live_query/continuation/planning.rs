use super::*;

#[test]
fn continuation_strategy_variants_are_preserved_by_phase_one_planning() {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let envelope = latest_envelope(&runtime);

    let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    store.append_canonical_commit(envelope.clone()).unwrap();
    store
        .acknowledge_cursor(crate::DurableCursorAcknowledgeRequest::new(
            "cursor-main",
            "subscriber-a",
            envelope.branch_context.clone(),
            "demo-feed",
            "schema:v1",
            1,
            envelope.commit.commit_id,
        ))
        .unwrap();

    let retained_basis = planned_basis_handle(
        &store,
        envelope.branch_context.clone(),
        envelope.commit.commit_id,
        ContinuationRetentionStatus::Retained,
    );
    let retained = store
        .plan_cursor_continuation(CursorContinuationRequest::new(
            "cursor-main",
            "subscriber-a",
            envelope.branch_context.clone(),
            "demo-feed",
            "schema:v1",
            1,
            retained_basis,
            demo_budget(),
        ))
        .unwrap();
    assert_eq!(
        retained.strategy(),
        ContinuationStrategy::AdmittedLayoutNarrow
    );

    let degraded_basis = planned_basis_handle(
        &store,
        envelope.branch_context.clone(),
        envelope.commit.commit_id,
        ContinuationRetentionStatus::Degraded {
            fallback_class: "authority_replay".to_string(),
        },
    );
    let degraded = store
        .plan_cursor_continuation(CursorContinuationRequest::new(
            "cursor-main",
            "subscriber-a",
            envelope.branch_context.clone(),
            "demo-feed",
            "schema:v1",
            1,
            degraded_basis,
            demo_budget(),
        ))
        .unwrap();
    assert_eq!(degraded.strategy(), ContinuationStrategy::ExplicitBroadened);
    let counters = store.counters();
    assert_eq!(counters.continuation_degraded_basis_count, 1);
    assert_eq!(counters.continuation_broadening_count, 1);
}

#[test]
fn continuation_planning_degrades_after_restart_visible_support_loss() {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let envelope = latest_envelope(&runtime);

    let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    store.append_canonical_commit(envelope.clone()).unwrap();
    store
        .acknowledge_cursor(crate::DurableCursorAcknowledgeRequest::new(
            "cursor-main",
            "subscriber-a",
            envelope.branch_context.clone(),
            "demo-feed",
            "schema:v1",
            1,
            envelope.commit.commit_id,
        ))
        .unwrap();

    let published = store
        .read_stable_basis(stable_basis_request_for_store(
            &store,
            envelope.branch_context.clone(),
            envelope.commit.commit_id,
            "schema-support:required",
            ContinuationRetentionStatus::Retained,
        ))
        .unwrap();
    let fetched = store
        .fetch_stable_basis(published.stable_basis_id())
        .unwrap();

    let plan = store
        .plan_cursor_continuation(CursorContinuationRequest::new(
            "cursor-main",
            "subscriber-a",
            envelope.branch_context.clone(),
            "demo-feed",
            "schema:v1",
            1,
            fetched,
            demo_budget(),
        ))
        .unwrap();

    assert_eq!(plan.strategy(), ContinuationStrategy::ExplicitBroadened);
    let counters = store.counters();
    assert_eq!(counters.continuation_degraded_basis_count, 1);
}

#[test]
fn retained_uniform_scope_continuation_fails_at_planning_typed_and_counts_mismatch() {
    let mut runtime = runtime_with_demo_schema();
    let entity_id: EntityId = create_entity(&mut runtime, "alpha");
    let basis_envelope = latest_envelope(&runtime);

    let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    store
        .append_canonical_commit(basis_envelope.clone())
        .unwrap();
    store
        .acknowledge_cursor(crate::DurableCursorAcknowledgeRequest::new(
            "cursor-main",
            "subscriber-a",
            basis_envelope.branch_context.clone(),
            "demo-feed",
            "schema:v1",
            1,
            basis_envelope.commit.commit_id,
        ))
        .unwrap();
    update_entity_on_branch(&mut runtime, entity_id, "beta", None);
    let latest_envelope = latest_envelope(&runtime);
    store
        .append_canonical_commit(latest_envelope.clone())
        .unwrap();
    let basis = store
        .read_stable_basis(uniform_scope_basis_request(
            &store,
            basis_envelope.branch_context.clone(),
            basis_envelope.commit.commit_id,
            ContinuationRetentionStatus::Retained,
        ))
        .unwrap();
    let error = store
        .plan_cursor_continuation(CursorContinuationRequest::new(
            "cursor-main",
            "subscriber-a",
            latest_envelope.branch_context.clone(),
            "demo-feed",
            "schema:v1",
            1,
            basis,
            demo_budget(),
        ))
        .expect_err("unsupported retained scopes must fail typed at planning");

    assert_eq!(
        error.kind(),
        &crate::StoreErrorKind::ContinuationScopeIncompatibility
    );
    let counters = store.counters();
    assert_eq!(counters.continuation_scope_mismatch_count, 1);
}

#[test]
fn phase_one_live_query_facade_surfaces_do_not_mutate_authoritative_records() {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let envelope = latest_envelope(&runtime);

    let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    store.append_canonical_commit(envelope.clone()).unwrap();
    store
        .acknowledge_cursor(crate::DurableCursorAcknowledgeRequest::new(
            "cursor-main",
            "subscriber-a",
            envelope.branch_context.clone(),
            "demo-feed",
            "schema:v1",
            1,
            envelope.commit.commit_id,
        ))
        .unwrap();

    let before = store.export_authoritative_records();
    let basis = planned_basis_handle(
        &store,
        envelope.branch_context.clone(),
        envelope.commit.commit_id,
        ContinuationRetentionStatus::Retained,
    );
    let plan = store
        .plan_cursor_continuation(CursorContinuationRequest::new(
            "cursor-main",
            "subscriber-a",
            envelope.branch_context.clone(),
            "demo-feed",
            "schema:v1",
            1,
            basis.clone(),
            demo_budget(),
        ))
        .unwrap();
    let batch_id = ContinuationBatchId::from_parts(
        plan.stable_basis(),
        "cursor-main",
        "subscriber-a",
        (envelope.commit.commit_id, envelope.commit.commit_id),
        basis.read_scope(),
        1,
    );
    let receipt = AdmittedNarrowBatchReceipt::new(
        batch_id,
        basis.stable_basis_id().clone(),
        "cursor-main",
        "subscriber-a",
        envelope.branch_context.clone(),
        "demo-feed",
        "schema:v1",
        1,
        basis.schema_boundary_artifact_id().to_string(),
        (envelope.commit.commit_id, envelope.commit.commit_id),
        vec![envelope.commit.commit_id],
        envelope.commit.commit_id,
        envelope.commit.commit_id,
        basis.read_scope().clone(),
        1,
        1,
        1,
        1,
        1,
    );
    store
        .admit_continuation_advance(receipt.into_advance_receipt())
        .unwrap();
    let after = store.export_authoritative_records();

    assert_eq!(before, after);

    let counters = store.counters();
    assert_eq!(counters.stable_basis_lookup_count, 1);
    assert_eq!(counters.stable_basis_read_count, 1);
    assert_eq!(counters.continuation_plan_count, 1);
    assert_eq!(counters.continuation_batch_count, 0);
    assert_eq!(counters.continuation_parity_count, 1);
}

#[path = "planning/budget.rs"]
mod budget;
