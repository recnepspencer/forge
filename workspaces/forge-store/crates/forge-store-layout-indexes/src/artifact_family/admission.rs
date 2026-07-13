use forge_store_security::{StoreCurrentSecurityScopeWitnessSet, StoreSecurityScopeIdentity};

use crate::catalog::{
    classify_family, require_production_authority, require_strategy_lifecycle,
    ArtifactFamilyClassification, ArtifactFamilyDenial, ArtifactFamilyLifecycleAdmission,
    ArtifactFamilyStrategyLane, PhysicalArtifactFamilyDeclaration,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdmittedPhysicalArtifactFamily {
    lifecycle: ArtifactFamilyLifecycleAdmission,
    security_identity: StoreSecurityScopeIdentity,
    authority_identity: forge_store_authority::StoreCurrentAuthorityIdentity,
}

impl AdmittedPhysicalArtifactFamily {
    pub(crate) fn admit(
        declaration: &'static PhysicalArtifactFamilyDeclaration,
        security: &StoreCurrentSecurityScopeWitnessSet,
    ) -> Result<Self, ArtifactFamilyDenial> {
        let classification = classify_family(declaration);
        let authority = require_production_authority(classification)?;
        let lifecycle = require_strategy_lifecycle(authority)?;
        Ok(Self {
            lifecycle,
            security_identity: security.key_scope().identity(),
            authority_identity: security.authority_identity(),
        })
    }

    pub const fn declaration(self) -> &'static PhysicalArtifactFamilyDeclaration {
        self.lifecycle.declaration()
    }

    pub const fn family_id(self) -> forge_store_contracts::DurableArtifactFamilyId {
        self.lifecycle.family_id()
    }

    pub const fn admitted_lane(self) -> ArtifactFamilyStrategyLane {
        self.lifecycle.admitted_lane()
    }

    pub const fn classification(self) -> ArtifactFamilyClassification {
        self.lifecycle.authority().classification()
    }

    pub const fn security_identity(self) -> StoreSecurityScopeIdentity {
        self.security_identity
    }

    pub const fn authority_identity(self) -> forge_store_authority::StoreCurrentAuthorityIdentity {
        self.authority_identity
    }

    pub(crate) const fn lifecycle(self) -> ArtifactFamilyLifecycleAdmission {
        self.lifecycle
    }
}
