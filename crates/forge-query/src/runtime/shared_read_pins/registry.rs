use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

use crate::runtime::shared_read::SharedReadDerivedViewState;

use super::{
    ForgeQuerySharedReadGenerationId, ForgeQuerySharedReadGenerationLease,
    ForgeQuerySharedReadPinnedSnapshot,
};
#[cfg(test)]
use super::ForgeQuerySharedReadCounters;

#[derive(Clone, Debug, Default)]
pub(in crate::runtime) struct ForgeQuerySharedReadPinRegistry {
    state: Arc<Mutex<ForgeQuerySharedReadPinRegistryState>>,
}

impl ForgeQuerySharedReadPinRegistry {
    pub(in crate::runtime) fn capture_committed_snapshot(
        &self,
        snapshot_token: impl Into<String>,
        derived_views: BTreeMap<String, SharedReadDerivedViewState>,
    ) {
        let mut state = self.state.lock().expect("shared-read pin registry lock");
        state.capture_committed_snapshot(snapshot_token.into(), derived_views);
    }

    pub(in crate::runtime) fn pin_current_generation(
        &self,
    ) -> Option<ForgeQuerySharedReadGenerationLease> {
        let snapshot = {
            let mut state = self.state.lock().expect("shared-read pin registry lock");
            state.pin_current_generation()
        }?;
        Some(ForgeQuerySharedReadGenerationLease::new(
            self.clone(),
            snapshot,
        ))
    }

    pub(in crate::runtime) fn pin_generation(&self, ordinal: u64) {
        let mut state = self.state.lock().expect("shared-read pin registry lock");
        state.pin_generation(ordinal);
    }

    pub(in crate::runtime) fn release_generation(&self, ordinal: u64) {
        let mut state = self.state.lock().expect("shared-read pin registry lock");
        state.release_generation(ordinal);
    }

    pub(in crate::runtime) fn contains_generation(&self, ordinal: u64) -> bool {
        let state = self.state.lock().expect("shared-read pin registry lock");
        state.generations.contains_key(&ordinal)
    }

    #[cfg(test)]
    pub(in crate::runtime) fn counters(&self) -> ForgeQuerySharedReadCounters {
        let state = self.state.lock().expect("shared-read pin registry lock");
        state.counters()
    }

    #[cfg(test)]
    pub(crate) fn force_retire_snapshot_token(&self, snapshot_token: &str) {
        let mut state = self.state.lock().expect("shared-read pin registry lock");
        state.force_retire_snapshot_token(snapshot_token);
    }
}

#[derive(Debug, Default)]
struct ForgeQuerySharedReadPinRegistryState {
    next_generation_ordinal: u64,
    current_generation_ordinal: Option<u64>,
    current_snapshot_token: Option<String>,
    generations: BTreeMap<u64, ForgeQuerySharedReadGenerationEntry>,
}

impl ForgeQuerySharedReadPinRegistryState {
    fn capture_committed_snapshot(
        &mut self,
        snapshot_token: String,
        derived_views: BTreeMap<String, SharedReadDerivedViewState>,
    ) {
        if self.current_snapshot_token.as_deref() == Some(snapshot_token.as_str()) {
            return;
        }

        if let Some(current) = self.current_generation_ordinal {
            if let Some(entry) = self.generations.get_mut(&current) {
                entry.retired = true;
            }
        }

        self.next_generation_ordinal += 1;
        let generation =
            ForgeQuerySharedReadGenerationId::new(self.next_generation_ordinal, snapshot_token);
        let ordinal = generation.ordinal();
        let snapshot = Arc::new(ForgeQuerySharedReadPinnedSnapshot::new(generation, derived_views));
        self.current_snapshot_token = Some(snapshot.generation().snapshot_token().to_string());
        self.current_generation_ordinal = Some(ordinal);
        self.generations.insert(
            ordinal,
            ForgeQuerySharedReadGenerationEntry {
                snapshot,
                pin_count: 0,
                retired: false,
            },
        );
        self.collect_retired_zero_pin_generations();
    }

    fn pin_current_generation(&mut self) -> Option<Arc<ForgeQuerySharedReadPinnedSnapshot>> {
        let ordinal = self.current_generation_ordinal?;
        let snapshot = self.generations.get(&ordinal)?.snapshot.clone();
        self.pin_generation(ordinal);
        Some(snapshot)
    }

    fn pin_generation(&mut self, ordinal: u64) {
        if let Some(entry) = self.generations.get_mut(&ordinal) {
            entry.pin_count += 1;
        }
    }

    fn release_generation(&mut self, ordinal: u64) {
        let Some(entry) = self.generations.get_mut(&ordinal) else {
            return;
        };
        entry.pin_count = entry.pin_count.saturating_sub(1);
        self.collect_retired_zero_pin_generations();
    }

    fn collect_retired_zero_pin_generations(&mut self) {
        let removable = self
            .generations
            .iter()
            .filter_map(|(ordinal, entry)| {
                (entry.retired && entry.pin_count == 0).then_some(*ordinal)
            })
            .collect::<Vec<_>>();
        for ordinal in removable {
            self.generations.remove(&ordinal);
        }
    }

    #[cfg(test)]
    fn counters(&self) -> ForgeQuerySharedReadCounters {
        let orphaned_generation_count = self
            .generations
            .values()
            .filter(|entry| entry.retired && entry.pin_count == 0)
            .count();
        let unretired_pin_count = self
            .generations
            .values()
            .map(|entry| entry.pin_count)
            .sum();
        ForgeQuerySharedReadCounters::new(orphaned_generation_count, unretired_pin_count)
    }

    #[cfg(test)]
    fn force_retire_snapshot_token(&mut self, snapshot_token: &str) {
        let removable = self
            .generations
            .iter()
            .filter_map(|(ordinal, entry)| {
                (entry.snapshot.generation().snapshot_token() == snapshot_token).then_some(*ordinal)
            })
            .collect::<Vec<_>>();
        for ordinal in removable {
            self.generations.remove(&ordinal);
            if self.current_generation_ordinal == Some(ordinal) {
                self.current_generation_ordinal = None;
                self.current_snapshot_token = None;
            }
        }
    }
}

#[derive(Debug)]
struct ForgeQuerySharedReadGenerationEntry {
    snapshot: Arc<ForgeQuerySharedReadPinnedSnapshot>,
    pin_count: usize,
    retired: bool,
}
