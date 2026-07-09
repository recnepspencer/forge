use super::*;

#[test]
fn interrupted_snapshot_publication_requires_rebuild_not_trusted_truth() {
    let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
    let mut runtime = runtime_with_demo_schema();
    let entity_id = create_entity(&mut runtime, "alpha");
    let first = latest_envelope(&runtime);
    store.append_canonical_commit(first).unwrap();
    update_entity_on_branch(&mut runtime, entity_id, "beta", None);
    let second = latest_envelope(&runtime);
    let second_id = second.commit.commit_id;
    store.append_canonical_commit(second).unwrap();
    let snapshot = store
        .capture_snapshot(SnapshotCaptureRequest::new(
            worth_relational::facade::history::BranchId("main".to_string()),
            second_id,
        ))
        .unwrap();
    store
        .remove_snapshot_image_for_test(snapshot.snapshot_id)
        .unwrap();

    let report = store
        .snapshot_maintenance_recovery_report(snapshot.snapshot_id)
        .unwrap();
    assert_eq!(
        report.publication_classification(),
        PublicationClassification::RequireRebuild
    );
    assert_eq!(
        report.action(),
        crate::SnapshotMaintenanceRecoveryAction::RequireRebuild
    );
    let maintenance_report = store.maintenance_recovery_report().unwrap();
    let snapshot_entry = maintenance_report
        .entries()
        .iter()
        .find(|entry| entry.family() == MaintenanceArtifactFamily::Snapshot)
        .unwrap();
    assert_eq!(
        snapshot_entry.disposition(),
        MaintenanceRecoveryDisposition::RequireRebuild
    );
}

#[test]
fn recovery_status_report_elevates_snapshot_rebuild_requirement_to_operator_surface() {
    let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
    let mut runtime = runtime_with_demo_schema();
    let entity_id = create_entity(&mut runtime, "alpha");
    let first = latest_envelope(&runtime);
    store.append_canonical_commit(first).unwrap();
    update_entity_on_branch(&mut runtime, entity_id, "beta", None);
    let second = latest_envelope(&runtime);
    let second_id = second.commit.commit_id;
    store.append_canonical_commit(second).unwrap();
    let snapshot = store
        .capture_snapshot(SnapshotCaptureRequest::new(
            worth_relational::facade::history::BranchId("main".to_string()),
            second_id,
        ))
        .unwrap();
    store
        .remove_snapshot_image_for_test(snapshot.snapshot_id)
        .unwrap();

    let outcome = crate::DurableRecoveryOutcome {
        decisions: Vec::new(),
        degraded: Vec::new(),
        source_reports: Vec::new(),
    };
    let plan = crate::recovery::DurableRecoveryPlan {
        pending_durable_mutation_ids: Vec::new(),
    };
    let report = crate::RecoveryStatusReport::new(
        &plan,
        &outcome,
        store.maintenance_recovery_report().unwrap(),
        store.support_artifact_recovery_report(),
    );

    assert_eq!(
        report.operator_disposition(),
        RecoveryOperatorDisposition::RebuildRequired
    );
    assert_eq!(report.maintenance().entries().len(), 4);
    assert!(report.recommended_actions().iter().any(|action| {
        action.kind() == RecoveryOperatorActionKind::RebuildMaintenanceArtifact
            && action.scope_identity().contains("snapshot:")
    }));
}
