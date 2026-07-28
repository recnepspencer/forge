use worth_store::physical_runtime::{
    MaintenancePhysicalAllocation, PhysicalOperationAllocationScope, ServingPhysicalRuntime,
};
use worth_store_test_support::harness::physical_residency::PhysicalResidencyStoreWorld;

use crate::{CompactionPlanningMemoryEnvelope, ImportExportMemoryEnvelope};

#[test]
fn compaction_layout_retains_and_releases_canonical_maintenance_allocation() {
    with_maintenance_allocation(128, |serving, allocation| {
        let report = CompactionPlanningMemoryEnvelope::from_store_allocation(allocation)
            .project_maintenance_queue_layout();

        assert_eq!(report.family_id().label(), "maintenance_queue_declaration");
        assert_eq!(
            report.allocation_scope(),
            PhysicalOperationAllocationScope::Maintenance
        );
        assert_eq!(report.declared_budget().allocation_bytes(), 128);
        assert_eq!(
            serving
                .residency_observation()
                .counters()
                .active_operation_bytes_for(PhysicalOperationAllocationScope::Maintenance),
            128
        );
    });
}

#[test]
fn import_export_layout_retains_and_releases_canonical_maintenance_allocation() {
    with_maintenance_allocation(96, |serving, allocation| {
        let report = ImportExportMemoryEnvelope::from_store_allocation(allocation)
            .project_maintenance_queue_layout();

        assert_eq!(report.family_id().label(), "maintenance_queue_declaration");
        assert_eq!(
            report.allocation_scope(),
            PhysicalOperationAllocationScope::Maintenance
        );
        assert_eq!(report.declared_budget().allocation_bytes(), 96);
        assert_eq!(
            serving
                .residency_observation()
                .counters()
                .active_operation_bytes_for(PhysicalOperationAllocationScope::Maintenance),
            96
        );
    });
}

fn with_maintenance_allocation<R>(
    bytes: u64,
    run: impl FnOnce(&ServingPhysicalRuntime, MaintenancePhysicalAllocation<'_>) -> R,
) -> R {
    let world =
        PhysicalResidencyStoreWorld::initialize("maintenance-allocation").expect("Store world");
    let allocation = world
        .serving()
        .physical_allocations()
        .admit_maintenance(std::num::NonZeroU64::new(bytes).expect("fixture bytes are nonzero"))
        .expect("real Store maintenance allocation should admit");
    let result = run(world.serving(), allocation);
    assert_eq!(
        world
            .serving()
            .residency_observation()
            .counters()
            .active_operation_bytes(),
        0,
    );
    assert!(!world.close().residency().requires_inspection());
    result
}
