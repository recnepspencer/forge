use std::sync::Arc;

use worth_store_physical_format::store_namespace::StableStoreIdentity;

use super::{
    pool::PoolInner, OperationAllocationScope, PhysicalResidencyCounters,
    PhysicalResidencyIncarnation,
};

#[derive(Debug)]
pub struct OperationAllocationGrant {
    pub(crate) owner: Arc<PoolInner>,
    pub(crate) scope: OperationAllocationScope,
    pub(crate) bytes: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OperationAllocationObservation {
    store: StableStoreIdentity,
    pool: PhysicalResidencyIncarnation,
    scope: OperationAllocationScope,
    bytes: u64,
    counters: PhysicalResidencyCounters,
}

impl OperationAllocationGrant {
    pub const fn bytes(&self) -> u64 {
        self.bytes
    }

    pub const fn scope(&self) -> OperationAllocationScope {
        self.scope
    }

    pub fn observation(&self) -> OperationAllocationObservation {
        OperationAllocationObservation {
            store: self.owner.store_identity(),
            pool: self.owner.incarnation(),
            scope: self.scope,
            bytes: self.bytes,
            counters: self.owner.counters(),
        }
    }
}

impl Drop for OperationAllocationGrant {
    fn drop(&mut self) {
        self.owner.release_operation(self.scope, self.bytes);
    }
}

impl OperationAllocationObservation {
    pub const fn store(self) -> StableStoreIdentity {
        self.store
    }

    pub const fn pool(self) -> PhysicalResidencyIncarnation {
        self.pool
    }

    pub const fn scope(self) -> OperationAllocationScope {
        self.scope
    }

    pub const fn bytes(self) -> u64 {
        self.bytes
    }

    pub const fn counters(self) -> PhysicalResidencyCounters {
        self.counters
    }
}
