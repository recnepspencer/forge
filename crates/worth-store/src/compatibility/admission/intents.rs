use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CompatibilityReadIntent {
    family_id: ArtifactFamilyId,
    target_semantic_version: ArtifactSemanticVersion,
}
impl CompatibilityReadIntent {
    pub fn new(
        family_id: ArtifactFamilyId,
        target_semantic_version: ArtifactSemanticVersion,
    ) -> Self {
        Self {
            family_id,
            target_semantic_version,
        }
    }

    pub fn family_id(&self) -> &ArtifactFamilyId {
        &self.family_id
    }

    pub fn target_semantic_version(&self) -> ArtifactSemanticVersion {
        self.target_semantic_version
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CompatibilityWriteIntent {
    family_id: ArtifactFamilyId,
    target_semantic_version: ArtifactSemanticVersion,
}

impl CompatibilityWriteIntent {
    pub fn new(
        family_id: ArtifactFamilyId,
        target_semantic_version: ArtifactSemanticVersion,
    ) -> Self {
        Self {
            family_id,
            target_semantic_version,
        }
    }

    pub fn family_id(&self) -> &ArtifactFamilyId {
        &self.family_id
    }

    pub fn target_semantic_version(&self) -> ArtifactSemanticVersion {
        self.target_semantic_version
    }
}
