mod allocations;
mod counters;

pub use allocations::{
    PhysicalResidencyAllocationEventSnapshot, PhysicalResidencyAllocationSnapshot,
};
pub use counters::PhysicalResidencyCounterSnapshot;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalResidencyObservation {
    store: worth_store_physical_format::store_namespace::StableStoreIdentity,
    store_generation: crate::physical_runtime::LifecycleGeneration,
    admitted_policy: crate::physical_runtime::record_serving::AdmittedPhysicalRecordResidencyPolicy,
    counters: PhysicalResidencyCounterSnapshot,
    allocations: PhysicalResidencyAllocationSnapshot,
}

impl PhysicalResidencyObservation {
    pub(in crate::physical_runtime) const fn new(
        store: worth_store_physical_format::store_namespace::StableStoreIdentity,
        store_generation: crate::physical_runtime::LifecycleGeneration,
        admitted_policy:
            crate::physical_runtime::record_serving::AdmittedPhysicalRecordResidencyPolicy,
        counters: worth_store_buffer_pool::PhysicalResidencyCounters,
        allocations: worth_store_buffer_pool::PhysicalResidencyAllocationEventSnapshot,
    ) -> Self {
        Self {
            store,
            store_generation,
            admitted_policy,
            counters: PhysicalResidencyCounterSnapshot::new(counters),
            allocations: PhysicalResidencyAllocationSnapshot::new(allocations),
        }
    }

    pub const fn store_identity(
        self,
    ) -> worth_store_physical_format::store_namespace::StableStoreIdentity {
        self.store
    }

    pub const fn store_generation(self) -> crate::physical_runtime::LifecycleGeneration {
        self.store_generation
    }

    pub const fn admitted_policy(
        self,
    ) -> crate::physical_runtime::record_serving::AdmittedPhysicalRecordResidencyPolicy {
        self.admitted_policy
    }

    pub const fn counters(self) -> PhysicalResidencyCounterSnapshot {
        self.counters
    }

    pub const fn allocations(self) -> PhysicalResidencyAllocationSnapshot {
        self.allocations
    }
}
