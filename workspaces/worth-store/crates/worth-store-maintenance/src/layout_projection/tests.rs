use worth_store_buffer_pool::{
    PhysicalOperationAllocationScope, PhysicalResidencyLimits, PhysicalResidencyPool,
    PhysicalSpeculativeWorkKind,
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
        .begin_operation(
            PhysicalOperationAllocationScope::Maintenance,
            std::num::NonZeroU64::new(128).unwrap(),
        )
        .expect("compaction allocation should admit");
    let report = CompactionPlanningMemoryEnvelope::from_allocation_grant(allocation)
        .expect("maintenance allocation should authorize compaction planning")
        .project_maintenance_queue_layout();

    assert_eq!(report.family_id().label(), "maintenance_queue_declaration");
    assert_eq!(
        report.allocation_scope(),
        PhysicalOperationAllocationScope::Maintenance
    );
    assert_eq!(report.declared_budget().allocation_bytes(), 128);
    assert_eq!(
        report
            .exact_counters()
            .active_operation_bytes_for(PhysicalOperationAllocationScope::Maintenance),
        128
    );

    drop(report);
    assert_eq!(
        pool.counters()
            .active_operation_bytes_for(PhysicalOperationAllocationScope::Maintenance),
        0
    );
    assert!(!pool.close().requires_inspection());
}

#[test]
fn import_export_layout_retains_and_releases_canonical_maintenance_allocation() {
    let pool = maintenance_pool(0x52);
    let allocation = pool
        .begin_operation(
            PhysicalOperationAllocationScope::Maintenance,
            std::num::NonZeroU64::new(96).unwrap(),
        )
        .expect("import-export allocation should admit");
    let report = ImportExportMemoryEnvelope::from_allocation_grant(allocation)
        .expect("maintenance allocation should authorize import-export work")
        .project_maintenance_queue_layout();

    assert_eq!(report.family_id().label(), "maintenance_queue_declaration");
    assert_eq!(
        report.allocation_scope(),
        PhysicalOperationAllocationScope::Maintenance
    );
    assert_eq!(report.declared_budget().allocation_bytes(), 96);
    assert_eq!(
        report
            .exact_counters()
            .peak_operation_bytes_for(PhysicalOperationAllocationScope::Maintenance),
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
        .begin_operation(
            PhysicalOperationAllocationScope::ForegroundRead,
            std::num::NonZeroU64::new(64).unwrap(),
        )
        .expect("foreground allocation should admit before semantic rejection");
    let denial = CompactionPlanningMemoryEnvelope::from_allocation_grant(allocation)
        .expect_err("foreground allocation cannot authorize maintenance");

    assert_eq!(
        denial,
        MaintenanceMemoryEnvelopeDenial::WrongAllocationScope {
            actual: PhysicalOperationAllocationScope::ForegroundRead,
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
    let limits = maintenance_limits();
    PhysicalResidencyPool::open(identity, limits).expect("maintenance fixture pool should open")
}

fn maintenance_limits() -> PhysicalResidencyLimits {
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
        .expect("maintenance fixture limits are admitted")
}

fn nonzero_bytes(value: u64) -> std::num::NonZeroU64 {
    std::num::NonZeroU64::new(value).unwrap()
}

fn nonzero_count(value: u32) -> std::num::NonZeroU32 {
    std::num::NonZeroU32::new(value).unwrap()
}
