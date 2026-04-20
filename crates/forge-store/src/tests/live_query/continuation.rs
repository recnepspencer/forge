use super::helpers::*;
use crate::{
    AdmittedNarrowBatchReceipt, ContinuationBatchBudget, ContinuationBatchId,
    ContinuationBatchResult, ContinuationRetentionStatus, ContinuationStrategy,
    CursorContinuationRequest, FetchWidth, ForgeStoreBuilder, LiveQueryComplexityStatus,
    MaxBatchItems, MaxCoveredCommits, MaxMaterializedBytes, MaxSupportRowsPerBatch,
};
use forge_relational::facade::identity::EntityId;

#[test]
fn continuation_reopens_through_sqlite_and_degrades_when_basis_support_is_not_retained() {
    let path = unique_test_sqlite_path("forge-store-live-query-continuation");
    let mut runtime = runtime_with_demo_schema();
    let entity_id: EntityId = create_entity(&mut runtime, "alpha");

    let initial = latest_envelope(&runtime);
    let mut store = ForgeStoreBuilder::new()
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
    let mut reopened = ForgeStoreBuilder::new().sqlite_file(path).build().unwrap();
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
    let fetched = store.fetch_stable_basis(published.stable_basis_id()).unwrap();

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
    store.append_canonical_commit(basis_envelope.clone()).unwrap();
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
    store.append_canonical_commit(latest_envelope.clone()).unwrap();
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
    assert_eq!(counters.continuation_batch_count, 1);
    assert_eq!(counters.continuation_parity_count, 1);
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
    assert_eq!(batch.complexity_status(), LiveQueryComplexityStatus::Verified);
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

#[test]
fn duplicate_continuation_acknowledgment_fails_typed() {
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

    assert_eq!(error.kind(), &crate::StoreErrorKind::ContinuationBatchDuplicate);
    let counters = store.counters();
    assert_eq!(counters.continuation_batch_duplicate_count, 1);
    assert_eq!(counters.continuation_illegal_acknowledgment_count, 1);
}

#[test]
fn gap_continuation_acknowledgment_fails_typed() {
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

    let basis = planned_basis_handle(
        &store,
        third.branch_context.clone(),
        initial.commit.commit_id,
        ContinuationRetentionStatus::Retained,
    );
    let forged = AdmittedNarrowBatchReceipt::new(
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
        .acknowledge_cursor_continuation(forged.into_advance_receipt())
        .expect_err("skipping the immediately next frontier must fail typed");

    assert_eq!(error.kind(), &crate::StoreErrorKind::ContinuationBatchGap);
    let counters = store.counters();
    assert_eq!(counters.continuation_batch_gap_count, 1);
    assert_eq!(counters.continuation_illegal_acknowledgment_count, 1);
}
