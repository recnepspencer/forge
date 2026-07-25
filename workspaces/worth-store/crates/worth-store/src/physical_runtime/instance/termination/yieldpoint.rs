use std::sync::{Arc, Condvar, Mutex};
use std::time::{Duration, Instant};

use super::PhysicalStoreClosePhase;

#[derive(Clone)]
pub struct CertificationPhysicalClosePauseGate {
    phase: PhysicalStoreClosePhase,
    shared: Arc<ClosePauseState>,
}

struct ClosePauseState {
    progress: Mutex<ClosePauseProgress>,
    changed: Condvar,
}

struct ClosePauseProgress {
    arrivals: usize,
    released: bool,
}

pub(super) struct PhysicalStoreCloseYieldpointOwner {
    gate: Mutex<Option<CertificationPhysicalClosePauseGate>>,
}

impl PhysicalStoreCloseYieldpointOwner {
    pub(super) fn new() -> Self {
        Self {
            gate: Mutex::new(None),
        }
    }

    pub(super) fn install(
        &self,
        phase: PhysicalStoreClosePhase,
    ) -> CertificationPhysicalClosePauseGate {
        let gate = CertificationPhysicalClosePauseGate::new(phase);
        *self
            .gate
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner()) = Some(gate.clone());
        gate
    }

    pub(super) fn pause(&self, phase: PhysicalStoreClosePhase) {
        let gate = self
            .gate
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .clone();
        if let Some(gate) = gate.filter(|gate| gate.phase() == phase) {
            gate.arrive_and_wait();
        }
    }
}

impl CertificationPhysicalClosePauseGate {
    fn new(phase: PhysicalStoreClosePhase) -> Self {
        Self {
            phase,
            shared: Arc::new(ClosePauseState {
                progress: Mutex::new(ClosePauseProgress {
                    arrivals: 0,
                    released: false,
                }),
                changed: Condvar::new(),
            }),
        }
    }

    pub const fn phase(&self) -> PhysicalStoreClosePhase {
        self.phase
    }

    pub fn await_arrival(&self) -> bool {
        let deadline = Instant::now() + Duration::from_secs(5);
        let mut progress = self
            .shared
            .progress
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        while progress.arrivals == 0 {
            let Some(remaining) = deadline.checked_duration_since(Instant::now()) else {
                return false;
            };
            let (next, wait) = self
                .shared
                .changed
                .wait_timeout(progress, remaining)
                .unwrap_or_else(|poisoned| poisoned.into_inner());
            progress = next;
            if wait.timed_out() && progress.arrivals == 0 {
                return false;
            }
        }
        true
    }

    pub fn release(&self) {
        let mut progress = self
            .shared
            .progress
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        progress.released = true;
        self.shared.changed.notify_all();
    }

    fn arrive_and_wait(&self) {
        let mut progress = self
            .shared
            .progress
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        progress.arrivals = progress.arrivals.saturating_add(1);
        self.shared.changed.notify_all();
        while !progress.released {
            progress = self
                .shared
                .changed
                .wait(progress)
                .unwrap_or_else(|poisoned| poisoned.into_inner());
        }
    }
}
