use super::helpers::*;
use crate::{
    AdmittedNarrowBatchReceipt, ContinuationBatchBudget, ContinuationBatchId,
    ContinuationBatchResult, ContinuationRetentionStatus, ContinuationStrategy,
    CursorContinuationRequest, FetchWidth, ForgeStoreBuilder, MaxBatchItems,
    MaxCoveredCommits, MaxMaterializedBytes, MaxSupportRowsPerBatch, StableBasisReadScope,
};
use forge_relational::facade::identity::EntityId;

#[test]
fn live_query_basis_continuation_equivalence_across_fetch_widths_is_certified() {
    let mut runtime = runtime_with_demo_schema();
    let entity_id: EntityId = create_entity(&mut runtime, "alpha");

    let mut primary_store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    let initial = append_latest_commit(&mut primary_store, &runtime);
    primary_store
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
    append_latest_commit(&mut primary_store, &runtime);
    update_entity_on_branch(&mut runtime, entity_id, "gamma", None);
    let latest = append_latest_commit(&mut primary_store, &runtime);

    let export = primary_store.export_authoritative_records();
    let mut control_store =
        crate::ForgeStore::restore_from_authoritative_export(export.admit_restore()).unwrap();
    control_store
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

    let primary_basis = primary_store
        .read_stable_basis(stable_basis_request_for_store(
            &primary_store,
            latest.branch_context.clone(),
            initial.commit.commit_id,
            "schema-support:v1",
            ContinuationRetentionStatus::Retained,
        ))
        .unwrap();
    let control_basis = control_store
        .read_stable_basis(stable_basis_request_for_store(
            &control_store,
            latest.branch_context.clone(),
            initial.commit.commit_id,
            "schema-support:v1",
            ContinuationRetentionStatus::Retained,
        ))
        .unwrap();

    let (primary_results, primary_frontier) = run_admitted_continuation_session(
        &mut primary_store,
        latest.branch_context.clone(),
        latest.commit.commit_id,
        primary_basis.clone(),
        1,
    );
    let (control_results, control_frontier) = run_admitted_continuation_session(
        &mut control_store,
        latest.branch_context.clone(),
        latest.commit.commit_id,
        control_basis.clone(),
        2,
    );

    let control_export = control_store.export_authoritative_records();
    let bundle = primary_store
        .milestone_8_certification_bundle(crate::Milestone8CertificationRequest::new(
            &control_export,
            &primary_basis,
            ContinuationStrategy::AdmittedLayoutNarrow,
            &primary_results,
            primary_frontier,
            ContinuationStrategy::AdmittedLayoutNarrow,
            &control_results,
            control_frontier,
            &[],
        ))
        .unwrap();

    assert!(bundle.certification_summary.truth_matches_control_lane);
    assert!(bundle.certification_summary.restore_truth_parity);
    assert!(
        bundle
            .certification_summary
            .control_lane_matches_authoritative_truth
    );
    assert!(bundle.certification_summary.admitted_lane_stayed_narrow);
    assert!(bundle.certification_summary.no_hidden_control_lane_fallback);
    assert_eq!(
        bundle.continuation.resolved_strategy,
        ContinuationStrategy::AdmittedLayoutNarrow
    );
    assert_eq!(
        bundle.continuation.covered_commit_ids,
        bundle.control_continuation.covered_commit_ids
    );
    assert_eq!(bundle.continuation.broadened_item_count, 0);
    assert_eq!(bundle.control_continuation.broadened_item_count, 0);
    assert_eq!(
        primary_store
            .counters()
            .continuation_control_lane_fallback_count,
        0
    );
    assert_eq!(primary_store.counters().continuation_broadened_item_count, 0);
    assert_eq!(control_store.counters().continuation_broadened_item_count, 0);
    assert!(!bundle.canonical_json().is_empty());
}

