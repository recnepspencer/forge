use forge_store_aspect_native::StoreAspectBoundaryFact;
use forge_store_contracts::{DurableArtifactClass, StableArtifactId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AspectNativeAuthorityRecord {
    artifact_id: StableArtifactId,
    artifact_class: DurableArtifactClass,
    boundary_fact: StoreAspectBoundaryFact,
}

impl AspectNativeAuthorityRecord {
    pub fn new(artifact_id: StableArtifactId, boundary_fact: StoreAspectBoundaryFact) -> Self {
        Self {
            artifact_id,
            artifact_class: DurableArtifactClass::Authoritative,
            boundary_fact,
        }
    }

    pub fn artifact_id(&self) -> &StableArtifactId {
        &self.artifact_id
    }

    pub const fn artifact_class(&self) -> DurableArtifactClass {
        self.artifact_class
    }

    pub const fn boundary_fact(&self) -> &StoreAspectBoundaryFact {
        &self.boundary_fact
    }
}
