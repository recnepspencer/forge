use worth_store::physical_runtime::{
    certification::CertificationPhysicalExecutionCheckpoint, PhysicalPrefetchIntent,
    PhysicalPrefetchOutcome, PhysicalResidencyCertification, PhysicalSpeculativeWorkKind,
    PhysicalWorkIdentity, PhysicalWorkSignalFamily, ServingPhysicalRuntime,
};
use worth_store_physical_format::RecordFrameCoordinate;

use super::{
    await_arrivals, require_evidence, require_pressure, ExpectedReadEvidence, FRAME_READ_BASIS,
};
use crate::bounded_residency::schedule::ExecutedPrefetchSchedule;
use crate::bounded_residency::schedule::{IndependentReadyWorkSelection, WorkerStartOrder};
use crate::bounded_residency::speculative_pressure::{
    causal_binding, counter_evidence, signal_requests, SpeculativeKindEvidence,
    SpeculativePathEvidence,
};

pub(in crate::bounded_residency::speculative_pressure) struct PrefetchProof {
    pub(in crate::bounded_residency::speculative_pressure) evidence: SpeculativeKindEvidence,
    pub(in crate::bounded_residency::speculative_pressure) schedule: ExecutedPrefetchSchedule,
}

pub(in crate::bounded_residency::speculative_pressure) fn prove(
    serving: &ServingPhysicalRuntime,
    residency: &PhysicalResidencyCertification,
    coordinates: &[RecordFrameCoordinate; 8],
    worker_order: WorkerStartOrder,
    ready_work_selection: IndependentReadyWorkSelection,
) -> Result<PrefetchProof, String> {
    let kind = PhysicalSpeculativeWorkKind::Prefetch;
    let counters_before = residency.counters();
    let signals_before = signal_requests(serving)?;
    let mut work = Vec::with_capacity(3);
    work.push(require_loaded(
        residency.prefetch(PhysicalPrefetchIntent::new(coordinates[0])),
    )?);
    let hit_signals_before = signal_requests(serving)?;
    match residency.prefetch(PhysicalPrefetchIntent::new(coordinates[0])) {
        PhysicalPrefetchOutcome::Hit { coordinate } if coordinate == coordinates[0] => {}
        outcome => return Err(format!("bounded prefetch hot path drifted: {outcome:?}")),
    }
    let hit_signal_requests = signal_requests(serving)?.saturating_sub(hit_signals_before);

    let workers = start_workers(
        serving,
        residency,
        coordinates[1],
        coordinates[2],
        worker_order,
    )?;
    let denial_signals_before = signal_requests(serving)?;
    let dropped = match residency.prefetch(PhysicalPrefetchIntent::new(coordinates[3])) {
        PhysicalPrefetchOutcome::Dropped(dropped) => dropped,
        outcome => {
            return Err(format!(
                "bounded prefetch one-past was not denied: {outcome:?}"
            ));
        }
    };
    require_pressure(dropped.pressure(), kind, 1, 2, 2, "bounded prefetch")?;
    let denial_signal_requests = signal_requests(serving)?.saturating_sub(denial_signals_before);
    work.extend(workers.complete(ready_work_selection)?);
    let effectful_signal_requests = signal_requests(serving)?.saturating_sub(signals_before);
    for identity in work {
        causal_binding::require_exact(
            serving,
            identity,
            PhysicalWorkSignalFamily::ReadFault,
            FRAME_READ_BASIS,
        )?;
    }
    let evidence = counter_evidence(
        kind,
        counters_before,
        residency.counters(),
        SpeculativePathEvidence {
            hits: 1,
            effectful_misses: 3,
            hit_signal_requests,
            denial_signal_requests,
            effectful_signal_requests,
        },
    );
    let evidence = require_evidence(
        evidence,
        ExpectedReadEvidence {
            attempts: 5,
            admissions: 4,
            completions: 4,
            peak: 2,
            hits: 1,
            misses: 3,
        },
        "prefetch",
    )?;
    Ok(PrefetchProof {
        evidence,
        schedule: ExecutedPrefetchSchedule::new(worker_order, ready_work_selection),
    })
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum PrefetchWorkerIdentity {
    First,
    Second,
}

struct PausedPrefetchWorker {
    identity: PrefetchWorkerIdentity,
    arrival_index: usize,
    thread: Option<std::thread::JoinHandle<PhysicalPrefetchOutcome>>,
}

struct PausedPrefetchWorkers {
    gate: Option<
        worth_store::physical_runtime::certification::CertificationPhysicalExecutionPauseGate,
    >,
    started: Vec<PausedPrefetchWorker>,
}

fn start_workers(
    serving: &ServingPhysicalRuntime,
    residency: &PhysicalResidencyCertification,
    first_coordinate: RecordFrameCoordinate,
    second_coordinate: RecordFrameCoordinate,
    order: WorkerStartOrder,
) -> Result<PausedPrefetchWorkers, String> {
    let gate = serving.certification_pause_physical_execution_at(
        CertificationPhysicalExecutionCheckpoint::BeforeBackendDispatch,
    );
    let (first_started, second_started) = match order {
        WorkerStartOrder::FirstThenSecond => (
            spawn(
                residency,
                PrefetchWorkerIdentity::First,
                first_coordinate,
                0,
            ),
            (PrefetchWorkerIdentity::Second, second_coordinate),
        ),
        WorkerStartOrder::SecondThenFirst => (
            spawn(
                residency,
                PrefetchWorkerIdentity::Second,
                second_coordinate,
                0,
            ),
            (PrefetchWorkerIdentity::First, first_coordinate),
        ),
    };
    let mut workers = PausedPrefetchWorkers {
        gate: Some(gate),
        started: vec![first_started],
    };
    await_arrivals(workers.gate(), 1)?;
    workers
        .started
        .push(spawn(residency, second_started.0, second_started.1, 1));
    await_arrivals(workers.gate(), 2)?;
    Ok(workers)
}

fn spawn(
    residency: &PhysicalResidencyCertification,
    identity: PrefetchWorkerIdentity,
    coordinate: RecordFrameCoordinate,
    arrival_index: usize,
) -> PausedPrefetchWorker {
    let residency = residency.clone();
    PausedPrefetchWorker {
        identity,
        arrival_index,
        thread: Some(std::thread::spawn(move || {
            residency.prefetch(PhysicalPrefetchIntent::new(coordinate))
        })),
    }
}

impl PausedPrefetchWorkers {
    fn gate(
        &self,
    ) -> &worth_store::physical_runtime::certification::CertificationPhysicalExecutionPauseGate
    {
        self.gate
            .as_ref()
            .expect("a running prefetch schedule owns its executor gate")
    }

    fn complete(
        mut self,
        selection: IndependentReadyWorkSelection,
    ) -> Result<[PhysicalWorkIdentity; 2], String> {
        let selected_identity = match selection {
            IndependentReadyWorkSelection::FirstWorkerThenSecond => PrefetchWorkerIdentity::First,
            IndependentReadyWorkSelection::SecondWorkerThenFirst => PrefetchWorkerIdentity::Second,
        };
        let selected_index = self
            .started
            .iter()
            .position(|worker| worker.identity == selected_identity)
            .expect("the closed worker-start order contains the selected worker");
        let remaining_index = 1 - selected_index;
        let selected_resumed =
            self.started[selected_index].select_then_release_downstream(self.gate());
        let selected = self.started[selected_index].join();
        let remaining = self.started[remaining_index].join();
        selected_resumed?;
        Ok([selected?, remaining?])
    }
}

impl Drop for PausedPrefetchWorkers {
    fn drop(&mut self) {
        drop(self.gate.take());
        for worker in &mut self.started {
            worker.join_if_running();
        }
    }
}

impl PausedPrefetchWorker {
    fn select_then_release_downstream(
        &self,
        gate: &worth_store::physical_runtime::certification::CertificationPhysicalExecutionPauseGate,
    ) -> Result<(), String> {
        let release = gate
            .select_arrival_then_release_downstream(self.arrival_index)
            .map_err(|failure| format!("bounded prefetch worker selection failed: {failure:?}"))?;
        if release.arrival_index() != self.arrival_index {
            return Err("bounded prefetch released a foreign executor arrival".to_owned());
        }
        Ok(())
    }

    fn join(&mut self) -> Result<PhysicalWorkIdentity, String> {
        require_loaded(
            self.thread
                .take()
                .expect("a prefetch worker is joined exactly once")
                .join()
                .map_err(|_| "bounded prefetch worker panicked".to_owned())?,
        )
    }

    fn join_if_running(&mut self) {
        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
    }
}

fn require_loaded(outcome: PhysicalPrefetchOutcome) -> Result<PhysicalWorkIdentity, String> {
    match outcome {
        PhysicalPrefetchOutcome::Loaded { work, .. } => Ok(work),
        outcome => Err(format!("bounded prefetch miss did not load: {outcome:?}")),
    }
}

#[cfg(test)]
mod tests {
    use std::panic::{catch_unwind, AssertUnwindSafe};

    use super::{PausedPrefetchWorker, PrefetchWorkerIdentity};

    #[test]
    fn drop_cleanup_accepts_a_worker_already_joined_by_success() {
        let mut worker = PausedPrefetchWorker {
            identity: PrefetchWorkerIdentity::First,
            arrival_index: 0,
            thread: None,
        };
        if catch_unwind(AssertUnwindSafe(|| worker.join_if_running())).is_err() {
            panic!("MUTANT_PREDICATE:prefetch-cleanup-double-joins-worker");
        }
    }
}
