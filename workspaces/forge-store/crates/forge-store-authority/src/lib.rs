#![forbid(unsafe_code)]

use forge_store_contracts::{DurableArtifactClass, StableArtifactId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CanonicalAuthorityRecord {
    artifact_id: StableArtifactId,
    artifact_class: DurableArtifactClass,
}

impl CanonicalAuthorityRecord {
    pub fn new(artifact_id: StableArtifactId) -> Self {
        Self {
            artifact_id,
            artifact_class: DurableArtifactClass::Authoritative,
        }
    }

    pub fn artifact_id(&self) -> &StableArtifactId {
        &self.artifact_id
    }

    pub const fn artifact_class(&self) -> DurableArtifactClass {
        self.artifact_class
    }
}
