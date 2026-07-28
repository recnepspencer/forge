use worth_store::physical_runtime::{
    PhysicalOperationAllocationScope as Scope, PhysicalRecordResidencyPolicy,
    PhysicalSpeculativeWorkKind as Speculation,
};

use super::{
    admitted_store_base, bytes, frames, PhysicalResidencyStoreConfiguration, FIXTURE_FRAME_BYTES,
    FIXTURE_METADATA_BYTES, SUCCESSOR_SCOPE_ALLOCATION_BYTES,
};

const SUCCESSOR_OPERATION_ENVELOPE_BYTES: u64 = SUCCESSOR_SCOPE_ALLOCATION_BYTES * 5;
const SUCCESSOR_TOTAL_ENVELOPE_BYTES: u64 = 512 * 1024;

pub(in crate::harness::physical_residency) fn successor_scope_pressure_configuration(
) -> PhysicalResidencyStoreConfiguration {
    let base = admitted_store_base();
    let format = base.format();
    let mut residency = PhysicalRecordResidencyPolicy::builder()
        .total_bytes(bytes(SUCCESSOR_TOTAL_ENVELOPE_BYTES))
        .resident_bytes(bytes(FIXTURE_FRAME_BYTES))
        .metadata_bytes(bytes(FIXTURE_METADATA_BYTES))
        .frame_entries(frames(8))
        .pinned_frames(frames(8))
        .pin_leases(frames(8))
        .dirty_frames(frames(4))
        .dirty_replacement_bytes(bytes(FIXTURE_FRAME_BYTES))
        .operation_bytes(bytes(SUCCESSOR_OPERATION_ENVELOPE_BYTES))
        .scope_bytes(Scope::ForegroundRead, bytes(FIXTURE_FRAME_BYTES))
        .scope_bytes(Scope::ForegroundWrite, bytes(FIXTURE_FRAME_BYTES));
    for scope in [
        Scope::Recovery,
        Scope::Scrub,
        Scope::Maintenance,
        Scope::Verification,
        Scope::Blob,
    ] {
        residency = residency.scope_bytes(scope, bytes(SUCCESSOR_SCOPE_ALLOCATION_BYTES * 2));
    }
    let residency = residency
        .speculative_frames(Speculation::Prefetch, frames(4))
        .speculative_frames(Speculation::ReadAhead, frames(4))
        .speculative_frames(Speculation::WriteBehind, frames(4))
        .admit(format)
        .into_result()
        .unwrap();
    base.with_residency(residency)
}
