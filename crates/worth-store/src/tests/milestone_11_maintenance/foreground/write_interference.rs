use super::super::*;
use super::branch_head::latest_branch_head;

fn create_alpha_commit(
    runtime: &mut worth_relational::facade::runtime::RelationalRuntime,
) -> Result<worth_relational::facade::history::CommitId, crate::StoreError> {
    Ok(super::super::super::harness::fixtures::runtime::create_entity_commit(runtime, "alpha"))
}

#[test]
fn authoritative_commit_append_reports_reservation_violation_on_escalated_same_locality_maintenance(
) {
    let path = unique_test_store_path("worth-store-m11-foreground-write-append");
    let mut runtime = runtime_with_demo_schema();
    let entity_id = create_entity(&mut runtime, "alpha");
    let initial = latest_envelope(&runtime);

    let mut store = WORTHStoreBuilder::new()
        .local_file(path.clone())
        .build()
        .unwrap();
    store.append_canonical_commit(initial).unwrap();
    let head = update_entity_on_branch_with_commit(&mut runtime, entity_id, "main-v2");
    store.append_canonical_commit(head.clone()).unwrap();
    store
        .materialize_milestone_6_layout_support(layout_request(
            head.branch_context.clone(),
            head.commit.commit_id,
        ))
        .unwrap();
    let snapshot = store
        .capture_snapshot(SnapshotCaptureRequest::new(
            head.branch_context.clone(),
            head.commit.commit_id,
        ))
        .unwrap();
    let batch = store
        .plan_retention_maintenance_batch(RetentionPolicyClass::Conservative(
            ConservativeRetentionPolicy::new(
                Vec::new(),
                vec![PinnedSnapshotPolicy::new(snapshot.snapshot_id)],
                vec![DerivedFamilyRetentionPolicy::Milestone6LayoutMaterialization],
            ),
        ))
        .unwrap();
    let receipt = store.admit_maintenance_batch(batch).unwrap();
    let declaration_id = receipt.admitted_declarations()[0]
        .declaration()
        .id()
        .clone();
    let update = update_entity_on_branch_with_commit(&mut runtime, entity_id, "foreground-write");
    drop(store);

    force_local_file_reserved(
        &path,
        &declaration_id,
        crate::MaintenancePlanFamily::Escalated,
        1,
    );

    let mut reopened = WORTHStoreBuilder::new().local_file(path).build().unwrap();
    let persisted = reopened.append_canonical_commit(update).unwrap();
    assert_eq!(
        persisted.foreground_isolation().posture(),
        crate::ForegroundInterferencePosture::ReservationViolation
    );
    assert_eq!(
        persisted.foreground_isolation().violation(),
        Some(crate::ForegroundIsolationViolation::SharedReservationConflict)
    );
}

#[test]
fn cursor_acknowledgment_reports_reservation_violation_on_escalated_same_locality_maintenance() {
    let path = unique_test_store_path("worth-store-m11-foreground-write-cursor");
    let (mut store, batch) = build_maintenance_ready_store_with_builder(
        WORTHStoreBuilder::new().local_file(path.clone()),
    );
    let receipt = store.admit_maintenance_batch(batch).unwrap();
    let declaration_id = receipt.admitted_declarations()[0]
        .declaration()
        .id()
        .clone();
    let (branch_id, commit_id) = latest_branch_head(&store);
    drop(store);

    force_local_file_reserved(
        &path,
        &declaration_id,
        crate::MaintenancePlanFamily::Escalated,
        1,
    );

    let mut reopened = WORTHStoreBuilder::new().local_file(path).build().unwrap();
    let checkpoint = reopened
        .acknowledge_cursor(crate::DurableCursorAcknowledgeRequest::new(
            "cursor-main",
            "subscriber-a",
            branch_id,
            "demo-feed",
            "schema:v1",
            1,
            commit_id,
        ))
        .unwrap();
    assert_eq!(
        checkpoint.foreground_isolation().posture(),
        crate::ForegroundInterferencePosture::ReservationViolation
    );
    assert_eq!(
        checkpoint.foreground_isolation().violation(),
        Some(crate::ForegroundIsolationViolation::SharedReservationConflict)
    );
}

#[test]
fn durable_publication_report_summarizes_foreground_write_interference() {
    let path = unique_test_store_path("worth-store-m11-publication-write-report");
    let mut durable = WORTHStoreBuilder::new()
        .local_file(path.clone())
        .durable_mode(runtime_with_demo_schema())
        .build()
        .unwrap();
    let acknowledged = durable
        .execute_mutation(crate::DurableMutationRequest::new(
            "create-alpha",
            create_alpha_commit,
        ))
        .unwrap();
    let durable_mutation_id = acknowledged.durable_mutation_id();
    let published = acknowledged.persisted().envelope().clone();
    let branch_id = published.branch_context.clone();
    let commit_id = published.commit.commit_id;
    let (mut store, _runtime) = durable.shutdown();

    store
        .materialize_milestone_6_layout_support(layout_request(branch_id.clone(), commit_id))
        .unwrap();
    let snapshot = store
        .capture_snapshot(SnapshotCaptureRequest::new(branch_id, commit_id))
        .unwrap();
    let batch = store
        .plan_retention_maintenance_batch(RetentionPolicyClass::Conservative(
            ConservativeRetentionPolicy::new(
                Vec::new(),
                vec![PinnedSnapshotPolicy::new(snapshot.snapshot_id)],
                vec![DerivedFamilyRetentionPolicy::Milestone6LayoutMaterialization],
            ),
        ))
        .unwrap();
    let receipt = store.admit_maintenance_batch(batch).unwrap();
    let declaration_id = receipt.admitted_declarations()[0]
        .declaration()
        .id()
        .clone();
    drop(store);

    force_local_file_reserved(
        &path,
        &declaration_id,
        crate::MaintenancePlanFamily::Escalated,
        1,
    );

    let reopened = WORTHStoreBuilder::new().local_file(path).build().unwrap();
    let report = reopened
        .durable_publication_report(durable_mutation_id, Some(commit_id))
        .unwrap();
    let isolation = report
        .foreground_write_isolation()
        .expect("durable publication report should carry same-branch write isolation");
    assert_eq!(
        isolation.posture(),
        crate::ForegroundInterferencePosture::ReservationViolation
    );
    assert_eq!(
        isolation.violation(),
        Some(crate::ForegroundIsolationViolation::SharedReservationConflict)
    );
}
