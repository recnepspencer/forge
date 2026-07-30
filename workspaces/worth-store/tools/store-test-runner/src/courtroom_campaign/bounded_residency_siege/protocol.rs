use std::num::NonZeroU32;

mod cancellation;
mod decoding;
mod generation_fencing;
mod work_reconciliation;

pub(super) use cancellation::{
    BoundedCancellationCaseObservation, BoundedCancellationDispatch, BoundedCancellationObligation,
    BoundedCancellationObservation, BoundedCancellationRecovery, BoundedCancellationSeam,
    BoundedCancellationSignal, BoundedCancellationTerminal,
};
pub(super) use decoding::parse;
#[cfg(test)]
pub(super) use decoding::parse_dirty;
pub(super) use generation_fencing::{
    BoundedResidencyGenerationCleanup, BoundedResidencyGenerationDenial,
    BoundedResidencyGenerationFenceCase, BoundedResidencyGenerationFenceEffects,
    BoundedResidencyGenerationFencingObservation,
};
#[cfg(test)]
pub(super) use work_reconciliation::exact_route_fixture;
pub(super) use work_reconciliation::{
    BoundedResidencyMediaRole, BoundedResidencySchedulerEvidenceClass,
    BoundedResidencySchedulerProfile, BoundedResidencySignalAspectRole,
    BoundedResidencySignalBindingObservation, BoundedResidencySignalFamily,
    BoundedResidencySignalFamilySet, BoundedResidencySignalLineageObservation,
    BoundedResidencySignalSettlement, BoundedResidencyWorkEffectFate, BoundedResidencyWorkFamily,
    BoundedResidencyWorkReconciliationObservation, BoundedResidencyWorkRecordObservation,
    BoundedResidencyWorkRecovery, BoundedResidencyWorkRouteObservation,
    BoundedResidencyWorkTerminalFate,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct BoundedResidencyScopeObservation {
    pub(super) admitted_scopes: u32,
    pub(super) exact_scope_denials: u32,
    pub(super) global_envelope_denied: bool,
    pub(super) global_denial_requested: u64,
    pub(super) global_denial_current: u64,
    pub(super) global_denial_limit: u64,
    pub(super) peak_operation_bytes: u64,
    pub(super) terminal_operation_bytes: u64,
    pub(super) all_effect_free: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct BoundedResidencyAllocationDimensionObservation {
    pub(super) name: &'static str,
    pub(super) attempts: u64,
    pub(super) admissions: u64,
    pub(super) releases: u64,
    pub(super) denials: u64,
    pub(super) allocator_failures: u64,
    pub(super) admitted_units: u64,
    pub(super) released_units: u64,
    pub(super) denied_units: u64,
    pub(super) active_units: u64,
    pub(super) current_units: u64,
    pub(super) peak_units: u64,
    pub(super) limit_units: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct BoundedResidencyAllocationObservation {
    pub(super) scopes: BoundedResidencyScopeObservation,
    pub(super) dimensions: [BoundedResidencyAllocationDimensionObservation; 19],
    pub(super) trace: BoundedResidencyAllocationTraceObservation,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct BoundedResidencyAllocationTraceObservation {
    pub(super) store: [u8; 16],
    pub(super) pool_incarnation: u64,
    pub(super) event_count: u64,
    pub(super) process: u32,
    pub(super) attributed_actualizations: u64,
    pub(super) events: Box<[BoundedResidencyAllocationBoundaryObservation]>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct BoundedResidencyAllocationBoundaryObservation {
    pub(super) sequence: u64,
    pub(super) kind: &'static str,
    pub(super) dimension: &'static str,
    pub(super) scope: Option<&'static str>,
    pub(super) requested_units: u64,
    pub(super) actual_units: u64,
    pub(super) process: u32,
    pub(super) physical_operation: Option<u64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct BoundedResidencyProcessAllocationObservation {
    pub(super) process: NonZeroU32,
    pub(super) largest_successful_request_bytes: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct BoundedResidencyReadObservation {
    pub(super) cold_effects: u64,
    pub(super) hot_effects: u64,
    pub(super) refault_effects: u64,
    pub(super) cold_metadata_effects: u64,
    pub(super) hot_metadata_effects: u64,
    pub(super) refault_metadata_effects: u64,
    pub(super) cold_work: u64,
    pub(super) hot_work: u64,
    pub(super) refault_work: u64,
    pub(super) physical_work: u64,
    pub(super) positioned_read_effects: u64,
    pub(super) metadata_read_effects: u64,
    pub(super) metadata_read_work_declared: u64,
    pub(super) metadata_read_work_dispatched: u64,
    pub(super) metadata_read_work_terminal: u64,
    pub(super) range_read_work_declared: u64,
    pub(super) range_read_work_dispatched: u64,
    pub(super) range_read_work_terminal: u64,
    pub(super) first_operation: u64,
    pub(super) last_operation: u64,
    pub(super) runtime_bound: bool,
    pub(super) peak_resident_bytes: u64,
    pub(super) peak_admitted_bytes: u64,
    pub(super) faults: u64,
    pub(super) source_loads: u64,
    pub(super) hits: u64,
    pub(super) evictions: u64,
    pub(super) caller_copy_operations: u64,
    pub(super) caller_copied_bytes: u64,
    pub(super) store_copy_operations: u64,
    pub(super) store_copied_bytes: u64,
    pub(super) peak_copy_width: u64,
    pub(super) store_maximum_copy_width: u64,
    pub(super) streaming_scratch_bytes: u64,
    pub(super) largest_record_bytes: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct BoundedResidencyPinObservation {
    pub(super) views: u32,
    pub(super) unique_frame_identities: u32,
    pub(super) zero_copy_events: u64,
    pub(super) peak_pinned_frames: u32,
    pub(super) peak_pin_leases: u32,
    pub(super) basis_matched: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct BoundedResidencyPinnedEvictionObservation {
    pub(super) forced_evictions: u64,
    pub(super) pinned_frames_before: u32,
    pub(super) pinned_frames_after: u32,
    pub(super) pin_leases_before: u32,
    pub(super) pin_leases_after: u32,
    pub(super) bases_preserved: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct BoundedResidencyDuplicateFaultObservation {
    pub(super) faults: u64,
    pub(super) source_loads: u64,
    pub(super) coalesced_waiters: u64,
    pub(super) pinned_frames: u32,
    pub(super) pin_leases: u32,
    pub(super) positioned_reads: u64,
    pub(super) owner_work: u64,
    pub(super) waiter_work: u64,
    pub(super) same_frame: bool,
    pub(super) same_prefix: bool,
    pub(super) waiter_created_work: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct BoundedResidencyDirtyObservation {
    pub(super) primary_publication: u64,
    pub(super) retry_publication: u64,
    pub(super) primary_candidate_writebacks: u64,
    pub(super) retry_candidate_writebacks: u64,
    pub(super) primary_candidate_publications: u64,
    pub(super) retry_candidate_publications: u64,
    pub(super) denied_candidate_publications: u64,
    pub(super) primary_last_candidate_operation: u64,
    pub(super) retry_last_candidate_operation: u64,
    pub(super) primary_records: u64,
    pub(super) retry_records: u64,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct BoundedResidencySpeculativeKindObservation {
    pub(super) attempts: u64,
    pub(super) admissions: u64,
    pub(super) denials: u64,
    pub(super) completions: u64,
    pub(super) peak_frames: u32,
    pub(super) terminal_frames: u32,
    pub(super) hits: u64,
    pub(super) effectful_misses: u64,
    pub(super) hit_signal_requests: u64,
    pub(super) denial_signal_requests: u64,
    pub(super) effectful_signal_requests: u64,
    pub(super) signal_family_exact: bool,
    pub(super) foundational_basis_exact: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct BoundedResidencySpeculationObservation {
    pub(super) prefetch: BoundedResidencySpeculativeKindObservation,
    pub(super) read_ahead: BoundedResidencySpeculativeKindObservation,
    pub(super) write_behind: BoundedResidencySpeculativeKindObservation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct BoundedResidencyCloseObservation {
    pub(super) inspection_required: bool,
    pub(super) resident_bytes: u64,
    pub(super) pinned_frames: u32,
    pub(super) pin_leases: u32,
    pub(super) dirty_frames: u32,
    pub(super) peak_resident_bytes: u64,
    pub(super) peak_admitted_bytes: u64,
    pub(super) peak_dirty_frames: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct BoundedResidencySiegeObservation {
    process: NonZeroU32,
    store: [u8; 16],
    runtime: u64,
    generation: u64,
    records: u64,
    payload_bytes: u64,
    directory_bytes: u64,
    resident_budget: u64,
    pub(super) schedule: [super::schedule::ScheduleDecision; 4],
    pub(super) process_allocation: BoundedResidencyProcessAllocationObservation,
    pub(super) reads: BoundedResidencyReadObservation,
    pub(super) pins: BoundedResidencyPinObservation,
    pub(super) pinned_eviction: BoundedResidencyPinnedEvictionObservation,
    pub(super) duplicate: BoundedResidencyDuplicateFaultObservation,
    pub(super) cancellation: BoundedCancellationObservation,
    pub(super) generation_fencing: BoundedResidencyGenerationFencingObservation,
    pub(super) dirty: BoundedResidencyDirtyObservation,
    pub(super) speculation: BoundedResidencySpeculationObservation,
    pub(super) work_reconciliation: BoundedResidencyWorkReconciliationObservation,
    pub(super) allocation: BoundedResidencyAllocationObservation,
    pub(super) close: BoundedResidencyCloseObservation,
}

impl BoundedResidencySiegeObservation {
    pub(super) const fn process(&self) -> NonZeroU32 {
        self.process
    }

    pub(super) const fn store(&self) -> [u8; 16] {
        self.store
    }

    pub(super) const fn runtime(&self) -> u64 {
        self.runtime
    }

    pub(super) const fn generation(&self) -> u64 {
        self.generation
    }

    pub(super) const fn records(&self) -> u64 {
        self.records
    }

    pub(super) const fn payload_bytes(&self) -> u64 {
        self.payload_bytes
    }

    pub(super) const fn directory_bytes(&self) -> u64 {
        self.directory_bytes
    }

    pub(super) const fn resident_budget(&self) -> u64 {
        self.resident_budget
    }
}

#[cfg(test)]
mod tests;
