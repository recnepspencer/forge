use std::collections::{BTreeMap, VecDeque};
use std::sync::{Mutex, RwLock};

use crate::identity::data::VersionId;
use crate::logic::runtime::RelationalRuntimeConfig;
use crate::storage::overlay::SnapshotState;

#[derive(Debug)]
pub(crate) struct VisibilityCache {
    pub(crate) states: RwLock<BTreeMap<VersionId, SnapshotState>>,
    pub(crate) residency: RwLock<BTreeMap<VersionId, VisibilityResidency>>,
    pub(crate) recent_policy: Mutex<DeterministicVersionWindowPolicy>,
}

impl VisibilityCache {
    pub(crate) fn new(config: &RelationalRuntimeConfig) -> Self {
        Self {
            states: RwLock::new(BTreeMap::new()),
            residency: RwLock::new(BTreeMap::new()),
            recent_policy: Mutex::new(DeterministicVersionWindowPolicy {
                recent_version_window: config.visibility.cache_policy.recent_version_window,
                order: VecDeque::new(),
                resident_count: 0,
            }),
        }
    }

    pub(crate) fn fork(&self) -> Self {
        Self {
            states: RwLock::new(
                self.states
                    .read()
                    .expect("visibility state lock poisoned")
                    .clone(),
            ),
            residency: RwLock::new(
                self.residency
                    .read()
                    .expect("visibility residency lock poisoned")
                    .clone(),
            ),
            recent_policy: Mutex::new(
                self.recent_policy
                    .lock()
                    .expect("recent visibility policy lock poisoned")
                    .clone(),
            ),
        }
    }

    pub(crate) fn clear(&self) {
        self.states
            .write()
            .expect("visibility state lock poisoned")
            .clear();
        self.residency
            .write()
            .expect("visibility residency lock poisoned")
            .clear();
        let mut recent_policy = self
            .recent_policy
            .lock()
            .expect("recent visibility policy lock poisoned");
        recent_policy.order.clear();
        recent_policy.resident_count = 0;
    }

    pub(crate) fn cached_version_count(&self) -> usize {
        self.states
            .read()
            .expect("visibility state lock poisoned")
            .len()
    }

    pub(crate) fn protected_version_count(&self, protect_active_snapshots: bool) -> usize {
        self.residency
            .read()
            .expect("visibility residency lock poisoned")
            .values()
            .filter(|entry| {
                entry.branch_head_refs > 0
                    || entry.replay_refs > 0
                    || (protect_active_snapshots && entry.active_snapshot_refs > 0)
            })
            .count()
    }

    pub(crate) fn recent_visibility_count(&self) -> usize {
        self.recent_policy
            .lock()
            .expect("recent visibility policy lock poisoned")
            .resident_count
    }

    pub(crate) fn tracked_branch_head_versions(&self) -> Vec<VersionId> {
        self.residency
            .read()
            .expect("visibility residency lock poisoned")
            .iter()
            .filter_map(|(version_id, residency)| {
                (residency.branch_head_refs > 0).then_some(*version_id)
            })
            .collect()
    }

    pub(crate) fn state_for_version(&self, version_id: VersionId) -> Option<SnapshotState> {
        self.states
            .read()
            .expect("visibility state lock poisoned")
            .get(&version_id)
            .cloned()
    }

    pub(crate) fn insert_state(&self, state: SnapshotState) {
        self.states
            .write()
            .expect("visibility state lock poisoned")
            .insert(state.handle.version_id, state);
    }

    pub(crate) fn remove_state(&self, version_id: VersionId) {
        self.states
            .write()
            .expect("visibility state lock poisoned")
            .remove(&version_id);
    }

    pub(crate) fn residency_for_version(&self, version_id: VersionId) -> VisibilityResidency {
        self.residency
            .read()
            .expect("visibility residency lock poisoned")
            .get(&version_id)
            .cloned()
            .unwrap_or_default()
    }

