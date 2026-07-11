use forge_store_blob_chunks::BlobCapsuleReadinessWitness;
use forge_store_contracts::DurableArtifactFamilyId;
use forge_store_layout_indexes::access_planning::S8AccessShape;
use forge_store_layout_indexes::layout_strategy_admission::{
    phase28_capsule_manifest_rule, AdmittedCapsuleManifestLayoutRule, Phase28LayoutAuthorityPosture,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct CapsuleLayoutAdmission {
    rule: AdmittedCapsuleManifestLayoutRule,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapsuleOperationLayoutReport {
    admission: CapsuleLayoutAdmission,
    family_id: DurableArtifactFamilyId,
    access_shape: S8AccessShape,
    posture: Phase28LayoutAuthorityPosture,
    declared_bytes: u64,
}

impl CapsuleOperationLayoutReport {
    pub fn from_blob_capsule_readiness(witness: &BlobCapsuleReadinessWitness) -> Self {
        Self::from_capsule_source(witness.declared_bytes())
    }

    fn from_capsule_source(declared_bytes: u64) -> Self {
        Self {
            admission: CapsuleLayoutAdmission {
                rule: phase28_capsule_manifest_rule()
                    .expect("phase-28 capsule manifest rule must stay admitted"),
            },
            family_id: DurableArtifactFamilyId::CapsuleArtifact,
            access_shape: S8AccessShape::ManifestGraphWalk,
            posture: Phase28LayoutAuthorityPosture::TerminalOnly,
            declared_bytes,
        }
    }

    pub const fn family_id(&self) -> DurableArtifactFamilyId {
        self.family_id
    }

    pub const fn declared_access_shape(&self) -> S8AccessShape {
        self.access_shape
    }

    pub const fn authority_posture(&self) -> Phase28LayoutAuthorityPosture {
        self.posture
    }

    pub const fn declared_bytes(&self) -> u64 {
        self.declared_bytes
    }

    pub const fn cannot_be_foreground_authority(&self) -> bool {
        true
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CapsuleOperationLayoutFamilyHome;
