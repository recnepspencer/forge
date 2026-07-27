use std::{
    sync::mpsc,
    time::{Duration, Instant},
};

use worth_store::physical_runtime::{
    AdmittedDirtyFrame, AdmittedPhysicalWriteback, PhysicalEffectIdentity,
    PhysicalEffectObligation, PhysicalResidencyCertification, PhysicalSignalSettlementOutcome,
    PhysicalWorkConsumerHandle, PhysicalWorkEffectFate, PhysicalWorkIdentity,
    PhysicalWorkRecoveryDisposition, PhysicalWritebackCounterSnapshot, PhysicalWritebackExecution,
    PhysicalWritebackSettlement, PhysicalWritebackTransitionFailure, ServingPhysicalRuntime,
};
use worth_store_physical_backend::{
    ArtifactRangeWriteDurabilityRequirement, MediaOperationRole, MediaPauseGate,
};
use worth_store_physical_format::{RecordArtifactFile, RecordFrameCoordinate};

pub(super) struct BoundedDirtyWritebackEvidence {
    pub(super) identity: PhysicalWorkIdentity,
    pub(super) source_work_count: u64,
    pub(super) first_source_work: PhysicalWorkIdentity,
    pub(super) last_source_work: PhysicalWorkIdentity,
    pub(super) effect: PhysicalEffectIdentity,
    pub(super) effect_fate: PhysicalWorkEffectFate,
    pub(super) recovery: PhysicalWorkRecoveryDisposition,
    pub(super) signal: PhysicalSignalSettlementOutcome,
    pub(super) dirty_at_pause: u32,
    pub(super) dirty_after_receipt: u32,
    pub(super) positioned_writes: u64,
    pub(super) candidate_publications: u64,
    pub(super) writebacks: u64,
    pub(super) active_claims_at_pause: u32,
    pub(super) eviction_releases_at_pause: u64,
    pub(super) competing_claim_denied: bool,
    pub(super) cancellation_settlement_continues: bool,
    pub(super) writeback_attempts: u64,
    pub(super) exact_receipts: u64,
    pub(super) retryable_writebacks: u64,
    pub(super) indeterminate_writebacks: u64,
    pub(super) inspection_required_writebacks: u64,
}

struct DirtySourceProvenance {
    work_count: u64,
    first_work: PhysicalWorkIdentity,
    last_work: PhysicalWorkIdentity,
}

struct DirtyPressureBaseline {
    residency: worth_store_buffer_pool::PhysicalResidencyCounters,
    writebacks: PhysicalWritebackCounterSnapshot,
    positioned_writes: u64,
}

struct DispatchedDirtyWriteEvidence {
    source: DirtySourceProvenance,
    baseline: DirtyPressureBaseline,
    pressure: PausedDirtyPressure,
}

struct PausedDirtyPressure {
    dirty_frames: u32,
    active_claims: u32,
    eviction_releases: u64,
    competing_claim_denied: bool,
    cancellation_settlement_continues: bool,
}

struct PreparedDirtyWriteback {
    residency: PhysicalResidencyCertification,
    coordinate: RecordFrameCoordinate,
    source: DirtySourceProvenance,
    baseline: DirtyPressureBaseline,
    consumer: PhysicalWorkConsumerHandle,
    admitted: AdmittedPhysicalWriteback,
}

