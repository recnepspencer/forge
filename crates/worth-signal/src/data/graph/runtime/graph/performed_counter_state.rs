use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

use crate::data::proof::invalidation::progression::InvalidationWorkBindingAxes;
use crate::data::telemetry::{InvalidationPerformedCounter, SignalInvalidationRealizedCounters};

#[derive(Debug)]
pub(crate) struct InvalidationPerformedCounterState {
    values: [AtomicU64; 24],
    observation_generation: AtomicU64,
    active_observation_generation: Arc<AtomicU64>,
    executed_work: Mutex<Vec<InvalidationWorkBindingAxes>>,
}

impl Default for InvalidationPerformedCounterState {
    fn default() -> Self {
        Self {
            values: std::array::from_fn(|_| AtomicU64::new(0)),
            observation_generation: AtomicU64::new(0),
            active_observation_generation: Arc::new(AtomicU64::new(0)),
            executed_work: Mutex::new(Vec::new()),
        }
    }
}

impl InvalidationPerformedCounterState {
    pub(crate) fn begin_observation(&self) -> u64 {
        let generation = self
            .observation_generation
            .fetch_add(1, Ordering::Relaxed)
            .saturating_add(1);
        self.reset();
        self.active_observation_generation
            .store(generation, Ordering::Release);
        generation
    }

    pub(crate) fn reset(&self) {
        for value in &self.values {
            value.store(0, Ordering::Relaxed);
        }
        self.executed_work
            .lock()
            .expect("performed work observation poisoned")
            .clear();
    }

    pub(crate) fn observation_generation(&self) -> u64 {
        self.observation_generation.load(Ordering::Relaxed)
    }

    pub(crate) fn add(&self, counter: InvalidationPerformedCounter, amount: u64) {
        self.values[counter.index()].fetch_add(amount, Ordering::Relaxed);
    }

    pub(crate) fn set(&self, counter: InvalidationPerformedCounter, value: u64) {
        self.values[counter.index()].store(value, Ordering::Relaxed);
    }

    pub(crate) fn record_max(&self, counter: InvalidationPerformedCounter, value: u64) {
        self.values[counter.index()].fetch_max(value, Ordering::Relaxed);
    }

    pub(crate) fn snapshot(&self) -> SignalInvalidationRealizedCounters {
        SignalInvalidationRealizedCounters::from_values(std::array::from_fn(|index| {
            self.values[index].load(Ordering::Relaxed)
        }))
    }

    pub(crate) fn record_executed_work(&self, binding: InvalidationWorkBindingAxes) {
        if self.active_observation_generation.load(Ordering::Acquire) == 0 {
            return;
        }
        self.executed_work
            .lock()
            .expect("performed work observation poisoned")
            .push(binding);
    }

    pub(crate) fn executed_work(&self) -> Vec<InvalidationWorkBindingAxes> {
        self.executed_work
            .lock()
            .expect("performed work observation poisoned")
            .clone()
    }

    pub(crate) fn finish_observation(&self, generation: u64) -> bool {
        self.active_observation_generation
            .compare_exchange(generation, 0, Ordering::AcqRel, Ordering::Acquire)
            .is_ok()
    }

    pub(crate) fn observation_liveness(&self) -> Arc<AtomicU64> {
        Arc::clone(&self.active_observation_generation)
    }
}
