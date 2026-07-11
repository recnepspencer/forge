use forge_store_contracts::{DurableArtifactFamilyId, DurableArtifactRebuildPosture};
use forge_store_layout_indexes::access_planning::S8AccessShape;
use forge_store_security::StoreSecurityScopeIdentity;

use super::behavior::{
    corruption_behavior_for, declared_rebuild_posture, BlobLayoutCorruptionBehavior,
    BlobLayoutScopeSafeAbsenceBehavior,
};
use super::{BlobLayoutAccessDenial, BlobLayoutAccessPathEvidence};
use crate::{BlobChunkDedupeCollisionPosture, BlobChunkDedupeShareClaim, BlobChunkIdentity};
use forge_store_contracts::StableDigest;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DedupeLayoutReport {
    family_id: DurableArtifactFamilyId,
    access_shape: S8AccessShape,
    rebuild_posture: DurableArtifactRebuildPosture,
    absence_behavior: BlobLayoutScopeSafeAbsenceBehavior,
    corruption_behavior: BlobLayoutCorruptionBehavior,
    content_digest: StableDigest,
    shared_identity: BlobChunkIdentity,
    candidate_identity: BlobChunkIdentity,
    security_scope: StoreSecurityScopeIdentity,
    collision_posture: BlobChunkDedupeCollisionPosture,
    counter_evidence: BlobLayoutAccessPathEvidence,
}

impl DedupeLayoutReport {
    fn admit_dedupe(
        claim: &BlobChunkDedupeShareClaim,
    ) -> Result<DedupeLayoutReport, BlobLayoutAccessDenial> {
        Ok(DedupeLayoutReport::from_claim(claim))
    }
}

impl DedupeLayoutReport {
    fn from_claim(claim: &BlobChunkDedupeShareClaim) -> Self {
        let family_id = DurableArtifactFamilyId::DedupeIndex;
        let rebuild_posture = declared_rebuild_posture(family_id);
        Self {
            family_id,
            access_shape: S8AccessShape::PointLookup,
            rebuild_posture,
            absence_behavior: BlobLayoutScopeSafeAbsenceBehavior::ExactIndex,
            corruption_behavior: corruption_behavior_for(rebuild_posture),
            content_digest: claim.content_digest().clone(),
            shared_identity: claim.existing_identity().clone(),
            candidate_identity: claim.candidate_identity().clone(),
            security_scope: claim.security_scope(),
            collision_posture: claim.collision_posture(),
            counter_evidence: BlobLayoutAccessPathEvidence::from_dedupe(
                family_id,
                claim.counters(),
            ),
        }
    }

    pub const fn family_id(&self) -> DurableArtifactFamilyId {
        self.family_id
    }

    pub const fn access_shape(&self) -> S8AccessShape {
        self.access_shape
    }

    pub const fn rebuild_posture(&self) -> DurableArtifactRebuildPosture {
        self.rebuild_posture
    }

    pub const fn absence_behavior(&self) -> BlobLayoutScopeSafeAbsenceBehavior {
        self.absence_behavior
    }

    pub const fn corruption_behavior(&self) -> BlobLayoutCorruptionBehavior {
        self.corruption_behavior
    }

    pub const fn content_digest(&self) -> &StableDigest {
        &self.content_digest
    }

    pub const fn shared_identity(&self) -> &BlobChunkIdentity {
        &self.shared_identity
    }

    pub const fn candidate_identity(&self) -> &BlobChunkIdentity {
        &self.candidate_identity
    }

    pub const fn security_scope(&self) -> StoreSecurityScopeIdentity {
        self.security_scope
    }

    pub const fn collision_posture(&self) -> BlobChunkDedupeCollisionPosture {
        self.collision_posture
    }

    pub const fn counter_evidence(&self) -> BlobLayoutAccessPathEvidence {
        self.counter_evidence
    }

    pub const fn requires_rebuild_parity(&self) -> bool {
        !matches!(
            self.rebuild_posture,
            DurableArtifactRebuildPosture::NoRebuild
        )
    }
}

impl BlobChunkDedupeShareClaim {
    pub fn admit_dedupe_layout(&self) -> Result<DedupeLayoutReport, BlobLayoutAccessDenial> {
        DedupeLayoutReport::admit_dedupe(self)
    }
}
