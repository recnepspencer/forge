use worth_store::physical_runtime::{
    BlobPhysicalAllocation, PhysicalOperationAllocationScope, ServingPhysicalRuntime,
};
use worth_store_test_support::harness::physical_residency::PhysicalResidencyStoreWorld;

pub(crate) fn with_blob_allocation<R>(
    bytes: u64,
    run: impl FnOnce(&ServingPhysicalRuntime, BlobPhysicalAllocation<'_>) -> R,
) -> R {
    let world =
        PhysicalResidencyStoreWorld::initialize("blob-allocation").expect("real Store world");
    let allocation = world
        .serving()
        .physical_allocations()
        .admit_blob(std::num::NonZeroU64::new(bytes).expect("fixture bytes are nonzero"))
        .expect("real Store Blob allocation should admit");
    let result = run(world.serving(), allocation);
    assert_eq!(
        world
            .serving()
            .residency_observation()
            .counters()
            .active_operation_bytes_for(PhysicalOperationAllocationScope::Blob),
        0,
        "Blob allocation must be released before the fixture closes",
    );
    assert!(!world.close().residency().requires_inspection());
    result
}
