use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::{Arc, Mutex, Weak};
use std::thread::JoinHandle;

use worth_proof::TransitionOutcome;

use super::capture::{
    PhysicalCheckpointCaptureFoundation, PhysicalCheckpointCaptureOwner,
    PhysicalCheckpointExecutionResult,
};
use super::{
    PhysicalCheckpointAttempt, PhysicalCheckpointCaptureFailure,
    PhysicalCheckpointCaptureFailureKind, PhysicalCheckpointHandle, PhysicalCheckpointRequest,
};
use crate::physical_runtime::{
    PhysicalCheckpointStartDeferred, PhysicalCheckpointStartDenial, PhysicalCheckpointStartFailure,
    PhysicalCheckpointStartOutcome, PhysicalCheckpointStartStale,
};

pub(in crate::physical_runtime) struct PhysicalCheckpointRuntimeOwner {
    capture: PhysicalCheckpointCaptureOwner,
    work_runtime: Weak<crate::physical_runtime::instance::PhysicalStoreWorkRuntime>,
    lifecycle: Mutex<PhysicalCheckpointLifecycleState>,
}

#[derive(Clone)]
pub struct PhysicalCheckpointSubmission {
    owner: Weak<PhysicalCheckpointRuntimeOwner>,
}

struct PhysicalCheckpointLifecycleState {
    accepting: bool,
    current: Option<Arc<PhysicalCheckpointAttempt>>,
    current_terminal: bool,
    worker: Option<JoinHandle<()>>,
    latest_publication: Option<super::PhysicalCheckpointPublication>,
    started: u64,
    completed: u64,
    proven_no_effect: u64,
    indeterminate: u64,
    worker_panics: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalCheckpointShutdown {
    started: u64,
    completed: u64,
    proven_no_effect: u64,
    indeterminate: u64,
    worker_panics: u64,
    latest_publication: Option<worth_store_physical_format::PhysicalCheckpointIdentity>,
}

impl PhysicalCheckpointRuntimeOwner {
    pub(in crate::physical_runtime) fn new(
        foundation: PhysicalCheckpointCaptureFoundation,
        work_runtime: &Arc<crate::physical_runtime::instance::PhysicalStoreWorkRuntime>,
    ) -> Arc<Self> {
        Arc::new(Self {
            capture: PhysicalCheckpointCaptureOwner::new(foundation),
            work_runtime: Arc::downgrade(work_runtime),
            lifecycle: Mutex::new(PhysicalCheckpointLifecycleState {
                accepting: true,
                current: None,
                current_terminal: false,
                worker: None,
                latest_publication: None,
                started: 0,
                completed: 0,
                proven_no_effect: 0,
                indeterminate: 0,
                worker_panics: 0,
            }),
        })
    }

    pub(in crate::physical_runtime) fn submission(
        owner: &Arc<Self>,
    ) -> PhysicalCheckpointSubmission {
        PhysicalCheckpointSubmission {
            owner: Arc::downgrade(owner),
        }
    }

    fn start(
        self: &Arc<Self>,
        request: PhysicalCheckpointRequest,
    ) -> PhysicalCheckpointStartOutcome {
        loop {
            let mut lifecycle = self.state();
            if !lifecycle.accepting {
                return TransitionOutcome::stale(PhysicalCheckpointStartStale::RuntimeClosing)
                    .into();
            }
            if let Some(current) = lifecycle.current.as_ref() {
                if current.idempotency_key() == request.idempotency_key() {
                    return TransitionOutcome::success(PhysicalCheckpointHandle::new(current))
                        .into();
                }
                if !lifecycle.current_terminal {
                    return TransitionOutcome::deferred(
                        PhysicalCheckpointStartDeferred::CaptureAlreadyActive,
                    )
                    .into();
                }
            }
            if let Some(worker) = lifecycle.worker.take() {
                lifecycle.current = None;
                lifecycle.current_terminal = false;
                drop(lifecycle);
                let _ = worker.join();
                continue;
            }

            let Some(work_runtime) = self.work_runtime.upgrade() else {
                return TransitionOutcome::stale(PhysicalCheckpointStartStale::WorkOwnerReleased)
                    .into();
            };
            let current_tick = match work_runtime.signal.clock_observation() {
                Ok(clock) => clock.current_tick(),
                Err(_) => {
                    return TransitionOutcome::stale(
                        PhysicalCheckpointStartStale::WorkOwnerReleased,
                    )
                    .into()
                }
            };
            if request.deadline().signal_deadline().get() <= current_tick {
                return TransitionOutcome::denied(PhysicalCheckpointStartDenial::DeadlineElapsed)
                    .into();
            }

            let admitted = match self.capture.admit() {
                Ok(admitted) => admitted,
                Err(failure) => return start_admission_failure(failure),
            };
            let attempt = PhysicalCheckpointAttempt::new(
                request.idempotency_key(),
                request.deadline(),
                admitted.basis().source(),
            );
            lifecycle.current = Some(Arc::clone(&attempt));
            lifecycle.current_terminal = false;
            let weak_owner = Arc::downgrade(self);
            let worker_attempt = Arc::clone(&attempt);
            let worker = match std::thread::Builder::new()
                .name(format!(
                    "worth-checkpoint-{}",
                    admitted.basis().identity().sequence().get()
                ))
                .spawn(move || run_worker(weak_owner, worker_attempt, admitted))
            {
                Ok(worker) => worker,
                Err(_) => {
                    lifecycle.current = None;
                    return TransitionOutcome::failed(
                        PhysicalCheckpointStartFailure::WorkerSpawnFailed,
                    )
                    .into();
                }
            };
            lifecycle.started = lifecycle.started.saturating_add(1);
            lifecycle.worker = Some(worker);
            return TransitionOutcome::success(PhysicalCheckpointHandle::new(&attempt)).into();
        }
    }

