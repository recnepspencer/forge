use std::collections::BTreeMap;

use crate::identity::data::VersionId;
use crate::snapshots::data::{SnapshotId, SnapshotReadPolicy};
use crate::storage::overlay::SnapshotState;

#[derive(Debug, Default)]
pub(crate) struct SnapshotHandles {
    active: BTreeMap<SnapshotId, SnapshotHandleBinding>,
    published: BTreeMap<SnapshotId, SnapshotState>,
    next_snapshot_id: u64,
}

impl SnapshotHandles {
    pub(crate) fn new() -> Self {
        Self {
            active: BTreeMap::new(),
            published: BTreeMap::new(),
            next_snapshot_id: 1,
        }
    }

    pub(crate) fn fork(&self) -> Self {
        Self {
            active: self.active.clone(),
            published: self.published.clone(),
            next_snapshot_id: self.next_snapshot_id,
        }
    }

    pub(crate) fn active_count(&self) -> usize {
        self.active.len()
    }

    pub(crate) fn published_count(&self) -> usize {
        self.published.len()
    }

    pub(crate) fn next_snapshot_id(&mut self) -> SnapshotId {
        let snapshot_id = SnapshotId(self.next_snapshot_id);
        self.next_snapshot_id += 1;
        snapshot_id
    }

    pub(crate) fn insert_active(
        &mut self,
        snapshot_id: SnapshotId,
        binding: SnapshotHandleBinding,
    ) {
        self.active.insert(snapshot_id, binding);
    }

    pub(crate) fn remove_active(
        &mut self,
        snapshot_id: SnapshotId,
    ) -> Option<SnapshotHandleBinding> {
        self.active.remove(&snapshot_id)
    }

    pub(crate) fn active_binding(&self, snapshot_id: SnapshotId) -> Option<&SnapshotHandleBinding> {
        self.active.get(&snapshot_id)
    }

    pub(crate) fn is_known_snapshot(&self, snapshot_id: SnapshotId) -> bool {
        self.active.contains_key(&snapshot_id) || self.published.contains_key(&snapshot_id)
    }

    pub(crate) fn active_versions(&self) -> impl Iterator<Item = VersionId> + '_ {
        self.active.values().map(|binding| binding.version_id)
    }

    pub(crate) fn insert_published(&mut self, state: SnapshotState) {
        self.published.insert(state.handle.snapshot_id, state);
    }

    pub(crate) fn remove_published(&mut self, snapshot_id: SnapshotId) -> Option<SnapshotState> {
        self.published.remove(&snapshot_id)
    }

    pub(crate) fn published_version(&self, snapshot_id: SnapshotId) -> Option<VersionId> {
        self.published
            .get(&snapshot_id)
            .map(|state| state.handle.version_id)
    }

    pub(crate) fn published_state(&self, snapshot_id: SnapshotId) -> Option<&SnapshotState> {
        self.published.get(&snapshot_id)
    }

    pub(crate) fn oldest_published_snapshot_id(&self) -> Option<SnapshotId> {
        self.published.keys().next().copied()
    }

    pub(crate) fn oldest_published_snapshot_ids(&self, count: usize) -> Vec<SnapshotId> {
        self.published.keys().take(count).copied().collect()
    }
}

#[derive(Debug, Clone)]
pub(crate) struct SnapshotHandleBinding {
    pub(crate) version_id: VersionId,
    pub(crate) read_policy: SnapshotReadPolicy,
}
