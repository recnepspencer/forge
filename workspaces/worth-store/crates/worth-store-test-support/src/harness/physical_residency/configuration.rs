use std::num::{NonZeroU32, NonZeroU64};

use worth_store::physical_runtime::{
    AdmittedPhysicalRecordFormat, AdmittedPhysicalRecordResidencyPolicy,
    AdmittedRecordAccessPolicy, AdmittedRecordPlacementPolicy, ManifestEntryCapacity,
    PhysicalOperationAllocationScope as Scope, PhysicalRecordAccessPolicy,
    PhysicalRecordFormatDeclaration, PhysicalRecordPlacementPolicy, PhysicalRecordResidencyPolicy,
    PhysicalSpeculativeWorkKind as Speculation,
};

pub const SUCCESSOR_SCOPE_ALLOCATION_BYTES: u64 = 16_384;

pub(super) struct PhysicalResidencyStoreConfiguration {
    pub(super) format: AdmittedPhysicalRecordFormat,
    pub(super) placement: AdmittedRecordPlacementPolicy,
    pub(super) access: AdmittedRecordAccessPolicy,
    pub(super) residency: AdmittedPhysicalRecordResidencyPolicy,
}

pub(super) fn admitted_physical_residency_store_configuration(
) -> PhysicalResidencyStoreConfiguration {
    let format = AdmittedPhysicalRecordFormat::admit(
        PhysicalRecordFormatDeclaration::builder().admit().unwrap(),
    );
    let placement = PhysicalRecordPlacementPolicy::builder()
        .manifest_capacity(ManifestEntryCapacity::new(64).unwrap())
        .admit(format)
        .unwrap();
    let access = PhysicalRecordAccessPolicy::builder().admit(format).unwrap();
    let operation_bytes = bytes(SUCCESSOR_SCOPE_ALLOCATION_BYTES * 5);
    let mut residency = PhysicalRecordResidencyPolicy::builder()
        .total_bytes(bytes(512 * 1024))
        .resident_bytes(bytes(SUCCESSOR_SCOPE_ALLOCATION_BYTES))
        .metadata_bytes(bytes(256 * 1024))
        .frame_entries(frames(8))
        .pinned_frames(frames(8))
        .pin_leases(frames(8))
        .dirty_frames(frames(4))
        .dirty_replacement_bytes(bytes(SUCCESSOR_SCOPE_ALLOCATION_BYTES))
        .operation_bytes(operation_bytes)
        .scope_bytes(
            Scope::ForegroundRead,
            bytes(SUCCESSOR_SCOPE_ALLOCATION_BYTES),
        )
        .scope_bytes(
            Scope::ForegroundWrite,
            bytes(SUCCESSOR_SCOPE_ALLOCATION_BYTES),
        );
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
    PhysicalResidencyStoreConfiguration {
        format,
        placement,
        access,
        residency,
    }
}

fn bytes(value: u64) -> NonZeroU64 {
    NonZeroU64::new(value).unwrap()
}

fn frames(value: u32) -> NonZeroU32 {
    NonZeroU32::new(value).unwrap()
}
