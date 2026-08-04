use std::{
    sync::mpsc,
    time::{Duration, Instant},
};

use worth_store::physical_runtime::{
    DataDispatchedPhysicalMutation, PhysicalDataDispatchOutcome, PhysicalRecordPressureEvidence,
    ServingPhysicalRuntime, WalDurablePhysicalMutation,
};
use worth_store_physical_backend::{MediaOperationContext, MediaPauseGate};

type DispatchResult = PhysicalDataDispatchOutcome;

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

pub(super) fn await_backend_gate(
    gate: &MediaPauseGate,
    receiver: &mpsc::Receiver<DispatchResult>,
) -> Result<MediaOperationContext, String> {
    let deadline = Instant::now() + Duration::from_secs(5);
    while gate.reached_context().is_none() && Instant::now() < deadline {
        match receiver.try_recv() {
            Ok(_) => return Err("primary mutation settled before backend dispatch".to_owned()),
            Err(mpsc::TryRecvError::Disconnected) => {
                return Err("primary canonical mutation disconnected before dispatch".to_owned())
            }
            Err(mpsc::TryRecvError::Empty) => std::thread::yield_now(),
        }
    }
    gate.reached_context()
        .ok_or_else(|| "primary canonical mutation did not reach its backend gate".to_owned())
}

pub(super) fn receive_pressure_denial(
    gate: &MediaPauseGate,
    receiver: mpsc::Receiver<DispatchResult>,
    thread: std::thread::JoinHandle<()>,
) -> Result<PressureDeniedDispatch, String> {
    let result = match receiver.recv_timeout(Duration::from_secs(5)) {
        Ok(result) => result,
        Err(_) => {
            gate.release();
            return Err("competing canonical mutation did not deny before effects".to_owned());
        }
    };
    thread
        .join()
        .map_err(|_| "competing canonical mutation panicked".to_owned())?;
    match result {
        PhysicalDataDispatchOutcome::Dispatched(_) => {
            gate.release();
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
