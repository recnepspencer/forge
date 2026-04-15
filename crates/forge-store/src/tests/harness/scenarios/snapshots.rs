use forge_relational::facade::history::CommitId;

use crate::{ForgeStore, SnapshotCaptureRequest, SnapshotReadRequest, SnapshotRestoreRequest};

use super::super::fixtures::{
    artifacts::append_three_mainline_commits,
    stores::{build_store_for_lane, StoreLane},
};

pub fn append_three_mainline_commits_for_store(
    store: &mut ForgeStore,
) -> (CommitId, CommitId, CommitId) {
    append_three_mainline_commits(store)
}

pub struct SnapshotParityRun {
    pub bundle_json: String,
}

pub fn snapshot_restore_equivalence_run(lane: StoreLane) -> SnapshotParityRun {
    let mut store = build_store_for_lane(lane, "forge-store-harness-snapshot");
    let (_, second_id, third_id) = append_three_mainline_commits(&mut store);
    let snapshot = store
        .capture_snapshot(SnapshotCaptureRequest::new(
            forge_relational::facade::history::BranchId("main".to_string()),
            second_id,
        ))
        .unwrap();

    let truth = store
        .read_snapshot(SnapshotReadRequest::snapshot_plus_tail(
            snapshot.snapshot_id,
            third_id,
        ))
        .unwrap()
        .image;
    let plan = store
        .plan_snapshot_restore(SnapshotRestoreRequest::new(snapshot.snapshot_id, third_id))
        .unwrap();
    let restored = store.execute_snapshot_restore(plan).unwrap().restored_image;
    let rebuilt = store.rebuild_snapshot(snapshot.snapshot_id).unwrap();
    let bundle = store.milestone_4_certification_bundle(&truth, &restored, &rebuilt);

    SnapshotParityRun {
        bundle_json: bundle.canonical_json(),
    }
}
