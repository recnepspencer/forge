use worth_store_buffer_pool::{
    OperationAllocationGrant, OperationAllocationScope, PhysicalResidencyDenial,
    PhysicalResidencyLimits, PhysicalResidencyPool,
};
use worth_store_physical_format::store_namespace::{
    ProposedStoreIdentity, StoreNamespaceIdentityRecord, StoreNamespaceVersion,
};
use worth_store_recovery_physics::RecoveryMemoryAllocation;

pub(crate) fn recovery_memory_allocation() -> RecoveryMemoryAllocation {
    let grant = operation_allocation(OperationAllocationScope::Recovery, 128)
        .expect("bounded recovery allocation should admit");
    RecoveryMemoryAllocation::from_allocation_grant(grant)
        .expect("recovery-scoped allocation should enter recovery")
}

pub(crate) fn operation_allocation(
    scope: OperationAllocationScope,
    bytes: u64,
) -> Result<OperationAllocationGrant, PhysicalResidencyDenial> {
    recovery_fixture_pool().begin_operation(scope, bytes)
}

fn recovery_fixture_pool() -> PhysicalResidencyPool {
    PhysicalResidencyPool::open(
        StoreNamespaceIdentityRecord::new(
            StoreNamespaceVersion::CURRENT,
            ProposedStoreIdentity::from_nonzero_bytes([0x43; 16])
                .expect("certification Store identity is nonzero"),
        )
        .published_identity(),
        PhysicalResidencyLimits::new(512, 1, 1, 512, 1)
            .expect("certification recovery limits are bounded"),
    )
    .expect("certification recovery pool should open")
}
