use std::sync::atomic::{AtomicU64, Ordering};

use super::{CanonicalizationMetrics, StoreCounterSnapshot, StoreCounters};

#[derive(Debug, Default)]
pub(super) struct ModeCounters {
    durable_mode_selection_count: AtomicU64,
    embedded_mode_selection_count: AtomicU64,
    absent_mode_selection_count: AtomicU64,
    hosted_runtime_start_count: AtomicU64,
    hosted_runtime_stop_count: AtomicU64,
    external_commit_intake_count: AtomicU64,
    external_checkpoint_intake_count: AtomicU64,
    embedded_checkpoint_authority_rejection_count: AtomicU64,
    cross_mode_canonical_boundary_reuse_count: AtomicU64,
    mode_misuse_rejection_count: AtomicU64,
    absent_mode_store_touch_count: AtomicU64,
    canonicalization_item_count: AtomicU64,
    canonicalization_duplicate_collapse_count: AtomicU64,
}

impl StoreCounters {
    pub fn record_durable_mode_selection(&self) {
        self.mode
            .durable_mode_selection_count
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_embedded_mode_selection(&self) {
        self.mode
            .embedded_mode_selection_count
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_hosted_runtime_start(&self) {
        self.mode
            .hosted_runtime_start_count
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_hosted_runtime_stop(&self) {
        self.mode
            .hosted_runtime_stop_count
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_external_commit_intake(&self) {
        self.mode
            .external_commit_intake_count
            .fetch_add(1, Ordering::Relaxed);
        self.mode
            .cross_mode_canonical_boundary_reuse_count
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_external_checkpoint_intake(&self) {
        self.mode
            .external_checkpoint_intake_count
            .fetch_add(1, Ordering::Relaxed);
    }

    #[cfg(test)]
    pub fn record_embedded_checkpoint_authority_rejection(&self) {
        self.mode
            .embedded_checkpoint_authority_rejection_count
            .fetch_add(1, Ordering::Relaxed);
    }

    #[cfg(test)]
    pub fn record_mode_misuse_rejection(&self) {
        self.mode
            .mode_misuse_rejection_count
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_canonicalization(&self, metrics: CanonicalizationMetrics) {
        self.mode
            .canonicalization_item_count
            .fetch_add(metrics.canonicalization_item_count, Ordering::Relaxed);
        self.mode
            .canonicalization_duplicate_collapse_count
            .fetch_add(
                metrics.canonicalization_duplicate_collapse_count,
                Ordering::Relaxed,
            );
    }
}

pub(super) fn write_snapshot(counters: &ModeCounters, snapshot: &mut StoreCounterSnapshot) {
    snapshot.durable_mode_selection_count = counters
        .durable_mode_selection_count
        .load(Ordering::Relaxed);
    snapshot.embedded_mode_selection_count = counters
        .embedded_mode_selection_count
        .load(Ordering::Relaxed);
    snapshot.absent_mode_selection_count =
        counters.absent_mode_selection_count.load(Ordering::Relaxed);
    snapshot.hosted_runtime_start_count =
        counters.hosted_runtime_start_count.load(Ordering::Relaxed);
    snapshot.hosted_runtime_stop_count = counters.hosted_runtime_stop_count.load(Ordering::Relaxed);
    snapshot.external_commit_intake_count = counters
        .external_commit_intake_count
        .load(Ordering::Relaxed);
    snapshot.external_checkpoint_intake_count = counters
        .external_checkpoint_intake_count
        .load(Ordering::Relaxed);
    snapshot.embedded_checkpoint_authority_rejection_count = counters
        .embedded_checkpoint_authority_rejection_count
        .load(Ordering::Relaxed);
    snapshot.cross_mode_canonical_boundary_reuse_count = counters
        .cross_mode_canonical_boundary_reuse_count
        .load(Ordering::Relaxed);
    snapshot.mode_misuse_rejection_count =
        counters.mode_misuse_rejection_count.load(Ordering::Relaxed);
    snapshot.absent_mode_store_touch_count = counters
        .absent_mode_store_touch_count
        .load(Ordering::Relaxed);
    snapshot.canonicalization_item_count =
        counters.canonicalization_item_count.load(Ordering::Relaxed);
    snapshot.canonicalization_duplicate_collapse_count = counters
        .canonicalization_duplicate_collapse_count
        .load(Ordering::Relaxed);
}
