use worth_store_buffer_pool::{OperationAllocationGrant, PhysicalResidencyDenial};
use worth_store_physical_format::store_namespace::StableStoreIdentity;

use crate::physical_runtime::{
    PhysicalOperationAllocationScope, PhysicalRecordResidencyFailure, PhysicalResidencyDimension,
};

/// A live operation-scope admission held only by certification code.
///
/// The lower allocation grant remains sealed inside this value. Certification
/// can prove scope accounting and release, but cannot spend the grant on pool
/// operations.
#[derive(Debug)]
pub struct CertificationScopedAllocation {
    allocation: OperationAllocationGrant,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CertificationScopeAdmissionFailure {
    Pressure(CertificationScopePressure),
    Residency(PhysicalRecordResidencyFailure),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CertificationScopePressure {
    store: StableStoreIdentity,
    dimension: PhysicalResidencyDimension,
    scope: PhysicalOperationAllocationScope,
    requested: u64,
    current: u64,
    limit: u64,
}

impl CertificationScopedAllocation {
    pub(super) const fn bind(allocation: OperationAllocationGrant) -> Self {
        Self { allocation }
    }

    pub fn store_identity(&self) -> StableStoreIdentity {
        self.allocation.observation().store()
    }

    pub const fn scope(&self) -> PhysicalOperationAllocationScope {
        self.allocation.scope()
    }

    pub const fn bytes(&self) -> u64 {
        self.allocation.bytes()
    }
}

impl CertificationScopeAdmissionFailure {
    pub(super) fn from_denial(denial: PhysicalResidencyDenial) -> Self {
        match denial {
            PhysicalResidencyDenial::Pressure(pressure) => {
                Self::Pressure(CertificationScopePressure {
                    store: pressure.store(),
                    dimension: pressure.dimension(),
                    scope: pressure.scope(),
                    requested: pressure.requested(),
                    current: pressure.current(),
                    limit: pressure.limit(),
                })
            }
            other => Self::Residency(other.into()),
        }
    }
}

impl CertificationScopePressure {
    pub const fn store_identity(self) -> StableStoreIdentity {
        self.store
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
