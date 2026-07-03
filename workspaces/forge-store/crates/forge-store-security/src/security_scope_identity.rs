use forge_store_aspect_native::StorePhysicalBoundaryWitness;

use crate::{StoreAuthenticityRequirement, StoreCustodyPosture, StoreKeyScope, StoreTenantScope};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StoreSecurityScopeIdentity {
    physical_witness: StorePhysicalBoundaryWitness,
    key_scope: StoreKeyScope,
    tenant_scope: StoreTenantScope,
    authenticity_requirement: StoreAuthenticityRequirement,
    custody_posture: StoreCustodyPosture,
}

impl StoreSecurityScopeIdentity {
    pub const fn from_physical_security_scope(
        physical_witness: StorePhysicalBoundaryWitness,
        key_scope: StoreKeyScope,
        tenant_scope: StoreTenantScope,
        authenticity_requirement: StoreAuthenticityRequirement,
        custody_posture: StoreCustodyPosture,
    ) -> Self {
        Self {
            physical_witness,
            key_scope,
            tenant_scope,
            authenticity_requirement,
            custody_posture,
        }
    }

    pub const fn physical_witness(&self) -> StorePhysicalBoundaryWitness {
        self.physical_witness
    }

    pub const fn key_scope(&self) -> StoreKeyScope {
        self.key_scope
    }

    pub const fn tenant_scope(&self) -> StoreTenantScope {
        self.tenant_scope
    }

    pub const fn authenticity_requirement(&self) -> StoreAuthenticityRequirement {
        self.authenticity_requirement
    }

    pub const fn custody_posture(&self) -> StoreCustodyPosture {
        self.custody_posture
    }
}
