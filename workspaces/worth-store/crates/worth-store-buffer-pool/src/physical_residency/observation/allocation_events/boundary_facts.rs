use super::actualization::PhysicalResidencyAllocationActualization;
use super::boundary_trace::{
    PhysicalResidencyAllocationBoundaryKind, PhysicalResidencyAllocationOperation,
};
use crate::{PhysicalOperationAllocationScope, PhysicalResidencyDimension};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct PhysicalResidencyAllocationBoundaryFacts {
    pub(super) kind: PhysicalResidencyAllocationBoundaryKind,
    pub(super) dimension: PhysicalResidencyDimension,
    pub(super) scope: Option<PhysicalOperationAllocationScope>,
    pub(super) requested_units: u64,
    pub(super) actual_units: u64,
    pub(super) operation: Option<PhysicalResidencyAllocationOperation>,
}

impl PhysicalResidencyAllocationBoundaryFacts {
    pub(super) const fn admission(
        dimension: PhysicalResidencyDimension,
        scope: Option<PhysicalOperationAllocationScope>,
        units: u64,
    ) -> Self {
        Self {
            kind: PhysicalResidencyAllocationBoundaryKind::Admission,
            dimension,
            scope,
            requested_units: units,
            actual_units: units,
            operation: None,
        }
    }

    pub(super) const fn release(
        dimension: PhysicalResidencyDimension,
        scope: Option<PhysicalOperationAllocationScope>,
        units: u64,
    ) -> Self {
        Self {
            kind: PhysicalResidencyAllocationBoundaryKind::Release,
            dimension,
            scope,
            requested_units: units,
            actual_units: units,
            operation: None,
        }
    }

    pub(super) const fn denial(
        dimension: PhysicalResidencyDimension,
        scope: Option<PhysicalOperationAllocationScope>,
        units: u64,
    ) -> Self {
        Self {
            kind: PhysicalResidencyAllocationBoundaryKind::Denial,
            dimension,
            scope,
            requested_units: units,
            actual_units: 0,
            operation: None,
        }
    }

    pub(super) const fn allocator_failure(
        dimension: PhysicalResidencyDimension,
        scope: Option<PhysicalOperationAllocationScope>,
        units: u64,
    ) -> Self {
        Self {
            kind: PhysicalResidencyAllocationBoundaryKind::AllocatorFailure,
            dimension,
            scope,
            requested_units: units,
            actual_units: 0,
            operation: None,
        }
    }

    pub(super) const fn actualization(
        actualization: PhysicalResidencyAllocationActualization,
    ) -> Self {
        Self {
            kind: PhysicalResidencyAllocationBoundaryKind::Actualization,
            dimension: actualization.dimension(),
            scope: Some(actualization.scope()),
            requested_units: actualization.requested_units(),
            actual_units: actualization.actual_units(),
            operation: actualization.operation(),
        }
    }
}
