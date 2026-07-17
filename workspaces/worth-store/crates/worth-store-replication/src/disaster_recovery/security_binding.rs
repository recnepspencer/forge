use worth_store_security::{
    StoreAuthenticityRequirement, StoreCustodyPosture, StoreKeyScope, StoreKeyVersionPosture,
    StoreSecurityScopeIdentity, StoreTenantScope,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DisasterRecoverySecurityBinding {
    scope_fingerprint: [u8; 32],
    key_scope: StoreKeyScope,
    key_version_posture: StoreKeyVersionPosture,
    tenant_scope: StoreTenantScope,
    authenticity_requirement: StoreAuthenticityRequirement,
    custody_posture: StoreCustodyPosture,
}

impl DisasterRecoverySecurityBinding {
    pub fn from_current_scope(scope: StoreSecurityScopeIdentity) -> Self {
        Self {
            scope_fingerprint: scope.stable_fingerprint(),
            key_scope: scope.key_scope(),
            key_version_posture: scope.key_version_posture(),
            tenant_scope: scope.tenant_scope(),
            authenticity_requirement: scope.authenticity_requirement(),
            custody_posture: scope.custody_posture(),
        }
    }

    pub(super) fn from_persisted_description(
        scope_fingerprint: [u8; 32],
        key_scope: StoreKeyScope,
        key_version_posture: StoreKeyVersionPosture,
        tenant_scope: StoreTenantScope,
        authenticity_requirement: StoreAuthenticityRequirement,
        custody_posture: StoreCustodyPosture,
    ) -> Option<Self> {
        if scope_fingerprint == [0; 32] {
            return None;
        }
        Some(Self {
            scope_fingerprint,
            key_scope,
            key_version_posture,
            tenant_scope,
            authenticity_requirement,
            custody_posture,
        })
    }

    pub const fn scope_fingerprint(self) -> [u8; 32] {
        self.scope_fingerprint
    }

    pub const fn key_scope(self) -> StoreKeyScope {
        self.key_scope
    }

    pub const fn key_version_posture(self) -> StoreKeyVersionPosture {
        self.key_version_posture
    }

    pub const fn tenant_scope(self) -> StoreTenantScope {
        self.tenant_scope
    }

    pub const fn authenticity_requirement(self) -> StoreAuthenticityRequirement {
        self.authenticity_requirement
    }

    pub const fn custody_posture(self) -> StoreCustodyPosture {
        self.custody_posture
    }
}
