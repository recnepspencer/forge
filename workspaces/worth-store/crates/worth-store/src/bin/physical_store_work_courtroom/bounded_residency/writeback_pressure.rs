mod append_batch;
mod append_pressure;
mod trace;

use worth_store::physical_runtime::{PhysicalSpeculativeWorkKind, ServingPhysicalRuntime};
use worth_store_physical_backend::{MediaOperationRole, MediaPauseGate};

use super::configuration::BoundedResidencyConfiguration;

pub(super) const CANDIDATE_WRITEBACK_POSITIONED_WRITE_ORDINAL: u64 = 2;

#[derive(Debug)]
pub(super) struct BoundedDirtyWritebackEvidence {
    pub(super) primary_publication: u64,
    pub(super) retry_publication: u64,
    pub(super) primary_candidate_writebacks: u64,
    pub(super) retry_candidate_writebacks: u64,
    pub(super) primary_candidate_publications: u64,
    pub(super) retry_candidate_publications: u64,
    pub(super) denied_candidate_publications: u64,
    pub(super) primary_last_candidate_operation: u64,
    pub(super) retry_last_candidate_operation: u64,
    pub(super) dirty_at_dispatch: u32,
    pub(super) dirty_peak: u32,
    pub(super) dirty_after_denial: u32,
    pub(super) dirty_after_primary: u32,
    pub(super) dirty_terminal: u32,
    pub(super) active_claims_at_dispatch: u32,
    pub(super) active_writebehind_at_dispatch: u32,
    pub(super) peak_writebehind: u32,
    pub(super) terminal_writebehind: u32,
    pub(super) pressure_requested: u64,
    pub(super) pressure_admitted: u64,
    pub(super) pressure_limit: u64,
    pub(super) pressure_basis_exact: bool,
    pub(super) pressure_retry_after_settlement: bool,
    pub(super) pressure_effect_free: bool,
    pub(super) cleanup_deletions: u64,
    pub(super) cleanup_complete: bool,
    pub(super) primary_records: u64,
    pub(super) retry_records: u64,
    pub(super) writebehind_attempts: u64,
    pub(super) writebehind_admissions: u64,
    pub(super) writebehind_denials: u64,
    pub(super) writebehind_completions: u64,
    pub(super) writeback_attempts: u64,
    pub(super) exact_receipts: u64,
    pub(super) retryable_writebacks: u64,
    pub(super) indeterminate_writebacks: u64,
    pub(super) inspection_required_writebacks: u64,
    pub(super) candidate_publications: u64,
    pub(super) writebacks: u64,
    pub(super) positioned_writes: u64,
}

pub(super) fn prove(
    serving: &ServingPhysicalRuntime,
    configuration: BoundedResidencyConfiguration,
    gate: MediaPauseGate,
) -> Result<BoundedDirtyWritebackEvidence, String> {
    let run = append_pressure::execute(serving, configuration, gate)?;
    let traces = CandidateWritebackTraces {
        primary: trace::candidate_writebacks(&run.primary)?,
        retry: trace::candidate_writebacks(&run.retry)?,
    };
    build_evidence(serving, run, traces)
}

struct CandidateWritebackTraces {
    primary: trace::CandidateWritebackTrace,
    retry: trace::CandidateWritebackTrace,
}

fn build_evidence(
    serving: &ServingPhysicalRuntime,
    run: append_pressure::OrdinaryAppendPressure,
    traces: CandidateWritebackTraces,
) -> Result<BoundedDirtyWritebackEvidence, String> {
    let append_pressure::OrdinaryAppendPressure {
        primary,
        retry,
        baseline,
        paused,
        pressure,
        dirty_after_primary,
        primary_candidate_publications,
        retry_candidate_publications,
        denied_candidate_publications,
    } = run;
    let after = serving.residency_observation().counters();
    let writebacks = serving.residency_observation().writebacks();
    let kind = PhysicalSpeculativeWorkKind::WriteBehind;
    let evidence = BoundedDirtyWritebackEvidence {
        primary_publication: primary.publication_identity(),
        retry_publication: retry.publication_identity(),
        primary_candidate_writebacks: traces.primary.count,
        retry_candidate_writebacks: traces.retry.count,
        primary_candidate_publications,
        retry_candidate_publications,
        denied_candidate_publications,
        primary_last_candidate_operation: traces.primary.last_operation,
        retry_last_candidate_operation: traces.retry.last_operation,
        dirty_at_dispatch: paused.dirty_at_dispatch,
        dirty_peak: after.peak_dirty_frames(),
        dirty_after_denial: paused.dirty_after_denial,
        dirty_after_primary,
        dirty_terminal: after.dirty_frames(),
        active_claims_at_dispatch: paused.active_claims_at_dispatch,
        active_writebehind_at_dispatch: paused.active_writebehind_at_dispatch,
        peak_writebehind: after.peak_speculative_frames(kind),
        terminal_writebehind: after.active_speculative_frames(kind),
        pressure_requested: pressure.requested(),
        pressure_admitted: pressure.admitted(),
        pressure_limit: pressure.limit(),
        pressure_basis_exact: paused.pressure_basis_exact,
        pressure_retry_after_settlement: paused.pressure_retry_after_settlement,
        pressure_effect_free: !pressure.effect_may_have_started(),
        cleanup_deletions: paused.cleanup_deletions,
        cleanup_complete: paused.cleanup_complete,
        primary_records: primary.record_ids().len() as u64,
        retry_records: retry.record_ids().len() as u64,
        writebehind_attempts: delta(
            after.speculative_attempts(kind),
            baseline.residency.speculative_attempts(kind),
        ),
        writebehind_admissions: delta(
            after.speculative_admissions(kind),
            baseline.residency.speculative_admissions(kind),
        ),
        writebehind_denials: delta(
            after.speculative_denials(kind),
            baseline.residency.speculative_denials(kind),
        ),
        writebehind_completions: delta(
            after.speculative_completions(kind),
            baseline.residency.speculative_completions(kind),
        ),
        writeback_attempts: delta(writebacks.attempts(), baseline.writebacks.attempts()),
        exact_receipts: delta(
            writebacks.exact_receipts(),
            baseline.writebacks.exact_receipts(),
        ),
        retryable_writebacks: delta(writebacks.retryable(), baseline.writebacks.retryable()),
        indeterminate_writebacks: delta(
            writebacks.indeterminate(),
            baseline.writebacks.indeterminate(),
        ),
        inspection_required_writebacks: delta(
            writebacks.inspection_required(),
            baseline.writebacks.inspection_required(),
        ),
        candidate_publications: delta(
            after.candidate_publications(),
            baseline.residency.candidate_publications(),
        ),
        writebacks: delta(after.writebacks(), baseline.residency.writebacks()),
        positioned_writes: delta(positioned_writes(serving), baseline.positioned_writes),
    };
    validate(evidence)
}

