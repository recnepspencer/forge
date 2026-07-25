use std::{
    sync::mpsc,
    time::{Duration, Instant},
};

use worth_proof::TransitionOutcome;
use worth_store::physical_runtime::{
    C6PhysicalWorkHandoff, C6PhysicalWorkSettlement, C6PhysicalWritebackExecution,
    C6PhysicalWritebackTransitionFailure, PhysicalEffectIdentity, PhysicalMutationWorkRequest,
    PhysicalSignalSettlementOutcome, PhysicalWorkEffectFate, PhysicalWorkIdentity,
    PhysicalWorkReadiness, PhysicalWorkRecoveryDisposition, PhysicalWorkSubmissionReceipt,
    ReadyPhysicalWork, ServingPhysicalRuntime,
};
use worth_store_contracts::QueueProducerResourceShape;
use worth_store_physical_backend::{MediaOperationRole, MediaPauseGate};

pub(super) struct C6DirtyPressureEvidence {
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
}

struct DirtySourceProvenance {
    work_count: u64,
    first_work: PhysicalWorkIdentity,
    last_work: PhysicalWorkIdentity,
}

struct DirtyPressureBaseline {
    residency: worth_store_buffer_pool::PhysicalResidencyCounters,
    positioned_writes: u64,
}

struct DispatchedDirtyWriteEvidence {
    source: DirtySourceProvenance,
    baseline: DirtyPressureBaseline,
    dirty_at_pause: u32,
}

pub(super) fn prove(
    serving: &ServingPhysicalRuntime,
    request: PhysicalMutationWorkRequest,
    gate: MediaPauseGate,
) -> Result<C6DirtyPressureEvidence, String> {
    serving.drain_clean_residency();
    let handoff = serving.c6_physical_work_handoff();
    let ready = ready_mutation(&handoff, request)?;
    let residency = handoff.residency_work();
    let coordinate = ready
        .intent()
        .scope()
        .coordinates()
        .first()
        .copied()
        .ok_or_else(|| "C.6 writeback request omitted its coordinate".to_owned())?;
    let lease = residency
        .pin_exact(coordinate)
        .map_err(|failure| format!("C.6 writeback frame load failed: {failure:?}"))?;
    let bytes = lease.bytes().to_vec();
    let dirty = residency
        .admit_dirty_frame(&ready, lease, bytes)
        .map_err(|failure| format!("C.6 dirty admission failed: {failure:?}"))?;
    let source = source_evidence(&handoff, &dirty)?;
    let baseline = DirtyPressureBaseline {
        residency: residency.counters(),
        positioned_writes: positioned_writes(serving),
    };
    let reservation = residency
        .reserve_writeback(&ready, &dirty)
        .map_err(|failure| format!("C.6 writeback reservation failed: {failure:?}"))?;
    let prepared = residency
        .prepare_writeback(ready, reservation, 1, writeback_shape())
        .map_err(|failure| format!("C.6 writeback preparation failed: {failure:?}"))?;
    let admitted = residency
        .admit_writeback(prepared, dirty)
        .map_err(|failure| format!("C.6 writeback admission failed: {failure:?}"))?;
    let (execution, receiver) = spawn_execution(residency.clone(), admitted);
    await_dispatch(&gate, &receiver)?;
    let dirty_at_pause = require_paused_write(serving, &gate, baseline.positioned_writes)?;
    gate.release();
    let settlement = receive_settlement(receiver, execution)?;
    settlement_evidence(
        serving,
        settlement,
        DispatchedDirtyWriteEvidence {
            source,
            baseline,
            dirty_at_pause,
        },
    )
}

fn ready_mutation(
    handoff: &C6PhysicalWorkHandoff,
    request: PhysicalMutationWorkRequest,
) -> Result<ReadyPhysicalWork, String> {
    let receipt = match handoff.mutation_submission().submit(request).into_raw() {
        TransitionOutcome::Success(receipt) => receipt,
        outcome => return Err(format!("C.6 mutation submission failed: {outcome:?}")),
    };
    ready_from_receipt(handoff, receipt)
}

fn ready_from_receipt(
    handoff: &C6PhysicalWorkHandoff,
    receipt: PhysicalWorkSubmissionReceipt,
) -> Result<ReadyPhysicalWork, String> {
    let admitted = handoff
        .admit_submitted_work(receipt)
        .map_err(|failure| format!("C.6 work admission failed: {failure:?}"))?;
    match handoff
        .request_work(admitted)
        .map_err(|failure| format!("C.6 work request failed: {failure:?}"))?
    {
        PhysicalWorkReadiness::Ready(ready) => Ok(ready),
        PhysicalWorkReadiness::Blocked(blocked) => Err(format!(
            "C.6 writeback unexpectedly blocked: {:?}",
            blocked.condition()
        )),
    }
}

fn source_evidence(
    handoff: &C6PhysicalWorkHandoff,
    dirty: &worth_store::physical_runtime::C6AdmittedDirtyFrame,
) -> Result<DirtySourceProvenance, String> {
    let first = dirty
        .first_source_physical_work()
        .ok_or_else(|| "C.6 dirty frame omitted its canonical source work".to_owned())?;
    let last = dirty
        .last_source_physical_work()
        .ok_or_else(|| "C.6 dirty frame omitted its terminal source work".to_owned())?;
    if dirty.source_physical_work_count() == 0
        || !handoff.identity().admits(first)
        || !handoff.identity().admits(last)
    {
        return Err("C.6 dirty source work escaped its handoff generation".to_owned());
    }
    Ok(DirtySourceProvenance {
        work_count: dirty.source_physical_work_count(),
        first_work: first,
        last_work: last,
    })
}

