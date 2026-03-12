use crate::data::dependency::DependencySnapshotStore;

use super::{remap_live_entry_handles, SignalGraph};

impl SignalGraph {
    pub(super) fn compact_dependency_snapshot_storage(&mut self) {
        let old_dependency_snapshots = std::mem::take(&mut self.topology.dependency_snapshots);
        self.observation.telemetry.storage.graph_storage_compaction_count += 1;
        self.observation.telemetry.storage.graph_storage_snapshot_rewrites +=
            old_dependency_snapshots.live_snapshot_count() as u64;

        let mut compacted_dependency_snapshots = DependencySnapshotStore::default();
        remap_live_entry_handles(
            self,
            |entry| entry.get_dep_snapshot_id(),
            |snapshot_id| compacted_dependency_snapshots.insert(old_dependency_snapshots.get(snapshot_id).clone()),
            |entry, snapshot_id| entry.set_dep_snapshot_id(snapshot_id),
        );

        self.topology.dependency_snapshots = compacted_dependency_snapshots;
    }
}
