use worth_store_buffer_pool::{
    OperationAllocationGrant, OperationAllocationScope, PhysicalResidencyLimits,
    PhysicalResidencyPool,
};
use worth_store_physical_format::store_namespace::{
    ProposedStoreIdentity, StoreNamespaceIdentityRecord, StoreNamespaceVersion,
};

pub(crate) fn blob_allocation(bytes: u64) -> (PhysicalResidencyPool, OperationAllocationGrant) {
    operation_allocation(OperationAllocationScope::Blob, bytes)
}

pub(crate) fn operation_allocation(
    scope: OperationAllocationScope,
    bytes: u64,
) -> (PhysicalResidencyPool, OperationAllocationGrant) {
    let operation_bytes = bytes.max(64);
    let pool = PhysicalResidencyPool::open(
        StoreNamespaceIdentityRecord::new(
            StoreNamespaceVersion::CURRENT,
            ProposedStoreIdentity::from_nonzero_bytes([0xB7; 16]).unwrap(),
        )
        .published_identity(),
        PhysicalResidencyLimits::new(operation_bytes, 1, 1, operation_bytes, 1).unwrap(),
    )
    .unwrap();
    let grant = pool.begin_operation(scope, bytes).unwrap();
    (pool, grant)
}

pub(crate) fn blob_allocation_grant(bytes: u64) -> OperationAllocationGrant {
    blob_allocation(bytes).1
}
