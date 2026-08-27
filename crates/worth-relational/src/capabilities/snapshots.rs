use crate::identity::data::VersionId;
use crate::runtime::RelationalRuntime;
use crate::snapshots::data::{SnapshotId, SnapshotReadPolicy};

pub(crate) trait SnapshotSource {
    fn active_snapshot_binding(
        &self,
        snapshot_id: SnapshotId,
    ) -> Option<(
        crate::history::data::BranchId,
        VersionId,
        SnapshotReadPolicy,
    )>;
}

impl SnapshotSource for RelationalRuntime {
    fn active_snapshot_binding(
        &self,
        snapshot_id: SnapshotId,
    ) -> Option<(
        crate::history::data::BranchId,
        VersionId,
        SnapshotReadPolicy,
    )> {
        self.visibility
            .active_handle_binding(snapshot_id)
            .map(|binding| {
                (
                    binding.branch_id.clone(),
                    binding.version_id,
                    binding.read_policy,
                )
            })
    }
}
