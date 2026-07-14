use forge_store_authority::StoreCurrentAuthorityWitness;

use crate::PhysicalArtifactFamilyDeclaration;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LayoutRebindRequired {
    family: &'static PhysicalArtifactFamilyDeclaration,
    bound_authority: StoreCurrentAuthorityWitness,
    bound_security: forge_store_security::StoreSecurityScopeIdentity,
    current_authority: forge_store_authority::StoreCurrentAuthorityIdentity,
    current_security: forge_store_security::StoreSecurityScopeIdentity,
}

impl LayoutRebindRequired {
    pub(crate) fn new(
        family: &'static PhysicalArtifactFamilyDeclaration,
        binding: &super::LayoutBindingWitness,
        current_family: crate::AdmittedPhysicalArtifactFamily,
        current_authority: &StoreCurrentAuthorityWitness,
    ) -> Self {
        Self {
            family,
            bound_authority: binding.bound_authority().clone(),
            bound_security: binding.security_identity(),
            current_authority: current_authority.authority_identity(),
            current_security: current_family.security_identity(),
        }
    }

    pub const fn family(&self) -> &'static PhysicalArtifactFamilyDeclaration {
        self.family
    }

    pub const fn bound_authority(&self) -> &StoreCurrentAuthorityWitness {
        &self.bound_authority
    }

    pub const fn bound_security(&self) -> forge_store_security::StoreSecurityScopeIdentity {
        self.bound_security
    }

    pub const fn current_authority(&self) -> forge_store_authority::StoreCurrentAuthorityIdentity {
        self.current_authority
    }

    pub const fn current_security(&self) -> forge_store_security::StoreSecurityScopeIdentity {
        self.current_security
    }
}