    pub(in crate::physical_runtime) fn stop_and_drain(
        self: Arc<Self>,
    ) -> PhysicalCheckpointShutdown {
        let (attempt, worker) = {
            let mut lifecycle = self.state();
            lifecycle.accepting = false;
            (lifecycle.current.clone(), lifecycle.worker.take())
        };
        if let Some(attempt) = attempt {
            attempt.mark_runtime_closing();
        }
        if let Some(worker) = worker {
            let _ = worker.join();
        }
        let lifecycle = self.state();
        let latest_publication = lifecycle.latest_publication.as_ref().map(|publication| {
            assert_eq!(
                publication.namespace_sync().action(),
                crate::physical_runtime::PhysicalCheckpointRecoveryAction::SynchronizeNamespace,
                "retained checkpoint publication must carry namespace-sync completion"
            );
            publication.basis().identity()
        });
        PhysicalCheckpointShutdown {
            started: lifecycle.started,
            completed: lifecycle.completed,
            proven_no_effect: lifecycle.proven_no_effect,
            indeterminate: lifecycle.indeterminate,
            worker_panics: lifecycle.worker_panics,
            latest_publication,
        }
    }

    fn record_result(&self, result: PhysicalCheckpointExecutionResult, panicked: bool) {
        let terminal = result.terminal();
        let mut lifecycle = self.state();
        lifecycle.current_terminal = true;
        if let Some(publication) = result.into_publication() {
            lifecycle.latest_publication = Some(publication);
        }
        match terminal {
            super::PhysicalCheckpointOutcome::Completed(_) => {
                lifecycle.completed = lifecycle.completed.saturating_add(1)
            }
            super::PhysicalCheckpointOutcome::ProvenNoEffect(_) => {
                lifecycle.proven_no_effect = lifecycle.proven_no_effect.saturating_add(1)
            }
            super::PhysicalCheckpointOutcome::Indeterminate(_) => {
                lifecycle.indeterminate = lifecycle.indeterminate.saturating_add(1)
            }
        }
        if panicked {
            lifecycle.worker_panics = lifecycle.worker_panics.saturating_add(1);
        }
    }

    fn state(&self) -> std::sync::MutexGuard<'_, PhysicalCheckpointLifecycleState> {
        self.lifecycle
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }
}

impl PhysicalCheckpointSubmission {
    pub fn start(&self, request: PhysicalCheckpointRequest) -> PhysicalCheckpointStartOutcome {
        let Some(owner) = self.owner.upgrade() else {
            return TransitionOutcome::stale(PhysicalCheckpointStartStale::RuntimeClosing).into();
        };
        owner.start(request)
    }
}

impl PhysicalCheckpointShutdown {
    pub const fn started(self) -> u64 {
        self.started
    }

    pub const fn completed(self) -> u64 {
        self.completed
    }

    pub const fn proven_no_effect(self) -> u64 {
        self.proven_no_effect
    }

    pub const fn indeterminate(self) -> u64 {
        self.indeterminate
    }

    pub const fn worker_panics(self) -> u64 {
        self.worker_panics
    }

    pub const fn latest_publication(
        self,
    ) -> Option<worth_store_physical_format::PhysicalCheckpointIdentity> {
        self.latest_publication
    }

    pub const fn requires_inspection(self) -> bool {
        self.indeterminate != 0 || self.worker_panics != 0
    }
}

fn run_worker(
    owner: Weak<PhysicalCheckpointRuntimeOwner>,
    attempt: Arc<PhysicalCheckpointAttempt>,
    admitted: super::capture::AdmittedPhysicalCheckpointCapture,
) {
    let Some(owner) = owner.upgrade() else {
        attempt.complete(super::PhysicalCheckpointOutcome::Indeterminate(
            super::IndeterminatePhysicalCheckpoint::new(
                attempt.identity(),
                attempt.idempotency_key(),
                PhysicalCheckpointCaptureFailureKind::RuntimeUnavailable,
            ),
        ));
        return;
    };
    let execution = catch_unwind(AssertUnwindSafe(|| {
        owner.capture.execute(admitted, &attempt)
    }));
    let (result, panicked) = match execution {
        Ok(result) => (result, false),
        Err(_) => (
            PhysicalCheckpointExecutionResult::indeterminate(
                attempt.identity(),
                attempt.idempotency_key(),
                PhysicalCheckpointCaptureFailureKind::WorkerPanicked,
            ),
            true,
        ),
    };
    let terminal = result.terminal();
    owner.record_result(result, panicked);
    attempt.complete(terminal);
}

fn start_admission_failure(
    failure: PhysicalCheckpointCaptureFailure,
) -> PhysicalCheckpointStartOutcome {
    match failure.kind() {
        PhysicalCheckpointCaptureFailureKind::NoDurableWalSource => {
            TransitionOutcome::denied(PhysicalCheckpointStartDenial::NoDurableWalSource).into()
        }
        PhysicalCheckpointCaptureFailureKind::ResidencyUnavailable => {
            TransitionOutcome::deferred(PhysicalCheckpointStartDeferred::ResidencyUnavailable)
                .into()
        }
        PhysicalCheckpointCaptureFailureKind::RuntimeUnavailable => {
            TransitionOutcome::stale(PhysicalCheckpointStartStale::WorkOwnerReleased).into()
        }
        kind => TransitionOutcome::failed(PhysicalCheckpointStartFailure::Capture(kind)).into(),
    }
}
