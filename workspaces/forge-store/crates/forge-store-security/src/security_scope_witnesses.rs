use crate::{
    StoreAuthenticityRequirement, StoreCustodyPosture, StoreKeyScope, StoreKeyVersionPosture,
    StoreSecurityScopeAdmissionReceipt, StoreSecurityScopeIdentity, StoreTenantScope,
};

#[derive(Debug, PartialEq, Eq)]
pub struct StoreCurrentKeyScopeWitness {
    identity: StoreSecurityScopeIdentity,
    key_scope: StoreKeyScope,
}

impl StoreCurrentKeyScopeWitness {
    pub(crate) const fn new(identity: StoreSecurityScopeIdentity) -> Self {
        Self {
            identity,
            key_scope: identity.key_scope(),
        }
    }

    pub const fn identity(&self) -> StoreSecurityScopeIdentity {
        self.identity
    }

    pub const fn key_scope(&self) -> StoreKeyScope {
        self.key_scope
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct StoreCurrentKeyVersionScopeWitness {
    identity: StoreSecurityScopeIdentity,
    key_version_posture: StoreKeyVersionPosture,
}

impl StoreCurrentKeyVersionScopeWitness {
    pub(crate) const fn new(identity: StoreSecurityScopeIdentity) -> Self {
        Self {
            identity,
            key_version_posture: identity.key_version_posture(),
        }
    }

    pub const fn identity(&self) -> StoreSecurityScopeIdentity {
        self.identity
    }

    pub const fn key_version_posture(&self) -> StoreKeyVersionPosture {
        self.key_version_posture
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct StoreCurrentTenantScopeWitness {
    identity: StoreSecurityScopeIdentity,
    tenant_scope: StoreTenantScope,
}

impl StoreCurrentTenantScopeWitness {
    pub(crate) const fn new(identity: StoreSecurityScopeIdentity) -> Self {
        Self {
            identity,
            tenant_scope: identity.tenant_scope(),
        }
    }

    pub const fn identity(&self) -> StoreSecurityScopeIdentity {
        self.identity
    }

    pub const fn tenant_scope(&self) -> StoreTenantScope {
        self.tenant_scope
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct StoreCurrentAuthenticityScopeWitness {
    identity: StoreSecurityScopeIdentity,
    requirement: StoreAuthenticityRequirement,
}

impl StoreCurrentAuthenticityScopeWitness {
    pub(crate) const fn new(identity: StoreSecurityScopeIdentity) -> Self {
        Self {
            identity,
            requirement: identity.authenticity_requirement(),
        }
    }

    pub const fn identity(&self) -> StoreSecurityScopeIdentity {
        self.identity
    }

    pub const fn requirement(&self) -> StoreAuthenticityRequirement {
        self.requirement
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct StoreCurrentCustodyScopeWitness {
    identity: StoreSecurityScopeIdentity,
    custody_posture: StoreCustodyPosture,
}

impl StoreCurrentCustodyScopeWitness {
    pub(crate) const fn new(identity: StoreSecurityScopeIdentity) -> Self {
        Self {
            identity,
            custody_posture: identity.custody_posture(),
        }
    }

    pub const fn identity(&self) -> StoreSecurityScopeIdentity {
        self.identity
    }

    pub const fn custody_posture(&self) -> StoreCustodyPosture {
        self.custody_posture
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct StoreCurrentSecurityScopeWitnessSet {
    key_scope: StoreCurrentKeyScopeWitness,
    key_version_scope: StoreCurrentKeyVersionScopeWitness,
    tenant_scope: StoreCurrentTenantScopeWitness,
    authenticity_scope: StoreCurrentAuthenticityScopeWitness,
    custody_scope: StoreCurrentCustodyScopeWitness,
}

impl StoreCurrentSecurityScopeWitnessSet {
    pub(crate) const fn new(identity: StoreSecurityScopeIdentity) -> Self {
        Self {
            key_scope: StoreCurrentKeyScopeWitness::new(identity),
            key_version_scope: StoreCurrentKeyVersionScopeWitness::new(identity),
            tenant_scope: StoreCurrentTenantScopeWitness::new(identity),
            authenticity_scope: StoreCurrentAuthenticityScopeWitness::new(identity),
            custody_scope: StoreCurrentCustodyScopeWitness::new(identity),
        }
    }

    pub const fn key_scope(&self) -> &StoreCurrentKeyScopeWitness {
        &self.key_scope
    }

    pub const fn key_version_scope(&self) -> &StoreCurrentKeyVersionScopeWitness {
        &self.key_version_scope
    }

    pub const fn tenant_scope(&self) -> &StoreCurrentTenantScopeWitness {
        &self.tenant_scope
    }

    pub const fn authenticity_scope(&self) -> &StoreCurrentAuthenticityScopeWitness {
        &self.authenticity_scope
    }

    pub const fn custody_scope(&self) -> &StoreCurrentCustodyScopeWitness {
        &self.custody_scope
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct StoreAdmittedSecurityScope {
    witnesses: StoreCurrentSecurityScopeWitnessSet,
    receipt: StoreSecurityScopeAdmissionReceipt,
}

impl StoreAdmittedSecurityScope {
    pub(crate) const fn new(
        witnesses: StoreCurrentSecurityScopeWitnessSet,
        receipt: StoreSecurityScopeAdmissionReceipt,
    ) -> Self {
        Self { witnesses, receipt }
    }

    pub const fn witnesses(&self) -> &StoreCurrentSecurityScopeWitnessSet {
        &self.witnesses
    }

    pub const fn receipt(&self) -> StoreSecurityScopeAdmissionReceipt {
        self.receipt
    }

    pub const fn identity(&self) -> StoreSecurityScopeIdentity {
        self.receipt.identity()
    }
}
