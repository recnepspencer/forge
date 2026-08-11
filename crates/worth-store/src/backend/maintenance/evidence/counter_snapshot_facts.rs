use crate::backend::engine::{StateBackedStoreBackend, StatePersistence};

#[derive(Debug, Clone)]
pub(super) struct MaintenanceCounterSnapshotFacts {
    snapshot: crate::StoreCounterSnapshot,
}

impl MaintenanceCounterSnapshotFacts {
    pub(super) fn observe<P: StatePersistence>(backend: &StateBackedStoreBackend<P>) -> Self {
        Self {
            snapshot: backend.counters().snapshot(),
        }
    }

    pub(super) fn snapshot(&self) -> &crate::StoreCounterSnapshot {
        &self.snapshot
    }
}
