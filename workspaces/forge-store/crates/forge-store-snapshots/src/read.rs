use crate::SnapshotId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SnapshotReadRequest {
    snapshot_id: SnapshotId,
    requested_page_count: u32,
}

impl SnapshotReadRequest {
    pub const fn new(snapshot_id: SnapshotId, requested_page_count: u32) -> Self {
        Self {
            snapshot_id,
            requested_page_count,
        }
    }
    pub const fn snapshot_id(&self) -> &SnapshotId {
        &self.snapshot_id
    }
    pub const fn requested_page_count(&self) -> u32 {
        self.requested_page_count
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SnapshotReadResult {
    snapshot_id: SnapshotId,
    returned_page_count: u32,
}

impl SnapshotReadResult {
    pub const fn new(snapshot_id: SnapshotId, returned_page_count: u32) -> Self {
        Self {
            snapshot_id,
            returned_page_count,
        }
    }
}
