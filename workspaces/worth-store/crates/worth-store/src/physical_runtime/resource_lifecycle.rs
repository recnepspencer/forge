use std::sync::Arc;

use super::diagnostics::RuntimeCounterCells;

pub(crate) struct ResourceLifecycle {
    counters: Arc<RuntimeCounterCells>,
}

impl ResourceLifecycle {
    pub(crate) fn new(counters: Arc<RuntimeCounterCells>) -> Self {
        Self { counters }
    }

    pub(crate) fn acquire_observation(&self) -> ObservationLease {
        ObservationLease::acquire(Arc::clone(&self.counters))
    }
}

pub(crate) struct ObservationLease {
    counters: Arc<RuntimeCounterCells>,
}

impl ObservationLease {
    fn acquire(counters: Arc<RuntimeCounterCells>) -> Self {
        counters.acquire_observation();
        Self { counters }
    }

    pub(crate) fn counters(&self) -> &RuntimeCounterCells {
        &self.counters
    }
}

impl Clone for ObservationLease {
    fn clone(&self) -> Self {
        Self::acquire(Arc::clone(&self.counters))
    }
}

impl Drop for ObservationLease {
    fn drop(&mut self) {
        self.counters.release_observation();
    }
}
