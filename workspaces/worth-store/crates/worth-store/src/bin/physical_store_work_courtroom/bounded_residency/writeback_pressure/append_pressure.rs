use std::{
    sync::mpsc,
    time::{Duration, Instant},
};

use worth_store::physical_runtime::{
    AdmittedRecordPlacementPolicy, PhysicalOperationAllocationScope,
    PhysicalRecordPressureEvidence, PhysicalResidencyCounterSnapshot, PhysicalResidencyDimension,
    PhysicalResidencyRetryPosture, PhysicalSpeculativeWorkKind, PhysicalWritebackCounterSnapshot,
    PublishedRecordBatch, RecordAppendError, ServingPhysicalRuntime,
};
use worth_store_physical_backend::{MediaOperationContext, MediaOperationRole, MediaPauseGate};

use super::{append_batch, positioned_writes};
use crate::bounded_residency::configuration::BoundedResidencyConfiguration;

pub(super) struct AppendPressureBaseline {
    pub(super) residency: PhysicalResidencyCounterSnapshot,
    pub(super) writebacks: PhysicalWritebackCounterSnapshot,
    pub(super) positioned_writes: u64,
    pub(super) deletions: u64,
}

pub(super) struct OrdinaryAppendPressure {
    pub(super) primary: PublishedRecordBatch,
    pub(super) retry: PublishedRecordBatch,
    pub(super) baseline: AppendPressureBaseline,
    pub(super) paused: PausedAppendPressure,
    pub(super) pressure: PhysicalRecordPressureEvidence,
    pub(super) dirty_after_primary: u32,
    pub(super) primary_candidate_publications: u64,
    pub(super) retry_candidate_publications: u64,
    pub(super) denied_candidate_publications: u64,
}

pub(super) struct PausedAppendPressure {
    pub(super) dirty_at_dispatch: u32,
    pub(super) dirty_after_denial: u32,
    pub(super) active_claims_at_dispatch: u32,
    pub(super) active_writebehind_at_dispatch: u32,
    pub(super) pressure_basis_exact: bool,
    pub(super) pressure_retry_after_settlement: bool,
    pub(super) cleanup_deletions: u64,
    pub(super) cleanup_complete: bool,
    candidate_publications_at_dispatch: u64,
    candidate_publications_after_denial: u64,
}

type AppendResult = Result<PublishedRecordBatch, RecordAppendError>;

pub(super) fn execute(
    serving: &ServingPhysicalRuntime,
    configuration: BoundedResidencyConfiguration,
    gate: MediaPauseGate,
) -> Result<OrdinaryAppendPressure, String> {
    let [primary_ordinal, denied_ordinal] = configuration.serving_append_ordinals();
    let (_, placement, _) = super::super::super::configuration::record_configuration();
    let baseline = AppendPressureBaseline {
        residency: serving.residency_observation().counters(),
        writebacks: serving.residency_observation().writebacks(),
        positioned_writes: positioned_writes(serving),
        deletions: serving.media_counters().deletions(),
    };
    let (primary_thread, primary_receiver) =
        spawn_append(serving, configuration, primary_ordinal, placement)?;
    let dispatch = match await_dispatch(&gate, &primary_receiver) {
        Ok(context) => context,
        Err(error) => {
            gate.release();
            let _ = primary_thread.join();
            return Err(error);
        }
    };
    let at_dispatch = serving.residency_observation().counters();
    let (denied_thread, denied_receiver) =
        spawn_append(serving, configuration, denied_ordinal, placement)?;
    let denial = receive_denial(&gate, denied_receiver, denied_thread)?;
    let pressure = denial
        .pressure()
        .ok_or_else(|| format!("competing ordinary append lacked pressure evidence: {denial:?}"))?;
    let paused_result = paused_evidence(serving, pressure, dispatch, at_dispatch, &baseline);
    gate.release();
    let primary_result = receive_append(primary_receiver, primary_thread, "primary append");
    let paused = paused_result?;
    let primary = primary_result?;
    let after_primary = serving.residency_observation().counters();
    let dirty_after_primary = after_primary.dirty_frames();
    let primary_candidate_publications = paused.candidate_publications_at_dispatch.saturating_add(
        after_primary
            .candidate_publications()
            .saturating_sub(paused.candidate_publications_after_denial),
    );
    let denied_candidate_publications = paused
        .candidate_publications_after_denial
        .saturating_sub(paused.candidate_publications_at_dispatch);
    let retry = serving
        .record_submission()
        .append_batch(
            append_batch::build(configuration, denied_ordinal)?,
            placement,
        )
        .map_err(|failure| format!("ordinary append retry failed: {failure:?}"))?;
    let retry_candidate_publications = serving
        .residency_observation()
        .counters()
        .candidate_publications()
        .saturating_sub(after_primary.candidate_publications());
    Ok(OrdinaryAppendPressure {
        primary,
        retry,
        baseline,
        paused,
        pressure,
        dirty_after_primary,
        primary_candidate_publications,
        retry_candidate_publications,
        denied_candidate_publications,
    })
}

