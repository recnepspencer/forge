use crate::memory_workspace::WorthQuerySnapshotIdentity;

use super::WorthQueryJournalSegmentIdentity;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryJournalReplayRequest {
    segment_identity: WorthQueryJournalSegmentIdentity,
    basis_snapshot_identity: Option<WorthQuerySnapshotIdentity>,
}

impl WorthQueryJournalReplayRequest {
    pub fn new(segment_identity: WorthQueryJournalSegmentIdentity) -> Self {
        Self {
            segment_identity,
            basis_snapshot_identity: None,
        }
    }

    pub fn with_basis_snapshot(mut self, snapshot_identity: WorthQuerySnapshotIdentity) -> Self {
        self.basis_snapshot_identity = Some(snapshot_identity);
        self
    }

    pub fn segment_identity(&self) -> &WorthQueryJournalSegmentIdentity {
        &self.segment_identity
    }

    pub fn basis_snapshot_identity(&self) -> Option<&WorthQuerySnapshotIdentity> {
        self.basis_snapshot_identity.as_ref()
    }
}
