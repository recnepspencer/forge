use std::io::Write;

use worth_store::physical_runtime::{
    ClosedRuntime, LifecycleGeneration, PhysicalWorkFilesystemProfileEvidence, RuntimeIdentity,
    ServingShutdownOutcome,
};
use worth_store_physical_format::store_namespace::StableStoreIdentity;

use super::super::process_allocation::ProcessAllocationEvidence;
use super::{
    allocation_pressure::{
        AllocationBoundaryEventEvidence, AllocationBoundaryTraceEvidence,
        AllocationDimensionEvidence, AllocationPressureEvidence,
    },
    cancellation::BoundedCancellationEvidence,
    configuration::BoundedResidencyConfiguration,
    generation_fencing::GenerationFencingEvidence,
    read_pressure::{
        BoundedReadPressureEvidence, DuplicateFaultEvidence, PinnedFramePressureEvidence,
    },
    schedule::BoundedResidencyExecutedSchedule,
    speculative_pressure::{BoundedSpeculativePressureEvidence, SpeculativeKindEvidence},
    work_reconciliation::{
        PhysicalWorkCausalRouteEvidence, PhysicalWorkReconciliationEvidence,
        PhysicalWorkReconciliationRecordEvidence, PhysicalWorkSignalBindingEvidence,
        PhysicalWorkSignalLineageEvidence, PhysicalWorkTerminalFateEvidence,
    },
    writeback_pressure::BoundedDirtyWritebackEvidence,
};

mod cancellation;
mod generation_fencing;
mod work_reconciliation;