type SettlementReceiver =
    mpsc::Receiver<Result<C6PhysicalWritebackExecution, C6PhysicalWritebackTransitionFailure>>;

fn spawn_execution(
    residency: worth_store::physical_runtime::C6PhysicalResidencyWork,
    admitted: worth_store::physical_runtime::C6AdmittedPhysicalWriteback,
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
            Ok(_) => return Err("C.6 writeback settled before backend dispatch".to_owned()),
            Err(mpsc::TryRecvError::Disconnected) => {
                return Err("C.6 writeback execution disconnected before dispatch".to_owned())
            }
            Err(mpsc::TryRecvError::Empty) => std::thread::yield_now(),
        }
    }
    if gate.reached_context().is_none() {
        gate.release();
        return Err("C.6 writeback did not reach its backend gate".to_owned());
    }
    Ok(())
}

fn require_paused_write(
    serving: &ServingPhysicalRuntime,
    gate: &MediaPauseGate,
    writes_before: u64,
) -> Result<u32, String> {
    let context = gate
        .reached_context()
        .ok_or_else(|| "C.6 dirty gate omitted its media context".to_owned())?;
    let dirty = serving.residency_counters().dirty_frames();
    let attempts = positioned_writes(serving).saturating_sub(writes_before);
    if context.role() != MediaOperationRole::PositionedWrite
        || context.identified_operation_ordinal() != Some(1)
        || dirty != 1
        || attempts != 1
    {
        gate.release();
        return Err("C.6 writeback did not retain exact dirty posture at dispatch".to_owned());
    }
    Ok(dirty)
}

fn receive_settlement(
    receiver: SettlementReceiver,
    execution: std::thread::JoinHandle<()>,
) -> Result<C6PhysicalWorkSettlement, String> {
    let result = receiver
        .recv_timeout(Duration::from_secs(2))
        .map_err(|_| "C.6 writeback did not settle after gate release".to_owned())?;
    execution
        .join()
        .map_err(|_| "C.6 writeback execution panicked".to_owned())?;
    match result
        .map_err(|failure| format!("C.6 writeback execution failed: {:?}", failure.cause()))?
    {
        C6PhysicalWritebackExecution::Settled(settlement) => Ok(settlement),
        C6PhysicalWritebackExecution::Retryable(retryable) => Err(format!(
            "C.6 unfaulted writeback unexpectedly required retry for {:?}",
            retryable.identity()
        )),
    }
}

fn settlement_evidence(
    serving: &ServingPhysicalRuntime,
    settlement: C6PhysicalWorkSettlement,
    dispatched: DispatchedDirtyWriteEvidence,
) -> Result<C6DirtyPressureEvidence, String> {
    let effect = settlement
        .effect()
        .ok_or_else(|| "C.6 writeback settlement omitted its backend effect".to_owned())?;
    let after = serving.residency_counters();
    let evidence = C6DirtyPressureEvidence {
        identity: settlement.identity(),
        source_work_count: dispatched.source.work_count,
        first_source_work: dispatched.source.first_work,
        last_source_work: dispatched.source.last_work,
        effect,
        effect_fate: settlement.effect_fate(),
        recovery: settlement.recovery(),
        signal: settlement.signal(),
        dirty_at_pause: dispatched.dirty_at_pause,
        dirty_after_receipt: after.dirty_frames(),
        positioned_writes: positioned_writes(serving)
            .saturating_sub(dispatched.baseline.positioned_writes),
        candidate_publications: after
            .candidate_publications()
            .saturating_sub(dispatched.baseline.residency.candidate_publications()),
        writebacks: after
            .writebacks()
            .saturating_sub(dispatched.baseline.residency.writebacks()),
    };
    validate_settlement(evidence)
}

fn validate_settlement(
    evidence: C6DirtyPressureEvidence,
) -> Result<C6DirtyPressureEvidence, String> {
    if evidence.effect.work() != evidence.identity
        || evidence.effect_fate != PhysicalWorkEffectFate::WriteCompleted
        || evidence.recovery != PhysicalWorkRecoveryDisposition::ContinueSettlement
        || evidence.signal != PhysicalSignalSettlementOutcome::Committed
        || evidence.dirty_after_receipt != 0
        || evidence.positioned_writes != 1
        || evidence.candidate_publications != 1
        || evidence.writebacks != 1
    {
        return Err("C.6 exact receipt did not settle the canonical writeback".to_owned());
    }
    Ok(evidence)
}

fn writeback_shape() -> QueueProducerResourceShape {
    QueueProducerResourceShape::new()
        .with_queue_slots(1)
        .with_bandwidth_tokens(8)
        .with_write_back_windows(1)
        .with_worker_permits(1)
}

fn positioned_writes(serving: &ServingPhysicalRuntime) -> u64 {
    serving
        .media_counters()
        .attempts_for(MediaOperationRole::PositionedWrite)
}
