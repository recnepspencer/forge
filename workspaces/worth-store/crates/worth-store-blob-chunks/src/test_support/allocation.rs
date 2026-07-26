use worth_store_buffer_pool::{
    OperationAllocationGrant, PhysicalOperationAllocationScope, PhysicalResidencyLimits,
    PhysicalResidencyPool, PhysicalSpeculativeWorkKind,
};
use worth_store_physical_format::store_namespace::{
    ProposedStoreIdentity, StoreNamespaceIdentityRecord, StoreNamespaceVersion,
};

pub(crate) fn blob_allocation(bytes: u64) -> (PhysicalResidencyPool, OperationAllocationGrant) {
    operation_allocation(PhysicalOperationAllocationScope::Blob, bytes)
}

pub(crate) fn operation_allocation(
    scope: PhysicalOperationAllocationScope,
    bytes: u64,
) -> (PhysicalResidencyPool, OperationAllocationGrant) {
    let operation_bytes = bytes.max(64);
    let pool = PhysicalResidencyPool::open(
        StoreNamespaceIdentityRecord::new(
            StoreNamespaceVersion::CURRENT,
            ProposedStoreIdentity::from_nonzero_bytes([0xB7; 16]).unwrap(),
        )
        .published_identity(),
        allocation_limits(operation_bytes),
    )
    .unwrap();
    let grant = pool
        .begin_operation(
            scope,
            std::num::NonZeroU64::new(bytes).expect("fixture allocation bytes are nonzero"),
        )
        .unwrap();
    (pool, grant)
}

fn allocation_limits(operation_bytes: u64) -> PhysicalResidencyLimits {
    use PhysicalOperationAllocationScope as Scope;
    use PhysicalSpeculativeWorkKind as Speculation;

    let total_bytes = operation_bytes
        .checked_mul(3)
        .and_then(|bytes| bytes.checked_add(4096))
        .expect("blob fixture total residency bytes fit");
    PhysicalResidencyLimits::builder()
        .total_bytes(nonzero_bytes(total_bytes))
        .resident_bytes(nonzero_bytes(operation_bytes))
        .metadata_bytes(nonzero_bytes(4096))
        .frame_entries(nonzero_count(1))
        .pinned_frames(nonzero_count(1))
        .pin_leases(nonzero_count(1))
        .dirty_frames(nonzero_count(1))
        .dirty_replacement_bytes(nonzero_bytes(operation_bytes))
        .operation_bytes(nonzero_bytes(operation_bytes))
        .scope_bytes(Scope::ForegroundRead, nonzero_bytes(operation_bytes))
        .scope_bytes(Scope::ForegroundWrite, nonzero_bytes(operation_bytes))
        .scope_bytes(Scope::Recovery, nonzero_bytes(operation_bytes))
        .scope_bytes(Scope::Scrub, nonzero_bytes(operation_bytes))
        .scope_bytes(Scope::Maintenance, nonzero_bytes(operation_bytes))
        .scope_bytes(Scope::Verification, nonzero_bytes(operation_bytes))
        .scope_bytes(Scope::Blob, nonzero_bytes(operation_bytes))
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

pub(crate) fn blob_allocation_grant(bytes: u64) -> OperationAllocationGrant {
    blob_allocation(bytes).1
}
