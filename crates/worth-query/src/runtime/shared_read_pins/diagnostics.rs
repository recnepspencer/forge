use crate::memory_workspace::WorthQuerySnapshotIdentity;

use super::WorthQuerySharedReadCounters;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQuerySharedReadPinningDiagnostics {
    counters: WorthQuerySharedReadCounters,
    generations: Vec<WorthQuerySharedReadGenerationDiagnostic>,
}

impl WorthQuerySharedReadPinningDiagnostics {
    pub(in crate::runtime) fn new(
        counters: WorthQuerySharedReadCounters,
        generations: Vec<WorthQuerySharedReadGenerationDiagnostic>,
    ) -> Self {
        Self {
            counters,
            generations,
        }
    }

    pub fn counters(&self) -> WorthQuerySharedReadCounters {
        self.counters
    }

    pub fn generations(&self) -> &[WorthQuerySharedReadGenerationDiagnostic] {
        &self.generations
    }

    pub fn retained_generation_count(&self) -> usize {
        self.generations.len()
    }

    pub fn retired_pinned_generation_count(&self) -> usize {
        self.generations
            .iter()
            .filter(|generation| generation.retired && generation.pin_count > 0)
            .count()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQuerySharedReadGenerationDiagnostic {
    ordinal: u64,
    snapshot_identity: WorthQuerySnapshotIdentity,
    current: bool,
    retired: bool,
    invalidated: bool,
    pin_count: usize,
}

impl WorthQuerySharedReadGenerationDiagnostic {
    pub(in crate::runtime) fn new(
        ordinal: u64,
        snapshot_identity: WorthQuerySnapshotIdentity,
        current: bool,
        retired: bool,
        invalidated: bool,
        pin_count: usize,
    ) -> Self {
        Self {
            ordinal,
            snapshot_identity,
            current,
            retired,
            invalidated,
            pin_count,
        }
    }

    pub fn ordinal(&self) -> u64 {
        self.ordinal
    }

    pub fn snapshot_identity(&self) -> &WorthQuerySnapshotIdentity {
        &self.snapshot_identity
    }

    pub fn is_current(&self) -> bool {
        self.current
    }

    pub fn is_retired(&self) -> bool {
        self.retired
    }

    pub fn is_invalidated(&self) -> bool {
        self.invalidated
    }

    pub fn pin_count(&self) -> usize {
        self.pin_count
    }
}
