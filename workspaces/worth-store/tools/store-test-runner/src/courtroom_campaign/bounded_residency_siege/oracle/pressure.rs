use super::super::{
    protocol::{
        BoundedResidencyDuplicateFaultObservation, BoundedResidencyPinnedEvictionObservation,
        BoundedResidencyReadObservation, BoundedResidencySiegeObservation,
    },
    world::{BoundedResidencySiegeWorld, DIRTY_FRAMES, PINNED_FRAMES, PIN_LEASES, RESIDENT_BYTES},
};

mod dirty_close;

use dirty_close::verify_dirty_and_close;

const EVICTION_PROTECTED_FRAMES: u32 = 3;

pub(super) fn verify_residency(
    world: &BoundedResidencySiegeWorld,
    child: &BoundedResidencySiegeObservation,
) -> Result<(), String> {
    let reads = child.reads;
    if child.resident_budget() != RESIDENT_BYTES {
        return Err("Courtroom C child used a foreign resident-byte budget".into());
    }
    verify_read_pressure(reads, world.admitted_byte_limit())?;
    let pins = child.pins;
    if pins.views != PIN_LEASES
        || pins.unique_frame_identities != PINNED_FRAMES
        || pins.zero_copy_events != 0
        || pins.peak_pinned_frames != PINNED_FRAMES
        || pins.peak_pin_leases != PIN_LEASES
        || !pins.basis_matched
    {
        return Err("Courtroom C public-view pressure did not prove its bounded handoff".into());
    }
    verify_pinned_eviction(child.pinned_eviction)?;
    verify_duplicate_fault(child.duplicate)?;
    let close = child.close;
    if close.peak_resident_bytes > RESIDENT_BYTES
        || close.peak_admitted_bytes > world.admitted_byte_limit()
        || close.peak_dirty_frames > DIRTY_FRAMES
        || close.peak_resident_bytes < reads.peak_resident_bytes
        || close.peak_admitted_bytes < reads.peak_admitted_bytes
    {
        return Err("Courtroom C final memory peaks escaped or omitted the joined siege".into());
    }
    verify_dirty_and_close(child)
}

fn verify_pinned_eviction(
    eviction: BoundedResidencyPinnedEvictionObservation,
) -> Result<(), String> {
    if eviction.forced_evictions == 0
        || eviction.pinned_frames_before != EVICTION_PROTECTED_FRAMES
        || eviction.pinned_frames_after != EVICTION_PROTECTED_FRAMES
        || eviction.pin_leases_before != EVICTION_PROTECTED_FRAMES
        || eviction.pin_leases_after != EVICTION_PROTECTED_FRAMES
        || !eviction.bases_preserved
    {
        return Err(
            "Courtroom C forced eviction did not preserve exact pinned-frame authority".into(),
        );
    }
    Ok(())
}

fn verify_duplicate_fault(
    duplicate: BoundedResidencyDuplicateFaultObservation,
) -> Result<(), String> {
    if duplicate.faults != 1
        || duplicate.source_loads != 1
        || duplicate.coalesced_waiters != 1
        || duplicate.pinned_frames != 1
        || duplicate.pin_leases != 2
        || duplicate.positioned_reads != 1
        || duplicate.owner_work == 0
        || duplicate.waiter_work != 0
        || !duplicate.same_frame
        || !duplicate.same_prefix
        || duplicate.waiter_created_work
    {
        return Err(
            "Courtroom C duplicate cold reads did not share one fault and source load".into(),
        );
    }
    Ok(())
}

fn verify_read_pressure(
    reads: BoundedResidencyReadObservation,
    admitted_byte_limit: u64,
) -> Result<(), String> {
    verify_read_capacity(&reads, admitted_byte_limit)?;
    verify_cold_read(&reads)?;
    verify_hot_read(&reads)?;
    verify_refault(&reads)?;
    verify_read_work_lifecycle(&reads)?;
    verify_read_residency_lifecycle(&reads)?;
    verify_read_copy_accounting(&reads)
}

