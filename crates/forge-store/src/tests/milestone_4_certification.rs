use crate::{ForgeStoreBuilder, SnapshotCaptureRequest, SnapshotReadRequest, StoreErrorKind};

use super::harness::{
    certification::{
        assertions::{assert_all_equal, assert_rejection_payloads_present},
        core::{AssertionClass, CanonicalRow, CertificationSuite, LaneResult, RejectionRow},
        requirements::{evaluate_completeness, SNAPSHOT_PLUS_TAIL_RESTORE_EQUIVALENCE_TEST},
    },
    corruption::snapshot::{
        corrupt_first_sqlite_snapshot_basis_version, corrupt_first_sqlite_snapshot_image,
        delete_first_sqlite_snapshot_image,
    },
    fixtures::{
        runtime::{
            create_entity, latest_envelope, runtime_with_demo_schema, update_entity_on_branch,
        },
        stores::unique_test_sqlite_path,
    },
    scenarios::snapshots::snapshot_restore_equivalence_run,
};

fn milestone_4_suite() -> CertificationSuite<String, String> {
    let in_memory = {
        let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
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
                forge_relational::facade::history::BranchId("main".to_string()),
                second_id,
            ))
            .unwrap();
        let truth = store
            .read_snapshot(SnapshotReadRequest::pure_snapshot(
                snapshot.snapshot_id,
                second_id,
            ))
            .unwrap()
            .image;
        let restored = store
            .restore_snapshot(snapshot.snapshot_id, second_id)
            .unwrap()
            .restored_image;
        let rebuilt = store.rebuild_snapshot(snapshot.snapshot_id).unwrap();
        (
            truth.canonical_json(),
            restored.canonical_json(),
            rebuilt.canonical_json(),
        )
    };
    let in_memory_bundle =
        snapshot_restore_equivalence_run(super::harness::fixtures::stores::StoreLane::InMemory);
    let sqlite =
        snapshot_restore_equivalence_run(super::harness::fixtures::stores::StoreLane::Sqlite);

    let path = unique_test_sqlite_path("forge-store-m4-snapshot-corruption");
    {
        let mut store = ForgeStoreBuilder::new()
            .sqlite_file(path.clone())
            .build()
            .unwrap();
        let mut runtime = runtime_with_demo_schema();
        create_entity(&mut runtime, "alpha");
        let first = latest_envelope(&runtime);
        let first_id = first.commit.commit_id;
        store.append_canonical_commit(first).unwrap();
        store
            .capture_snapshot(SnapshotCaptureRequest::new(
                forge_relational::facade::history::BranchId("main".to_string()),
                first_id,
            ))
            .unwrap();
    }
    corrupt_first_sqlite_snapshot_image(&path);
    let failure = ForgeStoreBuilder::new()
        .sqlite_file(path)
        .build()
        .unwrap_err();

    CertificationSuite::new(SNAPSHOT_PLUS_TAIL_RESTORE_EQUIVALENCE_TEST.suite_name)
        .with_canonical_row(CanonicalRow::new(
            "restore_rebuild_equivalence",
            vec![
                LaneResult::new("in_memory_truth", in_memory.0.clone()),
                LaneResult::new("in_memory_restore", in_memory.1.clone()),
                LaneResult::new("in_memory_rebuild", in_memory.2.clone()),
            ],
            &[AssertionClass::Equality, AssertionClass::ExactCounter],
        ))
        .with_canonical_row(CanonicalRow::new(
            "backend_variation_delete_rebuild",
            vec![
                LaneResult::new("in_memory_bundle", in_memory_bundle.bundle_json),
                LaneResult::new("sqlite_bundle", sqlite.bundle_json),
            ],
            &[AssertionClass::Equality],
        ))
        .with_rejection_row(RejectionRow::new(
            "typed_snapshot_failure",
            vec![LaneResult::new(
                "corrupted_sqlite_snapshot",
                format!("{:?}", failure.kind()),
            )],
            &[AssertionClass::TypedFailure],
        ))
}

