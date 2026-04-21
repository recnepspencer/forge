use super::*;

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
