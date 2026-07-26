use worth_store_physical_format::store_namespace::StableStoreIdentity;

use super::super::{
    PhysicalOperationAllocationScope, PhysicalResidencyDimension, PhysicalResidencyIncarnation,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalResidencyPressureDenial {
    store: StableStoreIdentity,
    pool: PhysicalResidencyIncarnation,
    dimension: PhysicalResidencyDimension,
    scope: PhysicalOperationAllocationScope,
    requested: u64,
    current: u64,
    limit: u64,
}

pub(in crate::physical_residency) struct PhysicalResidencyPressureDemand {
    pub(in crate::physical_residency) dimension: PhysicalResidencyDimension,
    pub(in crate::physical_residency) scope: PhysicalOperationAllocationScope,
    pub(in crate::physical_residency) requested: u64,
    pub(in crate::physical_residency) current: u64,
    pub(in crate::physical_residency) limit: u64,
}

impl PhysicalResidencyPressureDenial {
    pub(in crate::physical_residency) const fn new(
        store: StableStoreIdentity,
        pool: PhysicalResidencyIncarnation,
        demand: PhysicalResidencyPressureDemand,
    ) -> Self {
        Self {
            store,
            pool,
            dimension: demand.dimension,
            scope: demand.scope,
            requested: demand.requested,
            current: demand.current,
            limit: demand.limit,
        }
    }

    pub const fn store(self) -> StableStoreIdentity {
        self.store
    }

    pub const fn pool(self) -> PhysicalResidencyIncarnation {
        self.pool
    }

    pub const fn dimension(self) -> PhysicalResidencyDimension {
        self.dimension
    }

    pub const fn scope(self) -> PhysicalOperationAllocationScope {
        self.scope
    }

    pub const fn requested(self) -> u64 {
        self.requested
    }

    pub const fn current(self) -> u64 {
        self.current
    }

    pub const fn limit(self) -> u64 {
        self.limit
    }

    pub const fn effect_may_have_started(self) -> bool {
        false
    }
}
