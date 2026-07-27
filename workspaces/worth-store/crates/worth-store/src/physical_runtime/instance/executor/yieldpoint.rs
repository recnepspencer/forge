use std::sync::{Arc, Condvar, Mutex};
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

#[derive(Clone)]
pub struct CertificationPhysicalExecutionPauseGate {
    checkpoint: CertificationPhysicalExecutionCheckpoint,
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

pub(super) struct PhysicalExecutorYieldpointOwner {
    gates: [Mutex<Option<CertificationPhysicalExecutionPauseGate>>; 5],
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
            .unwrap_or_else(|poisoned| poisoned.into_inner()) = Some(gate.clone());
        gate
    }

    pub(super) fn pause(&self, checkpoint: CertificationPhysicalExecutionCheckpoint) {
        let gate = self.gates[checkpoint.index()]
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .clone();
        if let Some(gate) = gate {
            gate.arrive_and_wait();
        }
    }
}

impl CertificationPhysicalExecutionPauseGate {
    fn new(checkpoint: CertificationPhysicalExecutionCheckpoint) -> Self {
        Self {
            checkpoint,
            shared: Arc::new(PauseState {
                progress: Mutex::new(PauseProgress {
                    arrivals: 0,
                    released: false,
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

#[cfg(test)]
mod tests {
    use std::sync::mpsc;
    use std::time::Duration;

    use super::{CertificationPhysicalExecutionCheckpoint, PhysicalExecutorYieldpointOwner};

    #[test]
    fn named_executor_checkpoint_blocks_until_framework_gate_releases() {
        let owner = PhysicalExecutorYieldpointOwner::new();
        let checkpoint = CertificationPhysicalExecutionCheckpoint::BeforeBackendDispatch;
        let gate = owner.install(checkpoint);
        let (completed, completion) = mpsc::sync_channel(1);
        let execution = std::thread::spawn(move || {
            owner.pause(checkpoint);
            completed.send(()).unwrap();
        });

        assert!(gate.await_arrival());
        assert!(completion.recv_timeout(Duration::from_millis(20)).is_err());
        gate.release();
        completion.recv_timeout(Duration::from_secs(1)).unwrap();
        execution.join().unwrap();
    }

    #[test]
    fn execution_checkpoints_retain_independent_pause_authority() {
        let owner = PhysicalExecutorYieldpointOwner::new();
        let pre_dispatch =
            owner.install(CertificationPhysicalExecutionCheckpoint::BeforeBackendDispatch);
        let post_read = owner
            .install(CertificationPhysicalExecutionCheckpoint::AfterReadBeforeSchedulerSettlement);
        let post_write = owner.install(
            CertificationPhysicalExecutionCheckpoint::AfterExactWriteBeforeSchedulerSettlement,
        );
        let post_residency_write = owner.install(
            CertificationPhysicalExecutionCheckpoint::AfterResidencyWriteBeforeSchedulerSettlement,
        );
        let post_catalog = owner.install(
            CertificationPhysicalExecutionCheckpoint::
                AfterCatalogReplacementBeforeSchedulerSettlement,
        );

        let execution = std::thread::spawn(move || {
            owner.pause(CertificationPhysicalExecutionCheckpoint::BeforeBackendDispatch);
        });
        assert!(pre_dispatch.await_arrival());
        assert_eq!(post_read.arrival_count(), 0);
        assert_eq!(post_write.arrival_count(), 0);
        assert_eq!(post_residency_write.arrival_count(), 0);
        assert_eq!(post_catalog.arrival_count(), 0);
        pre_dispatch.release();
        execution.join().unwrap();
    }
}
