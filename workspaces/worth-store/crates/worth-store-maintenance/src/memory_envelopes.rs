use worth_store_buffer_pool::{
    OperationAllocationGrant, OperationAllocationObservation, PhysicalOperationAllocationScope,
    PhysicalResidencyCounters,
};

#[derive(Debug)]
pub struct CompactionPlanningMemoryEnvelope {
    allocation: OperationAllocationGrant,
}

impl CompactionPlanningMemoryEnvelope {
    pub fn from_allocation_grant(
        allocation: OperationAllocationGrant,
    ) -> Result<Self, MaintenanceMemoryEnvelopeDenial> {
        require_maintenance_scope(allocation).map(|allocation| Self { allocation })
    }

    pub const fn allocation_scope(&self) -> PhysicalOperationAllocationScope {
        self.allocation.scope()
    }

    pub const fn allocation_bytes(&self) -> u64 {
        self.allocation.bytes()
    }

    pub fn allocation_observation(&self) -> OperationAllocationObservation {
        self.allocation.observation()
    }

    pub fn counters(&self) -> PhysicalResidencyCounters {
        self.allocation.observation().counters()
    }

    pub const fn proves_compaction_validity(&self) -> bool {
        false
    }

    pub const fn proves_retained_truth_preservation(&self) -> bool {
        false
    }
}

#[derive(Debug)]
pub struct ImportExportMemoryEnvelope {
    allocation: OperationAllocationGrant,
}

impl ImportExportMemoryEnvelope {
    pub fn from_allocation_grant(
        allocation: OperationAllocationGrant,
    ) -> Result<Self, MaintenanceMemoryEnvelopeDenial> {
        require_maintenance_scope(allocation).map(|allocation| Self { allocation })
    }

    pub const fn allocation_scope(&self) -> PhysicalOperationAllocationScope {
        self.allocation.scope()
    }

    pub const fn allocation_bytes(&self) -> u64 {
        self.allocation.bytes()
    }

    pub fn allocation_observation(&self) -> OperationAllocationObservation {
        self.allocation.observation()
    }

    pub fn counters(&self) -> PhysicalResidencyCounters {
        self.allocation.observation().counters()
    }

    pub const fn proves_import_export_semantic_correctness(&self) -> bool {
        false
    }

    pub const fn proves_replication_correctness(&self) -> bool {
        false
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaintenanceMemoryEnvelopeDenial {
    WrongAllocationScope {
        actual: PhysicalOperationAllocationScope,
    },
}

fn require_maintenance_scope(
    allocation: OperationAllocationGrant,
) -> Result<OperationAllocationGrant, MaintenanceMemoryEnvelopeDenial> {
    if allocation.scope() == PhysicalOperationAllocationScope::Maintenance {
        Ok(allocation)
    } else {
        Err(MaintenanceMemoryEnvelopeDenial::WrongAllocationScope {
            actual: allocation.scope(),
        })
    }
}