pub(super) fn prove(
    serving: &ServingPhysicalRuntime,
    gate: MediaPauseGate,
) -> Result<BoundedDirtyWritebackEvidence, String> {
    let PreparedDirtyWriteback {
        residency,
        coordinate,
        source,
        baseline,
        consumer,
        admitted,
    } = prepare_dirty_writeback(serving)?;
    let (execution, receiver) = spawn_execution(residency.clone(), admitted);
    await_dispatch(&gate, &receiver)?;
    let pressure = require_paused_write(
        serving,
        &residency,
        coordinate,
        &gate,
        baseline.positioned_writes,
    )?;
    let cancellation = serving
        .cancel_physical_work(consumer)
        .map_err(|failure| format!("post-dispatch writeback cancellation failed: {failure:?}"))?;
    let pressure = PausedDirtyPressure {
        cancellation_settlement_continues: cancellation.obligation()
            == PhysicalEffectObligation::SettlementContinues,
        ..pressure
    };
    if !pressure.cancellation_settlement_continues {
        gate.release();
        return Err("post-dispatch cancellation did not preserve settlement".to_owned());
    }
    gate.release();
    let settlement = receive_settlement(receiver, execution)?;
    settlement_evidence(
        serving,
        settlement,
        DispatchedDirtyWriteEvidence {
            source,
            baseline,
            pressure,
        },
    )
}

fn prepare_dirty_writeback(
    serving: &ServingPhysicalRuntime,
) -> Result<PreparedDirtyWriteback, String> {
    let residency = serving.certification_physical_residency();
    residency.drain_unpinned_clean_frames();
    let coordinate =
        RecordFrameCoordinate::new(RecordArtifactFile::BootstrapCatalog, 8, 8).unwrap();
    let lease = residency
        .pin_exact(coordinate)
        .map_err(|failure| format!("writeback frame load failed: {failure:?}"))?;
    let dirty = residency
        .admit_dirty_frame(lease, |source, target| {
            target.copy_from_slice(source);
        })
        .map_err(|failure| format!("dirty admission failed: {failure:?}"))?;
    let source = source_evidence(&dirty)?;
    let baseline = DirtyPressureBaseline {
        residency: residency.counters(),
        writebacks: serving.residency_observation().writebacks(),
        positioned_writes: positioned_writes(serving),
    };
    let prepared = residency
        .prepare_writeback(
            dirty,
            ArtifactRangeWriteDurabilityRequirement::BufferedWrite,
        )
        .map_err(|failure| format!("writeback preparation failed: {:?}", failure.cause()))?;
    let ready = residency
        .request_writeback(prepared)
        .map_err(|failure| format!("writeback readiness failed: {:?}", failure.cause()))?;
    let consumer = ready.consumer_handle();
    let admitted = residency
        .admit_writeback(ready)
        .map_err(|failure| format!("writeback admission failed: {:?}", failure.cause()))?;
    Ok(PreparedDirtyWriteback {
        residency,
        coordinate,
        source,
        baseline,
        consumer,
        admitted,
    })
}

fn source_evidence(dirty: &AdmittedDirtyFrame) -> Result<DirtySourceProvenance, String> {
    let first = dirty
        .first_source_physical_work()
        .ok_or_else(|| "dirty frame omitted its canonical source work".to_owned())?;
    let last = dirty
        .last_source_physical_work()
        .ok_or_else(|| "dirty frame omitted its terminal source work".to_owned())?;
    if dirty.source_physical_work_count() == 0 {
        return Err("dirty source work count was empty".to_owned());
    }
    Ok(DirtySourceProvenance {
        work_count: dirty.source_physical_work_count(),
        first_work: first,
        last_work: last,
    })
}

type SettlementReceiver =
    mpsc::Receiver<Result<PhysicalWritebackExecution, PhysicalWritebackTransitionFailure>>;

fn spawn_execution(
    residency: PhysicalResidencyCertification,
    admitted: AdmittedPhysicalWriteback,
) -> (std::thread::JoinHandle<()>, SettlementReceiver) {
    let (sender, receiver) = mpsc::sync_channel(1);
    let execution = std::thread::spawn(move || {
        let _ = sender.send(residency.execute_writeback(admitted));
    });
    (execution, receiver)
}