pub(super) struct BoundedResidencyWorldEvidence {
    pub(super) identity: PhysicalWorkCourtroomWorldIdentity,
    pub(super) records: usize,
    pub(super) payload_bytes: u64,
    pub(super) directory_bytes: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct PhysicalWorkCourtroomWorldIdentity {
    store: StableStoreIdentity,
    runtime: RuntimeIdentity,
    generation: LifecycleGeneration,
}

impl PhysicalWorkCourtroomWorldIdentity {
    pub(super) const fn new(
        store: StableStoreIdentity,
        runtime: RuntimeIdentity,
        generation: LifecycleGeneration,
    ) -> Self {
        Self {
            store,
            runtime,
            generation,
        }
    }
}

pub(super) struct BoundedResidencyEvidence {
    pub(super) configuration: BoundedResidencyConfiguration,
    pub(super) schedule: BoundedResidencyExecutedSchedule,
    pub(super) world: BoundedResidencyWorldEvidence,
    pub(super) process_allocation: ProcessAllocationEvidence,
    pub(super) reads: BoundedReadPressureEvidence,
    pub(super) pins: PinnedFramePressureEvidence,
    pub(super) duplicate: DuplicateFaultEvidence,
    pub(super) cancellation: BoundedCancellationEvidence,
    pub(super) generation_fencing: GenerationFencingEvidence,
    pub(super) dirty: BoundedDirtyWritebackEvidence,
    pub(super) speculation: BoundedSpeculativePressureEvidence,
    pub(super) work_reconciliation: PhysicalWorkReconciliationEvidence,
    pub(super) allocation: AllocationPressureEvidence,
    pub(super) filesystem: PhysicalWorkFilesystemProfileEvidence,
    pub(super) close: ServingShutdownOutcome<ClosedRuntime>,
}

pub(super) fn emit(evidence: BoundedResidencyEvidence) -> Result<(), String> {
    println!("BOUNDED_RESIDENCY_SCHEDULE {}", evidence.schedule.encoded());
    emit_world(&evidence.configuration, &evidence.world);
    emit_process_allocation(evidence.process_allocation);
    super::super::filesystem_profile::emit(&evidence.filesystem);
    emit_reads(&evidence.reads);
    emit_pins(&evidence.pins);
    emit_pinned_eviction(&evidence.pins);
    emit_duplicate(&evidence.duplicate);
    cancellation::emit(&evidence.cancellation);
    generation_fencing::emit(&evidence.generation_fencing);
    emit_dirty(&evidence.dirty);
    emit_speculation(&evidence.speculation);
    work_reconciliation::emit(&evidence.work_reconciliation);
    emit_allocation(&evidence.allocation);
    emit_close(&evidence.close);
    std::io::stdout()
        .flush()
        .map_err(|error| format!("C.6 courtroom markers failed to flush: {error}"))
}

fn emit_allocation(allocation: &AllocationPressureEvidence) {
    let scopes = &allocation.scopes;
    println!(
        "BOUNDED_RESIDENCY_SCOPES {} {} {} {} {} {} {} {} {}",
        scopes.admitted_scopes,
        scopes.exact_scope_denials,
        scopes.global_envelope_denied,
        scopes.global_denial_requested,
        scopes.global_denial_current,
        scopes.global_denial_limit,
        scopes.peak_operation_bytes,
        scopes.terminal_operation_bytes,
        scopes.all_effect_free,
    );
    for dimension in &allocation.dimensions {
        emit_allocation_dimension(dimension);
    }
    emit_allocation_trace(&allocation.trace);
}

fn emit_allocation_dimension(dimension: &AllocationDimensionEvidence) {
    println!(
        "BOUNDED_RESIDENCY_ALLOCATION {} {} {} {} {} {} {} {} {} {} {} {} {}",
        dimension.name,
        dimension.attempts,
        dimension.admissions,
        dimension.releases,
        dimension.denials,
        dimension.allocator_failures,
        dimension.admitted_units,
        dimension.released_units,
        dimension.denied_units,
        dimension.active_units,
        dimension.current_units,
        dimension.peak_units,
        dimension.limit_units,
    );
}

fn emit_allocation_trace(trace: &AllocationBoundaryTraceEvidence) {
    println!(
        "BOUNDED_RESIDENCY_ALLOCATION_TRACE {} {} {} {} {}",
        hex(&trace.store),
        trace.pool_incarnation,
        trace.event_count,
        trace.process,
        trace.attributed_actualizations,
    );
    for event in &trace.events {
        emit_allocation_event(event);
    }
}

fn emit_allocation_event(event: &AllocationBoundaryEventEvidence) {
    let operation = event
        .physical_operation
        .map_or_else(|| "none".to_owned(), |operation| operation.to_string());
    println!(
        "BOUNDED_RESIDENCY_ALLOCATION_EVENT {} {} {} {} {} {} {} {}",
        event.sequence,
        event.kind,
        event.dimension,
        event.scope.unwrap_or("none"),
        event.requested_units,
        event.actual_units,
        event.process,
        operation,
    );
}

fn emit_world(
    configuration: &BoundedResidencyConfiguration,
    world: &BoundedResidencyWorldEvidence,
) {
    println!(
        "BOUNDED_RESIDENCY_WORLD {} {} {} {} {} {} {} {}",
        std::process::id(),
        hex(&world.identity.store.bytes()),
        world.identity.runtime.get(),
        world.identity.generation.get(),
        world.records,
        world.payload_bytes,
        world.directory_bytes,
        configuration.resident_bytes(),
    );
}

fn emit_process_allocation(evidence: ProcessAllocationEvidence) {
    println!(
        "BOUNDED_RESIDENCY_PROCESS_ALLOCATION {} {}",
        std::process::id(),
        evidence.largest_successful_request_bytes(),
    );
}

fn emit_reads(reads: &BoundedReadPressureEvidence) {
    println!(
        "BOUNDED_RESIDENCY_READS {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}",
        reads.cold_read_effects,
        reads.hot_read_effects,
        reads.refault_effects,
        reads.cold_metadata_effects,
        reads.hot_metadata_effects,
        reads.refault_metadata_effects,
        reads.cold_read_work,
        reads.hot_read_work,
        reads.refault_work,
        reads.read_work,
        reads.positioned_read_effects,
        reads.metadata_read_effects,
        reads.metadata_read_work_declared,
        reads.metadata_read_work_dispatched,
        reads.metadata_read_work_terminal,
        reads.range_read_work_declared,
        reads.range_read_work_dispatched,
        reads.range_read_work_terminal,
        reads.first_operation,
        reads.last_operation,
        reads.runtime_bound,
        reads.peak_resident_bytes,
        reads.peak_admitted_bytes,
        reads.faults,
        reads.source_loads,
        reads.hits,
        reads.evictions,
        reads.caller_copy_operations,
        reads.caller_copied_bytes,
        reads.store_copy_operations,
        reads.store_copied_bytes,
        reads.peak_copy_width,
        reads.store_maximum_copy_width,
        reads.streaming_scratch_bytes,
        reads.largest_record_bytes,
    );
}

fn emit_duplicate(duplicate: &DuplicateFaultEvidence) {
    println!(
        "BOUNDED_RESIDENCY_DUPLICATE {} {} {} {} {} {} {} {} {} {} {}",
        duplicate.faults,
        duplicate.source_loads,
        duplicate.coalesced_waiters,
        duplicate.pinned_frames,
        duplicate.pin_leases,
        duplicate.positioned_reads,
        duplicate.owner_work,
        duplicate.waiter_work,
        duplicate.same_frame,
        duplicate.same_prefix,
        duplicate.waiter_created_work,
    );
}

fn emit_pins(pins: &PinnedFramePressureEvidence) {
    let pins = &pins.saturation;
    println!(
        "BOUNDED_RESIDENCY_PINS {} {} {} {} {} {:?} {:?} {} {} {} {:?} {} {}",
        pins.views,
        pins.unique_frame_identities,
        pins.zero_copy_events,
        pins.peak_pinned_frames,
        pins.peak_pin_leases,
        pins.dimension,
        pins.scope,
        pins.requested,
        pins.admitted,
        pins.limit,
        pins.retry_posture,
        pins.effect_may_have_started,
        pins.basis_matched,
    );
}

fn emit_pinned_eviction(pins: &PinnedFramePressureEvidence) {
    let eviction = &pins.eviction;
    println!(
        "BOUNDED_RESIDENCY_PINNED_EVICTION {} {} {} {} {} {}",
        eviction.forced_evictions,
        eviction.pinned_frames_before,
        eviction.pinned_frames_after,
        eviction.pin_leases_before,
        eviction.pin_leases_after,
        eviction.bases_preserved,
    );
}

fn emit_dirty(dirty: &BoundedDirtyWritebackEvidence) {
    println!(
        "BOUNDED_RESIDENCY_DIRTY {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}",
        dirty.primary_publication,
        dirty.retry_publication,
        dirty.primary_candidate_writebacks,
        dirty.retry_candidate_writebacks,
        dirty.primary_candidate_publications,
        dirty.retry_candidate_publications,
        dirty.denied_candidate_publications,
        dirty.primary_last_candidate_operation,
        dirty.retry_last_candidate_operation,
        dirty.primary_records,
        dirty.retry_records,
        dirty.dirty_at_dispatch,
        dirty.dirty_peak,
        dirty.dirty_after_denial,
        dirty.dirty_after_primary,
        dirty.dirty_terminal,
        dirty.active_claims_at_dispatch,
        dirty.active_writebehind_at_dispatch,
        dirty.peak_writebehind,
        dirty.terminal_writebehind,
        dirty.pressure_requested,
        dirty.pressure_admitted,
        dirty.pressure_limit,
        dirty.pressure_basis_exact,
        dirty.pressure_retry_after_settlement,
        dirty.pressure_effect_free,
        dirty.cleanup_deletions,
        dirty.cleanup_complete,
        dirty.writebehind_attempts,
        dirty.writebehind_admissions,
        dirty.writebehind_denials,
        dirty.writebehind_completions,
        dirty.writeback_attempts,
        dirty.exact_receipts,
        dirty.retryable_writebacks,
        dirty.indeterminate_writebacks,
        dirty.inspection_required_writebacks,
        dirty.candidate_publications,
        dirty.writebacks,
        dirty.positioned_writes,
    );
}

fn emit_speculation(speculation: &BoundedSpeculativePressureEvidence) {
    emit_speculative_kind("BOUNDED_RESIDENCY_PREFETCH", &speculation.prefetch);
    emit_speculative_kind("BOUNDED_RESIDENCY_READ_AHEAD", &speculation.read_ahead);
    emit_speculative_kind("BOUNDED_RESIDENCY_WRITE_BEHIND", &speculation.write_behind);
}

fn emit_speculative_kind(marker: &str, evidence: &SpeculativeKindEvidence) {
    println!(
        "{marker} {} {} {} {} {} {} {} {} {} {} {} {} {}",
        evidence.attempts,
        evidence.admissions,
        evidence.denials,
        evidence.completions,
        evidence.peak_frames,
        evidence.terminal_frames,
        evidence.hits,
        evidence.effectful_misses,
        evidence.hit_signal_requests,
        evidence.denial_signal_requests,
        evidence.effectful_signal_requests,
        evidence.signal_family_exact,
        evidence.foundational_basis_exact,
    );
}

fn emit_close(close: &ServingShutdownOutcome<ClosedRuntime>) {
    let residency = close.residency().counters();
    println!(
        "BOUNDED_RESIDENCY_CLOSE {} {} {} {} {} {} {} {}",
        close.residency().requires_inspection(),
        residency.resident_bytes(),
        residency.pinned_frames(),
        residency.pin_leases(),
        residency.dirty_frames(),
        residency.peak_resident_bytes(),
        residency.peak_admitted_bytes(),
        residency.peak_dirty_frames(),
    );
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}