    pub(crate) fn update_residency(
        &self,
        version_id: VersionId,
        update: impl FnOnce(&mut VisibilityResidency),
    ) {
        let mut residency = self
            .residency
            .write()
            .expect("visibility residency lock poisoned");
        let entry = residency.entry(version_id).or_default();
        update(entry);
        if entry.branch_head_refs == 0
            && entry.replay_refs == 0
            && entry.active_snapshot_refs == 0
            && !entry.recent_resident
        {
            residency.remove(&version_id);
        }
    }

    pub(crate) fn recent_window(&self) -> usize {
        self.recent_policy
            .lock()
            .expect("recent visibility policy lock poisoned")
            .recent_version_window
    }

    pub(crate) fn resident_recent_count(&self) -> usize {
        self.recent_policy
            .lock()
            .expect("recent visibility policy lock poisoned")
            .resident_count
    }

    pub(crate) fn recent_candidate_count(&self) -> usize {
        self.recent_policy
            .lock()
            .expect("recent visibility policy lock poisoned")
            .order
            .len()
    }

    pub(crate) fn enqueue_recent_candidate(&self, version_id: VersionId) {
        self.recent_policy
            .lock()
            .expect("recent visibility policy lock poisoned")
            .order
            .push_back(version_id);
    }

    pub(crate) fn pop_oldest_recent_candidate(&self) -> Option<VersionId> {
        self.recent_policy
            .lock()
            .expect("recent visibility policy lock poisoned")
            .order
            .pop_front()
    }

    pub(crate) fn mark_recent_resident(&self, version_id: VersionId) -> bool {
        {
            let mut residency = self
                .residency
                .write()
                .expect("visibility residency lock poisoned");
            let entry = residency.entry(version_id).or_default();
            if entry.recent_resident {
                return false;
            }
            entry.recent_resident = true;
        }
        let mut recent_policy = self
            .recent_policy
            .lock()
            .expect("recent visibility policy lock poisoned");
        recent_policy.order.push_back(version_id);
        recent_policy.resident_count += 1;
        true
    }

    pub(crate) fn evict_recent_resident_if_unprotected(&self, version_id: VersionId) -> bool {
        let mut residency = self
            .residency
            .write()
            .expect("visibility residency lock poisoned");
        let Some(entry) = residency.get_mut(&version_id) else {
            return false;
        };
        if !entry.recent_resident {
            return false;
        }
        if entry.branch_head_refs > 0 || entry.replay_refs > 0 || entry.active_snapshot_refs > 0 {
            return false;
        }
        entry.recent_resident = false;
        self.recent_policy
            .lock()
            .expect("recent visibility policy lock poisoned")
            .resident_count -= 1;
        if entry.branch_head_refs == 0 && entry.replay_refs == 0 && entry.active_snapshot_refs == 0
        {
            residency.remove(&version_id);
        }
        true
    }

    pub(crate) fn clear_branch_head_residency(&self, tracked_versions: &[VersionId]) {
        let mut residency = self
            .residency
            .write()
            .expect("visibility residency lock poisoned");
        for version_id in tracked_versions {
            if let Some(entry) = residency.get_mut(version_id) {
                entry.branch_head_refs = 0;
                if entry.replay_refs == 0
                    && entry.active_snapshot_refs == 0
                    && !entry.recent_resident
                {
                    residency.remove(version_id);
                }
            }
        }
    }
}

#[derive(Debug, Clone, Default)]
pub(crate) struct VisibilityResidency {
    pub(crate) branch_head_refs: u32,
    pub(crate) replay_refs: u32,
    pub(crate) active_snapshot_refs: u32,
    pub(crate) recent_resident: bool,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct DeterministicVersionWindowPolicy {
    pub(crate) recent_version_window: usize,
    pub(crate) order: VecDeque<VersionId>,
    pub(crate) resident_count: usize,
}
