use std::sync::{Arc, Condvar, Mutex, Weak};
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CertificationPhysicalExecutionCheckpoint {
    BeforeBackendDispatch,
    AfterReadBeforeSchedulerSettlement,
    AfterExactWriteBeforeSchedulerSettlement,
    AfterResidencyWriteBeforeSchedulerSettlement,
    AfterCatalogReplacementBeforeSchedulerSettlement,
}

impl CertificationPhysicalExecutionCheckpoint {
    const fn index(self) -> usize {
        match self {
            Self::BeforeBackendDispatch => 0,
            Self::AfterReadBeforeSchedulerSettlement => 1,
            Self::AfterExactWriteBeforeSchedulerSettlement => 2,
            Self::AfterResidencyWriteBeforeSchedulerSettlement => 3,
            Self::AfterCatalogReplacementBeforeSchedulerSettlement => 4,
        }
    }
}

pub struct CertificationPhysicalExecutionPauseGate {
    checkpoint: CertificationPhysicalExecutionCheckpoint,
    shared: Arc<PauseState>,
}

pub struct CertificationPhysicalExecutionRelease {
    arrival_index: usize,
    shared: Arc<PauseState>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CertificationPhysicalExecutionSelectionFailure {
    ArrivalUnavailable,
    ResumptionTimedOut,
}

impl CertificationPhysicalExecutionRelease {
    pub const fn arrival_index(&self) -> usize {
        self.arrival_index
    }

    pub fn await_resumption(&self) -> bool {
        let deadline = Instant::now() + Duration::from_secs(5);
        let mut progress = self
            .shared
            .progress
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        while progress.arrivals.get(self.arrival_index) != Some(&ArrivalProgress::Resumed) {
            let Some(remaining) = deadline.checked_duration_since(Instant::now()) else {
                return false;
            };
            let (next, wait) = self
                .shared
                .changed
                .wait_timeout(progress, remaining)
                .unwrap_or_else(|poisoned| poisoned.into_inner());
            progress = next;
            if wait.timed_out()
                && progress.arrivals.get(self.arrival_index) != Some(&ArrivalProgress::Resumed)
            {
                return false;
            }
        }
        true
    }
}

struct PauseState {
    progress: Mutex<PauseProgress>,
    changed: Condvar,
}

struct PauseProgress {
    arrivals: Vec<ArrivalProgress>,
    all_released: bool,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ArrivalProgress {
    Paused,
    Released,
    Resumed,
}

pub(super) struct PhysicalExecutorYieldpointOwner {
    gates: [Mutex<Option<Weak<PauseState>>>; 5],
}

impl PhysicalExecutorYieldpointOwner {
    pub(super) fn new() -> Self {
        Self {
            gates: [
                Mutex::new(None),
                Mutex::new(None),
                Mutex::new(None),
                Mutex::new(None),
                Mutex::new(None),
            ],
        }
    }

    pub(super) fn install(
        &self,
        checkpoint: CertificationPhysicalExecutionCheckpoint,
    ) -> CertificationPhysicalExecutionPauseGate {
        let gate = CertificationPhysicalExecutionPauseGate::new(checkpoint);
        *self.gates[checkpoint.index()]
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner()) = Some(Arc::downgrade(&gate.shared));
        gate
    }

    pub(super) fn pause(&self, checkpoint: CertificationPhysicalExecutionCheckpoint) {
        let shared = self.gates[checkpoint.index()]
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .as_ref()
            .and_then(Weak::upgrade);
        if let Some(shared) = shared {
            shared.arrive_and_wait();
        }
    }
}

impl CertificationPhysicalExecutionPauseGate {
    fn new(checkpoint: CertificationPhysicalExecutionCheckpoint) -> Self {
        Self {
            checkpoint,
            shared: Arc::new(PauseState {
                progress: Mutex::new(PauseProgress {
                    arrivals: Vec::new(),
                    all_released: false,
                }),
                changed: Condvar::new(),
            }),
        }
    }

    pub const fn checkpoint(&self) -> CertificationPhysicalExecutionCheckpoint {
        self.checkpoint
    }

    pub fn arrival_count(&self) -> usize {
        self.shared
            .progress
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .arrivals
            .len()
    }

    pub fn await_arrival(&self) -> bool {
        self.await_arrivals(1)
    }

    pub fn await_arrivals(&self, expected: usize) -> bool {
        if expected == 0 {
            return true;
        }
        let deadline = Instant::now() + Duration::from_secs(5);
        let mut progress = self
            .shared
            .progress
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        while progress.arrivals.len() < expected {
            let Some(remaining) = deadline.checked_duration_since(Instant::now()) else {
                return false;
            };
            let (next, wait) = self
                .shared
                .changed
                .wait_timeout(progress, remaining)
                .unwrap_or_else(|poisoned| poisoned.into_inner());
            progress = next;
            if wait.timed_out() && progress.arrivals.len() < expected {
                return false;
            }
        }
        true
    }

    pub fn release_arrival(
        &self,
        arrival_index: usize,
    ) -> Option<CertificationPhysicalExecutionRelease> {
        let mut progress = self
            .shared
            .progress
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        if progress.all_released
            || progress.arrivals.get(arrival_index).copied() != Some(ArrivalProgress::Paused)
        {
            return None;
        }
        progress.arrivals[arrival_index] = ArrivalProgress::Released;
        self.shared.changed.notify_all();
        Some(CertificationPhysicalExecutionRelease {
            arrival_index,
            shared: Arc::clone(&self.shared),
        })
    }

    pub fn select_arrival_then_release_downstream(
        &self,
        arrival_index: usize,
    ) -> Result<CertificationPhysicalExecutionRelease, CertificationPhysicalExecutionSelectionFailure>
    {
        let release = self
            .release_arrival(arrival_index)
            .ok_or(CertificationPhysicalExecutionSelectionFailure::ArrivalUnavailable)?;
        if !release.await_resumption() {
            self.release();
            return Err(CertificationPhysicalExecutionSelectionFailure::ResumptionTimedOut);
        }
        self.release();
        Ok(release)
    }

    pub fn release(&self) {
        let mut progress = self
            .shared
            .progress
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        progress.all_released = true;
        self.shared.changed.notify_all();
    }
}

impl Drop for CertificationPhysicalExecutionPauseGate {
    fn drop(&mut self) {
        self.release();
    }
}

impl PauseState {
    fn arrive_and_wait(&self) {
        let mut progress = self
            .progress
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let arrival_index = progress.arrivals.len();
        progress.arrivals.push(ArrivalProgress::Paused);
        self.changed.notify_all();
        while !progress.all_released && progress.arrivals[arrival_index] == ArrivalProgress::Paused
        {
            progress = self
                .changed
                .wait(progress)
                .unwrap_or_else(|poisoned| poisoned.into_inner());
        }
        progress.arrivals[arrival_index] = ArrivalProgress::Resumed;
        self.changed.notify_all();
    }
}

#[cfg(test)]
mod tests;
