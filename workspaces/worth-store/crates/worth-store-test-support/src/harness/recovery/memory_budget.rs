use worth_store_recovery_physics::RecoveryMemoryAllocation;

use crate::harness::physical_residency::PhysicalResidencyStoreWorld;

pub fn with_recovery_memory_allocation<R>(
    run: impl FnOnce(RecoveryMemoryAllocation<'_>) -> R,
) -> R {
    let world =
        PhysicalResidencyStoreWorld::initialize("recovery-memory-allocation").expect("Store world");
    let allocation = world
        .serving()
        .physical_allocations()
        .admit_recovery(std::num::NonZeroU64::new(128).expect("fixture bytes are nonzero"))
        .expect("real Store recovery allocation should admit");
    let result = run(RecoveryMemoryAllocation::from_store_allocation(allocation));
    assert!(!world.close().residency().requires_inspection());
    result
}