fn validate(
    evidence: BoundedDirtyWritebackEvidence,
) -> Result<BoundedDirtyWritebackEvidence, String> {
    let candidate_writebacks = evidence
        .primary_candidate_writebacks
        .saturating_add(evidence.retry_candidate_writebacks);
    if evidence.primary_publication == 0
        || evidence.retry_publication == 0
        || evidence.primary_publication == evidence.retry_publication
        || evidence.primary_candidate_writebacks == 0
        || evidence.retry_candidate_writebacks == 0
        || evidence.primary_candidate_publications <= evidence.primary_candidate_writebacks
        || evidence.retry_candidate_publications <= evidence.retry_candidate_writebacks
        || evidence.primary_last_candidate_operation == 0
        || evidence.retry_last_candidate_operation <= evidence.primary_last_candidate_operation
        || evidence.primary_records != 1
        || evidence.retry_records != 1
    {
        return Err(format!(
            "ordinary append identities did not reconcile: {evidence:?}"
        ));
    }
    if evidence.dirty_at_dispatch != 1
        || evidence.dirty_peak != 2
        || evidence.dirty_after_denial != 1
        || evidence.dirty_after_primary != 0
        || evidence.dirty_terminal != 0
        || evidence.active_claims_at_dispatch != 1
    {
        return Err(format!(
            "ordinary append dirty saturation did not reconcile: {evidence:?}"
        ));
    }
    if evidence.active_writebehind_at_dispatch != 1
        || evidence.peak_writebehind != 1
        || evidence.terminal_writebehind != 0
        || evidence.pressure_requested != 1
        || evidence.pressure_admitted != 1
        || evidence.pressure_limit != 1
        || !evidence.pressure_basis_exact
        || !evidence.pressure_retry_after_settlement
        || !evidence.pressure_effect_free
        || evidence.cleanup_deletions == 0
        || !evidence.cleanup_complete
    {
        return Err(format!(
            "ordinary append write-behind pressure did not reconcile: {evidence:?}"
        ));
    }
    if evidence.writebehind_attempts != candidate_writebacks.saturating_add(1)
        || evidence.writebehind_admissions != candidate_writebacks
        || evidence.writebehind_denials != 1
        || evidence.writebehind_completions != candidate_writebacks
        || evidence.writeback_attempts != candidate_writebacks.saturating_add(1)
        || evidence.exact_receipts != candidate_writebacks
        || evidence.retryable_writebacks != 0
        || evidence.indeterminate_writebacks != 0
        || evidence.inspection_required_writebacks != 0
        || evidence.denied_candidate_publications != 1
        || evidence.candidate_publications
            != evidence
                .primary_candidate_publications
                .saturating_add(evidence.retry_candidate_publications)
                .saturating_add(evidence.denied_candidate_publications)
        || evidence.writebacks != candidate_writebacks
        || evidence.positioned_writes < candidate_writebacks
    {
        return Err(format!(
            "ordinary append writeback counters did not reconcile: {evidence:?}"
        ));
    }
    Ok(evidence)
}

fn delta(after: u64, before: u64) -> u64 {
    after.saturating_sub(before)
}

pub(super) fn positioned_writes(serving: &ServingPhysicalRuntime) -> u64 {
    serving
        .media_counters()
        .attempts_for(MediaOperationRole::PositionedWrite)
}
