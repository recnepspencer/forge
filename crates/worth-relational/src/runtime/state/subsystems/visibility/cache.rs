use std::collections::{BTreeMap, VecDeque};
use std::sync::{Mutex, RwLock};

use crate::runtime::RelationalRuntimeConfig;
use crate::visibility::snapshot_states::{SnapshotState, VisibilitySnapshotStateKey};

#[derive(Debug)]
pub(crate) struct VisibilityCache {
    pub(crate) states: RwLock<BTreeMap<VisibilitySnapshotStateKey, SnapshotState>>,
    pub(crate) residency: RwLock<BTreeMap<VisibilitySnapshotStateKey, VisibilityResidency>>,
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

    pub(crate) fn protected_state_keys(
        &self,
        protect_active_snapshots: bool,
    ) -> Vec<VisibilitySnapshotStateKey> {
        self.residency
            .read()
            .expect("visibility residency lock poisoned")
            .iter()
            .filter_map(|(key, entry)| {
                (entry.branch_head_refs > 0
                    || entry.replay_refs > 0
                    || (protect_active_snapshots && entry.active_snapshot_refs > 0))
                    .then_some(key.clone())
            })
            .collect()
    }

    pub(crate) fn recent_visibility_count(&self) -> usize {
        self.recent_policy
            .lock()
            .expect("recent visibility policy lock poisoned")
            .resident_count
    }

    pub(crate) fn tracked_branch_head_states(&self) -> Vec<VisibilitySnapshotStateKey> {
        self.residency
            .read()
            .expect("visibility residency lock poisoned")
            .iter()
            .filter_map(|(key, residency)| (residency.branch_head_refs > 0).then_some(key.clone()))
            .collect()
    }

    pub(crate) fn state(&self, key: &VisibilitySnapshotStateKey) -> Option<SnapshotState> {
        self.states
            .read()
            .expect("visibility state lock poisoned")
            .get(key)
            .cloned()
    }

    pub(crate) fn insert_state(&self, state: SnapshotState) {
        let key = state.basis.key();
        self.states
            .write()
            .expect("visibility state lock poisoned")
            .insert(key, state);
    }

    pub(crate) fn remove_state(&self, key: &VisibilitySnapshotStateKey) {
        self.states
            .write()
            .expect("visibility state lock poisoned")
            .remove(key);
    }

    pub(crate) fn residency(&self, key: &VisibilitySnapshotStateKey) -> VisibilityResidency {
        self.residency
            .read()
            .expect("visibility residency lock poisoned")
            .get(key)
            .cloned()
            .unwrap_or_default()
    }

    pub(crate) fn update_residency(
        &self,
        key: &VisibilitySnapshotStateKey,
        update: impl FnOnce(&mut VisibilityResidency),
    ) {
        let mut residency = self
            .residency
            .write()
            .expect("visibility residency lock poisoned");
        let entry = residency.entry(key.clone()).or_default();
        update(entry);
        if entry.branch_head_refs == 0
            && entry.replay_refs == 0
            && entry.active_snapshot_refs == 0
            && !entry.recent_resident
        {
            residency.remove(key);
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

    pub(crate) fn enqueue_recent_candidate(&self, key: VisibilitySnapshotStateKey) {
        self.recent_policy
            .lock()
            .expect("recent visibility policy lock poisoned")
            .order
            .push_back(key);
    }

    pub(crate) fn pop_oldest_recent_candidate(&self) -> Option<VisibilitySnapshotStateKey> {
        self.recent_policy
            .lock()
            .expect("recent visibility policy lock poisoned")
            .order
            .pop_front()
    }

    pub(crate) fn mark_recent_resident(&self, key: &VisibilitySnapshotStateKey) -> bool {
        {
            let mut residency = self
                .residency
                .write()
                .expect("visibility residency lock poisoned");
            let entry = residency.entry(key.clone()).or_default();
            if entry.recent_resident {
                return false;
            }
            entry.recent_resident = true;
        }
        let mut recent_policy = self
            .recent_policy
            .lock()
            .expect("recent visibility policy lock poisoned");
        recent_policy.order.push_back(key.clone());
        recent_policy.resident_count += 1;
        true
    }

    pub(crate) fn evict_recent_resident_if_unprotected(
        &self,
        key: &VisibilitySnapshotStateKey,
    ) -> bool {
        let mut residency = self
            .residency
            .write()
            .expect("visibility residency lock poisoned");
        let Some(entry) = residency.get_mut(key) else {
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
            residency.remove(key);
        }
        true
    }

    pub(crate) fn clear_branch_head_residency(
        &self,
        tracked_states: &[VisibilitySnapshotStateKey],
    ) {
        let mut residency = self
            .residency
            .write()
            .expect("visibility residency lock poisoned");
        for key in tracked_states {
            if let Some(entry) = residency.get_mut(key) {
                entry.branch_head_refs = 0;
                if entry.replay_refs == 0
                    && entry.active_snapshot_refs == 0
                    && !entry.recent_resident
                {
                    residency.remove(key);
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
    pub(crate) order: VecDeque<VisibilitySnapshotStateKey>,
    pub(crate) resident_count: usize,
}
