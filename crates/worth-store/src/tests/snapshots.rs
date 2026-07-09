use crate::{WORTHStoreBuilder, SnapshotCaptureRequest, SnapshotReadRequest, StoreErrorKind};

use super::harness::{
    fixtures::stores::unique_test_sqlite_path,
    scenarios::snapshots::append_three_mainline_commits_for_store,
};

#[test]
fn snapshot_plus_tail_restore_matches_direct_point_in_time_read() {
    let mut store = WORTHStoreBuilder::new()
        .in_memory()
        .build()
        .expect("store should build");
    let (_, second_id, third_id) = append_three_mainline_commits_for_store(&mut store);

    let snapshot = store
        .capture_snapshot(SnapshotCaptureRequest::new(
            worth_relational::facade::history::BranchId("main".to_string()),
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
    let plan = store
        .plan_snapshot_restore(crate::SnapshotRestoreRequest::new(
            snapshot.snapshot_id,
            third_id,
        ))
        .expect("snapshot restore plan should succeed");
    let restored = store
        .execute_snapshot_restore(plan.clone())
        .expect("snapshot restore should succeed");

    assert_eq!(
        tailed.image.canonical_json(),
        restored.restored_image.canonical_json()
    );
    assert_ne!(pure.image.canonical_json(), tailed.image.canonical_json());
    assert_eq!(plan.tail_commit_ids().len(), 1);
    assert_eq!(store.counters().snapshot_read_count, 2);
    assert_eq!(
        store.counters().snapshot_read_tail_commit_count,
        1,
        "snapshot-tail reads must expose exactly the replay suffix width they consumed"
    );
    assert_eq!(store.counters().snapshot_read_tail_replay_count, 1);
    assert_eq!(
        store.counters().snapshot_restore_tail_commit_count,
        1,
        "restore should count exactly one tail commit beyond the snapshot frontier"
    );
    assert_eq!(store.counters().snapshot_restore_tail_replay_count, 1);
}

#[test]
fn snapshot_rebuild_uses_basis_when_image_is_missing() {
    let mut store = WORTHStoreBuilder::new()
        .in_memory()
        .build()
        .expect("store should build");
    let (_, second_id, _) = append_three_mainline_commits_for_store(&mut store);

    let snapshot = store
        .capture_snapshot(SnapshotCaptureRequest::new(
            worth_relational::facade::history::BranchId("main".to_string()),
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
    let mut store = WORTHStoreBuilder::new()
        .in_memory()
        .build()
        .expect("store should build");
    let (first_id, second_id, _) = append_three_mainline_commits_for_store(&mut store);

    let snapshot = store
        .capture_snapshot(SnapshotCaptureRequest::new(
            worth_relational::facade::history::BranchId("main".to_string()),
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
    let path = unique_test_sqlite_path("worth-store-m4-snapshot-reopen");
    let (snapshot_id, second_id) = {
        let mut store = WORTHStoreBuilder::new()
            .sqlite_file(path.clone())
            .build()
            .expect("sqlite store should build");
        let (_, second_id, _) = append_three_mainline_commits_for_store(&mut store);
        let snapshot_id = store
            .capture_snapshot(SnapshotCaptureRequest::new(
                worth_relational::facade::history::BranchId("main".to_string()),
                second_id,
            ))
            .expect("snapshot should capture")
            .snapshot_id;
        (snapshot_id, second_id)
    };

    let reopened = WORTHStoreBuilder::new()
        .sqlite_file(path)
        .build()
        .expect("reopened sqlite store should build");
    let read = reopened
        .read_snapshot(SnapshotReadRequest::pure_snapshot(snapshot_id, second_id))
        .expect("reopened snapshot should read");
    assert_eq!(read.snapshot_id, snapshot_id);
}