fn await_dispatch(gate: &MediaPauseGate, receiver: &SettlementReceiver) -> Result<(), String> {
    let deadline = Instant::now() + Duration::from_secs(2);
    while gate.reached_context().is_none() && Instant::now() < deadline {
        match receiver.try_recv() {
            Ok(_) => return Err("writeback settled before backend dispatch".to_owned()),
            Err(mpsc::TryRecvError::Disconnected) => {
                return Err("writeback execution disconnected before dispatch".to_owned())
            }
            Err(mpsc::TryRecvError::Empty) => std::thread::yield_now(),
        }
    }
    if gate.reached_context().is_none() {
        gate.release();
        return Err("writeback did not reach its backend gate".to_owned());
    }
    Ok(())
}

fn require_paused_write(
    serving: &ServingPhysicalRuntime,
    residency: &PhysicalResidencyCertification,
    coordinate: RecordFrameCoordinate,
    gate: &MediaPauseGate,
    writes_before: u64,
) -> Result<PausedDirtyPressure, String> {
    let context = gate
        .reached_context()
        .ok_or_else(|| "dirty gate omitted its media context".to_owned())?;
    let counters = serving.residency_observation().counters();
    let dirty = counters.dirty_frames();
    let attempts = positioned_writes(serving).saturating_sub(writes_before);
    let eviction_releases = residency.drain_unpinned_clean_frames();
    let competing_claim_denied = matches!(
        residency.probe_competing_writeback_claim(coordinate),
        Err(worth_store_buffer_pool::PhysicalResidencyDenial::WriteBackFrameAlreadyClaimed)
    );
    if context.role() != MediaOperationRole::PositionedWrite
        || context.identified_operation_ordinal() != Some(1)
        || dirty != 1
        || counters.active_writeback_claims() != 1
        || attempts != 1
        || eviction_releases != 0
        || !competing_claim_denied
    {
        gate.release();
        return Err("writeback did not retain exact dirty posture at dispatch".to_owned());
    }
    Ok(PausedDirtyPressure {
        dirty_frames: dirty,
        active_claims: counters.active_writeback_claims(),
        eviction_releases,
        competing_claim_denied,
        cancellation_settlement_continues: false,
    })
}

fn receive_settlement(
    receiver: SettlementReceiver,
    execution: std::thread::JoinHandle<()>,
) -> Result<PhysicalWritebackSettlement, String> {
    let result = receiver
        .recv_timeout(Duration::from_secs(2))
        .map_err(|_| "writeback did not settle after gate release".to_owned())?;
    execution
        .join()
        .map_err(|_| "writeback execution panicked".to_owned())?;
    match result.map_err(|failure| format!("writeback execution failed: {:?}", failure.cause()))? {
        PhysicalWritebackExecution::Clean(settlement) => Ok(settlement),
        PhysicalWritebackExecution::Retryable(retryable) => Err(format!(
            "unfaulted writeback unexpectedly required retry for {:?}",
            retryable.settled().intent().identity()
        )),
        PhysicalWritebackExecution::InspectionRequired(inspection) => Err(format!(
            "unfaulted writeback required inspection for {:?}",
            inspection.settlement().identity()
        )),
    }
}

