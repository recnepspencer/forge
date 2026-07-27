use std::io::Write;

use worth_store::physical_runtime::{
    ClosedRuntime, LifecycleGeneration, PhysicalWorkFilesystemProfileEvidence, RuntimeIdentity,
    ServingShutdownOutcome,
};
use worth_store_physical_format::store_namespace::StableStoreIdentity;

use super::{
    read_pressure::{
        BoundedReadPressureEvidence, PinnedFramePressureEvidence, ResidencyCancellationEvidence,
    },
    writeback_pressure::BoundedDirtyWritebackEvidence,
    BoundedResidencyConfiguration,
};

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
    pub(super) world: BoundedResidencyWorldEvidence,
    pub(super) reads: BoundedReadPressureEvidence,
    pub(super) pins: PinnedFramePressureEvidence,
    pub(super) cancellation: ResidencyCancellationEvidence,
    pub(super) dirty: BoundedDirtyWritebackEvidence,
    pub(super) filesystem: PhysicalWorkFilesystemProfileEvidence,
    pub(super) close: ServingShutdownOutcome<ClosedRuntime>,
}

pub(super) fn emit(evidence: BoundedResidencyEvidence) -> Result<(), String> {
    emit_world(&evidence.configuration, &evidence.world);
    super::super::filesystem_profile::emit(&evidence.filesystem);
    emit_reads(&evidence.reads);
    emit_pins(&evidence.pins);
    emit_cancellation(&evidence.cancellation);
    emit_dirty(&evidence.dirty);
    emit_close(&evidence.close);
    std::io::stdout()
        .flush()
        .map_err(|error| format!("C.6 courtroom markers failed to flush: {error}"))
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

fn emit_reads(reads: &BoundedReadPressureEvidence) {
    println!(
        "BOUNDED_RESIDENCY_READS {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}",
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
    );
}

fn emit_pins(pins: &PinnedFramePressureEvidence) {
    println!(
        "BOUNDED_RESIDENCY_PINS {} {} {} {} {} {:?} {:?} {} {} {}",
        pins.cold_work,
        pins.hot_work,
        pins.refault_work,
        pins.peak_pinned_frames,
        pins.peak_pin_leases,
        pins.dimension,
        pins.scope,
        pins.requested,
        pins.admitted,
        pins.limit,
    );
}

fn emit_cancellation(cancellation: &ResidencyCancellationEvidence) {
    println!(
        "BOUNDED_RESIDENCY_CANCEL {} {} {} {} {} {} {}",
        cancellation.physical_work,
        cancellation.first_operation,
        cancellation.last_operation,
        cancellation.runtime_bound,
        cancellation.unread_payload_bytes,
        cancellation.open_media_effects,
        cancellation.cancellation_media_effects,
    );
}

fn emit_dirty(dirty: &BoundedDirtyWritebackEvidence) {
    println!(
        "BOUNDED_RESIDENCY_DIRTY {} {} {} {} {} {:?} {:?} {:?} {} {} {} {} {} {} {} {} {} {} {} {} {} {}",
        dirty.identity.operation().get(),
        dirty.source_work_count,
        dirty.first_source_work.operation().get(),
        dirty.last_source_work.operation().get(),
        dirty.effect.backend_operation().value(),
        dirty.effect_fate,
        dirty.recovery,
        dirty.signal,
        dirty.dirty_at_pause,
        dirty.dirty_after_receipt,
        dirty.positioned_writes,
        dirty.candidate_publications,
        dirty.writebacks,
        dirty.active_claims_at_pause,
        dirty.eviction_releases_at_pause,
        dirty.competing_claim_denied,
        dirty.cancellation_settlement_continues,
        dirty.writeback_attempts,
        dirty.exact_receipts,
        dirty.retryable_writebacks,
        dirty.indeterminate_writebacks,
        dirty.inspection_required_writebacks,
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
