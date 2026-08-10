use serde::{Deserialize, Serialize};

use sha2::{Digest, Sha256};

use super::versions::ArtifactCompatibilityWindow;
pub use worth_store_contracts::ArtifactFamilyId;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct CompatibilityManifestDigest(String);

impl CompatibilityManifestDigest {
    pub(crate) fn compute(
        family_id: &ArtifactFamilyId,
        window: &ArtifactCompatibilityWindow,
        authority_label: &str,
    ) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(family_id.as_str().as_bytes());
        hasher.update(authority_label.as_bytes());
        hasher.update(window.minimum_format().value().to_le_bytes());
        hasher.update(window.maximum_format().value().to_le_bytes());
        hasher.update(window.minimum_semantic().value().to_le_bytes());
        hasher.update(window.maximum_semantic().value().to_le_bytes());
        Self(format!("{:x}", hasher.finalize()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthoritativeCompatibilityManifest {
    family_id: ArtifactFamilyId,
    window: ArtifactCompatibilityWindow,
    digest: CompatibilityManifestDigest,
}

impl AuthoritativeCompatibilityManifest {
    pub(crate) fn new(family_id: ArtifactFamilyId, window: ArtifactCompatibilityWindow) -> Self {
        let digest = CompatibilityManifestDigest::compute(&family_id, &window, "authoritative");
        Self {
            family_id,
            window,
            digest,
        }
    }

    pub fn family_id(&self) -> &ArtifactFamilyId {
        &self.family_id
    }

    pub fn window(&self) -> &ArtifactCompatibilityWindow {
        &self.window
    }

    pub fn digest(&self) -> &CompatibilityManifestDigest {
        &self.digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DerivedCompatibilityManifest {
    family_id: ArtifactFamilyId,
    window: ArtifactCompatibilityWindow,
    digest: CompatibilityManifestDigest,
}

impl DerivedCompatibilityManifest {
    pub(crate) fn new(family_id: ArtifactFamilyId, window: ArtifactCompatibilityWindow) -> Self {
        let digest = CompatibilityManifestDigest::compute(&family_id, &window, "derived");
        Self {
            family_id,
            window,
            digest,
        }
    }

    pub fn family_id(&self) -> &ArtifactFamilyId {
        &self.family_id
    }

    pub fn window(&self) -> &ArtifactCompatibilityWindow {
        &self.window
    }

    pub fn digest(&self) -> &CompatibilityManifestDigest {
        &self.digest
    }
}
