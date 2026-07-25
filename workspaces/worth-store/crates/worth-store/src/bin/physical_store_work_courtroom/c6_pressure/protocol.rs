use std::io::Write;

use worth_store::physical_runtime::{
    C6PhysicalWorkHandoffIdentity, ClosedRuntime, PhysicalWorkFilesystemProfileEvidence,
    ServingShutdownOutcome,
};

use super::{
    dirty_pressure::C6DirtyPressureEvidence,
    read_pressure::{C6CancellationEvidence, C6PinPressureEvidence, C6ReadPressureEvidence},
    C6PressureConfiguration,
};

pub(super) struct C6WorldEvidence {
    pub(super) identity: C6PhysicalWorkHandoffIdentity,
    pub(super) records: usize,
    pub(super) payload_bytes: u64,
    pub(super) directory_bytes: u64,
}

pub(super) struct C6PressureEvidence {
    pub(super) configuration: C6PressureConfiguration,
    pub(super) world: C6WorldEvidence,
    pub(super) reads: C6ReadPressureEvidence,
    pub(super) pins: C6PinPressureEvidence,
    pub(super) cancellation: C6CancellationEvidence,
    pub(super) dirty: C6DirtyPressureEvidence,
    pub(super) filesystem: PhysicalWorkFilesystemProfileEvidence,
    pub(super) close: ServingShutdownOutcome<ClosedRuntime>,
}

pub(super) fn emit(evidence: C6PressureEvidence) -> Result<(), String> {
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

fn emit_world(configuration: &C6PressureConfiguration, world: &C6WorldEvidence) {
    println!(
        "C5_1_C6_WORLD {} {} {} {} {} {} {} {}",
        std::process::id(),
        hex(&world.identity.store().bytes()),
        world.identity.runtime().get(),
        world.identity.generation().get(),
        world.records,
        world.payload_bytes,
        world.directory_bytes,
        configuration.resident_bytes(),
    );
}

fn emit_reads(reads: &C6ReadPressureEvidence) {
    println!(
        "C5_1_C6_READS {} {} {} {} {} {} {} {} {}",
        reads.cold_read_effects,
        reads.hot_read_effects,
        reads.refault_effects,
        reads.read_work,
        reads.peak_resident_bytes,
        reads.peak_admitted_bytes,
        reads.faults,
        reads.hits,
        reads.evictions,
    );
}

fn emit_pins(pins: &C6PinPressureEvidence) {
    println!(
        "C5_1_C6_PINS {} {} {} {} {} {:?}",
        pins.cold_work,
        pins.hot_work,
        pins.refault_work,
        pins.peak_pinned_frames,
        pins.peak_pin_leases,
        pins.denial,
    );
}

fn emit_cancellation(cancellation: &C6CancellationEvidence) {
    println!(
        "C5_1_C6_CANCEL {} {} {} {} {} {} {}",
        cancellation.physical_work,
        cancellation.first_operation,
        cancellation.last_operation,
        cancellation.handoff_bound,
        cancellation.unread_payload_bytes,
        cancellation.open_media_effects,
        cancellation.cancellation_media_effects,
    );
}

fn emit_dirty(dirty: &C6DirtyPressureEvidence) {
    println!(
        "C5_1_C6_DIRTY {} {} {} {} {} {:?} {:?} {:?} {} {} {} {} {}",
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
    );
}

fn emit_close(close: &ServingShutdownOutcome<ClosedRuntime>) {
    let residency = close.residency().counters();
    println!(
        "C5_1_C6_CLOSE {} {} {} {} {} {} {} {}",
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
