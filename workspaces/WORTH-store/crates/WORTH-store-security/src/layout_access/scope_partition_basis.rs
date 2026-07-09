use crate::{
    StoreAuthenticityRequirement, StoreCurrentSecurityScopeWitnessSet, StoreCustodyPosture,
    StoreKeyScope, StoreKeyVersionPosture, StoreSecurityScopeIdentity, StoreTenantScope,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StoreLayoutAccessSecurityBoundaryWitness {
    identity: StoreSecurityScopeIdentity,
}

impl StoreLayoutAccessSecurityBoundaryWitness {
    pub(crate) const fn new(identity: StoreSecurityScopeIdentity) -> Self {
        Self { identity }
    }

    pub const fn identity(self) -> StoreSecurityScopeIdentity {
        self.identity
    }

    pub const fn key_scope(self) -> StoreKeyScope {
        self.identity.key_scope()
    }

    pub const fn key_version_posture(self) -> StoreKeyVersionPosture {
        self.identity.key_version_posture()
    }

    pub const fn tenant_scope(self) -> StoreTenantScope {
        self.identity.tenant_scope()
    }

    pub const fn authenticity_requirement(self) -> StoreAuthenticityRequirement {
        self.identity.authenticity_requirement()
    }

    pub const fn custody_posture(self) -> StoreCustodyPosture {
        self.identity.custody_posture()
    }
}

pub fn admit_layout_access_security_boundary(
    security_scope: &StoreCurrentSecurityScopeWitnessSet,
) -> StoreLayoutAccessSecurityBoundaryWitness {
    let identity = security_scope.key_scope().identity();
    debug_assert_eq!(identity, security_scope.key_version_scope().identity());
    debug_assert_eq!(identity, security_scope.tenant_scope().identity());
    debug_assert_eq!(identity, security_scope.authenticity_scope().identity());
    debug_assert_eq!(identity, security_scope.custody_scope().identity());
    StoreLayoutAccessSecurityBoundaryWitness::new(identity)
}