#[test]
fn live_query_basis_continuation_equivalence_through_restart_is_certified() {
    let path = unique_test_sqlite_path("forge-store-live-query-equivalence-restart");
    let mut runtime = runtime_with_demo_schema();
    let entity_id: EntityId = create_entity(&mut runtime, "alpha");

    let initial = latest_envelope(&runtime);
    let mut restarted_store = ForgeStoreBuilder::new()
        .sqlite_file(path.clone())
        .build()
        .unwrap();
    restarted_store
        .append_canonical_commit(initial.clone())
        .unwrap();
    restarted_store
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
    restarted_store
        .append_canonical_commit(second.clone())
        .unwrap();
    let basis = restarted_store
        .read_stable_basis(stable_basis_request_for_store(
            &restarted_store,
            second.branch_context.clone(),
            initial.commit.commit_id,
            "schema-support:v1",
            ContinuationRetentionStatus::Retained,
        ))
        .unwrap();
    let stable_basis_id = basis.stable_basis_id().clone();
    let (before_restart_results, _) = run_admitted_continuation_session(
        &mut restarted_store,
        second.branch_context.clone(),
        second.commit.commit_id,
        basis.clone(),
        1,
    );
    drop(restarted_store);

    update_entity_on_branch(&mut runtime, entity_id, "gamma", None);
    let third = latest_envelope(&runtime);
    let mut reopened_store = ForgeStoreBuilder::new().sqlite_file(path).build().unwrap();
    reopened_store.append_canonical_commit(third.clone()).unwrap();
    let reopened_basis = reopened_store.fetch_stable_basis(&stable_basis_id).unwrap();
    let plan = reopened_store
        .plan_cursor_continuation(CursorContinuationRequest::new(
            "cursor-main",
            "subscriber-a",
            third.branch_context.clone(),
            "demo-feed",
            "schema:v1",
            1,
            reopened_basis.clone(),
            ContinuationBatchBudget::new(
                FetchWidth::new(1),
                MaxBatchItems::new(8),
                MaxCoveredCommits::new(8),
                MaxMaterializedBytes::new(4_096),
                MaxSupportRowsPerBatch::new(24),
            ),
        ))
        .unwrap();
    let after_restart_result = reopened_store.execute_cursor_continuation(plan).unwrap();
    let restarted_frontier = after_restart_result
        .to_frontier_commit_id()
        .expect("degraded restart continuation must still expose a covered frontier");
    let after_restart_results = vec![after_restart_result];

    let mut control_store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    control_store.append_canonical_commit(initial.clone()).unwrap();
    control_store
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
    control_store.append_canonical_commit(second.clone()).unwrap();
    control_store.append_canonical_commit(third.clone()).unwrap();
    let control_basis = control_store
        .read_stable_basis(stable_basis_request_for_store(
            &control_store,
            third.branch_context.clone(),
            initial.commit.commit_id,
            "schema-support:v1",
            ContinuationRetentionStatus::Retained,
        ))
        .unwrap();
    let (control_results, control_frontier) = run_admitted_continuation_session(
        &mut control_store,
        third.branch_context.clone(),
        third.commit.commit_id,
        control_basis.clone(),
        2,
    );

    let mut resumed_results = before_restart_results.clone();
    resumed_results.extend(after_restart_results.iter().cloned());

    let control_export = control_store.export_authoritative_records();
    let bundle = reopened_store
        .milestone_8_certification_bundle(crate::Milestone8CertificationRequest::new(
            &control_export,
            &reopened_basis,
            ContinuationStrategy::ExplicitBroadened,
            &resumed_results,
            restarted_frontier,
            ContinuationStrategy::AdmittedLayoutNarrow,
            &control_results,
            control_frontier,
            &[],
        ))
        .unwrap();

    assert!(bundle.certification_summary.truth_matches_control_lane);
    assert!(bundle.certification_summary.restore_truth_parity);
    assert!(
        bundle
            .certification_summary
            .control_lane_matches_authoritative_truth
    );
    assert_eq!(
        bundle.basis.retention_status,
        ContinuationRetentionStatus::Degraded {
            fallback_class: "authority_replay".to_string()
        }
    );
    assert_eq!(
        bundle.continuation.resolved_strategy,
        ContinuationStrategy::ExplicitBroadened
    );
    assert_eq!(
        bundle.continuation.covered_commit_ids,
        bundle.control_continuation.covered_commit_ids
    );
    assert_eq!(bundle.continuation.batch_count, 2);
    assert_eq!(
        bundle.continuation.final_frontier_commit_id,
        third.commit.commit_id
    );
    assert_eq!(
        bundle.control_continuation.final_frontier_commit_id,
        third.commit.commit_id
    );
    assert_eq!(
        reopened_store
            .counters()
            .continuation_control_lane_fallback_count,
        0
    );
    assert_eq!(reopened_store.counters().continuation_broadened_item_count, 1);
}