fn settlement_evidence(
    serving: &ServingPhysicalRuntime,
    settlement: PhysicalWritebackSettlement,
    dispatched: DispatchedDirtyWriteEvidence,
) -> Result<BoundedDirtyWritebackEvidence, String> {
    let effect = settlement
        .effect()
        .ok_or_else(|| "writeback settlement omitted its backend effect".to_owned())?;
    let after = serving.residency_observation().counters();
    let writebacks = serving.residency_observation().writebacks();
    let evidence = BoundedDirtyWritebackEvidence {
        identity: settlement.identity(),
        source_work_count: dispatched.source.work_count,
        first_source_work: dispatched.source.first_work,
        last_source_work: dispatched.source.last_work,
        effect,
        effect_fate: settlement.effect_fate(),
        recovery: settlement.recovery(),
        signal: settlement.signal(),
        dirty_at_pause: dispatched.pressure.dirty_frames,
        dirty_after_receipt: after.dirty_frames(),
        positioned_writes: positioned_writes(serving)
            .saturating_sub(dispatched.baseline.positioned_writes),
        candidate_publications: after
            .candidate_publications()
            .saturating_sub(dispatched.baseline.residency.candidate_publications()),
        writebacks: after
            .writebacks()
            .saturating_sub(dispatched.baseline.residency.writebacks()),
        active_claims_at_pause: dispatched.pressure.active_claims,
        eviction_releases_at_pause: dispatched.pressure.eviction_releases,
        competing_claim_denied: dispatched.pressure.competing_claim_denied,
        cancellation_settlement_continues: dispatched.pressure.cancellation_settlement_continues,
        writeback_attempts: writebacks
            .attempts()
            .saturating_sub(dispatched.baseline.writebacks.attempts()),
        exact_receipts: writebacks
            .exact_receipts()
            .saturating_sub(dispatched.baseline.writebacks.exact_receipts()),
        retryable_writebacks: writebacks
            .retryable()
            .saturating_sub(dispatched.baseline.writebacks.retryable()),
        indeterminate_writebacks: writebacks
            .indeterminate()
            .saturating_sub(dispatched.baseline.writebacks.indeterminate()),
        inspection_required_writebacks: writebacks
            .inspection_required()
            .saturating_sub(dispatched.baseline.writebacks.inspection_required()),
    };
    validate_settlement(evidence)
}

fn validate_settlement(
    evidence: BoundedDirtyWritebackEvidence,
) -> Result<BoundedDirtyWritebackEvidence, String> {
    if evidence.effect.work() != evidence.identity
        || evidence.effect_fate != PhysicalWorkEffectFate::WriteCompleted
        || evidence.recovery != PhysicalWorkRecoveryDisposition::ContinueSettlement
        || evidence.signal != PhysicalSignalSettlementOutcome::ReconciledFromPhysicalTruth
        || evidence.dirty_after_receipt != 0
        || evidence.positioned_writes != 1
        || evidence.candidate_publications != 1
        || evidence.writebacks != 1
        || evidence.active_claims_at_pause != 1
        || evidence.eviction_releases_at_pause != 0
        || !evidence.competing_claim_denied
        || !evidence.cancellation_settlement_continues
        || evidence.writeback_attempts != 1
        || evidence.exact_receipts != 1
        || evidence.retryable_writebacks != 0
        || evidence.indeterminate_writebacks != 0
        || evidence.inspection_required_writebacks != 0
    {
        return Err(format!(
            "exact receipt did not settle the canonical writeback: \
             effect_matches_identity={} effect_fate={:?} recovery={:?} signal={:?} \
             dirty_after_receipt={} positioned_writes={} candidate_publications={} writebacks={} \
             active_claims_at_pause={} eviction_releases_at_pause={} \
             competing_claim_denied={} cancellation_settlement_continues={} \
             writeback_attempts={} exact_receipts={} retryable_writebacks={} \
             indeterminate_writebacks={} inspection_required_writebacks={}",
            evidence.effect.work() == evidence.identity,
            evidence.effect_fate,
            evidence.recovery,
            evidence.signal,
            evidence.dirty_after_receipt,
            evidence.positioned_writes,
            evidence.candidate_publications,
            evidence.writebacks,
            evidence.active_claims_at_pause,
            evidence.eviction_releases_at_pause,
            evidence.competing_claim_denied,
            evidence.cancellation_settlement_continues,
            evidence.writeback_attempts,
            evidence.exact_receipts,
            evidence.retryable_writebacks,
            evidence.indeterminate_writebacks,
            evidence.inspection_required_writebacks,
        ));
    }
    Ok(evidence)
}

fn positioned_writes(serving: &ServingPhysicalRuntime) -> u64 {
    serving
        .media_counters()
        .attempts_for(MediaOperationRole::PositionedWrite)
}
