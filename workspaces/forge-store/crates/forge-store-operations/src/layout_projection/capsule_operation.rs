use forge_store_blob_chunks::BlobCapsuleReadinessWitness;
use forge_store_contracts::DurableArtifactFamilyId;
use forge_store_layout_indexes::observation::AccessShape;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapsuleOperationLayoutReport {
    family_id: DurableArtifactFamilyId,
    access_shape: AccessShape,
    declared_bytes: u64,
}

impl CapsuleOperationLayoutReport {
    pub fn from_blob_capsule_readiness(witness: &BlobCapsuleReadinessWitness) -> Self {
        Self::from_capsule_source(witness.declared_bytes())
    }

    fn from_capsule_source(declared_bytes: u64) -> Self {
        Self {
            family_id: DurableArtifactFamilyId::CapsuleArtifact,
            access_shape: AccessShape::ManifestGraphWalk,
            declared_bytes,
        }
    }

    pub const fn family_id(&self) -> DurableArtifactFamilyId {
        self.family_id
    }

    pub const fn declared_access_shape(&self) -> AccessShape {
        self.access_shape
    }

    pub const fn declared_bytes(&self) -> u64 {
        self.declared_bytes
    }

    pub const fn cannot_be_foreground_authority(&self) -> bool {
        true
    }
}