#[test]
fn milestone_8_certification_rejects_control_lane_surface_as_authority_when_it_mismatches() {
    let mut runtime = runtime_with_demo_schema();
    let entity_id: EntityId = create_entity(&mut runtime, "alpha");

    let mut primary_store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    let initial = append_latest_commit(&mut primary_store, &runtime);
    primary_store
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
    append_latest_commit(&mut primary_store, &runtime);
    update_entity_on_branch(&mut runtime, entity_id, "gamma", None);
    let latest = append_latest_commit(&mut primary_store, &runtime);

    let export = primary_store.export_authoritative_records();
    let mut control_store =
        crate::ForgeStore::restore_from_authoritative_export(export.admit_restore()).unwrap();
    control_store
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

    let primary_basis = primary_store
        .read_stable_basis(stable_basis_request_for_store(
            &primary_store,
            latest.branch_context.clone(),
            initial.commit.commit_id,
            "schema-support:v1",
            ContinuationRetentionStatus::Retained,
        ))
        .unwrap();
    let control_basis = control_store
        .read_stable_basis(stable_basis_request_for_store(
            &control_store,
            latest.branch_context.clone(),
            initial.commit.commit_id,
            "schema-support:v1",
            ContinuationRetentionStatus::Retained,
        ))
        .unwrap();

    let (mut primary_results, _primary_frontier) = run_admitted_continuation_session(
        &mut primary_store,
        latest.branch_context.clone(),
        latest.commit.commit_id,
        primary_basis.clone(),
        1,
    );
    let (control_results, control_frontier) = run_admitted_continuation_session(
        &mut control_store,
        latest.branch_context.clone(),
        latest.commit.commit_id,
        control_basis,
        1,
    );
    primary_results.truncate(1);
    let mismatched_frontier = primary_results
        .last()
        .and_then(ContinuationBatchResult::to_frontier_commit_id)
        .expect("truncated hostile continuation evidence must still expose a frontier");

    let control_export = control_store.export_authoritative_records();
    let bundle = primary_store
        .milestone_8_certification_bundle(crate::Milestone8CertificationRequest::new(
            &control_export,
            &primary_basis,
            ContinuationStrategy::AdmittedLayoutNarrow,
            &primary_results,
            mismatched_frontier,
            ContinuationStrategy::AdmittedLayoutNarrow,
            &control_results,
            control_frontier,
            &[],
        ))
        .unwrap();

    assert_ne!(
        bundle.continuation.covered_commit_ids,
        bundle.control_continuation.covered_commit_ids
    );
    assert!(
        !bundle
            .certification_summary
            .control_lane_matches_authoritative_truth
    );
    assert!(bundle.certification_summary.truth_matches_control_lane);
}

#[test]
fn milestone_8_certification_records_failure_markers_as_non_certified() {
    let mut runtime = runtime_with_demo_schema();
    let entity_id: EntityId = create_entity(&mut runtime, "alpha");

    let mut primary_store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    let initial = append_latest_commit(&mut primary_store, &runtime);
    primary_store
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
    let latest = append_latest_commit(&mut primary_store, &runtime);

    let export = primary_store.export_authoritative_records();
    let mut control_store =
        crate::ForgeStore::restore_from_authoritative_export(export.admit_restore()).unwrap();
    control_store
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

    let primary_basis = primary_store
        .read_stable_basis(stable_basis_request_for_store(
            &primary_store,
            latest.branch_context.clone(),
            initial.commit.commit_id,
            "schema-support:v1",
            ContinuationRetentionStatus::Retained,
        ))
        .unwrap();
    let control_basis = control_store
        .read_stable_basis(stable_basis_request_for_store(
            &control_store,
            latest.branch_context.clone(),
            initial.commit.commit_id,
            "schema-support:v1",
            ContinuationRetentionStatus::Retained,
        ))
        .unwrap();

    let (primary_results, primary_frontier) = run_admitted_continuation_session(
        &mut primary_store,
        latest.branch_context.clone(),
        latest.commit.commit_id,
        primary_basis.clone(),
        1,
    );
    let (control_results, control_frontier) = run_admitted_continuation_session(
        &mut control_store,
        latest.branch_context.clone(),
        latest.commit.commit_id,
        control_basis,
        1,
    );
    let failure_markers = vec!["synthetic-gap-detected".to_string()];

    let control_export = control_store.export_authoritative_records();
    let bundle = primary_store
        .milestone_8_certification_bundle(crate::Milestone8CertificationRequest::new(
            &control_export,
            &primary_basis,
            ContinuationStrategy::AdmittedLayoutNarrow,
            &primary_results,
            primary_frontier,
            ContinuationStrategy::AdmittedLayoutNarrow,
            &control_results,
            control_frontier,
            &failure_markers,
        ))
        .unwrap();

    assert!(!bundle.failure_digest.is_empty());
    assert!(!bundle.certification_summary.no_failure_markers);
    assert!(
        bundle
            .certification_summary
            .control_lane_matches_authoritative_truth
    );
    assert!(bundle.certification_summary.truth_matches_control_lane);
}

