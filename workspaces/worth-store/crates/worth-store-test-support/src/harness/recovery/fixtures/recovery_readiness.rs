use worth_store_buffer_pool::{
    PhysicalOperationAllocationScope, PhysicalResidencyLimits, PhysicalResidencyPool,
    PhysicalSpeculativeWorkKind,
};
use worth_store_contracts::{
    BoundedCounterRecap, BufferPoolAuthorityRecap, DenialBehaviorRecap, DeniedBoundaryKind,
    IntegrityInspectionLifetimeLaw, NoMaterializationWitness, PhysicalAuthorityRecap,
    PhysicalIntegrityReadinessPayload, ProtectedIntegrityViewCapability,
    ScrubPlanningAllocationEnvelope, VerifierResidentEnvelope,
};
use worth_store_physical_format::store_namespace::{
    ProposedStoreIdentity, StoreNamespaceIdentityRecord, StoreNamespaceVersion,
};
use worth_store_recovery_physics::RecoveryMemoryAllocation;

pub(super) fn physical_integrity_model_payload() -> PhysicalIntegrityReadinessPayload {
    PhysicalIntegrityReadinessPayload::from_physical_substrate_closeout_evidence(
        ProtectedIntegrityViewCapability::protected_views(1).unwrap(),
        VerifierResidentEnvelope::bounded(128, 1).unwrap(),
        ScrubPlanningAllocationEnvelope::bounded(64).unwrap(),
        IntegrityInspectionLifetimeLaw::lease_scoped(),
        NoMaterializationWitness::observed_zero(0, 0).unwrap(),
        BoundedCounterRecap::exact(128, 1, 0, 64, 0, 0).unwrap(),
        DenialBehaviorRecap::from_named_boundaries(&DeniedBoundaryKind::ALL).unwrap(),
        PhysicalAuthorityRecap::from_physical_format_authority(4, 2, 2).unwrap(),
        BufferPoolAuthorityRecap::physical_substrate_authority(true, true, true, true).unwrap(),
    )
}

pub(super) fn recovery_memory_allocation() -> RecoveryMemoryAllocation {
    let pool = PhysicalResidencyPool::open(
        StoreNamespaceIdentityRecord::new(
            StoreNamespaceVersion::CURRENT,
            ProposedStoreIdentity::from_nonzero_bytes([0x53; 16]).unwrap(),
        )
        .published_identity(),
        recovery_limits(),
    )
    .unwrap();
    let allocation = pool
        .begin_operation(
            PhysicalOperationAllocationScope::Recovery,
            std::num::NonZeroU64::new(128).unwrap(),
        )
        .unwrap();
    RecoveryMemoryAllocation::from_allocation_grant(allocation).unwrap()
}

fn recovery_limits() -> PhysicalResidencyLimits {
    use PhysicalOperationAllocationScope as Scope;
    use PhysicalSpeculativeWorkKind as Speculation;

    PhysicalResidencyLimits::builder()
        .total_bytes(nonzero_bytes(5632))
        .resident_bytes(nonzero_bytes(512))
        .metadata_bytes(nonzero_bytes(4096))
        .frame_entries(nonzero_count(1))
        .pinned_frames(nonzero_count(1))
        .pin_leases(nonzero_count(1))
        .dirty_frames(nonzero_count(1))
        .dirty_replacement_bytes(nonzero_bytes(512))
        .operation_bytes(nonzero_bytes(512))
        .scope_bytes(Scope::ForegroundRead, nonzero_bytes(512))
        .scope_bytes(Scope::ForegroundWrite, nonzero_bytes(512))
        .scope_bytes(Scope::Recovery, nonzero_bytes(512))
        .scope_bytes(Scope::Scrub, nonzero_bytes(512))
        .scope_bytes(Scope::Maintenance, nonzero_bytes(512))
        .scope_bytes(Scope::Verification, nonzero_bytes(512))
        .scope_bytes(Scope::Blob, nonzero_bytes(512))
        .speculative_frames(Speculation::Prefetch, nonzero_count(1))
        .speculative_frames(Speculation::ReadAhead, nonzero_count(1))
        .speculative_frames(Speculation::WriteBehind, nonzero_count(1))
        .admit(std::num::NonZeroU64::MIN)
        .unwrap()
}

fn nonzero_bytes(value: u64) -> std::num::NonZeroU64 {
    std::num::NonZeroU64::new(value).unwrap()
}

fn nonzero_count(value: u32) -> std::num::NonZeroU32 {
    std::num::NonZeroU32::new(value).unwrap()
}

pub(super) fn physical_authority() -> PhysicalAuthorityRecap {
    PhysicalAuthorityRecap::from_physical_format_authority(3, 2, 1).unwrap()
}
