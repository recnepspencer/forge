use worth_store::physical_runtime::{
    MaintenancePhysicalAllocation, PhysicalOperationAllocationScope,
};

#[derive(Debug)]
pub struct CompactionPlanningMemoryEnvelope<'runtime> {
    allocation: MaintenancePhysicalAllocation<'runtime>,
}

impl<'runtime> CompactionPlanningMemoryEnvelope<'runtime> {
    pub const fn from_store_allocation(
        allocation: MaintenancePhysicalAllocation<'runtime>,
    ) -> Self {
        Self { allocation }
    }

    pub const fn allocation_scope(&self) -> PhysicalOperationAllocationScope {
        PhysicalOperationAllocationScope::Maintenance
    }

    pub const fn allocation_bytes(&self) -> u64 {
        self.allocation.bytes()
    }

    pub const fn proves_compaction_validity(&self) -> bool {
        false
    }

    pub const fn proves_retained_truth_preservation(&self) -> bool {
        false
    }
}

#[derive(Debug)]
pub struct ImportExportMemoryEnvelope<'runtime> {
    allocation: MaintenancePhysicalAllocation<'runtime>,
}

impl<'runtime> ImportExportMemoryEnvelope<'runtime> {
    pub const fn from_store_allocation(
        allocation: MaintenancePhysicalAllocation<'runtime>,
    ) -> Self {
        Self { allocation }
    }

    pub const fn allocation_scope(&self) -> PhysicalOperationAllocationScope {
        PhysicalOperationAllocationScope::Maintenance
    }

    pub const fn allocation_bytes(&self) -> u64 {
        self.allocation.bytes()
    }

    pub const fn proves_import_export_semantic_correctness(&self) -> bool {
        false
    }

    pub const fn proves_replication_correctness(&self) -> bool {
        false
    }
}
