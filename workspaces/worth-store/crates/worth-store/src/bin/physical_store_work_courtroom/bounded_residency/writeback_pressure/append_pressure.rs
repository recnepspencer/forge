use sha2::{Digest, Sha256};
use worth_proof::NonEmpty;
use worth_store::physical_runtime::{
    certification::CertificationDurableMutationInput, CompletedPhysicalRootPublication,
    PhysicalDataDispatchOutcome, PhysicalDurabilityGroupBasis, PhysicalManifestCapacityTransition,
    PhysicalMutationIdempotencyMaterial, PhysicalOperationAllocationScope,
    PhysicalRecordPressureEvidence, PhysicalResidencyCounterSnapshot, PhysicalResidencyDimension,
    PhysicalResidencyRetryPosture, PhysicalSpeculativeWorkKind, PhysicalWritebackCounterSnapshot,
    ServingPhysicalRuntime, WalDurablePhysicalMutation,
};
use worth_store_physical_backend::{MediaOperationContext, MediaOperationRole, MediaPauseGate};

use super::{append_batch, data_effects, dispatch_coordination, positioned_writes};
use crate::bounded_residency::configuration::BoundedResidencyConfiguration;

pub(super) struct AppendPressureBaseline {
    pub(super) residency: PhysicalResidencyCounterSnapshot,
    pub(super) writebacks: PhysicalWritebackCounterSnapshot,
    pub(super) positioned_writes: u64,
    pub(super) deletions: u64,
}

pub(super) struct OrdinaryAppendPressure {
    pub(super) completed: CompletedPhysicalRootPublication,
    pub(super) primary_trace: data_effects::CandidateWritebackTrace,
    pub(super) retry_trace: data_effects::CandidateWritebackTrace,
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

struct PreparedPressureGroup {
    basis: PhysicalDurabilityGroupBasis,
    primary: WalDurablePhysicalMutation,
    competing: WalDurablePhysicalMutation,
}

struct PausedEvidenceInput<'evidence> {
    serving: &'evidence ServingPhysicalRuntime,
    pressure: PhysicalRecordPressureEvidence,
    dispatch: MediaOperationContext,
    at_dispatch: PhysicalResidencyCounterSnapshot,
    baseline: &'evidence AppendPressureBaseline,
}

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
    let group = prepare_pressure_group(
        serving,
        configuration,
        placement,
        [primary_ordinal, denied_ordinal],
    )?;
    let (primary_thread, primary_receiver) = dispatch_coordination::spawn(serving, group.primary);
    let dispatch = match dispatch_coordination::await_backend_gate(&gate, &primary_receiver) {
        Ok(context) => context,
        Err(error) => {
            gate.release();
            let _ = primary_thread.join();
            return Err(error);
        }
    };
    let at_dispatch = serving.residency_observation().counters();
    let (denied_thread, denied_receiver) = dispatch_coordination::spawn(serving, group.competing);
    let denial =
        dispatch_coordination::receive_pressure_denial(&gate, denied_receiver, denied_thread)?;
    let pressure = denial.evidence;
    let paused_result = paused_evidence(PausedEvidenceInput {
        serving,
        pressure,
        dispatch,
        at_dispatch,
        baseline: &baseline,
    });
    gate.release();
    let primary_result = dispatch_coordination::receive_dispatched(
        primary_receiver,
        primary_thread,
        "primary mutation",
    );
    let paused = paused_result?;
    let primary_dispatched = primary_result?;
    let primary_trace = data_effects::candidate_writebacks(&primary_dispatched)?;
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
    let retry_dispatched = match serving
        .certification_record_submission()
        .dispatch_wal_durable_data(denial.durable)
    {
        PhysicalDataDispatchOutcome::Dispatched(dispatched) => dispatched,
        PhysicalDataDispatchOutcome::RetryableAfterCleanup(_) => {
            return Err("canonical pressure retry required repeated cleanup".to_owned())
        }
        PhysicalDataDispatchOutcome::NotStarted { cause, .. } => {
            return Err(format!(
                "canonical pressure retry remained not-started: {cause:?}"
            ))
        }
        PhysicalDataDispatchOutcome::Indeterminate(indeterminate) => {
            return Err(format!(
                "canonical pressure retry became indeterminate: {:?}",
                indeterminate.cause()
            ))
        }
    };
    let retry_trace = data_effects::candidate_writebacks(&retry_dispatched)?;
    let completed = serving.certification_complete_dispatched_group(
        group.basis,
        NonEmpty::new(primary_dispatched, vec![retry_dispatched]),
    );
    let retry_candidate_publications = serving
        .residency_observation()
        .counters()
        .candidate_publications()
        .saturating_sub(after_primary.candidate_publications());
    Ok(OrdinaryAppendPressure {
        completed,
        primary_trace,
        retry_trace,
        baseline,
        paused,
        pressure,
        dirty_after_primary,
        primary_candidate_publications,
        retry_candidate_publications,
        denied_candidate_publications,
    })
}

fn prepare_pressure_group(
    serving: &ServingPhysicalRuntime,
    configuration: BoundedResidencyConfiguration,
    placement: worth_store::physical_runtime::AdmittedRecordPlacementPolicy,
    ordinals: [usize; 2],
) -> Result<PreparedPressureGroup, String> {
    let [primary_ordinal, competing_ordinal] = ordinals;
    let (basis, durable_group) = serving.certification_prepare_wal_durable_group(
        placement,
        PhysicalManifestCapacityTransition::PreserveCurrent,
        NonEmpty::new(
            CertificationDurableMutationInput::new(
                mutation_material(primary_ordinal),
                append_batch::build(configuration, primary_ordinal)?,
            ),
            vec![CertificationDurableMutationInput::new(
                mutation_material(competing_ordinal),
                append_batch::build(configuration, competing_ordinal)?,
            )],
        ),
    );
    let mut durable_group = durable_group.into_vec().into_iter();
    let primary = durable_group
        .next()
        .expect("the primary group member must remain present");
    let competing = durable_group
        .next()
        .expect("the pressure group member must remain present");
    if durable_group.next().is_some() {
        return Err("canonical pressure group gained an undeclared member".to_owned());
    }
    Ok(PreparedPressureGroup {
        basis,
        primary,
        competing,
    })
}

fn mutation_material(ordinal: usize) -> PhysicalMutationIdempotencyMaterial {
    let mut digest = Sha256::new();
    digest.update(b"worth-store.bounded-residency.writeback-pressure.v1");
    digest.update((ordinal as u64).to_le_bytes());
    PhysicalMutationIdempotencyMaterial::new(digest.finalize().into())
}

fn paused_evidence(input: PausedEvidenceInput<'_>) -> Result<PausedAppendPressure, String> {
    let PausedEvidenceInput {
        serving,
        pressure,
        dispatch,
        at_dispatch,
        baseline,
    } = input;
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
            "canonical mutation did not retain exact paused pressure posture: \
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
