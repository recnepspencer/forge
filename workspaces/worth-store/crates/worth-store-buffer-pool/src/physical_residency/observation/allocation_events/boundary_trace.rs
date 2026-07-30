use worth_store_physical_format::store_namespace::StableStoreIdentity;

use super::super::super::{
    PhysicalOperationAllocationScope, PhysicalResidencyDimension, PhysicalResidencyIncarnation,
};
#[cfg(feature = "certification-test-authority")]
use super::boundary_facts::PhysicalResidencyAllocationBoundaryFacts;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalResidencyAllocationBoundaryKind {
    Admission,
    Release,
    Denial,
    AllocatorFailure,
    Actualization,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalResidencyAllocationOperation(std::num::NonZeroU64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalResidencyAllocationBoundaryEvent {
    sequence: u64,
    kind: PhysicalResidencyAllocationBoundaryKind,
    dimension: PhysicalResidencyDimension,
    scope: Option<PhysicalOperationAllocationScope>,
    requested_units: u64,
    actual_units: u64,
    process: u32,
    operation: Option<PhysicalResidencyAllocationOperation>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalResidencyAllocationTrace {
    store: StableStoreIdentity,
    pool: PhysicalResidencyIncarnation,
    events: Vec<PhysicalResidencyAllocationBoundaryEvent>,
}

impl PhysicalResidencyAllocationOperation {
    pub const fn new(operation: std::num::NonZeroU64) -> Self {
        Self(operation)
    }

    pub const fn get(self) -> u64 {
        self.0.get()
    }
}

impl PhysicalResidencyAllocationBoundaryEvent {
    #[cfg(feature = "certification-test-authority")]
    pub(super) fn new(sequence: u64, facts: PhysicalResidencyAllocationBoundaryFacts) -> Self {
        Self {
            sequence,
            kind: facts.kind,
            dimension: facts.dimension,
            scope: facts.scope,
            requested_units: facts.requested_units,
            actual_units: facts.actual_units,
            process: std::process::id(),
            operation: facts.operation,
        }
    }

    pub const fn sequence(self) -> u64 {
        self.sequence
    }

    pub const fn kind(self) -> PhysicalResidencyAllocationBoundaryKind {
        self.kind
    }

    pub const fn dimension(self) -> PhysicalResidencyDimension {
        self.dimension
    }

    pub const fn scope(self) -> Option<PhysicalOperationAllocationScope> {
        self.scope
    }

    pub const fn requested_units(self) -> u64 {
        self.requested_units
    }

    pub const fn actual_units(self) -> u64 {
        self.actual_units
    }

    pub const fn process(self) -> u32 {
        self.process
    }

    pub const fn operation(self) -> Option<PhysicalResidencyAllocationOperation> {
        self.operation
    }
}

impl PhysicalResidencyAllocationTrace {
    #[cfg(feature = "certification-test-authority")]
    pub(super) fn new(
        store: StableStoreIdentity,
        pool: PhysicalResidencyIncarnation,
        events: Vec<PhysicalResidencyAllocationBoundaryEvent>,
    ) -> Self {
        Self {
            store,
            pool,
            events,
        }
    }

    pub const fn store(&self) -> StableStoreIdentity {
        self.store
    }

    pub const fn pool(&self) -> PhysicalResidencyIncarnation {
        self.pool
    }

    pub fn events(&self) -> &[PhysicalResidencyAllocationBoundaryEvent] {
        &self.events
    }
}
