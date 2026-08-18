//! Scoped ownership of one live snapshot handle.

use super::data::{SnapshotHandle, SnapshotId};

#[derive(Debug, Clone)]
pub struct SnapshotGuard {
    handle: SnapshotHandle,
}

impl SnapshotGuard {
    #[cfg(test)]
    pub(crate) fn new(handle: SnapshotHandle) -> Self {
        Self { handle }
    }

    pub fn handle(&self) -> &SnapshotHandle {
        &self.handle
    }

    pub fn snapshot_id(&self) -> SnapshotId {
        self.handle.snapshot_id
    }
}
