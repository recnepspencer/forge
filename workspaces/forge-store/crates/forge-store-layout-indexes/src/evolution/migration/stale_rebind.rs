use forge_store_authority::StoreCurrentAuthorityWitness;

use crate::PhysicalArtifactFamilyDeclaration;

use super::LayoutVersion;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LayoutStaleBinding {
    family: &'static PhysicalArtifactFamilyDeclaration,
    bound_version: LayoutVersion,
    observed_version: LayoutVersion,
}

impl LayoutStaleBinding {
    pub(crate) const fn new(
        family: &'static PhysicalArtifactFamilyDeclaration,
        bound_version: LayoutVersion,
        observed_version: LayoutVersion,
    ) -> Self {
        Self {
            family,
            bound_version,
            observed_version,
        }
    }

    pub const fn family(self) -> &'static PhysicalArtifactFamilyDeclaration {
        self.family
    }

    pub const fn bound_version(self) -> LayoutVersion {
        self.bound_version
    }

    pub const fn observed_version(self) -> LayoutVersion {
        self.observed_version
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LayoutRebindRequired {
    family: &'static PhysicalArtifactFamilyDeclaration,
    bound_authority: StoreCurrentAuthorityWitness,
}

impl LayoutRebindRequired {
    pub(crate) fn new(
        family: &'static PhysicalArtifactFamilyDeclaration,
        bound_authority: &StoreCurrentAuthorityWitness,
    ) -> Self {
        Self {
            family,
            bound_authority: bound_authority.clone(),
        }
    }

    pub const fn family(&self) -> &'static PhysicalArtifactFamilyDeclaration {
        self.family
    }

    pub const fn bound_authority(&self) -> &StoreCurrentAuthorityWitness {
        &self.bound_authority
    }
}