fn spawn_append(
    serving: &ServingPhysicalRuntime,
    configuration: BoundedResidencyConfiguration,
    ordinal: usize,
    placement: AdmittedRecordPlacementPolicy,
) -> Result<(std::thread::JoinHandle<()>, mpsc::Receiver<AppendResult>), String> {
    let submission = serving.record_submission();
    let batch = append_batch::build(configuration, ordinal)?;
    let (sender, receiver) = mpsc::sync_channel(1);
    let thread = std::thread::spawn(move || {
        let _ = sender.send(submission.append_batch(batch, placement));
    });
    Ok((thread, receiver))
}

fn await_dispatch(
    gate: &MediaPauseGate,
    receiver: &mpsc::Receiver<AppendResult>,
) -> Result<MediaOperationContext, String> {
    let deadline = Instant::now() + Duration::from_secs(5);
    while gate.reached_context().is_none() && Instant::now() < deadline {
        match receiver.try_recv() {
            Ok(result) => {
                return Err(format!(
                    "primary ordinary append settled before backend dispatch: {result:?}"
                ))
            }
            Err(mpsc::TryRecvError::Disconnected) => {
                return Err("primary ordinary append disconnected before dispatch".to_owned())
            }
            Err(mpsc::TryRecvError::Empty) => std::thread::yield_now(),
        }
    }
    gate.reached_context()
        .ok_or_else(|| "primary ordinary append did not reach its backend gate".to_owned())
}

fn receive_denial(
    gate: &MediaPauseGate,
    receiver: mpsc::Receiver<AppendResult>,
    thread: std::thread::JoinHandle<()>,
) -> Result<RecordAppendError, String> {
    let result = match receiver.recv_timeout(Duration::from_secs(5)) {
        Ok(result) => result,
        Err(_) => {
            gate.release();
            return Err("competing ordinary append did not deny before effects".to_owned());
        }
    };
    thread
        .join()
        .map_err(|_| "competing ordinary append panicked".to_owned())?;
    match result {
        Ok(published) => {
            gate.release();
            Err(format!(
                "competing ordinary append bypassed pressure as publication {}",
                published.publication_identity()
            ))
        }
        Err(denial) => Ok(denial),
    }
}

fn receive_append(
    receiver: mpsc::Receiver<AppendResult>,
    thread: std::thread::JoinHandle<()>,
    label: &str,
) -> Result<PublishedRecordBatch, String> {
    let result = receiver
        .recv_timeout(Duration::from_secs(30))
        .map_err(|_| format!("{label} did not settle after gate release"))?;
    thread.join().map_err(|_| format!("{label} panicked"))?;
    result.map_err(|failure| format!("{label} failed: {failure:?}"))
}

