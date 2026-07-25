use std::{
    sync::{Arc, Condvar, Mutex},
    time::{Duration, Instant},
};

pub struct CertificationPhysicalSignalPauseGate {
    shared: Arc<PauseState>,
}

struct PauseState {
    progress: Mutex<PauseProgress>,
    changed: Condvar,
}

struct PauseProgress {
    arrivals: usize,
    released: bool,
}

impl CertificationPhysicalSignalPauseGate {
    pub(super) fn new() -> Self {
        Self {
            shared: Arc::new(PauseState {
                progress: Mutex::new(PauseProgress {
                    arrivals: 0,
                    released: false,
                }),
                changed: Condvar::new(),
            }),
        }
    }

    pub(super) fn worker_handle(&self) -> Self {
        Self {
            shared: Arc::clone(&self.shared),
        }
    }

    pub fn await_arrivals(&self, expected: usize) -> bool {
        let deadline = Instant::now() + Duration::from_secs(5);
        let mut progress = self
            .shared
            .progress
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
            .progress
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        progress.released = true;
        self.shared.changed.notify_all();
    }

    pub(super) fn arrive_and_wait(&self) {
        let mut progress = self
            .shared
            .progress
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        if progress.released {
            return;
        }
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

impl Drop for CertificationPhysicalSignalPauseGate {
    fn drop(&mut self) {
        self.release();
    }
}

#[cfg(test)]
mod tests {
    use std::{sync::mpsc, time::Duration};

    use super::CertificationPhysicalSignalPauseGate;

    #[test]
    fn dropping_the_caller_gate_releases_an_arrived_worker() {
        let gate = CertificationPhysicalSignalPauseGate::new();
        let worker_gate = gate.worker_handle();
        let (finished, observed) = mpsc::sync_channel(0);
        let worker = std::thread::spawn(move || {
            worker_gate.arrive_and_wait();
            let _ = finished.send(());
        });

        assert!(gate.await_arrivals(1));
        drop(gate);
        assert!(observed.recv_timeout(Duration::from_secs(1)).is_ok());
        worker.join().unwrap();
    }
}
