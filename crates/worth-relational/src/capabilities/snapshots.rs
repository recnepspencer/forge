use crate::identity::data::VersionId;
use crate::logic::runtime::RelationalRuntime;
use crate::snapshots::data::{SnapshotId, SnapshotReadPolicy};

pub(crate) trait SnapshotSource {
    fn active_snapshot_binding(
        &self,
        snapshot_id: SnapshotId,
    ) -> Option<(VersionId, SnapshotReadPolicy)>;
    fn execution_basis_binding(
        &self,
        snapshot_id: SnapshotId,
    ) -> Option<(VersionId, SnapshotReadPolicy)>;
    fn published_snapshot_version(&self, snapshot_id: SnapshotId) -> Option<VersionId>;
}

impl SnapshotSource for RelationalRuntime {
    fn active_snapshot_binding(
        &self,
        snapshot_id: SnapshotId,
    ) -> Option<(VersionId, SnapshotReadPolicy)> {
        self.visibility
            .active_handle_binding(snapshot_id)
            .map(|binding| (binding.version_id, binding.read_policy))
    }

    fn published_snapshot_version(&self, snapshot_id: SnapshotId) -> Option<VersionId> {
        self.visibility.published_snapshot_version(snapshot_id)
    }

    fn execution_basis_binding(
        &self,
        snapshot_id: SnapshotId,
    ) -> Option<(VersionId, SnapshotReadPolicy)> {
        self.visibility
            .execution_basis_binding(snapshot_id)
            .map(|binding| (binding.version_id, binding.read_policy))
    }
}
