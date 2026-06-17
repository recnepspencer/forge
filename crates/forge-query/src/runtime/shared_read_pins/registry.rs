use std::collections::{BTreeMap, BTreeSet};
use std::sync::{Arc, Mutex};

use crate::memory_workspace::ForgeQuerySnapshotIdentity;

use super::ForgeQuerySharedReadCounters;
use super::{
    collect_retired_zero_pin_generations, ForgeQuerySharedReadCurrentGeneration,
    ForgeQuerySharedReadGenerationDiagnostic, ForgeQuerySharedReadGenerationEntry,
    ForgeQuerySharedReadGenerationId, ForgeQuerySharedReadGenerationLease,
    ForgeQuerySharedReadHotPathMeasurement, ForgeQuerySharedReadPinnedSnapshot,
    ForgeQuerySharedReadPinningDiagnostics,
};

#[derive(Clone, Debug)]
pub(in crate::runtime) struct ForgeQuerySharedReadPinRegistry {
    state: Arc<Mutex<ForgeQuerySharedReadPinRegistryState>>,
    current_generation: Arc<ForgeQuerySharedReadCurrentGeneration>,
    hot_path_measurement: ForgeQuerySharedReadHotPathMeasurement,
}

impl Default for ForgeQuerySharedReadPinRegistry {
    fn default() -> Self {
        Self {
            state: Arc::new(Mutex::new(ForgeQuerySharedReadPinRegistryState::default())),
            current_generation: Arc::new(ForgeQuerySharedReadCurrentGeneration::default()),
            hot_path_measurement: ForgeQuerySharedReadHotPathMeasurement::default(),
        }
    }
}

impl ForgeQuerySharedReadPinRegistry {
    pub(in crate::runtime) fn capture_committed_snapshot(
        &self,
        snapshot_identity: ForgeQuerySnapshotIdentity,
    ) -> ForgeQuerySharedReadGenerationId {
        let mut state = self.state.lock().expect("shared-read pin registry lock");
        state.capture_committed_snapshot(snapshot_identity, &self.current_generation)
    }

    pub(in crate::runtime) fn pin_current_generation(
        &self,
    ) -> Option<ForgeQuerySharedReadGenerationLease> {
        loop {
            let entry = self.current_generation.load()?;
            entry.pin();
            if !entry.is_invalidated() {
                return Some(ForgeQuerySharedReadGenerationLease::new(
                    self.clone(),
                    entry,
                ));
            }
            let remaining_pin_count = entry.release_pin();
            if entry.is_retired() && remaining_pin_count == 0 {
                self.drain_retired_generation();
            }
        }
    }

    pub(in crate::runtime) fn has_current_generation(&self) -> bool {
        self.current_generation
            .load()
            .is_some_and(|entry| !entry.is_retired())
    }

    pub(in crate::runtime) fn release_generation(
        &self,
        entry: Arc<ForgeQuerySharedReadGenerationEntry>,
    ) {
        let remaining_pin_count = entry.release_pin();
        if !(entry.is_retired() && remaining_pin_count == 0) {
            return;
        }
        self.drain_retired_generation();
    }

    pub(in crate::runtime) fn drain_retired_generation(&self) {
        let mut state = self.state.lock().expect("shared-read pin registry lock");
        state.collect_retired_zero_pin_generations();
    }

    pub(in crate::runtime) fn counters(&self) -> ForgeQuerySharedReadCounters {
        let state = self.state.lock().expect("shared-read pin registry lock");
        state.counters(
            self.hot_path_measurement
                .committed_read_hot_path_lock_count(),
        )
    }

    pub(in crate::runtime) fn diagnostics(&self) -> ForgeQuerySharedReadPinningDiagnostics {
        let state = self.state.lock().expect("shared-read pin registry lock");
        let counters = state.counters(
            self.hot_path_measurement
                .committed_read_hot_path_lock_count(),
        );
        ForgeQuerySharedReadPinningDiagnostics::new(counters, state.generation_diagnostics())
    }

    pub(in crate::runtime) fn retained_generation_ordinals(&self) -> BTreeSet<u64> {
        let state = self.state.lock().expect("shared-read pin registry lock");
        state.retained_generation_ordinals()
    }

