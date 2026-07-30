use serde_json::{json, Value};

use super::super::protocol::{
    BoundedResidencyDuplicateFaultObservation, BoundedResidencyPinObservation,
    BoundedResidencyPinnedEvictionObservation, BoundedResidencyReadObservation,
};

pub(super) fn reads(reads: BoundedResidencyReadObservation) -> Value {
    json!({
        "cold_effects": reads.cold_effects,
        "hot_effects": reads.hot_effects,
        "refault_effects": reads.refault_effects,
        "cold_metadata_effects": reads.cold_metadata_effects,
        "hot_metadata_effects": reads.hot_metadata_effects,
        "refault_metadata_effects": reads.refault_metadata_effects,
        "cold_work": reads.cold_work,
        "hot_work": reads.hot_work,
        "refault_work": reads.refault_work,
        "physical_work": reads.physical_work,
        "positioned_read_effects": reads.positioned_read_effects,
        "metadata_read_effects": reads.metadata_read_effects,
        "metadata_read_work": {
            "declared": reads.metadata_read_work_declared,
            "dispatched": reads.metadata_read_work_dispatched,
            "terminal": reads.metadata_read_work_terminal,
        },
        "range_read_work": {
            "declared": reads.range_read_work_declared,
            "dispatched": reads.range_read_work_dispatched,
            "terminal": reads.range_read_work_terminal,
        },
        "first_operation": reads.first_operation,
        "last_operation": reads.last_operation,
        "runtime_bound": reads.runtime_bound,
        "peak_resident_bytes": reads.peak_resident_bytes,
        "peak_admitted_bytes": reads.peak_admitted_bytes,
        "faults": reads.faults,
        "source_loads": reads.source_loads,
        "hits": reads.hits,
        "evictions": reads.evictions,
        "bounded_copy": {
            "caller": {
                "operations": reads.caller_copy_operations,
                "bytes": reads.caller_copied_bytes,
                "maximum_width": reads.peak_copy_width,
            },
            "store": {
                "operations": reads.store_copy_operations,
                "bytes": reads.store_copied_bytes,
                "maximum_width": reads.store_maximum_copy_width,
            },
            "scratch_bytes": reads.streaming_scratch_bytes,
            "largest_record_bytes": reads.largest_record_bytes,
        },
    })
}

pub(super) fn pins(
    pins: BoundedResidencyPinObservation,
    eviction: BoundedResidencyPinnedEvictionObservation,
) -> Value {
    json!({
        "views": pins.views,
        "unique_frame_identities": pins.unique_frame_identities,
        "zero_copy_events": pins.zero_copy_events,
        "peak_pinned_frames": pins.peak_pinned_frames,
        "peak_pin_leases": pins.peak_pin_leases,
        "basis_matched": pins.basis_matched,
        "over_pin_denial": "pin-lease-budget-exceeded",
        "forced_eviction": {
            "evictions": eviction.forced_evictions,
            "pinned_frames_before": eviction.pinned_frames_before,
            "pinned_frames_after": eviction.pinned_frames_after,
            "pin_leases_before": eviction.pin_leases_before,
            "pin_leases_after": eviction.pin_leases_after,
            "bases_preserved": eviction.bases_preserved,
        },
    })
}

pub(super) fn duplicate_fault(duplicate: BoundedResidencyDuplicateFaultObservation) -> Value {
    json!({
        "faults": duplicate.faults,
        "source_loads": duplicate.source_loads,
        "coalesced_waiters": duplicate.coalesced_waiters,
        "pinned_frames": duplicate.pinned_frames,
        "pin_leases": duplicate.pin_leases,
        "positioned_reads": duplicate.positioned_reads,
        "owner_work": duplicate.owner_work,
        "waiter_work": duplicate.waiter_work,
        "same_frame": duplicate.same_frame,
        "same_prefix": duplicate.same_prefix,
        "waiter_created_work": duplicate.waiter_created_work,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pin_projection_retains_every_forced_eviction_fact() {
        let value = pins(
            BoundedResidencyPinObservation {
                views: 6,
                unique_frame_identities: 4,
                zero_copy_events: 0,
                peak_pinned_frames: 4,
                peak_pin_leases: 6,
                basis_matched: true,
            },
            BoundedResidencyPinnedEvictionObservation {
                forced_evictions: 9,
                pinned_frames_before: 3,
                pinned_frames_after: 3,
                pin_leases_before: 3,
                pin_leases_after: 3,
                bases_preserved: true,
            },
        );
        let eviction = &value["forced_eviction"];
        assert_eq!(eviction["evictions"], 9);
        assert_eq!(eviction["pinned_frames_before"], 3);
        assert_eq!(eviction["pinned_frames_after"], 3);
        assert_eq!(eviction["pin_leases_before"], 3);
        assert_eq!(eviction["pin_leases_after"], 3);
        assert_eq!(eviction["bases_preserved"], true);
    }
}
