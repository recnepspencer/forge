use forge_store_readiness::{
    accept_s5_1_admitted_security_scope_readiness, S51SecurityScopeReadinessReservation,
};
use forge_store_security::StoreSecurityScopeIdentity;

use crate::{
    RepairBlastRadiusCounterSnapshot, RepairBlastRadiusDeclaration, RepairBlastRadiusDenial,
    RepairBlastRadiusReadiness, RepairPhysicalRegion,
};

#[derive(Debug, PartialEq, Eq)]
pub struct RepairBlastRadiusPlan {
    declaration: RepairBlastRadiusDeclaration,
}

impl RepairBlastRadiusPlan {
    pub fn declare(declaration: RepairBlastRadiusDeclaration) -> Self {
        Self { declaration }
    }

    pub fn admit_with_store_blast_radius(
        self,
    ) -> Result<RepairBlastRadiusReadiness, RepairBlastRadiusDenial> {
        let (physical_region, admitted_scope, counters) = self.declaration.into_parts();
        let readiness = accept_s5_1_admitted_security_scope_readiness(
            S51SecurityScopeReadinessReservation::repair_blast_radius(),
            admitted_scope,
        );
        RepairBlastRadiusReadiness::from_admitted_readiness(physical_region, readiness, counters)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepairReadPlan {
    security_scope: StoreSecurityScopeIdentity,
    physical_region: RepairPhysicalRegion,
    counters: RepairBlastRadiusCounterSnapshot,
}

impl RepairReadPlan {
    pub(crate) const fn from_readiness(
        security_scope: StoreSecurityScopeIdentity,
        physical_region: RepairPhysicalRegion,
        counters: RepairBlastRadiusCounterSnapshot,
    ) -> Self {
        Self {
            security_scope,
            physical_region,
            counters,
        }
    }

    pub const fn security_scope(&self) -> StoreSecurityScopeIdentity {
        self.security_scope
    }

    pub fn physical_region(&self) -> &RepairPhysicalRegion {
        &self.physical_region
    }

    pub const fn counters(&self) -> RepairBlastRadiusCounterSnapshot {
        self.counters
    }
}
