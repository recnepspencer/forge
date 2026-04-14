use crate::{ForgeStoreBuilder, SnapshotCaptureRequest, SnapshotReadRequest, StoreErrorKind};

use super::support::{
    create_entity, latest_envelope, runtime_with_demo_schema, unique_test_sqlite_path,
    update_entity_on_branch,
};

fn append_three_mainline_commits(
    store: &mut crate::ForgeStore,
) -> (
    forge_relational::facade::history::CommitId,
    forge_relational::facade::history::CommitId,
    forge_relational::facade::history::CommitId,
) {
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

    update_entity_on_branch(&mut runtime, entity_id, "gamma", None);
    let third = latest_envelope(&runtime);
    let third_id = third.commit.commit_id;
    store
        .append_canonical_commit(third)
        .expect("third commit should append");

    (first_id, second_id, third_id)
}

#[test]
fn snapshot_plus_tail_restore_matches_direct_point_in_time_read() {
    let mut store = ForgeStoreBuilder::new()
        .in_memory()
        .build()
        .expect("store should build");
    let (_, second_id, third_id) = append_three_mainline_commits(&mut store);

    let snapshot = store
        .capture_snapshot(SnapshotCaptureRequest::new(
            forge_relational::facade::history::BranchId("main".to_string()),
            second_id,
        ))
        .expect("snapshot should capture");

    let pure = store
        .read_snapshot(SnapshotReadRequest::pure_snapshot(
            snapshot.snapshot_id,
            second_id,
        ))
        .expect("pure snapshot read should succeed");
    let tailed = store
        .read_snapshot(SnapshotReadRequest::snapshot_plus_tail(
            snapshot.snapshot_id,
            third_id,
        ))
        .expect("snapshot-plus-tail read should succeed");
    let restored = store
        .restore_snapshot(snapshot.snapshot_id, third_id)
        .expect("snapshot restore should succeed");

    assert_eq!(
        tailed.image.canonical_json(),
        restored.restored_image.canonical_json()
    );
    assert_ne!(pure.image.canonical_json(), tailed.image.canonical_json());
    assert_eq!(
        store.counters().snapshot_restore_tail_commit_count,
        1,
        "restore should count exactly one tail commit beyond the snapshot frontier"
    );
}

#[test]
fn snapshot_rebuild_uses_basis_when_image_is_missing() {
    let mut store = ForgeStoreBuilder::new()
        .in_memory()
        .build()
        .expect("store should build");
    let (_, second_id, _) = append_three_mainline_commits(&mut store);

    let snapshot = store
        .capture_snapshot(SnapshotCaptureRequest::new(
            forge_relational::facade::history::BranchId("main".to_string()),
            second_id,
        ))
        .expect("snapshot should capture");
    let expected = store
        .read_snapshot(SnapshotReadRequest::pure_snapshot(
            snapshot.snapshot_id,
            second_id,
        ))
        .expect("pure snapshot read should succeed")
        .image;

    store
        .remove_snapshot_image_for_test(snapshot.snapshot_id)
        .expect("test should remove snapshot image");

    let read_failure = store
        .read_snapshot(SnapshotReadRequest::pure_snapshot(
            snapshot.snapshot_id,
            second_id,
        ))
        .expect_err("pure snapshot read should fail without image");
    assert_eq!(
        read_failure.kind(),
        &StoreErrorKind::SnapshotPublicationStateGap
    );

    let rebuilt = store
        .rebuild_snapshot(snapshot.snapshot_id)
        .expect("rebuild should succeed from basis");
    assert_eq!(rebuilt.canonical_json(), expected.canonical_json());
    assert_eq!(store.counters().snapshot_rebuild_count, 1);
}

#[test]
fn snapshot_restore_rejects_pre_snapshot_target() {
    let mut store = ForgeStoreBuilder::new()
        .in_memory()
        .build()
        .expect("store should build");
    let (first_id, second_id, _) = append_three_mainline_commits(&mut store);

    let snapshot = store
        .capture_snapshot(SnapshotCaptureRequest::new(
            forge_relational::facade::history::BranchId("main".to_string()),
            second_id,
        ))
        .expect("snapshot should capture");

    let error = store
        .restore_snapshot(snapshot.snapshot_id, first_id)
        .expect_err("restore should reject pre-snapshot target");
    assert_eq!(error.kind(), &StoreErrorKind::SnapshotRestoreTargetIllegal);
    assert_eq!(store.counters().snapshot_basis_mismatch_count, 1);
}

#[test]
fn sqlite_snapshot_reopen_preserves_pure_read() {
    let path = unique_test_sqlite_path("forge-store-m4-snapshot-reopen");
    let (snapshot_id, second_id) = {
        let mut store = ForgeStoreBuilder::new()
            .sqlite_file(path.clone())
            .build()
            .expect("sqlite store should build");
        let (_, second_id, _) = append_three_mainline_commits(&mut store);
        let snapshot_id = store
            .capture_snapshot(SnapshotCaptureRequest::new(
                forge_relational::facade::history::BranchId("main".to_string()),
                second_id,
            ))
            .expect("snapshot should capture")
            .snapshot_id;
        (snapshot_id, second_id)
    };

    let reopened = ForgeStoreBuilder::new()
        .sqlite_file(path)
        .build()
        .expect("reopened sqlite store should build");
    let read = reopened
        .read_snapshot(SnapshotReadRequest::pure_snapshot(snapshot_id, second_id))
        .expect("reopened snapshot should read");
    assert_eq!(read.snapshot_id, snapshot_id);
}
