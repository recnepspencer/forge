use crate::UiInspectionSupportWorld;

use super::{UiEvidenceAuthorityArtifactIdentity, UiEvidenceAuthorityGeneration};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiEvidenceAuthorityBinding {
    artifact_identity: UiEvidenceAuthorityArtifactIdentity,
    authority_generation: UiEvidenceAuthorityGeneration,
    world: Option<UiInspectionSupportWorld>,
}

impl UiEvidenceAuthorityBinding {
    pub const fn new(
        artifact_identity: UiEvidenceAuthorityArtifactIdentity,
        authority_generation: UiEvidenceAuthorityGeneration,
        world: Option<UiInspectionSupportWorld>,
    ) -> Self {
        Self {
            artifact_identity,
            authority_generation,
            world,
        }
    }

    pub const fn artifact_identity(self) -> UiEvidenceAuthorityArtifactIdentity {
        self.artifact_identity
    }

    pub const fn authority_generation(self) -> UiEvidenceAuthorityGeneration {
        self.authority_generation
    }

    pub const fn world(self) -> Option<UiInspectionSupportWorld> {
        self.world
    }
}
