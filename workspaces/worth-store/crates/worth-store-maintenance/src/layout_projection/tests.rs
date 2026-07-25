use worth_store_buffer_pool::{
    OperationAllocationScope, PhysicalResidencyLimits, PhysicalResidencyPool,
};
use worth_store_physical_format::store_namespace::{
    ProposedStoreIdentity, StoreNamespaceIdentityRecord, StoreNamespaceVersion,
};

use crate::{
    CompactionPlanningMemoryEnvelope, ImportExportMemoryEnvelope, MaintenanceMemoryEnvelopeDenial,
};

#[test]
fn compaction_layout_retains_and_releases_canonical_maintenance_allocation() {
    let pool = maintenance_pool(0x51);
    let allocation = pool
        .begin_operation(OperationAllocationScope::Maintenance, 128)
        .expect("compaction allocation should admit");
    let report = CompactionPlanningMemoryEnvelope::from_allocation_grant(allocation)
        .expect("maintenance allocation should authorize compaction planning")
        .project_maintenance_queue_layout();

    assert_eq!(report.family_id().label(), "maintenance_queue_declaration");
    assert_eq!(
        report.allocation_scope(),
        OperationAllocationScope::Maintenance
    );
    assert_eq!(report.declared_budget().allocation_bytes(), 128);
    assert_eq!(
        report
            .exact_counters()
            .active_operation_bytes_for(OperationAllocationScope::Maintenance),
        128
    );

    drop(report);
    assert_eq!(
        pool.counters()
            .active_operation_bytes_for(OperationAllocationScope::Maintenance),
        0
    );
    assert!(!pool.close().requires_inspection());
}

#[test]
fn import_export_layout_retains_and_releases_canonical_maintenance_allocation() {
    let pool = maintenance_pool(0x52);
    let allocation = pool
        .begin_operation(OperationAllocationScope::Maintenance, 96)
        .expect("import-export allocation should admit");
    let report = ImportExportMemoryEnvelope::from_allocation_grant(allocation)
        .expect("maintenance allocation should authorize import-export work")
        .project_maintenance_queue_layout();

    assert_eq!(report.family_id().label(), "maintenance_queue_declaration");
    assert_eq!(
        report.allocation_scope(),
        OperationAllocationScope::Maintenance
    );
    assert_eq!(report.declared_budget().allocation_bytes(), 96);
    assert_eq!(
        report
            .exact_counters()
            .peak_operation_bytes_for(OperationAllocationScope::Maintenance),
        96
    );

    drop(report);
    assert_eq!(pool.counters().active_operation_bytes(), 0);
    assert!(!pool.close().requires_inspection());
}

#[test]
fn maintenance_envelope_rejects_and_releases_wrong_scope() {
    let pool = maintenance_pool(0x53);
    let allocation = pool
        .begin_operation(OperationAllocationScope::ForegroundRead, 64)
        .expect("foreground allocation should admit before semantic rejection");
    let denial = CompactionPlanningMemoryEnvelope::from_allocation_grant(allocation)
        .expect_err("foreground allocation cannot authorize maintenance");

    assert_eq!(
        denial,
        MaintenanceMemoryEnvelopeDenial::WrongAllocationScope {
            actual: OperationAllocationScope::ForegroundRead,
        }
    );
    assert_eq!(pool.counters().active_operation_bytes(), 0);
    assert!(!pool.close().requires_inspection());
}

fn maintenance_pool(identity_byte: u8) -> PhysicalResidencyPool {
    let identity = StoreNamespaceIdentityRecord::new(
        StoreNamespaceVersion::CURRENT,
        ProposedStoreIdentity::from_nonzero_bytes([identity_byte; 16])
            .expect("maintenance fixture Store identity is nonzero"),
    )
    .published_identity();
    let limits = PhysicalResidencyLimits::new(512, 1, 1, 512, 1)
        .expect("maintenance fixture limits are bounded");
    PhysicalResidencyPool::open(identity, limits).expect("maintenance fixture pool should open")
}