    pub(in crate::runtime) fn record_committed_read_hot_path_lock_for_certification(&self) {
        self.hot_path_measurement
            .record_committed_read_hot_path_lock();
    }

    pub(in crate::runtime) fn force_retire_snapshot_identity(
        &self,
        snapshot_identity: &ForgeQuerySnapshotIdentity,
    ) {
        let mut state = self.state.lock().expect("shared-read pin registry lock");
        state.force_retire_snapshot_identity(snapshot_identity, &self.current_generation);
    }
}

#[derive(Debug, Default)]
struct ForgeQuerySharedReadPinRegistryState {
    next_generation_ordinal: u64,
    current_generation_ordinal: Option<u64>,
    current_snapshot_identity: Option<ForgeQuerySnapshotIdentity>,
    generations: BTreeMap<u64, Arc<ForgeQuerySharedReadGenerationEntry>>,
}

impl ForgeQuerySharedReadPinRegistryState {
    fn capture_committed_snapshot(
        &mut self,
        snapshot_identity: ForgeQuerySnapshotIdentity,
        current_generation: &ForgeQuerySharedReadCurrentGeneration,
    ) -> ForgeQuerySharedReadGenerationId {
        if self.current_snapshot_identity.as_ref() == Some(&snapshot_identity) {
            let ordinal = self
                .current_generation_ordinal
                .expect("current snapshot identity requires current generation");
            return self
                .generations
                .get(&ordinal)
                .expect("current generation should exist")
                .snapshot()
                .generation()
                .clone();
        }

        if let Some(current) = self.current_generation_ordinal {
            if let Some(entry) = self.generations.get(&current) {
                entry.retire();
            }
        }

        self.next_generation_ordinal += 1;
        let generation =
            ForgeQuerySharedReadGenerationId::new(self.next_generation_ordinal, snapshot_identity);
        let ordinal = generation.ordinal();
        let snapshot = ForgeQuerySharedReadPinnedSnapshot::new(generation.clone());
        self.current_snapshot_identity = Some(snapshot.generation().snapshot_identity().clone());
        self.current_generation_ordinal = Some(ordinal);
        let entry = Arc::new(ForgeQuerySharedReadGenerationEntry::new(snapshot));
        current_generation.publish(Arc::clone(&entry));
        self.generations.insert(ordinal, entry);
        self.collect_retired_zero_pin_generations();
        generation
    }

    fn collect_retired_zero_pin_generations(&mut self) {
        collect_retired_zero_pin_generations(&mut self.generations);
    }

    fn counters(&self, committed_read_hot_path_lock_count: usize) -> ForgeQuerySharedReadCounters {
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

    fn retained_generation_ordinals(&self) -> BTreeSet<u64> {
        self.generations.keys().copied().collect()
    }

    fn generation_diagnostics(&self) -> Vec<ForgeQuerySharedReadGenerationDiagnostic> {
        self.generations
            .iter()
            .map(|(ordinal, entry)| {
                ForgeQuerySharedReadGenerationDiagnostic::new(
                    *ordinal,
                    entry.snapshot().generation().snapshot_identity().clone(),
                    self.current_generation_ordinal == Some(*ordinal),
                    entry.is_retired(),
                    entry.is_invalidated(),
                    entry.pin_count(),
                )
            })
            .collect()
    }

    fn force_retire_snapshot_identity(
        &mut self,
        snapshot_identity: &ForgeQuerySnapshotIdentity,
        current_generation: &ForgeQuerySharedReadCurrentGeneration,
    ) {
        let affected = self
            .generations
            .iter()
            .filter_map(|(ordinal, entry)| {
                (entry.snapshot().generation().snapshot_identity() == snapshot_identity)
                    .then_some(*ordinal)
            })
            .collect::<Vec<_>>();
        for ordinal in affected {
            let Some(entry) = self.generations.get(&ordinal) else {
                continue;
            };
            entry.invalidate();
            if self.current_generation_ordinal == Some(ordinal) {
                current_generation.clear_if_generation(entry);
                self.current_generation_ordinal = None;
                self.current_snapshot_identity = None;
            }
        }
        self.collect_retired_zero_pin_generations();
    }
}