fn paused_evidence(
    serving: &ServingPhysicalRuntime,
    pressure: PhysicalRecordPressureEvidence,
    dispatch: MediaOperationContext,
    at_dispatch: PhysicalResidencyCounterSnapshot,
    baseline: &AppendPressureBaseline,
) -> Result<PausedAppendPressure, String> {
    let context = serving.media_counters();
    let paused = serving.residency_observation().counters();
    let kind = PhysicalSpeculativeWorkKind::WriteBehind;
    let pressure_basis_exact = pressure.basis().store_identity() == serving.store_identity()
        && pressure.basis().frame_coordinate().is_some()
        && pressure.store_generation() == serving.residency_observation().store_generation()
        && pressure.scope() == PhysicalOperationAllocationScope::ForegroundWrite
        && pressure.dimension() == PhysicalResidencyDimension::SpeculativeFrames(kind);
    let pressure_retry_after_settlement =
        pressure.retry_posture() == PhysicalResidencyRetryPosture::AfterWritebackSettlement;
    let writebehind_attempts = paused
        .speculative_attempts(kind)
        .saturating_sub(baseline.residency.speculative_attempts(kind));
    let writebehind_admissions = paused
        .speculative_admissions(kind)
        .saturating_sub(baseline.residency.speculative_admissions(kind));
    let writebehind_denials = paused
        .speculative_denials(kind)
        .saturating_sub(baseline.residency.speculative_denials(kind));
    let positioned_write_delta = context
        .attempts_for(MediaOperationRole::PositionedWrite)
        .saturating_sub(baseline.positioned_writes);
    let cleanup_deletions = context.deletions().saturating_sub(baseline.deletions);
    let cleanup_complete = cleanup_deletions > 0 && serving.publication_residue().is_empty();
    let candidate_publications_at_dispatch = at_dispatch
        .candidate_publications()
        .saturating_sub(baseline.residency.candidate_publications());
    let candidate_publications_after_denial = paused
        .candidate_publications()
        .saturating_sub(baseline.residency.candidate_publications());
    if dispatch.role() != MediaOperationRole::PositionedWrite
        || dispatch.identified_operation_ordinal()
            != Some(super::CANDIDATE_WRITEBACK_POSITIONED_WRITE_ORDINAL)
        || dispatch.store() != Some(serving.store_identity())
        || dispatch.runtime_incarnation() != Some(serving.runtime_identity().get())
        || dispatch.operation().is_none()
        || at_dispatch.dirty_frames() != 1
        || at_dispatch.active_writeback_claims() != 1
        || at_dispatch.active_speculative_frames(kind) != 1
        || paused.dirty_frames() != 1
        || paused.peak_dirty_frames() != 2
        || paused.active_writeback_claims() != 1
        || paused.active_speculative_frames(kind) != 1
        || paused.peak_speculative_frames(kind) != 1
        || writebehind_attempts != 2
        || writebehind_admissions != 1
        || writebehind_denials != 1
        || positioned_write_delta != 3
        || pressure.requested() != 1
        || pressure.admitted() != 1
        || pressure.limit() != 1
        || pressure.effect_may_have_started()
        || !pressure_basis_exact
        || !pressure_retry_after_settlement
        || !cleanup_complete
        || candidate_publications_at_dispatch != 1
        || candidate_publications_after_denial
            != candidate_publications_at_dispatch.saturating_add(1)
    {
        return Err(format!(
            "ordinary append did not retain exact paused pressure posture: \
             dispatch={dispatch:?}, at_dispatch={at_dispatch:?}, paused={paused:?}, \
             pressure={pressure:?}, writebehind_attempts={writebehind_attempts}, \
             writebehind_admissions={writebehind_admissions}, \
             writebehind_denials={writebehind_denials}, \
             positioned_write_delta={positioned_write_delta}, \
             cleanup_deletions={cleanup_deletions}, cleanup_complete={cleanup_complete}, \
             candidate_publications_at_dispatch={candidate_publications_at_dispatch}, \
             candidate_publications_after_denial={candidate_publications_after_denial}"
        ));
    }
    Ok(PausedAppendPressure {
        dirty_at_dispatch: at_dispatch.dirty_frames(),
        dirty_after_denial: paused.dirty_frames(),
        active_claims_at_dispatch: at_dispatch.active_writeback_claims(),
        active_writebehind_at_dispatch: at_dispatch.active_speculative_frames(kind),
        pressure_basis_exact,
        pressure_retry_after_settlement,
        cleanup_deletions,
        cleanup_complete,
        candidate_publications_at_dispatch,
        candidate_publications_after_denial,
    })
}
