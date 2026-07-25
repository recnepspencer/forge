use worth_store_buffer_pool::{
    OperationAllocationScope, PhysicalResidencyLimits, PhysicalResidencyPool,
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
use worth_store_recovery_physics::RecoveryMemoryEnvelope;

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

pub(super) fn recovery_memory_envelope() -> RecoveryMemoryEnvelope {
    let pool = PhysicalResidencyPool::open(
        StoreNamespaceIdentityRecord::new(
            StoreNamespaceVersion::CURRENT,
            ProposedStoreIdentity::from_nonzero_bytes([0x53; 16]).unwrap(),
        )
        .published_identity(),
        PhysicalResidencyLimits::new(512, 1, 1, 512, 1).unwrap(),
    )
    .unwrap();
    let allocation = pool
        .begin_operation(OperationAllocationScope::Recovery, 128)
        .unwrap();
    RecoveryMemoryEnvelope::from_allocation_grant(&allocation, 1).unwrap()
}

pub(super) fn physical_authority() -> PhysicalAuthorityRecap {
    PhysicalAuthorityRecap::from_physical_format_authority(3, 2, 1).unwrap()
}
