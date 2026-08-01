use std::cell::Cell;
use std::rc::Rc;

use super::{
    UiObservationResourceRetirementCause, UiObservationResourceRetirementReport,
    UiObservationResourceSnapshot,
};

#[derive(Debug, Default)]
pub(crate) struct UiObservationResourceLedger {
    inner: Rc<UiObservationResourceLedgerInner>,
}

#[derive(Debug, Default)]
struct UiObservationResourceLedgerInner {
    generation: Cell<u128>,
    sets: Cell<usize>,
    observations: Cell<usize>,
    bytes: Cell<usize>,
}

pub(crate) struct UiObservationSetLease {
    inner: Rc<UiObservationResourceLedgerInner>,
    generation: u128,
    observations: usize,
    bytes: usize,
}

impl UiObservationResourceLedger {
    pub(crate) fn retain_set(&self, observations: usize, bytes: usize) -> UiObservationSetLease {
        let next_sets = self
            .inner
            .sets
            .get()
            .checked_add(1)
            .expect("set count fits");
        let next_observations = self
            .inner
            .observations
            .get()
            .checked_add(observations)
            .expect("bounded observation count fits");
        let next_bytes = self
            .inner
            .bytes
            .get()
            .checked_add(bytes)
            .expect("bounded observation bytes fit");
        self.inner.sets.set(next_sets);
        self.inner.observations.set(next_observations);
        self.inner.bytes.set(next_bytes);
        UiObservationSetLease {
            inner: Rc::clone(&self.inner),
            generation: self.inner.generation.get(),
            observations,
            bytes,
        }
    }

    pub(crate) fn snapshot(&self) -> UiObservationResourceSnapshot {
        UiObservationResourceSnapshot::from_retained_sets(
            self.inner.sets.get(),
            self.inner.observations.get(),
            self.inner.bytes.get(),
        )
    }

    pub(crate) fn retire(
        &mut self,
        cause: UiObservationResourceRetirementCause,
    ) -> UiObservationResourceRetirementReport {
        let snapshot = self.snapshot();
        let next_generation = self
            .inner
            .generation
            .get()
            .checked_add(1)
            .expect("observation resource generation cannot exhaust in one process");
        self.inner.generation.set(next_generation);
        self.inner.sets.set(0);
        self.inner.observations.set(0);
        self.inner.bytes.set(0);
        UiObservationResourceRetirementReport::new(cause, snapshot)
    }
}

impl Drop for UiObservationSetLease {
    fn drop(&mut self) {
        if self.generation != self.inner.generation.get() {
            return;
        }
        self.inner
            .sets
            .set(self.inner.sets.get().checked_sub(1).expect("lease is live"));
        self.inner.observations.set(
            self.inner
                .observations
                .get()
                .checked_sub(self.observations)
                .expect("lease observations are retained"),
        );
        self.inner.bytes.set(
            self.inner
                .bytes
                .get()
                .checked_sub(self.bytes)
                .expect("lease bytes are retained"),
        );
    }
}
