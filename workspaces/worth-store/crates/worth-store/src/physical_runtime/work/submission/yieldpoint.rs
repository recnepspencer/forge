use std::{
    sync::{Arc, Condvar, Mutex},
    time::{Duration, Instant},
};

#[derive(Clone)]
pub struct CertificationPhysicalSubmissionPauseGate {
    shared: Arc<PauseState>,
}

struct PauseState {
    state: Mutex<PauseProgress>,
    changed: Condvar,
}

struct PauseProgress {
    arrivals: usize,
    released: bool,
}

impl CertificationPhysicalSubmissionPauseGate {
    pub(in crate::physical_runtime::work) fn new() -> Self {
        Self {
            shared: Arc::new(PauseState {
                state: Mutex::new(PauseProgress {
                    arrivals: 0,
                    released: false,
                }),
                changed: Condvar::new(),
            }),
        }
    }

    pub fn await_arrivals(&self, expected: usize) -> bool {
        let deadline = Instant::now() + Duration::from_secs(5);
        let mut progress = self
            .shared
            .state
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        while progress.arrivals < expected {
            let Some(remaining) = deadline.checked_duration_since(Instant::now()) else {
                return false;
            };
            let (next, wait) = self
                .shared
                .changed
                .wait_timeout(progress, remaining)
                .unwrap_or_else(|poisoned| poisoned.into_inner());
            progress = next;
            if wait.timed_out() && progress.arrivals < expected {
                return false;
            }
        }
        true
    }

    pub fn release(&self) {
        let mut progress = self
            .shared
            .state
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        progress.released = true;
        self.shared.changed.notify_all();
    }

    pub(in crate::physical_runtime::work) fn arrive_and_wait(&self) {
        let mut progress = self
            .shared
            .state
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        progress.arrivals += 1;
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
