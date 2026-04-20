use super::*;

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