#[test]
fn milestone_8_certification_rejects_duplicate_commit_surface() {
    let mut runtime = runtime_with_demo_schema();
    let entity_id: EntityId = create_entity(&mut runtime, "alpha");

    let mut primary_store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    let initial = append_latest_commit(&mut primary_store, &runtime);
    primary_store
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
    let latest = append_latest_commit(&mut primary_store, &runtime);

    let export = primary_store.export_authoritative_records();
    let mut control_store =
        crate::ForgeStore::restore_from_authoritative_export(export.admit_restore()).unwrap();
    control_store
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

    let primary_basis = primary_store
        .read_stable_basis(stable_basis_request_for_store(
            &primary_store,
            latest.branch_context.clone(),
            initial.commit.commit_id,
            "schema-support:v1",
            ContinuationRetentionStatus::Retained,
        ))
        .unwrap();
    let control_basis = control_store
        .read_stable_basis(stable_basis_request_for_store(
            &control_store,
            latest.branch_context.clone(),
            initial.commit.commit_id,
            "schema-support:v1",
            ContinuationRetentionStatus::Retained,
        ))
        .unwrap();

    let (control_results, control_frontier) = run_admitted_continuation_session(
        &mut control_store,
        latest.branch_context.clone(),
        latest.commit.commit_id,
        control_basis,
        1,
    );
    let forged_duplicate =
        ContinuationBatchResult::AdmittedNarrow(AdmittedNarrowBatchReceipt::new(
            ContinuationBatchId::from_parts(
                &primary_basis,
                "cursor-main",
                "subscriber-a",
                (latest.commit.commit_id, latest.commit.commit_id),
                primary_basis.read_scope(),
                1,
            ),
            primary_basis.stable_basis_id().clone(),
            "cursor-main",
            "subscriber-a",
            latest.branch_context.clone(),
            "demo-feed",
            "schema:v1",
            1,
            primary_basis.schema_boundary_artifact_id().to_string(),
            (latest.commit.commit_id, latest.commit.commit_id),
            vec![latest.commit.commit_id, latest.commit.commit_id],
            initial.commit.commit_id,
            latest.commit.commit_id,
            primary_basis.read_scope().clone(),
            1,
            2,
            2,
            2,
            1,
        ));

    let control_export = control_store.export_authoritative_records();
    let error = primary_store
        .milestone_8_certification_bundle(crate::Milestone8CertificationRequest::new(
            &control_export,
            &primary_basis,
            ContinuationStrategy::AdmittedLayoutNarrow,
            &[forged_duplicate],
            latest.commit.commit_id,
            ContinuationStrategy::AdmittedLayoutNarrow,
            &control_results,
            control_frontier,
            &[],
        ))
        .expect_err("duplicate continuation evidence must be rejected before certification");

    assert_eq!(error.kind(), &crate::StoreErrorKind::ContinuationBatchDuplicate);
}

#[test]
fn milestone_8_certification_rejects_mislabeled_strategy() {
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
            degraded_basis.clone(),
            demo_budget(),
        ))
        .unwrap();
    let broadened_result = store.execute_cursor_continuation(plan).unwrap();
    assert!(matches!(
        broadened_result,
        ContinuationBatchResult::Broadened(_)
    ));

    let control_export = store.export_authoritative_records();
    let error = store
        .milestone_8_certification_bundle(crate::Milestone8CertificationRequest::new(
            &control_export,
            &degraded_basis,
            ContinuationStrategy::AdmittedLayoutNarrow,
            std::slice::from_ref(&broadened_result),
            latest.commit.commit_id,
            ContinuationStrategy::ExplicitBroadened,
            std::slice::from_ref(&broadened_result),
            latest.commit.commit_id,
            &[],
        ))
        .expect_err("certification must reject a mislabeled continuation strategy");

    assert_eq!(
        error.kind(),
        &crate::StoreErrorKind::ContinuationCursorIncompatibility
    );
}

