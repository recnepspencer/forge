use worth_store_aspect_native::StoreAspectBoundaryFact;
use worth_store_contracts::{DurableArtifactClass, StableArtifactId};

use crate::{require_current_store_authority, StoreCurrentAuthorityWitness};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AspectNativeAuthorityRecord {
    artifact_id: StableArtifactId,
    artifact_class: DurableArtifactClass,
    current_authority: StoreCurrentAuthorityWitness,
}

impl AspectNativeAuthorityRecord {
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

    pub const fn boundary_fact(&self) -> &worth_store_aspect_native::StoreAspectBoundaryFact {
        self.current_authority.boundary_fact()
    }
}

pub fn admit_aspect_native_authority_record(
    artifact_id: StableArtifactId,
    boundary_fact: StoreAspectBoundaryFact,
) -> AspectNativeAuthorityRecord {
    AspectNativeAuthorityRecord::from_current_authority(
        artifact_id,
        require_current_store_authority(boundary_fact),
    )
}
