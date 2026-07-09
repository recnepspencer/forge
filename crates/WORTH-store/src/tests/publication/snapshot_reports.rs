use super::*;

#[test]
fn snapshot_publication_report_classifies_missing_image_and_missing_basis() {
    let mut store = WORTHStoreBuilder::new()
        .in_memory()
        .build()
        .expect("store should build");
    let mut runtime = runtime_with_demo_schema();
    let entity_id = create_entity(&mut runtime, "alpha");
    let first = latest_envelope(&runtime);
    store
        .append_canonical_commit(first)
        .expect("first commit should append");
    update_entity_on_branch(&mut runtime, entity_id, "beta", None);
    let second = latest_envelope(&runtime);
    let second_id = second.commit.commit_id;
    store
        .append_canonical_commit(second)
        .expect("second commit should append");

    let snapshot = store
        .capture_snapshot(SnapshotCaptureRequest::new(
            worth_relational::facade::history::BranchId("main".to_string()),
            second_id,
        ))
        .expect("snapshot should capture");

    let published = store
        .snapshot_publication_report(snapshot.snapshot_id)
        .expect("published snapshot report should build");
    assert_eq!(
        published.classification(),
        PublicationClassification::RetainTrusted
    );

    store
        .remove_snapshot_image_for_test(snapshot.snapshot_id)
        .expect("test should remove snapshot image");
    let missing_image = store
        .snapshot_publication_report(snapshot.snapshot_id)
        .expect("missing-image snapshot report should build");
    assert_eq!(
        missing_image.classification(),
        PublicationClassification::RequireRebuild
    );

    let mut store = WORTHStoreBuilder::new()
        .in_memory()
        .build()
        .expect("fresh store should build");
    let mut runtime = runtime_with_demo_schema();
    let entity_id = create_entity(&mut runtime, "alpha");
    let first = latest_envelope(&runtime);
    store
        .append_canonical_commit(first)
        .expect("fresh first commit should append");
    update_entity_on_branch(&mut runtime, entity_id, "beta", None);
    let second = latest_envelope(&runtime);
    let second_id = second.commit.commit_id;
    store
        .append_canonical_commit(second)
        .expect("fresh second commit should append");
    let second_snapshot = store
        .capture_snapshot(SnapshotCaptureRequest::new(
            worth_relational::facade::history::BranchId("main".to_string()),
            second_id,
        ))
        .expect("second snapshot should capture");
    store
        .remove_snapshot_basis_for_test(second_snapshot.snapshot_id)
        .expect("test should remove snapshot basis");
    let missing_basis = store
        .snapshot_publication_report(second_snapshot.snapshot_id)
        .expect("missing-basis snapshot report should build");
    assert_eq!(
        missing_basis.classification(),
        PublicationClassification::RequireQuarantine
    );
}

#[test]
fn snapshot_maintenance_recovery_classifies_invalid_relation_explicitly() {
    let path = crate::tests::harness::fixtures::stores::unique_test_sqlite_path(
        "worth-store-snapshot-maintenance-invalid-relation",
    );
    {
        let mut store = WORTHStoreBuilder::new()
            .sqlite_file(path.clone())
            .build()
            .expect("store should build");
        let mut runtime = runtime_with_demo_schema();
        let entity_id = create_entity(&mut runtime, "alpha");
        let first = latest_envelope(&runtime);
        store
            .append_canonical_commit(first)
            .expect("first commit should append");
        update_entity_on_branch(&mut runtime, entity_id, "beta", None);
        let second = latest_envelope(&runtime);
        let second_id = second.commit.commit_id;
        store
            .append_canonical_commit(second)
            .expect("second commit should append");
        store
            .capture_snapshot(SnapshotCaptureRequest::new(
                worth_relational::facade::history::BranchId("main".to_string()),
                second_id,
            ))
            .expect("snapshot should capture");
    }

    crate::tests::harness::corruption::snapshot::corrupt_first_sqlite_snapshot_image(&path);
    let error = WORTHStoreBuilder::new()
        .sqlite_file(path.clone())
        .build()
        .expect_err("corrupted sqlite snapshot should fail on reopen");
    assert!(matches!(
        error.kind(),
        crate::StoreErrorKind::SnapshotIntegrityFailure
            | crate::StoreErrorKind::SnapshotDigestMismatch
            | crate::StoreErrorKind::BackendIntegrityViolation
    ));

    let mut store = WORTHStoreBuilder::new()
        .in_memory()
        .build()
        .expect("in-memory store should build");
    let mut runtime = runtime_with_demo_schema();
    let entity_id = create_entity(&mut runtime, "alpha");
    let first = latest_envelope(&runtime);
    store
        .append_canonical_commit(first)
        .expect("first commit should append");
    update_entity_on_branch(&mut runtime, entity_id, "beta", None);
    let second = latest_envelope(&runtime);
    let second_id = second.commit.commit_id;
    store
        .append_canonical_commit(second)
        .expect("second commit should append");
    let snapshot = store
        .capture_snapshot(SnapshotCaptureRequest::new(
            worth_relational::facade::history::BranchId("main".to_string()),
            second_id,
        ))
        .expect("snapshot should capture");
    store
        .corrupt_snapshot_basis_digest_for_test(snapshot.snapshot_id)
        .expect("test should corrupt snapshot basis digest");

    let report = store
        .snapshot_maintenance_recovery_report(snapshot.snapshot_id)
        .expect("maintenance recovery report should build");
    assert_eq!(
        report.publication_classification(),
        PublicationClassification::RetainTrusted
    );
    assert!(!report.relation_valid());
    assert_eq!(
        report.action(),
        SnapshotMaintenanceRecoveryAction::RequireQuarantine
    );
}
