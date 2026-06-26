#![forbid(unsafe_code)]

mod aspect_native_authority;

pub use aspect_native_authority::AspectNativeAuthorityRecord;

use forge_store_aspect_native::StoreAspectBoundaryFact;
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

pub fn admit_aspect_native_authority_record(
    artifact_id: StableArtifactId,
    boundary_fact: StoreAspectBoundaryFact,
) -> AspectNativeAuthorityRecord {
    AspectNativeAuthorityRecord::new(artifact_id, boundary_fact)
}