fn verify_read_capacity(
    reads: &BoundedResidencyReadObservation,
    admitted_byte_limit: u64,
) -> Result<(), String> {
    if reads.peak_resident_bytes > RESIDENT_BYTES || reads.peak_admitted_bytes > admitted_byte_limit
    {
        return Err("Courtroom C read residency exceeded admitted capacity".into());
    }
    Ok(())
}

fn verify_cold_read(reads: &BoundedResidencyReadObservation) -> Result<(), String> {
    if reads.cold_effects == 0
        || reads.cold_work != reads.cold_metadata_effects
        || reads.cold_effects >= reads.cold_work
    {
        return Err("Courtroom C cold read work did not reconcile".into());
    }
    Ok(())
}

fn verify_hot_read(reads: &BoundedResidencyReadObservation) -> Result<(), String> {
    if reads.hot_effects != 0 || reads.hot_metadata_effects != 0 || reads.hot_work != 0 {
        return Err("Courtroom C hot read created physical work or effects".into());
    }
    Ok(())
}

fn verify_refault(reads: &BoundedResidencyReadObservation) -> Result<(), String> {
    if reads.refault_effects == 0
        || reads.refault_work != reads.refault_metadata_effects
        || reads.refault_effects >= reads.refault_work
    {
        return Err("Courtroom C refault work did not reconcile".into());
    }
    Ok(())
}

fn verify_read_work_lifecycle(reads: &BoundedResidencyReadObservation) -> Result<(), String> {
    let active_declared_work = reads
        .metadata_read_work_declared
        .checked_add(reads.range_read_work_declared);
    let active_dispatched_work = reads
        .metadata_read_work_dispatched
        .checked_add(reads.range_read_work_dispatched);
    let terminal_work = reads
        .metadata_read_work_terminal
        .checked_add(reads.range_read_work_terminal);
    let expected_metadata_effects = reads
        .metadata_read_work_terminal
        .checked_add(reads.range_read_work_terminal);
    let operation_span = reads
        .last_operation
        .checked_sub(reads.first_operation)
        .and_then(|difference| difference.checked_add(1));
    if reads.physical_work == 0
        || reads.first_operation == 0
        || operation_span != Some(reads.physical_work)
        || !reads.runtime_bound
        || active_declared_work != Some(0)
        || active_dispatched_work != Some(0)
        || terminal_work != Some(reads.physical_work)
        || reads.positioned_read_effects != reads.range_read_work_terminal
        || Some(reads.metadata_read_effects) != expected_metadata_effects
        || reads.range_read_work_declared != 0
        || reads.range_read_work_dispatched != 0
    {
        return Err("Courtroom C read work lifecycle did not reconcile".into());
    }
    Ok(())
}

fn verify_read_residency_lifecycle(reads: &BoundedResidencyReadObservation) -> Result<(), String> {
    if reads.range_read_work_terminal != reads.faults
        || reads.source_loads != reads.faults
        || reads.faults == 0
        || reads.hits == 0
        || reads.evictions == 0
    {
        return Err("Courtroom C read residency lifecycle did not reconcile".into());
    }
    Ok(())
}

fn verify_read_copy_accounting(reads: &BoundedResidencyReadObservation) -> Result<(), String> {
    if reads.caller_copy_operations == 0
        || reads.caller_copy_operations != reads.store_copy_operations
        || reads.caller_copied_bytes == 0
        || reads.caller_copied_bytes != reads.store_copied_bytes
        || reads.peak_copy_width == 0
        || reads.peak_copy_width != reads.store_maximum_copy_width
        || reads.peak_copy_width > reads.streaming_scratch_bytes
        || reads.streaming_scratch_bytes >= reads.largest_record_bytes
    {
        return Err("Courtroom C read copy accounting did not reconcile".into());
    }
    Ok(())
}

#[cfg(test)]
#[path = "pressure/tests/mod.rs"]
mod tests;
