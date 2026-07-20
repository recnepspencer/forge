use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

#[derive(Clone, Debug, Default)]
pub(in crate::runtime) struct WorthQueryPublishedArtifactCounters {
    shared_read_mint_row_clone_count: Arc<AtomicUsize>,
    published_artifact_registry_lease_count: Arc<AtomicUsize>,
    reader_derived_evaluation_count: Arc<AtomicUsize>,
    dropped_generation_count: Arc<AtomicUsize>,
}

impl WorthQueryPublishedArtifactCounters {
    pub(in crate::runtime) fn record_registry_lease(&self) {
        self.published_artifact_registry_lease_count
            .fetch_add(1, Ordering::SeqCst);
    }

    pub(in crate::runtime) fn record_dropped_generations(&self, count: usize) {
        self.dropped_generation_count
            .fetch_add(count, Ordering::SeqCst);
    }

    pub(in crate::runtime) fn snapshot(&self) -> WorthQueryPublishedArtifactCounterSnapshot {
        WorthQueryPublishedArtifactCounterSnapshot::new(
            self.shared_read_mint_row_clone_count.load(Ordering::SeqCst),
            self.published_artifact_registry_lease_count
                .load(Ordering::SeqCst),
            self.reader_derived_evaluation_count.load(Ordering::SeqCst),
            self.dropped_generation_count.load(Ordering::SeqCst),
        )
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthQueryPublishedArtifactCounterSnapshot {
    shared_read_mint_row_clone_count: usize,
    published_artifact_registry_lease_count: usize,
    reader_derived_evaluation_count: usize,
    dropped_generation_count: usize,
}

impl WorthQueryPublishedArtifactCounterSnapshot {
    pub(in crate::runtime) fn new(
        shared_read_mint_row_clone_count: usize,
        published_artifact_registry_lease_count: usize,
        reader_derived_evaluation_count: usize,
        dropped_generation_count: usize,
    ) -> Self {
        Self {
            shared_read_mint_row_clone_count,
            published_artifact_registry_lease_count,
            reader_derived_evaluation_count,
            dropped_generation_count,
        }
    }

    pub fn shared_read_mint_row_clone_count(self) -> usize {
        self.shared_read_mint_row_clone_count
    }

    pub fn published_artifact_registry_lease_count(self) -> usize {
        self.published_artifact_registry_lease_count
    }

    pub fn reader_derived_evaluation_count(self) -> usize {
        self.reader_derived_evaluation_count
    }

    pub fn dropped_generation_count(self) -> usize {
        self.dropped_generation_count
    }
}
