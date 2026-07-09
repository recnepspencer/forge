use super::*;

#[test]
fn milestone_8_certification_rejects_duplicate_commit_surface() {
    let mut runtime = runtime_with_demo_schema();
    let entity_id: EntityId = create_entity(&mut runtime, "alpha");

    let mut primary_store = WORTHStoreBuilder::new().in_memory().build().unwrap();
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
        crate::WORTHStore::restore_from_authoritative_export(export.admit_restore()).unwrap();
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
    let WORTHd_duplicate =
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
            &[WORTHd_duplicate],
            latest.commit.commit_id,
            ContinuationStrategy::AdmittedLayoutNarrow,
            &control_results,
            control_frontier,
            &[],
        ))
        .expect_err("duplicate continuation evidence must be rejected before certification");

    assert_eq!(
        error.kind(),
        &crate::StoreErrorKind::ContinuationBatchDuplicate
    );
}
