use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ReaderCapabilitySet {
    family_id: ArtifactFamilyId,
    semantic_versions: Vec<ArtifactSemanticVersion>,
}
impl ReaderCapabilitySet {
    pub fn new(
        family_id: ArtifactFamilyId,
        mut semantic_versions: Vec<ArtifactSemanticVersion>,
    ) -> Self {
        semantic_versions.sort();
        semantic_versions.dedup();
        Self {
            family_id,
            semantic_versions,
        }
    }

    pub fn family_id(&self) -> &ArtifactFamilyId {
        &self.family_id
    }

    pub fn semantic_versions(&self) -> &[ArtifactSemanticVersion] {
        &self.semantic_versions
    }

    pub fn admits_semantic_version(&self, version: ArtifactSemanticVersion) -> bool {
        self.semantic_versions.binary_search(&version).is_ok()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WriterCapabilitySet {
    family_id: ArtifactFamilyId,
    semantic_versions: Vec<ArtifactSemanticVersion>,
}

impl WriterCapabilitySet {
    pub fn new(
        family_id: ArtifactFamilyId,
        mut semantic_versions: Vec<ArtifactSemanticVersion>,
    ) -> Self {
        semantic_versions.sort();
        semantic_versions.dedup();
        Self {
            family_id,
            semantic_versions,
        }
    }

    pub fn family_id(&self) -> &ArtifactFamilyId {
        &self.family_id
    }

    pub fn semantic_versions(&self) -> &[ArtifactSemanticVersion] {
        &self.semantic_versions
    }

    pub fn admits_semantic_version(&self, version: ArtifactSemanticVersion) -> bool {
        self.semantic_versions.binary_search(&version).is_ok()
    }
}
