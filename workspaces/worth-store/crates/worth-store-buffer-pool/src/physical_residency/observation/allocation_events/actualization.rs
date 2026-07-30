use super::PhysicalResidencyAllocationOperation;
use crate::{PhysicalOperationAllocationScope, PhysicalResidencyDimension};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct PhysicalResidencyRequestedAllocationUnits(u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct PhysicalResidencyActualAllocationUnits(u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct PhysicalResidencyAllocationActualization {
    dimension: PhysicalResidencyDimension,
    scope: PhysicalOperationAllocationScope,
    requested: PhysicalResidencyRequestedAllocationUnits,
    actual: PhysicalResidencyActualAllocationUnits,
    operation: Option<PhysicalResidencyAllocationOperation>,
}

impl PhysicalResidencyRequestedAllocationUnits {
    pub(crate) const fn new(units: u64) -> Self {
        Self(units)
    }
}

impl PhysicalResidencyActualAllocationUnits {
    pub(crate) const fn new(units: u64) -> Self {
        Self(units)
    }
}

impl PhysicalResidencyAllocationActualization {
    pub(crate) const fn new(
        dimension: PhysicalResidencyDimension,
        scope: PhysicalOperationAllocationScope,
        requested: PhysicalResidencyRequestedAllocationUnits,
        actual: PhysicalResidencyActualAllocationUnits,
    ) -> Self {
        Self {
            dimension,
            scope,
            requested,
            actual,
            operation: None,
        }
    }

    pub(crate) const fn with_operation(
        mut self,
        operation: Option<PhysicalResidencyAllocationOperation>,
    ) -> Self {
        self.operation = operation;
        self
    }

    pub(super) const fn dimension(self) -> PhysicalResidencyDimension {
        self.dimension
    }

    pub(super) const fn scope(self) -> PhysicalOperationAllocationScope {
        self.scope
    }

    pub(super) const fn requested_units(self) -> u64 {
        self.requested.0
    }

    pub(super) const fn actual_units(self) -> u64 {
        self.actual.0
    }

    pub(super) const fn operation(self) -> Option<PhysicalResidencyAllocationOperation> {
        self.operation
    }
}
