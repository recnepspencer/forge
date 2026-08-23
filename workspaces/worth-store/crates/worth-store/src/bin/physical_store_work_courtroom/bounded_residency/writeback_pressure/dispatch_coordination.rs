use std::{
    sync::mpsc,
    time::{Duration, Instant},
};

use worth_store::physical_runtime::production::PhysicalMutationPauseGate;
use worth_store::physical_runtime::{
    DataDispatchedPhysicalMutation, PhysicalDataDispatchOutcome, PhysicalRecordPressureEvidence,
    ServingPhysicalRuntime, WalDurablePhysicalMutation,
};
use worth_store_physical_backend::{MediaOperationContext, MediaPauseGate};

type DispatchResult = PhysicalDataDispatchOutcome;

const PRESSURE_DENIAL_TIMEOUT: Duration = Duration::from_secs(5);

pub(super) struct PressureGates {
    media: MediaPauseGate,
    mutation: PhysicalMutationPauseGate,
}

impl PressureGates {
    pub(super) fn new(media: MediaPauseGate, mutation: PhysicalMutationPauseGate) -> Self {
        Self { media, mutation }
    }

    pub(super) fn await_backend_gate(
        &self,
        receiver: &mpsc::Receiver<DispatchResult>,
    ) -> Result<MediaOperationContext, String> {
        let deadline = Instant::now() + Duration::from_secs(5);
        while self.media.reached_context().is_none() && Instant::now() < deadline {
            match receiver.try_recv() {
                Ok(_) => return Err("primary mutation settled before backend dispatch".to_owned()),
                Err(mpsc::TryRecvError::Disconnected) => {
                    return Err("primary canonical mutation disconnected before dispatch".to_owned())
                }
                Err(mpsc::TryRecvError::Empty) => std::thread::yield_now(),
            }
        }
        self.media
            .reached_context()
            .ok_or_else(|| "primary canonical mutation did not reach its backend gate".to_owned())
    }

    pub(super) fn await_mutation_gate(&self) -> Result<(), String> {
        self.mutation.await_arrival().then_some(()).ok_or_else(|| {
            "primary canonical mutation did not reach post-admission checkpoint".to_owned()
        })
    }

    pub(super) fn release_media(&self) {
        self.media.release();
    }

    pub(super) fn release_mutation(&self) {
        self.mutation.release();
    }

    pub(super) fn release_all(&self) {
        self.mutation.release();
        self.media.release();
    }
}

impl Drop for PressureGates {
    fn drop(&mut self) {
        self.release_all();
    }
}

pub(super) struct PressureDeniedDispatch {
    pub(super) durable: WalDurablePhysicalMutation,
    pub(super) evidence: PhysicalRecordPressureEvidence,
}

pub(super) fn spawn(
    serving: &ServingPhysicalRuntime,
    durable: WalDurablePhysicalMutation,
) -> (std::thread::JoinHandle<()>, mpsc::Receiver<DispatchResult>) {
    let submission = serving.certification_record_submission();
    let (sender, receiver) = mpsc::sync_channel(1);
    let thread = std::thread::spawn(move || {
        let _ = sender.send(submission.dispatch_wal_durable_data(durable));
    });
    (thread, receiver)
}

pub(super) fn receive_pressure_denial(
    gates: &PressureGates,
    receiver: mpsc::Receiver<DispatchResult>,
    thread: std::thread::JoinHandle<()>,
) -> Result<PressureDeniedDispatch, String> {
    let result = match receiver.recv_timeout(PRESSURE_DENIAL_TIMEOUT) {
        Ok(result) => result,
        Err(_) => {
            gates.release_all();
            let _ = thread.join();
            return Err("competing canonical mutation did not deny before effects".to_owned());
        }
    };
    thread
        .join()
        .map_err(|_| "competing canonical mutation panicked".to_owned())?;
    match result {
        PhysicalDataDispatchOutcome::Dispatched(_) => {
            gates.release_all();
            Err("competing canonical mutation bypassed pressure".to_owned())
        }
        PhysicalDataDispatchOutcome::RetryableAfterCleanup(retry) => {
            if retry.discarded_effects().is_empty() || retry.deleted_artifacts().is_empty() {
                return Err("canonical pressure retry omitted cleanup evidence".to_owned());
            }
            let evidence = retry.pressure();
            Ok(PressureDeniedDispatch {
                durable: retry.into_durable(),
                evidence,
            })
        }
        PhysicalDataDispatchOutcome::NotStarted { cause, .. } => Err(format!(
            "competing canonical mutation denied without pressure evidence: {cause:?}"
        )),
        PhysicalDataDispatchOutcome::Indeterminate(indeterminate) => Err(format!(
            "competing canonical mutation became indeterminate: {:?}",
            indeterminate.cause()
        )),
    }
}

pub(super) fn receive_dispatched(
    receiver: mpsc::Receiver<DispatchResult>,
    thread: std::thread::JoinHandle<()>,
    label: &str,
) -> Result<DataDispatchedPhysicalMutation, String> {
    let result = receiver
        .recv_timeout(Duration::from_secs(30))
        .map_err(|_| format!("{label} did not settle after gate release"))?;
    thread.join().map_err(|_| format!("{label} panicked"))?;
    match result {
        PhysicalDataDispatchOutcome::Dispatched(dispatched) => Ok(dispatched),
        PhysicalDataDispatchOutcome::RetryableAfterCleanup(_) => {
            Err(format!("{label} unexpectedly required cleanup retry"))
        }
        PhysicalDataDispatchOutcome::NotStarted { cause, .. } => {
            Err(format!("{label} did not start: {cause:?}"))
        }
        PhysicalDataDispatchOutcome::Indeterminate(indeterminate) => Err(format!(
            "{label} became indeterminate: {:?}",
            indeterminate.cause()
        )),
    }
}
