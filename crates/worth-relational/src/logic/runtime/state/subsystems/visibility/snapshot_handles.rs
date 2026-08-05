use std::collections::BTreeMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, MutexGuard};

use crate::history::data::BranchId;
use crate::identity::data::VersionId;
use crate::snapshots::data::{SnapshotId, SnapshotReadPolicy};

#[derive(Debug, Default)]
pub(crate) struct SnapshotHandles {
    active: BTreeMap<SnapshotId, SnapshotHandleBinding>,
    published: BTreeMap<SnapshotId, SnapshotHandleBinding>,
    next_snapshot_id: AtomicU64,
}

impl SnapshotHandles {
    pub(crate) fn new() -> Self {
        Self {
            active: BTreeMap::new(),
            published: BTreeMap::new(),
            next_snapshot_id: AtomicU64::new(1),
        }
    }

    pub(crate) fn fork(&self) -> Self {
        Self {
            active: self.active.clone(),
            published: self.published.clone(),
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
        self.published.insert(snapshot_id, binding);
    }

    pub(crate) fn remove_published(
        &mut self,
        snapshot_id: SnapshotId,
    ) -> Option<SnapshotHandleBinding> {
        self.published.remove(&snapshot_id)
    }

    pub(crate) fn published_version(&self, snapshot_id: SnapshotId) -> Option<VersionId> {
        self.published
            .get(&snapshot_id)
            .map(|binding| binding.version_id)
    }

    pub(crate) fn published_versions(&self) -> impl Iterator<Item = VersionId> + '_ {
        self.published.values().map(|binding| binding.version_id)
    }

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

    pub(crate) fn oldest_published_snapshot_id(&self) -> Option<SnapshotId> {
        self.published.keys().next().copied()
    }
}

#[derive(Debug)]
pub(crate) struct SnapshotHandleBinding {
    pub(crate) branch_id: BranchId,
    pub(crate) version_id: VersionId,
    pub(crate) read_policy: SnapshotReadPolicy,
}

impl SnapshotHandleBinding {
    pub(crate) fn new(
        branch_id: BranchId,
        version_id: VersionId,
        read_policy: SnapshotReadPolicy,
    ) -> Self {
        Self {
            branch_id,
            version_id,
            read_policy,
        }
    }
}

impl Clone for SnapshotHandleBinding {
    fn clone(&self) -> Self {
        Self::new(self.branch_id.clone(), self.version_id, self.read_policy)
    }
}

#[derive(Clone, Debug)]
pub(crate) struct ExecutionBasisBinding {
    pub(crate) branch_id: BranchId,
    pub(crate) version_id: VersionId,
    pub(crate) read_policy: SnapshotReadPolicy,
    lease_ordinal: u64,
}

#[derive(Debug)]
pub(crate) struct ExecutionBasisRegistry {
    state: Mutex<ExecutionBasisRegistryState>,
    next_lease_ordinal: AtomicU64,
}

#[derive(Debug, Default)]
struct ExecutionBasisRegistryState {
    bindings: BTreeMap<SnapshotId, ExecutionBasisBinding>,
    version_ref_counts: BTreeMap<VersionId, usize>,
}

impl ExecutionBasisRegistry {
    pub(crate) fn new() -> Arc<Self> {
        Arc::new(Self {
            state: Mutex::new(ExecutionBasisRegistryState::default()),
            next_lease_ordinal: AtomicU64::new(1),
        })
    }

    pub(crate) fn admit(
        &self,
        snapshot_id: SnapshotId,
        branch_id: BranchId,
        version_id: VersionId,
        read_policy: SnapshotReadPolicy,
    ) -> u64 {
        let lease_ordinal = self.next_lease_ordinal.fetch_add(1, Ordering::Relaxed);
        let mut state = self.lock_state();
        state.bindings.insert(
            snapshot_id,
            ExecutionBasisBinding {
                branch_id,
                version_id,
                read_policy,
                lease_ordinal,
            },
        );
        *state.version_ref_counts.entry(version_id).or_default() += 1;
        lease_ordinal
    }

    pub(crate) fn binding(&self, snapshot_id: SnapshotId) -> Option<ExecutionBasisBinding> {
        self.lock_state().bindings.get(&snapshot_id).cloned()
    }

    pub(crate) fn retains(
        &self,
        snapshot_id: SnapshotId,
        branch_id: &BranchId,
        version_id: VersionId,
        read_policy: SnapshotReadPolicy,
        lease_ordinal: u64,
    ) -> bool {
        self.binding(snapshot_id).is_some_and(|binding| {
            binding.version_id == version_id
                && &binding.branch_id == branch_id
                && binding.read_policy == read_policy
                && binding.lease_ordinal == lease_ordinal
        })
    }

    pub(crate) fn retains_identity(&self, snapshot_id: SnapshotId, lease_ordinal: u64) -> bool {
        self.binding(snapshot_id)
            .is_some_and(|binding| binding.lease_ordinal == lease_ordinal)
    }

    pub(crate) fn release(&self, snapshot_id: SnapshotId, lease_ordinal: u64) -> bool {
        let mut state = self.lock_state();
        if state
            .bindings
            .get(&snapshot_id)
            .is_none_or(|binding| binding.lease_ordinal != lease_ordinal)
        {
            return false;
        }
        let binding = state
            .bindings
            .remove(&snapshot_id)
            .expect("checked execution basis binding must remain present");
        let remove_version = state
            .version_ref_counts
            .get_mut(&binding.version_id)
            .is_some_and(|count| {
                *count -= 1;
                *count == 0
            });
        if remove_version {
            state.version_ref_counts.remove(&binding.version_id);
        }
        true
    }

    pub(crate) fn oldest_version(&self) -> Option<VersionId> {
        self.lock_state()
            .version_ref_counts
            .first_key_value()
            .map(|(version_id, _count)| *version_id)
    }

    pub(crate) fn retains_version(&self, version_id: VersionId) -> bool {
        self.lock_state()
            .version_ref_counts
            .contains_key(&version_id)
    }

    fn lock_state(&self) -> MutexGuard<'_, ExecutionBasisRegistryState> {
        self.state
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }
}
