use worth_store_buffer_pool::{
    OperationAllocationScope, PhysicalResidencyLimits, PhysicalResidencyPool,
};
use worth_store_physical_format::store_namespace::{
    ProposedStoreIdentity, StoreNamespaceIdentityRecord, StoreNamespaceVersion,
};
use worth_store_recovery_physics::RecoveryMemoryAllocation;

pub fn recovery_memory_allocation() -> RecoveryMemoryAllocation {
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
    RecoveryMemoryAllocation::from_allocation_grant(allocation).unwrap()
}