#[test]
fn milestone_4_certification_bundle_proves_restore_and_rebuild_equivalence() {
    let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    let mut runtime = runtime_with_demo_schema();
    let entity_id = create_entity(&mut runtime, "alpha");
    let first = latest_envelope(&runtime);
    let first_id = first.commit.commit_id;
    store.append_canonical_commit(first).unwrap();
    update_entity_on_branch(&mut runtime, entity_id, "beta", None);
    let second = latest_envelope(&runtime);
    let second_id = second.commit.commit_id;
    store.append_canonical_commit(second).unwrap();

    let snapshot = store
        .capture_snapshot(SnapshotCaptureRequest::new(
            forge_relational::facade::history::BranchId("main".to_string()),
            second_id,
        ))
        .unwrap();

    let truth = store
        .read_snapshot(SnapshotReadRequest::pure_snapshot(
            snapshot.snapshot_id,
            second_id,
        ))
        .unwrap()
        .image;
    let restored = store
        .restore_snapshot(snapshot.snapshot_id, second_id)
        .unwrap()
        .restored_image;
    let rebuilt = store.rebuild_snapshot(snapshot.snapshot_id).unwrap();

    let bundle = store.milestone_4_certification_bundle(&truth, &restored, &rebuilt);
    assert_eq!(bundle.truth_digest, bundle.restore_digest);
    assert_eq!(bundle.truth_digest, bundle.rebuild_digest);
    assert!(!bundle.artifact_digest.is_empty());
    assert!(bundle.canonical_json().contains(&bundle.truth_digest));
    assert_eq!(bundle.counter_snapshot.snapshot_capture_count, 1);
    assert!(bundle.counter_snapshot.snapshot_capture_byte_count > 0);
    assert_eq!(bundle.counter_snapshot.snapshot_read_count, 1);
    assert_eq!(bundle.counter_snapshot.snapshot_read_tail_commit_count, 0);
    assert_eq!(bundle.counter_snapshot.snapshot_read_tail_replay_count, 0);
    assert_eq!(bundle.counter_snapshot.snapshot_restore_count, 1);
    assert_eq!(bundle.counter_snapshot.snapshot_rebuild_count, 1);
    assert_eq!(first_id.0, 1);

    let suite = milestone_4_suite();
    assert_all_equal(&suite.canonical_rows()[0]);
}

#[test]
fn milestone_4_certification_bundle_matches_across_backend_variation_and_delete_rebuild_lane() {
    let suite = milestone_4_suite();
    assert_all_equal(&suite.canonical_rows()[1]);
    let completeness = evaluate_completeness(&suite, &SNAPSHOT_PLUS_TAIL_RESTORE_EQUIVALENCE_TEST);
    assert!(completeness.missing_rows().is_empty());
    assert!(completeness.missing_assertion_classes().is_empty());
}

#[test]
fn sqlite_snapshot_corruption_fails_typed_on_reopen() {
    let path = unique_test_sqlite_path("forge-store-m4-snapshot-corruption");
    {
        let mut store = ForgeStoreBuilder::new()
            .sqlite_file(path.clone())
            .build()
            .unwrap();
        let mut runtime = runtime_with_demo_schema();
        create_entity(&mut runtime, "alpha");
        let first = latest_envelope(&runtime);
        let first_id = first.commit.commit_id;
        store.append_canonical_commit(first).unwrap();
        store
            .capture_snapshot(SnapshotCaptureRequest::new(
                forge_relational::facade::history::BranchId("main".to_string()),
                first_id,
            ))
            .unwrap();
    }

    corrupt_first_sqlite_snapshot_image(&path);
    let error = ForgeStoreBuilder::new()
        .sqlite_file(path)
        .build()
        .expect_err("corrupted snapshot image should fail on reopen");
    assert!(matches!(
        error.kind(),
        StoreErrorKind::SnapshotIntegrityFailure
            | StoreErrorKind::SnapshotDigestMismatch
            | StoreErrorKind::BackendIntegrityViolation
    ));

    let suite = milestone_4_suite();
    assert_rejection_payloads_present(&suite.rejection_rows()[0]);
}

#[test]
fn sqlite_missing_snapshot_image_fails_with_publication_gap_on_reopen() {
    let path = unique_test_sqlite_path("forge-store-m4-snapshot-gap");
    {
        let mut store = ForgeStoreBuilder::new()
            .sqlite_file(path.clone())
            .build()
            .unwrap();
        let mut runtime = runtime_with_demo_schema();
        create_entity(&mut runtime, "alpha");
        let first = latest_envelope(&runtime);
        let first_id = first.commit.commit_id;
        store.append_canonical_commit(first).unwrap();
        store
            .capture_snapshot(SnapshotCaptureRequest::new(
                forge_relational::facade::history::BranchId("main".to_string()),
                first_id,
            ))
            .unwrap();
    }

    delete_first_sqlite_snapshot_image(&path);
    let error = ForgeStoreBuilder::new()
        .sqlite_file(path)
        .build()
        .expect_err("missing snapshot image should fail on reopen");
    assert_eq!(error.kind(), &StoreErrorKind::SnapshotPublicationStateGap);
}

#[test]
fn sqlite_snapshot_version_mismatch_fails_typed_on_reopen() {
    let path = unique_test_sqlite_path("forge-store-m4-snapshot-version");
    {
        let mut store = ForgeStoreBuilder::new()
            .sqlite_file(path.clone())
            .build()
            .unwrap();
        let mut runtime = runtime_with_demo_schema();
        create_entity(&mut runtime, "alpha");
        let first = latest_envelope(&runtime);
        let first_id = first.commit.commit_id;
        store.append_canonical_commit(first).unwrap();
        store
            .capture_snapshot(SnapshotCaptureRequest::new(
                forge_relational::facade::history::BranchId("main".to_string()),
                first_id,
            ))
            .unwrap();
    }

    corrupt_first_sqlite_snapshot_basis_version(&path);
    let error = ForgeStoreBuilder::new()
        .sqlite_file(path)
        .build()
        .expect_err("version-mismatched snapshot should fail on reopen");
    assert_eq!(
        error.kind(),
        &StoreErrorKind::SnapshotFamilyVersionUnsupported
    );
}
