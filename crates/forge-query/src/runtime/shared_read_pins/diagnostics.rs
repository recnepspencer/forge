use crate::memory_workspace::ForgeQuerySnapshotIdentity;

use super::ForgeQuerySharedReadCounters;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQuerySharedReadPinningDiagnostics {
    counters: ForgeQuerySharedReadCounters,
    generations: Vec<ForgeQuerySharedReadGenerationDiagnostic>,
}

impl ForgeQuerySharedReadPinningDiagnostics {
    pub(in crate::runtime) fn new(
        counters: ForgeQuerySharedReadCounters,
        generations: Vec<ForgeQuerySharedReadGenerationDiagnostic>,
    ) -> Self {
        Self {
            counters,
            generations,
        }
    }

    pub fn counters(&self) -> ForgeQuerySharedReadCounters {
        self.counters
    }

    pub fn generations(&self) -> &[ForgeQuerySharedReadGenerationDiagnostic] {
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
pub struct ForgeQuerySharedReadGenerationDiagnostic {
    ordinal: u64,
    snapshot_identity: ForgeQuerySnapshotIdentity,
    current: bool,
    retired: bool,
    invalidated: bool,
    pin_count: usize,
}

impl ForgeQuerySharedReadGenerationDiagnostic {
    pub(in crate::runtime) fn new(
        ordinal: u64,
        snapshot_identity: ForgeQuerySnapshotIdentity,
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

    pub fn snapshot_identity(&self) -> &ForgeQuerySnapshotIdentity {
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
