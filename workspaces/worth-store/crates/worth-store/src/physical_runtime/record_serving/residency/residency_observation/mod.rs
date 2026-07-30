#[cfg(feature = "certification-test-authority")]
mod allocation_trace;
mod allocations;
mod counters;
mod writeback;

#[cfg(feature = "certification-test-authority")]
pub use allocation_trace::{
    PhysicalResidencyAllocationBoundaryEvent, PhysicalResidencyAllocationBoundaryKind,
    PhysicalResidencyAllocationTrace,
};
pub use allocations::{
    PhysicalResidencyAllocationEventSnapshot, PhysicalResidencyAllocationSnapshot,
};
pub use counters::PhysicalResidencyCounterSnapshot;
pub(super) use writeback::PhysicalWritebackCounterCells;
pub use writeback::PhysicalWritebackCounterSnapshot;

/// Read-only physical residency evidence for one serving Store generation.
///
/// The observation binds the admitted policy to current counters, allocation
/// events, and writeback outcomes. It exposes no allocation, eviction, dirty,
/// retry, or lifecycle control.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalResidencyObservation {
    store: worth_store_physical_format::store_namespace::StableStoreIdentity,
    store_generation: crate::physical_runtime::LifecycleGeneration,
    admitted_policy: crate::physical_runtime::record_serving::AdmittedPhysicalRecordResidencyPolicy,
    counters: PhysicalResidencyCounterSnapshot,
    allocations: PhysicalResidencyAllocationSnapshot,
    writebacks: PhysicalWritebackCounterSnapshot,
}

impl PhysicalResidencyObservation {
    pub(in crate::physical_runtime) const fn new(
        store: worth_store_physical_format::store_namespace::StableStoreIdentity,
        store_generation: crate::physical_runtime::LifecycleGeneration,
        admitted_policy:
            crate::physical_runtime::record_serving::AdmittedPhysicalRecordResidencyPolicy,
        counters: worth_store_buffer_pool::PhysicalResidencyCounters,
        allocations: worth_store_buffer_pool::PhysicalResidencyAllocationEventSnapshot,
        writebacks: PhysicalWritebackCounterSnapshot,
    ) -> Self {
        Self {
            store,
            store_generation,
            admitted_policy,
            counters: PhysicalResidencyCounterSnapshot::new(counters),
            allocations: PhysicalResidencyAllocationSnapshot::new(allocations),
            writebacks,
        }
    }

    /// Returns the stable physical Store identity.
    pub const fn store_identity(
        self,
    ) -> worth_store_physical_format::store_namespace::StableStoreIdentity {
        self.store
    }

    /// Returns the serving lifecycle generation being observed.
    pub const fn store_generation(self) -> crate::physical_runtime::LifecycleGeneration {
        self.store_generation
    }

    /// Returns the policy admitted for this Store instance.
    pub const fn admitted_policy(
        self,
    ) -> crate::physical_runtime::record_serving::AdmittedPhysicalRecordResidencyPolicy {
        self.admitted_policy
    }

    /// Returns current, peak, and transition residency counters.
    pub const fn counters(self) -> PhysicalResidencyCounterSnapshot {
        self.counters
    }

    /// Returns allocation-event evidence grouped by residency dimension.
    pub const fn allocations(self) -> PhysicalResidencyAllocationSnapshot {
        self.allocations
    }

    /// Returns dirty-frame writeback outcome counters.
    pub const fn writebacks(self) -> PhysicalWritebackCounterSnapshot {
        self.writebacks
    }
}
