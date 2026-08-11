use super::{LineageArtifactId, LineageRecord, LineageRecordKind, SnapshotRestoreKind};
use crate::data::handle::NodeId;
use crate::state::{SignalBranchId, SignalSnapshotId};

impl LineageRecord {
    pub fn snapshot_restore(
        sequence: u64,
        emitted_on_branch_id: SignalBranchId,
        snapshot_id: SignalSnapshotId,
        node: Option<NodeId>,
        artifact_id: Option<LineageArtifactId>,
        restore_kind: SnapshotRestoreKind,
    ) -> Self {
        Self::new(
            sequence,
            emitted_on_branch_id,
            LineageRecordKind::SnapshotRestore {
                snapshot_id,
                node,
                artifact_id,
                restore_kind,
            },
        )
    }
}
