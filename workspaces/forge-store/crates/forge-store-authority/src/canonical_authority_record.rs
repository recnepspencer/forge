use forge_store_contracts::{DurableArtifactClass, StableArtifactId};

use crate::StoreCurrentAuthorityWitness;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CanonicalAuthorityRecord {
    artifact_id: StableArtifactId,
    artifact_class: DurableArtifactClass,
    current_authority: StoreCurrentAuthorityWitness,
}

impl CanonicalAuthorityRecord {
    pub fn from_current_authority(
        artifact_id: StableArtifactId,
        current_authority: StoreCurrentAuthorityWitness,
    ) -> Self {
        Self {
            artifact_id,
            artifact_class: DurableArtifactClass::Authoritative,
            current_authority,
        }
    }

    pub fn artifact_id(&self) -> &StableArtifactId {
        &self.artifact_id
    }

    pub const fn artifact_class(&self) -> DurableArtifactClass {
        self.artifact_class
    }

    pub const fn current_authority(&self) -> &StoreCurrentAuthorityWitness {
        &self.current_authority
    }
}
