use crate::{ForgeStoreBuilder, SnapshotCaptureRequest, SnapshotReadRequest, StoreErrorKind};

use super::support::{
    corrupt_first_sqlite_snapshot_image, create_entity, latest_envelope, runtime_with_demo_schema,
    unique_test_sqlite_path, update_entity_on_branch,
};

#[test]
fn milestone_4_certification_bundle_proves_restore_and_rebuild_equivalence() {
    let mut store = ForgeStoreBuilder::new()
        .in_memory()
        .build()
        .expect("store should build");
    let mut runtime = runtime_with_demo_schema();
    let entity_id = create_entity(&mut runtime, "alpha");
    let first = latest_envelope(&runtime);
    let first_id = first.commit.commit_id;
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
            forge_relational::facade::history::BranchId("main".to_string()),
            second_id,
        ))
        .expect("snapshot should capture");

    let truth = store
        .read_snapshot(SnapshotReadRequest::pure_snapshot(
            snapshot.snapshot_id,
            second_id,
        ))
        .expect("truth snapshot should read")
        .image;
    let restored = store
        .restore_snapshot(snapshot.snapshot_id, second_id)
        .expect("restore should succeed")
        .restored_image;
    let rebuilt = store
        .rebuild_snapshot(snapshot.snapshot_id)
        .expect("rebuild should succeed");

    let bundle = store.milestone_4_certification_bundle(&truth, &restored, &rebuilt);
    assert_eq!(bundle.truth_digest, bundle.restore_digest);
    assert_eq!(bundle.truth_digest, bundle.rebuild_digest);
    assert!(bundle.canonical_json().contains(&bundle.truth_digest));
    assert_eq!(bundle.counter_snapshot.snapshot_capture_count, 1);
    assert_eq!(bundle.counter_snapshot.snapshot_read_count, 1);
    assert_eq!(bundle.counter_snapshot.snapshot_restore_count, 1);
    assert_eq!(bundle.counter_snapshot.snapshot_rebuild_count, 1);
    assert_eq!(
        first_id.0, 1,
        "demo runtime should produce first mainline commit"
    );
}

#[test]
fn sqlite_snapshot_corruption_fails_typed_on_reopen() {
    let path = unique_test_sqlite_path("forge-store-m4-snapshot-corruption");
    {
        let mut store = ForgeStoreBuilder::new()
            .sqlite_file(path.clone())
            .build()
            .expect("sqlite store should build");
        let mut runtime = runtime_with_demo_schema();
        create_entity(&mut runtime, "alpha");
        let first = latest_envelope(&runtime);
        let first_id = first.commit.commit_id;
        store
            .append_canonical_commit(first)
            .expect("first commit should append");
        store
            .capture_snapshot(SnapshotCaptureRequest::new(
                forge_relational::facade::history::BranchId("main".to_string()),
                first_id,
            ))
            .expect("snapshot should capture");
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
}
