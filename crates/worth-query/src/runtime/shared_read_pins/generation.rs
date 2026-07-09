use std::sync::{
    atomic::{AtomicBool, AtomicUsize, Ordering},
    Arc,
};

use crate::memory_workspace::WorthQuerySnapshotIdentity;

use super::WorthQuerySharedReadPinRegistry;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::runtime) struct WorthQuerySharedReadGenerationId {
    ordinal: u64,
    snapshot_identity: WorthQuerySnapshotIdentity,
}

impl WorthQuerySharedReadGenerationId {
    pub(in crate::runtime) fn new(
        ordinal: u64,
        snapshot_identity: WorthQuerySnapshotIdentity,
    ) -> Self {
        Self {
            ordinal,
            snapshot_identity,
        }
    }

    pub(in crate::runtime) fn ordinal(&self) -> u64 {
        self.ordinal
    }

    pub(in crate::runtime) fn snapshot_identity(&self) -> &WorthQuerySnapshotIdentity {
        &self.snapshot_identity
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(in crate::runtime) struct WorthQuerySharedReadPinnedSnapshot {
    generation: WorthQuerySharedReadGenerationId,
}

impl WorthQuerySharedReadPinnedSnapshot {
    pub(in crate::runtime) fn new(generation: WorthQuerySharedReadGenerationId) -> Self {
        Self { generation }
    }

    pub(in crate::runtime) fn generation(&self) -> &WorthQuerySharedReadGenerationId {
        &self.generation
    }
}

#[derive(Debug)]
pub(in crate::runtime) struct WorthQuerySharedReadGenerationEntry {
    snapshot: WorthQuerySharedReadPinnedSnapshot,
    pin_count: AtomicUsize,
    retired: AtomicBool,
    invalidated: AtomicBool,
}

impl WorthQuerySharedReadGenerationEntry {
    pub(in crate::runtime) fn new(snapshot: WorthQuerySharedReadPinnedSnapshot) -> Self {
        Self {
            snapshot,
            pin_count: AtomicUsize::new(0),
            retired: AtomicBool::new(false),
            invalidated: AtomicBool::new(false),
        }
    }

    pub(in crate::runtime) fn snapshot(&self) -> &WorthQuerySharedReadPinnedSnapshot {
        &self.snapshot
    }

    pub(in crate::runtime) fn pin(&self) {
        self.pin_count.fetch_add(1, Ordering::SeqCst);
    }

    pub(in crate::runtime) fn release_pin(&self) -> usize {
        self.pin_count
            .fetch_sub(1, Ordering::SeqCst)
            .saturating_sub(1)
    }

    pub(in crate::runtime) fn pin_count(&self) -> usize {
        self.pin_count.load(Ordering::SeqCst)
    }

    pub(in crate::runtime) fn retire(&self) {
        self.retired.store(true, Ordering::SeqCst);
    }

    #[allow(dead_code)]
    pub(in crate::runtime) fn invalidate(&self) {
        self.invalidated.store(true, Ordering::SeqCst);
        self.retire();
    }

    pub(in crate::runtime) fn is_retired(&self) -> bool {
        self.retired.load(Ordering::SeqCst)
    }

    pub(in crate::runtime) fn is_invalidated(&self) -> bool {
        self.invalidated.load(Ordering::SeqCst)
    }
}

#[derive(Debug)]
pub(in crate::runtime) struct WorthQuerySharedReadGenerationLease {
    registry: WorthQuerySharedReadPinRegistry,
    entry: Arc<WorthQuerySharedReadGenerationEntry>,
}

impl PartialEq for WorthQuerySharedReadGenerationLease {
    fn eq(&self, other: &Self) -> bool {
        self.entry.snapshot() == other.entry.snapshot()
    }
}

impl WorthQuerySharedReadGenerationLease {
    pub(in crate::runtime) fn new(
        registry: WorthQuerySharedReadPinRegistry,
        entry: Arc<WorthQuerySharedReadGenerationEntry>,
    ) -> Self {
        Self { registry, entry }
    }

    pub(in crate::runtime) fn generation(&self) -> &WorthQuerySharedReadGenerationId {
        self.entry.snapshot().generation()
    }

    pub(in crate::runtime) fn is_generation_live(&self) -> bool {
        !self.entry.is_invalidated()
    }
}

impl Clone for WorthQuerySharedReadGenerationLease {
    fn clone(&self) -> Self {
        self.entry.pin();
        Self {
            registry: self.registry.clone(),
            entry: Arc::clone(&self.entry),
        }
    }
}

impl Drop for WorthQuerySharedReadGenerationLease {
    fn drop(&mut self) {
        self.registry.release_generation(Arc::clone(&self.entry));
    }
}
