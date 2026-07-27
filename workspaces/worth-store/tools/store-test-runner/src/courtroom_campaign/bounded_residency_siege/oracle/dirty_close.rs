use crate::courtroom_campaign::bounded_residency_siege::protocol::BoundedResidencySiegeObservation;

pub(super) fn verify_dirty_and_close(
    child: BoundedResidencySiegeObservation,
) -> Result<(), String> {
    let dirty = child.dirty;
    if dirty.work_operation == 0
        || dirty.source_work_count != 1
        || dirty.first_source_operation == 0
        || dirty.first_source_operation != dirty.last_source_operation
        || dirty.work_operation == dirty.first_source_operation
        || dirty.backend_operation == 0
        || dirty.dirty_at_pause != 1
        || dirty.dirty_after_receipt != 0
        || dirty.positioned_writes != 1
        || dirty.candidate_publications != 1
        || dirty.writebacks != 1
        || dirty.active_claims_at_pause != 1
        || dirty.eviction_releases_at_pause != 0
        || !dirty.competing_claim_denied
        || !dirty.cancellation_settlement_continues
        || dirty.writeback_attempts != 1
        || dirty.exact_receipts != 1
        || dirty.retryable_writebacks != 0
        || dirty.indeterminate_writebacks != 0
        || dirty.inspection_required_writebacks != 0
    {
        return Err("Courtroom C dirty work did not remain dirty through exact receipt".into());
    }
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