#[test]
fn milestone_8_certification_rejects_mislabeled_scope() {
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
    let primary_basis = store
        .read_stable_basis(stable_basis_request_for_store(
            &store,
            latest.branch_context.clone(),
            initial.commit.commit_id,
            "schema-support:v1",
            ContinuationRetentionStatus::Retained,
        ))
        .unwrap();
    let forged_scope_receipt =
        ContinuationBatchResult::AdmittedNarrow(AdmittedNarrowBatchReceipt::new(
            ContinuationBatchId::from_parts(
                &primary_basis,
                "cursor-main",
                "subscriber-a",
                (latest.commit.commit_id, latest.commit.commit_id),
                primary_basis.read_scope(),
                1,
            ),
            primary_basis.stable_basis_id().clone(),
            "cursor-main",
            "subscriber-a",
            latest.branch_context.clone(),
            "demo-feed",
            "schema:v1",
            1,
            primary_basis.schema_boundary_artifact_id().to_string(),
            (latest.commit.commit_id, latest.commit.commit_id),
            vec![latest.commit.commit_id],
            initial.commit.commit_id,
            latest.commit.commit_id,
            StableBasisReadScope::UniformEntitySet(crate::EntitySetUniformAspectScope::new(vec![
                "entity-alpha".to_string(),
                "entity-beta".to_string(),
            ])),
            1,
            1,
            1,
            1,
            1,
        ));

    let control_export = store.export_authoritative_records();
    let error = store
        .milestone_8_certification_bundle(crate::Milestone8CertificationRequest::new(
            &control_export,
            &primary_basis,
            ContinuationStrategy::AdmittedLayoutNarrow,
            &[forged_scope_receipt],
            latest.commit.commit_id,
            ContinuationStrategy::AdmittedLayoutNarrow,
            &[],
            initial.commit.commit_id,
            &[],
        ))
        .expect_err("certification must reject a mismatched receipt scope");

    assert_eq!(
        error.kind(),
        &crate::StoreErrorKind::ContinuationScopeIncompatibility
    );
}

#[test]
fn milestone_8_certification_rejects_non_monotonic_commit_surface() {
    let mut runtime = runtime_with_demo_schema();
    let entity_id: EntityId = create_entity(&mut runtime, "alpha");

    let mut primary_store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    let initial = append_latest_commit(&mut primary_store, &runtime);
    primary_store
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
    let second = append_latest_commit(&mut primary_store, &runtime);
    update_entity_on_branch(&mut runtime, entity_id, "gamma", None);
    let third = append_latest_commit(&mut primary_store, &runtime);

    let export = primary_store.export_authoritative_records();
    let mut control_store =
        crate::ForgeStore::restore_from_authoritative_export(export.admit_restore()).unwrap();
    control_store
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

    let primary_basis = primary_store
        .read_stable_basis(stable_basis_request_for_store(
            &primary_store,
            third.branch_context.clone(),
            initial.commit.commit_id,
            "schema-support:v1",
            ContinuationRetentionStatus::Retained,
        ))
        .unwrap();
    let control_basis = control_store
        .read_stable_basis(stable_basis_request_for_store(
            &control_store,
            third.branch_context.clone(),
            initial.commit.commit_id,
            "schema-support:v1",
            ContinuationRetentionStatus::Retained,
        ))
        .unwrap();

    let (control_results, control_frontier) = run_admitted_continuation_session(
        &mut control_store,
        third.branch_context.clone(),
        third.commit.commit_id,
        control_basis,
        2,
    );
    let forged_out_of_order =
        ContinuationBatchResult::AdmittedNarrow(AdmittedNarrowBatchReceipt::new(
            ContinuationBatchId::from_parts(
                &primary_basis,
                "cursor-main",
                "subscriber-a",
                (second.commit.commit_id, third.commit.commit_id),
                primary_basis.read_scope(),
                1,
            ),
            primary_basis.stable_basis_id().clone(),
            "cursor-main",
            "subscriber-a",
            third.branch_context.clone(),
            "demo-feed",
            "schema:v1",
            1,
            primary_basis.schema_boundary_artifact_id().to_string(),
            (second.commit.commit_id, third.commit.commit_id),
            vec![third.commit.commit_id, second.commit.commit_id],
            initial.commit.commit_id,
            second.commit.commit_id,
            primary_basis.read_scope().clone(),
            1,
            2,
            2,
            2,
            1,
        ));

    let control_export = control_store.export_authoritative_records();
    let error = primary_store
        .milestone_8_certification_bundle(crate::Milestone8CertificationRequest::new(
            &control_export,
            &primary_basis,
            ContinuationStrategy::AdmittedLayoutNarrow,
            &[forged_out_of_order],
            second.commit.commit_id,
            ContinuationStrategy::AdmittedLayoutNarrow,
            &control_results,
            control_frontier,
            &[],
        ))
        .expect_err("non-monotonic continuation evidence must be rejected before certification");

    assert_eq!(
        error.kind(),
        &crate::StoreErrorKind::ContinuationBatchOrderingViolation
    );
}
