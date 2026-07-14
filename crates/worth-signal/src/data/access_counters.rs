use std::sync::atomic::{AtomicU64, Ordering};

#[cfg(test)]
use serde::{Deserialize, Serialize};

#[cfg(test)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct AccessCounterSnapshot {
    pub materialized_entry_reads: u64,
    pub materialized_entry_writes: u64,
    pub runtime_artifact_warm_reads: u64,
    pub runtime_artifact_state_reads: u64,
    pub retained_artifact_reads: u64,
    pub reconstructed_artifact_reads: u64,
}

#[cfg(test)]
impl AccessCounterSnapshot {
    pub(crate) fn delta_since(self, before: Self) -> Self {
        Self {
            materialized_entry_reads: self
                .materialized_entry_reads
                .saturating_sub(before.materialized_entry_reads),
            materialized_entry_writes: self
                .materialized_entry_writes
                .saturating_sub(before.materialized_entry_writes),
            runtime_artifact_warm_reads: self
                .runtime_artifact_warm_reads
                .saturating_sub(before.runtime_artifact_warm_reads),
            runtime_artifact_state_reads: self
                .runtime_artifact_state_reads
                .saturating_sub(before.runtime_artifact_state_reads),
            retained_artifact_reads: self
                .retained_artifact_reads
                .saturating_sub(before.retained_artifact_reads),
            reconstructed_artifact_reads: self
                .reconstructed_artifact_reads
                .saturating_sub(before.reconstructed_artifact_reads),
        }
    }
}

struct AccessCounters {
    materialized_entry_reads: AtomicU64,
    materialized_entry_writes: AtomicU64,
    runtime_artifact_warm_reads: AtomicU64,
    runtime_artifact_state_reads: AtomicU64,
    retained_artifact_reads: AtomicU64,
    reconstructed_artifact_reads: AtomicU64,
}

static ACCESS_COUNTERS: AccessCounters = AccessCounters {
    materialized_entry_reads: AtomicU64::new(0),
    materialized_entry_writes: AtomicU64::new(0),
    runtime_artifact_warm_reads: AtomicU64::new(0),
    runtime_artifact_state_reads: AtomicU64::new(0),
    retained_artifact_reads: AtomicU64::new(0),
    reconstructed_artifact_reads: AtomicU64::new(0),
};

#[cfg(test)]
pub(crate) fn snapshot() -> AccessCounterSnapshot {
    AccessCounterSnapshot {
        materialized_entry_reads: ACCESS_COUNTERS
            .materialized_entry_reads
            .load(Ordering::Relaxed),
        materialized_entry_writes: ACCESS_COUNTERS
            .materialized_entry_writes
            .load(Ordering::Relaxed),
        runtime_artifact_warm_reads: ACCESS_COUNTERS
            .runtime_artifact_warm_reads
            .load(Ordering::Relaxed),
        runtime_artifact_state_reads: ACCESS_COUNTERS
            .runtime_artifact_state_reads
            .load(Ordering::Relaxed),
        retained_artifact_reads: ACCESS_COUNTERS
            .retained_artifact_reads
            .load(Ordering::Relaxed),
        reconstructed_artifact_reads: ACCESS_COUNTERS
            .reconstructed_artifact_reads
            .load(Ordering::Relaxed),
    }
}

pub(crate) fn note_materialized_entry_read() {
    ACCESS_COUNTERS
        .materialized_entry_reads
        .fetch_add(1, Ordering::Relaxed);
}

pub(crate) fn note_materialized_entry_write() {
    ACCESS_COUNTERS
        .materialized_entry_writes
        .fetch_add(1, Ordering::Relaxed);
}

pub(crate) fn note_runtime_artifact_warm_read() {
    ACCESS_COUNTERS
        .runtime_artifact_warm_reads
        .fetch_add(1, Ordering::Relaxed);
}

pub(crate) fn note_runtime_artifact_state_read() {
    ACCESS_COUNTERS
        .runtime_artifact_state_reads
        .fetch_add(1, Ordering::Relaxed);
}

pub(crate) fn note_retained_artifact_read() {
    ACCESS_COUNTERS
        .retained_artifact_reads
        .fetch_add(1, Ordering::Relaxed);
}

pub(crate) fn note_reconstructed_artifact_read() {
    ACCESS_COUNTERS
        .reconstructed_artifact_reads
        .fetch_add(1, Ordering::Relaxed);
}
