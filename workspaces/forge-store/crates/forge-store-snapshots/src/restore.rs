use crate::SnapshotId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SnapshotRestorePlan {
    snapshot_id: SnapshotId,
    restore_frontier_pages: u32,
}

impl SnapshotRestorePlan {
    pub const fn new(snapshot_id: SnapshotId, restore_frontier_pages: u32) -> Self {
        Self {
            snapshot_id,
            restore_frontier_pages,
        }
    }
}
