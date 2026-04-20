use super::*;

#[test]
fn durable_cursor_resume_survives_sqlite_reopen() {
    let path = unique_test_sqlite_path("forge-store-cursor-resume");
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let envelope = latest_envelope(&runtime);
    let branch_id = envelope.branch_context.clone();
    let commit_id = envelope.commit.commit_id;

    let mut store = ForgeStoreBuilder::new()
        .sqlite_file(path.clone())
        .build()
        .unwrap();
    store.append_canonical_commit(envelope).unwrap();
    let persisted = store
        .acknowledge_cursor(DurableCursorAcknowledgeRequest::new(
            "cursor-main",
            "subscriber-a",
            branch_id.clone(),
            "demo-feed",
            "schema:v1",
            1,
            commit_id,
        ))
        .unwrap();
    assert_eq!(persisted.record().checkpoint_sequence, 1);
    drop(store);

    let reopened = ForgeStoreBuilder::new().sqlite_file(path).build().unwrap();
    let plan = reopened
        .plan_cursor_resume(DurableCursorResumeRequest::new(
            "cursor-main",
            "subscriber-a",
            branch_id,
            "demo-feed",
            "schema:v1",
            1,
        ))
        .unwrap();

    assert_eq!(plan.identity().cursor_id, "cursor-main");
    assert_eq!(plan.latest_checkpoint().basis_commit_id, commit_id);
    assert_eq!(plan.latest_checkpoint().checkpoint_sequence, 1);

    let counters = reopened.counters();
    assert_eq!(counters.cursor_resume_count, 1);
    assert_eq!(counters.cursor_identity_lookup_count, 1);
    assert_eq!(counters.cursor_resume_support_rows_read, 2);
}

#[test]
fn cursor_resume_and_acknowledgment_support_explicit_witness_vocabulary() {
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

    let admitted = store
        .admit_cursor_resume(DurableCursorResumeRequest::new(
            "cursor-main",
            "subscriber-a",
            envelope.branch_context.clone(),
            "demo-feed",
            "schema:v1",
            1,
        ))
        .unwrap();
    assert_eq!(admitted.identity().cursor_id, "cursor-main");

    let witness = store
        .admit_resumed_cursor_advance(
            &admitted,
            DurableCursorAcknowledgeRequest::new(
                "cursor-main",
                "subscriber-a",
                envelope.branch_context.clone(),
                "demo-feed",
                "schema:v1",
                1,
                envelope.commit.commit_id,
            ),
        )
        .unwrap();
    let persisted = store
        .acknowledge_resumed_cursor_progress(&admitted, witness)
        .unwrap();
    assert_eq!(persisted.record().checkpoint_sequence, 2);
}

#[test]
fn durable_cursor_regression_is_rejected() {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let first = latest_envelope(&runtime);
    create_entity(&mut runtime, "beta");
    let second = latest_envelope(&runtime);

    let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    store.append_canonical_commit(first.clone()).unwrap();
    store.append_canonical_commit(second.clone()).unwrap();

    store
        .acknowledge_cursor(DurableCursorAcknowledgeRequest::new(
            "cursor-main",
            "subscriber-a",
            second.branch_context.clone(),
            "demo-feed",
            "schema:v1",
            1,
            second.commit.commit_id,
        ))
        .unwrap();

    let error = store
        .acknowledge_cursor(DurableCursorAcknowledgeRequest::new(
            "cursor-main",
            "subscriber-a",
            first.branch_context.clone(),
            "demo-feed",
            "schema:v1",
            1,
            first.commit.commit_id,
        ))
        .expect_err("earlier frontier must be rejected as cursor regression");

    assert_eq!(error.kind(), &StoreErrorKind::CursorRegression);
    assert_eq!(store.counters().cursor_regression_reject_count, 1);
}

#[test]
fn durable_cursor_equivalence_basis_is_not_mutable() {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let envelope = latest_envelope(&runtime);

    let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    store.append_canonical_commit(envelope.clone()).unwrap();
    store
        .acknowledge_cursor(DurableCursorAcknowledgeRequest::new(
            "cursor-main",
            "subscriber-a",
            envelope.branch_context.clone(),
            "demo-feed",
            "schema:v1",
            1,
            envelope.commit.commit_id,
        ))
        .unwrap();

    let error = store
        .acknowledge_cursor(DurableCursorAcknowledgeRequest::new(
            "cursor-main",
            "subscriber-a",
            envelope.branch_context.clone(),
            "different-feed",
            "schema:v1",
            1,
            envelope.commit.commit_id,
        ))
        .expect_err("changing feed shape must mint a new cursor identity");

    assert_eq!(error.kind(), &StoreErrorKind::CursorEquivalenceViolation);
    assert_eq!(store.counters().cursor_equivalence_reject_count, 1);
}
