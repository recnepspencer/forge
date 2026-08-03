use crate::courtroom_campaign::bounded_residency_siege::protocol::{
    BoundedResidencyDirtyObservation, BoundedResidencySiegeObservation,
};

#[cfg(test)]
#[path = "dirty_close/tests.rs"]
mod tests;

pub(super) fn verify_dirty_and_close(
    child: &BoundedResidencySiegeObservation,
) -> Result<(), String> {
    verify_dirty(child.dirty)?;
    let close = child.close;
    if close.inspection_required
        || close.resident_bytes != 0
        || close.pinned_frames != 0
        || close.pin_leases != 0
        || close.dirty_frames != 0
    {
        return Err("Courtroom C close retained residency or inspection posture".into());
    }
    Ok(())
}

pub(super) fn verify_dirty(dirty: BoundedResidencyDirtyObservation) -> Result<(), String> {
    verify_publications(dirty)?;
    verify_dirty_saturation(dirty)?;
    verify_writebehind_pressure(dirty)?;
    verify_writeback_settlement(dirty)
}

fn verify_publications(dirty: BoundedResidencyDirtyObservation) -> Result<(), String> {
    if dirty.primary_publication == 0
        || dirty.retry_publication == 0
        || dirty.primary_publication == dirty.retry_publication
        || dirty.primary_candidate_writebacks == 0
        || dirty.retry_candidate_writebacks == 0
        || dirty.primary_candidate_publications <= dirty.primary_candidate_writebacks
        || dirty.retry_candidate_publications <= dirty.retry_candidate_writebacks
        || dirty.denied_candidate_publications != 1
        || dirty.primary_last_candidate_operation == 0
        || dirty.retry_last_candidate_operation <= dirty.primary_last_candidate_operation
        || dirty.primary_records != 1
        || dirty.retry_records != 1
    {
        return Err("Courtroom C ordinary append publications did not reconcile".into());
    }
    Ok(())
}

fn verify_dirty_saturation(dirty: BoundedResidencyDirtyObservation) -> Result<(), String> {
    if dirty.dirty_at_dispatch != 1
        || dirty.dirty_peak != 2
        || dirty.dirty_after_denial != 1
        || dirty.dirty_after_primary != 0
        || dirty.dirty_terminal != 0
        || dirty.active_claims_at_dispatch != 1
    {
        return Err("Courtroom C dirty-frame saturation or cleanup did not reconcile".into());
    }
    Ok(())
}

fn verify_writebehind_pressure(dirty: BoundedResidencyDirtyObservation) -> Result<(), String> {
    if dirty.active_writebehind_at_dispatch != 1
        || dirty.peak_writebehind != 1
        || dirty.terminal_writebehind != 0
        || dirty.pressure_requested != 1
        || dirty.pressure_admitted != 1
        || dirty.pressure_limit != 1
        || !dirty.pressure_basis_exact
        || !dirty.pressure_retry_after_settlement
        || !dirty.pressure_effect_free
        || dirty.cleanup_deletions == 0
        || !dirty.cleanup_complete
    {
        return Err("Courtroom C write-behind saturation did not reconcile".into());
    }
    Ok(())
}

fn verify_writeback_settlement(dirty: BoundedResidencyDirtyObservation) -> Result<(), String> {
    let candidate_writebacks = dirty
        .primary_candidate_writebacks
        .saturating_add(dirty.retry_candidate_writebacks);
    if dirty.writebehind_attempts != candidate_writebacks.saturating_add(1)
        || dirty.writebehind_admissions != candidate_writebacks
        || dirty.writebehind_denials != 1
        || dirty.writebehind_completions != candidate_writebacks
        || dirty.writeback_attempts != candidate_writebacks.saturating_add(1)
        || dirty.exact_receipts != candidate_writebacks
        || dirty.retryable_writebacks != 0
        || dirty.indeterminate_writebacks != 0
        || dirty.inspection_required_writebacks != 0
        || dirty.candidate_publications
            != dirty
                .primary_candidate_publications
                .saturating_add(dirty.retry_candidate_publications)
                .saturating_add(dirty.denied_candidate_publications)
        || dirty.writebacks != candidate_writebacks
        || dirty.positioned_writes < candidate_writebacks
    {
        return Err("Courtroom C exact writeback settlement did not reconcile".into());
    }
    Ok(())
}
