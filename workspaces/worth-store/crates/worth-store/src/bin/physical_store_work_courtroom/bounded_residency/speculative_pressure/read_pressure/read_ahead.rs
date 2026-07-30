use worth_store::physical_runtime::{
    certification::CertificationPhysicalExecutionCheckpoint, PhysicalReadAheadFrameOutcome,
    PhysicalReadAheadIntent, PhysicalReadAheadOutcome, PhysicalResidencyCertification,
    PhysicalSpeculativeWorkKind, PhysicalWorkIdentity, PhysicalWorkSignalFamily,
    ServingPhysicalRuntime,
};
use worth_store_physical_format::RecordFrameCoordinate;

use super::{
    await_arrivals, require_evidence, require_pressure, ExpectedReadEvidence, FRAME_READ_BASIS,
};
use crate::bounded_residency::speculative_pressure::{
    causal_binding, counter_evidence, signal_requests, SpeculativeKindEvidence,
    SpeculativePathEvidence,
};

pub(in crate::bounded_residency::speculative_pressure) fn prove(
    serving: &ServingPhysicalRuntime,
    residency: &PhysicalResidencyCertification,
    coordinates: &[RecordFrameCoordinate; 8],
) -> Result<SpeculativeKindEvidence, String> {
    let kind = PhysicalSpeculativeWorkKind::ReadAhead;
    let counters_before = residency.counters();
    let signals_before = signal_requests(serving)?;
    let mixed = [coordinates[0], coordinates[4]];
    let mut work = require_mixed(
        residency.read_ahead(PhysicalReadAheadIntent::new(&mixed).expect("mixed read-ahead")),
        mixed,
    )?;
    let hit_signal_requests = signal_requests(serving)?
        .saturating_sub(signals_before)
        .saturating_sub(1);

    let held_coordinates = [coordinates[5], coordinates[6]];
    let held = PausedReadAhead::start(serving, residency, held_coordinates);
    await_arrivals(held.gate(), 1)?;
    let denial_signals_before = signal_requests(serving)?;
    let denied_coordinates = [coordinates[7]];
    let dropped = match residency
        .read_ahead(PhysicalReadAheadIntent::new(&denied_coordinates).expect("denied read-ahead"))
    {
        PhysicalReadAheadOutcome::Dropped(dropped) => dropped,
        outcome => {
            return Err(format!(
                "bounded read-ahead one-past was not denied: {outcome:?}"
            ));
        }
    };
    require_pressure(dropped.pressure(), kind, 1, 2, 2, "bounded read-ahead")?;
    let denial_signal_requests = signal_requests(serving)?.saturating_sub(denial_signals_before);
    work.extend(require_loaded(held.complete()?, held_coordinates)?);
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
    require_evidence(
        evidence,
        ExpectedReadEvidence {
            attempts: 3,
            admissions: 2,
            completions: 2,
            peak: 2,
            hits: 1,
            misses: 3,
        },
        "read-ahead",
    )
}

struct PausedReadAhead {
    gate: Option<
        worth_store::physical_runtime::certification::CertificationPhysicalExecutionPauseGate,
    >,
    thread: Option<std::thread::JoinHandle<PhysicalReadAheadOutcome>>,
}

impl PausedReadAhead {
    fn start(
        serving: &ServingPhysicalRuntime,
        residency: &PhysicalResidencyCertification,
        coordinates: [RecordFrameCoordinate; 2],
    ) -> Self {
        let gate = serving.certification_pause_physical_execution_at(
            CertificationPhysicalExecutionCheckpoint::BeforeBackendDispatch,
        );
        let residency = residency.clone();
        Self {
            gate: Some(gate),
            thread: Some(std::thread::spawn(move || {
                residency.read_ahead(
                    PhysicalReadAheadIntent::new(&coordinates).expect("held read-ahead"),
                )
            })),
        }
    }

    fn gate(
        &self,
    ) -> &worth_store::physical_runtime::certification::CertificationPhysicalExecutionPauseGate
    {
        self.gate
            .as_ref()
            .expect("a running read-ahead schedule owns its executor gate")
    }

    fn complete(mut self) -> Result<PhysicalReadAheadOutcome, String> {
        self.gate().release();
        self.thread
            .take()
            .expect("a read-ahead worker is joined exactly once")
            .join()
            .map_err(|_| "bounded read-ahead worker panicked".to_owned())
    }
}

impl Drop for PausedReadAhead {
    fn drop(&mut self) {
        drop(self.gate.take());
        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
    }
}

fn require_mixed(
    outcome: PhysicalReadAheadOutcome,
    coordinates: [RecordFrameCoordinate; 2],
) -> Result<Vec<PhysicalWorkIdentity>, String> {
    let batch = match outcome {
        PhysicalReadAheadOutcome::Complete(batch) => batch,
        outcome => return Err(format!("bounded mixed read-ahead failed: {outcome:?}")),
    };
    if batch.hits() != 1 || batch.loaded() != 1 || batch.coalesced() != 0 || batch.failed() != 0 {
        return Err(format!(
            "bounded mixed read-ahead posture drifted: {:?}",
            batch.frames()
        ));
    }
    match batch.frames() {
        [PhysicalReadAheadFrameOutcome::Hit { coordinate }, PhysicalReadAheadFrameOutcome::Loaded {
            coordinate: loaded,
            work,
        }] if *coordinate == coordinates[0] && *loaded == coordinates[1] => Ok(vec![*work]),
        frames => Err(format!(
            "bounded mixed read-ahead frames drifted: {frames:?}"
        )),
    }
}

fn require_loaded(
    outcome: PhysicalReadAheadOutcome,
    coordinates: [RecordFrameCoordinate; 2],
) -> Result<Vec<PhysicalWorkIdentity>, String> {
    let batch = match outcome {
        PhysicalReadAheadOutcome::Complete(batch) => batch,
        outcome => return Err(format!("bounded held read-ahead failed: {outcome:?}")),
    };
    if batch.hits() != 0 || batch.loaded() != 2 || batch.coalesced() != 0 || batch.failed() != 0 {
        return Err(format!(
            "bounded held read-ahead posture drifted: {:?}",
            batch.frames()
        ));
    }
    batch
        .frames()
        .iter()
        .zip(coordinates)
        .map(|(frame, expected)| match frame {
            PhysicalReadAheadFrameOutcome::Loaded { coordinate, work }
                if *coordinate == expected =>
            {
                Ok(*work)
            }
            frame => Err(format!("bounded held read-ahead frame drifted: {frame:?}")),
        })
        .collect()
}
