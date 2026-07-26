use worth_store_buffer_pool::{
    OperationAllocationGrant, PhysicalOperationAllocationScope, PhysicalResidencyDenial,
    PhysicalResidencyLimits, PhysicalResidencyPool, PhysicalSpeculativeWorkKind,
};
use worth_store_physical_format::store_namespace::{
    ProposedStoreIdentity, StoreNamespaceIdentityRecord, StoreNamespaceVersion,
};
use worth_store_recovery_physics::RecoveryMemoryAllocation;

pub(crate) fn recovery_memory_allocation() -> RecoveryMemoryAllocation {
    let grant = operation_allocation(PhysicalOperationAllocationScope::Recovery, 128)
        .expect("bounded recovery allocation should admit");
    RecoveryMemoryAllocation::from_allocation_grant(grant)
        .expect("recovery-scoped allocation should enter recovery")
}

pub(crate) fn operation_allocation(
    scope: PhysicalOperationAllocationScope,
    bytes: u64,
) -> Result<OperationAllocationGrant, PhysicalResidencyDenial> {
    recovery_fixture_pool().begin_operation(
        scope,
        std::num::NonZeroU64::new(bytes).expect("courtroom allocation bytes are nonzero"),
    )
}

fn recovery_fixture_pool() -> PhysicalResidencyPool {
    PhysicalResidencyPool::open(
        StoreNamespaceIdentityRecord::new(
            StoreNamespaceVersion::CURRENT,
            ProposedStoreIdentity::from_nonzero_bytes([0x43; 16])
                .expect("certification Store identity is nonzero"),
        )
        .published_identity(),
        recovery_limits(),
    )
    .expect("certification recovery pool should open")
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
        .expect("certification recovery limits are admitted")
}

fn nonzero_bytes(value: u64) -> std::num::NonZeroU64 {
    std::num::NonZeroU64::new(value).unwrap()
}

fn nonzero_count(value: u32) -> std::num::NonZeroU32 {
    std::num::NonZeroU32::new(value).unwrap()
}
