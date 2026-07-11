use super::{ArtifactFamilyClassification, PhysicalArtifactFamilyDeclaration};
use forge_store_contracts::DurableArtifactFamilyId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArtifactFamilyStrategyLane {
    HotPath,
    MaintenancePath,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArtifactFamilyAuthorityWitness {
    classification: ArtifactFamilyClassification,
}

impl ArtifactFamilyAuthorityWitness {
    pub(crate) const fn new(classification: ArtifactFamilyClassification) -> Self {
        Self { classification }
    }

    pub const fn classification(self) -> ArtifactFamilyClassification {
        self.classification
    }

    pub const fn declaration(self) -> &'static PhysicalArtifactFamilyDeclaration {
        self.classification.declaration()
    }

    pub const fn family_id(self) -> DurableArtifactFamilyId {
        self.classification.family_id()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArtifactFamilyLifecycleAdmission {
    authority: ArtifactFamilyAuthorityWitness,
    admitted_lane: ArtifactFamilyStrategyLane,
}

impl ArtifactFamilyLifecycleAdmission {
    pub(crate) const fn new(
        authority: ArtifactFamilyAuthorityWitness,
        admitted_lane: ArtifactFamilyStrategyLane,
    ) -> Self {
        Self {
            authority,
            admitted_lane,
        }
    }

    pub const fn authority(self) -> ArtifactFamilyAuthorityWitness {
        self.authority
    }

    pub const fn declaration(self) -> &'static PhysicalArtifactFamilyDeclaration {
        self.authority.declaration()
    }

    pub const fn family_id(self) -> DurableArtifactFamilyId {
        self.authority.family_id()
    }

    pub const fn admitted_lane(self) -> ArtifactFamilyStrategyLane {
        self.admitted_lane
    }
}
