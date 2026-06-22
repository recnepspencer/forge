use crate::memory_workspace::ForgeQuerySnapshotIdentity;

use super::ForgeQueryJournalSegmentIdentity;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryJournalReplayRequest {
    segment_identity: ForgeQueryJournalSegmentIdentity,
    basis_snapshot_identity: Option<ForgeQuerySnapshotIdentity>,
}

impl ForgeQueryJournalReplayRequest {
    pub fn new(segment_identity: ForgeQueryJournalSegmentIdentity) -> Self {
        Self {
            segment_identity,
            basis_snapshot_identity: None,
        }
    }

    pub fn with_basis_snapshot(mut self, snapshot_identity: ForgeQuerySnapshotIdentity) -> Self {
        self.basis_snapshot_identity = Some(snapshot_identity);
        self
    }

    pub fn segment_identity(&self) -> &ForgeQueryJournalSegmentIdentity {
        &self.segment_identity
    }

    pub fn basis_snapshot_identity(&self) -> Option<&ForgeQuerySnapshotIdentity> {
        self.basis_snapshot_identity.as_ref()
    }
}
