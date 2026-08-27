use std::collections::BTreeMap;
use std::sync::atomic::{AtomicU64, Ordering};

use crate::history::data::BranchId;
use crate::identity::data::VersionId;
use crate::snapshots::data::{SnapshotId, SnapshotReadPolicy};

#[derive(Debug, Default)]
pub(crate) struct SnapshotHandles {
    active: BTreeMap<SnapshotId, SnapshotHandleBinding>,
    published: BTreeMap<SnapshotId, SnapshotHandleBinding>,
    published_by_version: BTreeMap<VersionId, SnapshotId>,
    next_snapshot_id: AtomicU64,
}

impl SnapshotHandles {
    pub(crate) fn new() -> Self {
        Self {
            active: BTreeMap::new(),
            published: BTreeMap::new(),
            published_by_version: BTreeMap::new(),
            next_snapshot_id: AtomicU64::new(1),
        }
    }

    pub(crate) fn fork(&self) -> Self {
        Self {
            active: self.active.clone(),
            published: self.published.clone(),
            published_by_version: self.published_by_version.clone(),
            next_snapshot_id: AtomicU64::new(self.next_snapshot_id.load(Ordering::Relaxed)),
        }
    }

    pub(crate) fn active_count(&self) -> usize {
        self.active.len()
    }

    pub(crate) fn published_count(&self) -> usize {
        self.published.len()
    }

    pub(crate) fn next_snapshot_id(&self) -> SnapshotId {
        SnapshotId(self.next_snapshot_id.fetch_add(1, Ordering::Relaxed))
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

    pub(crate) fn insert_published(
        &mut self,
        snapshot_id: SnapshotId,
        binding: SnapshotHandleBinding,
    ) {
        let version_id = binding.version_id;
        if let Some(previous) = self.published.insert(snapshot_id, binding) {
            if self.published_by_version.get(&previous.version_id) == Some(&snapshot_id) {
                self.published_by_version.remove(&previous.version_id);
            }
        }
        if let Some(displaced_snapshot_id) =
            self.published_by_version.insert(version_id, snapshot_id)
        {
            if displaced_snapshot_id != snapshot_id {
                self.published.remove(&displaced_snapshot_id);
            }
        }
    }

    pub(crate) fn remove_published(
        &mut self,
        snapshot_id: SnapshotId,
    ) -> Option<SnapshotHandleBinding> {
        let binding = self.published.remove(&snapshot_id)?;
        if self.published_by_version.get(&binding.version_id) == Some(&snapshot_id) {
            self.published_by_version.remove(&binding.version_id);
        }
        Some(binding)
    }

    pub(crate) fn published_versions(&self) -> impl Iterator<Item = VersionId> + '_ {
        self.published.values().map(|binding| binding.version_id)
    }

    #[cfg(test)]
    pub(crate) fn retains_published_version(&self, version_id: VersionId) -> bool {
        self.published
            .values()
            .any(|binding| binding.version_id == version_id)
    }

    pub(crate) fn published_binding(
        &self,
        snapshot_id: SnapshotId,
    ) -> Option<&SnapshotHandleBinding> {
        self.published.get(&snapshot_id)
    }

    pub(crate) fn published_binding_for_version(
        &self,
        version_id: VersionId,
    ) -> Option<(SnapshotId, SnapshotHandleBinding)> {
        let snapshot_id = *self.published_by_version.get(&version_id)?;
        self.published
            .get(&snapshot_id)
            .cloned()
            .map(|binding| (snapshot_id, binding))
    }

    pub(crate) fn oldest_published_snapshot_id(&self) -> Option<SnapshotId> {
        self.published.keys().next().copied()
    }
}

#[derive(Debug)]
pub(crate) struct SnapshotHandleBinding {
    pub(crate) branch_id: BranchId,
    pub(crate) version_id: VersionId,
    pub(crate) read_policy: SnapshotReadPolicy,
    pub(crate) basis: crate::visibility::snapshot_states::VisibilitySnapshotBasis,
}

impl SnapshotHandleBinding {
    pub(crate) fn new(
        basis: crate::visibility::snapshot_states::VisibilitySnapshotBasis,
        read_policy: SnapshotReadPolicy,
    ) -> Self {
        Self {
            branch_id: basis.branch_id().clone(),
            version_id: basis.version_id(),
            read_policy,
            basis,
        }
    }
}

impl Clone for SnapshotHandleBinding {
    fn clone(&self) -> Self {
        Self::new(self.basis.clone(), self.read_policy)
    }
}
