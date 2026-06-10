use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
#[cfg(test)]
use std::sync::atomic::AtomicUsize;
#[cfg(test)]
use std::sync::atomic::Ordering;

use crate::runtime::shared_read::SharedReadDerivedViewState;

use super::{
    ForgeQuerySharedReadGenerationEntry, ForgeQuerySharedReadGenerationId,
    ForgeQuerySharedReadGenerationLease, ForgeQuerySharedReadPinnedSnapshot,
};
#[cfg(test)]
use super::ForgeQuerySharedReadCounters;

#[derive(Clone, Debug)]
pub(in crate::runtime) struct ForgeQuerySharedReadPinRegistry {
    state: Arc<Mutex<ForgeQuerySharedReadPinRegistryState>>,
    #[cfg(test)]
    committed_read_hot_path_lock_count: Arc<AtomicUsize>,
}

impl Default for ForgeQuerySharedReadPinRegistry {
    fn default() -> Self {
        Self {
            state: Arc::new(Mutex::new(ForgeQuerySharedReadPinRegistryState::default())),
            #[cfg(test)]
            committed_read_hot_path_lock_count: Arc::new(AtomicUsize::new(0)),
        }
    }
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
        let entry = {
            let mut state = self.state.lock().expect("shared-read pin registry lock");
            state.pin_current_generation()
        }?;
        Some(ForgeQuerySharedReadGenerationLease::new(self.clone(), entry))
    }

    pub(in crate::runtime) fn has_current_generation(&self) -> bool {
        let state = self.state.lock().expect("shared-read pin registry lock");
        state.current_generation_ordinal.is_some()
    }

    pub(in crate::runtime) fn release_generation(
        &self,
        entry: Arc<ForgeQuerySharedReadGenerationEntry>,
    ) {
        let remaining_pin_count = entry.release_pin();
        if !(entry.is_retired() && remaining_pin_count == 0) {
            return;
        }
        let mut state = self.state.lock().expect("shared-read pin registry lock");
        state.collect_retired_zero_pin_generations();
    }

    #[cfg(test)]
    pub(in crate::runtime) fn counters(&self) -> ForgeQuerySharedReadCounters {
        let state = self.state.lock().expect("shared-read pin registry lock");
        state.counters(
            self.committed_read_hot_path_lock_count.load(Ordering::SeqCst),
        )
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
    generations: BTreeMap<u64, Arc<ForgeQuerySharedReadGenerationEntry>>,
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
            if let Some(entry) = self.generations.get(&current) {
                entry.retire();
            }
        }

        self.next_generation_ordinal += 1;
        let generation =
            ForgeQuerySharedReadGenerationId::new(self.next_generation_ordinal, snapshot_token);
        let ordinal = generation.ordinal();
        let snapshot = ForgeQuerySharedReadPinnedSnapshot::new(generation, derived_views);
        self.current_snapshot_token = Some(snapshot.generation().snapshot_token().to_string());
        self.current_generation_ordinal = Some(ordinal);
        self.generations.insert(
            ordinal,
            Arc::new(ForgeQuerySharedReadGenerationEntry::new(snapshot)),
        );
        self.collect_retired_zero_pin_generations();
    }

    fn pin_current_generation(&mut self) -> Option<Arc<ForgeQuerySharedReadGenerationEntry>> {
        let ordinal = self.current_generation_ordinal?;
        let entry = Arc::clone(self.generations.get(&ordinal)?);
        entry.pin();
        Some(entry)
    }

    fn collect_retired_zero_pin_generations(&mut self) {
        let removable = self
            .generations
            .iter()
            .filter_map(|(ordinal, entry)| {
                (entry.is_retired() && entry.pin_count() == 0).then_some(*ordinal)
            })
            .collect::<Vec<_>>();
        for ordinal in removable {
            self.generations.remove(&ordinal);
        }
    }

    #[cfg(test)]
    fn counters(
        &self,
        committed_read_hot_path_lock_count: usize,
    ) -> ForgeQuerySharedReadCounters {
        let orphaned_generation_count = self
            .generations
            .values()
            .filter(|entry| entry.is_retired() && entry.pin_count() == 0)
            .count();
        let unretired_pin_count = self
            .generations
            .values()
            .map(|entry| entry.pin_count())
            .sum();
        ForgeQuerySharedReadCounters::new(
            committed_read_hot_path_lock_count,
            orphaned_generation_count,
            unretired_pin_count,
        )
    }

    #[cfg(test)]
    fn force_retire_snapshot_token(&mut self, snapshot_token: &str) {
        let affected = self
            .generations
            .iter()
            .filter_map(|(ordinal, entry)| {
                (entry.snapshot().generation().snapshot_token() == snapshot_token).then_some(*ordinal)
            })
            .collect::<Vec<_>>();
        for ordinal in affected {
            let Some(entry) = self.generations.get(&ordinal) else {
                continue;
            };
            entry.retire();
            if self.current_generation_ordinal == Some(ordinal) {
                self.current_generation_ordinal = None;
                self.current_snapshot_token = None;
            }
        }
        self.collect_retired_zero_pin_generations();
    }
}
