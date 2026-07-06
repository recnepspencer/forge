use forge_store_contracts::StableDigest;

use crate::{
    BlobChunkCanonicalEquivalence, BlobChunkDedupeCollisionPosture, BlobChunkDedupeCounterSnapshot,
    BlobChunkDedupePolicy, BlobChunkDedupeReferenceRegistry, BlobChunkIdentity,
    BlobChunkRegisteredDedupeReference, BlobChunkSecurityMetadataWitness,
};

#[derive(Debug, PartialEq, Eq)]
pub struct BlobChunkDedupeReceipt {
    content_digest: StableDigest,
    security_metadata: BlobChunkSecurityMetadataWitness,
    policy: BlobChunkDedupePolicy,
    equivalence: BlobChunkCanonicalEquivalence,
    collision_posture: BlobChunkDedupeCollisionPosture,
    counters: BlobChunkDedupeCounterSnapshot,
}

pub type BlobChunkDedupeShareClaim = BlobChunkDedupeReceipt;

impl BlobChunkDedupeReceipt {
    pub(crate) fn from_admitted_equivalence(
        content_digest: StableDigest,
        security_metadata: BlobChunkSecurityMetadataWitness,
        policy: BlobChunkDedupePolicy,
        equivalence: BlobChunkCanonicalEquivalence,
        collision_posture: BlobChunkDedupeCollisionPosture,
        counters: BlobChunkDedupeCounterSnapshot,
    ) -> Self {
        Self {
            content_digest,
            security_metadata,
            policy,
            equivalence,
            collision_posture,
            counters,
        }
    }

    pub const fn content_digest(&self) -> &StableDigest {
        &self.content_digest
    }

    pub const fn security_metadata(&self) -> BlobChunkSecurityMetadataWitness {
        self.security_metadata
    }

    pub const fn security_scope(&self) -> forge_store_security::StoreSecurityScopeIdentity {
        self.security_metadata.identity()
    }

    pub const fn policy(&self) -> BlobChunkDedupePolicy {
        self.policy
    }

    pub fn equivalence(&self) -> BlobChunkCanonicalEquivalence {
        self.equivalence.clone()
    }

    pub const fn existing_identity(&self) -> &BlobChunkIdentity {
        self.equivalence.existing_identity()
    }

    pub const fn candidate_identity(&self) -> &BlobChunkIdentity {
        self.equivalence.candidate_identity()
    }

    pub const fn collision_posture(&self) -> BlobChunkDedupeCollisionPosture {
        self.collision_posture
    }

    pub const fn counters(&self) -> BlobChunkDedupeCounterSnapshot {
        self.counters
    }

    pub fn admit_into_reference_registry(
        self,
        registry: &mut BlobChunkDedupeReferenceRegistry,
    ) -> Result<BlobChunkRegisteredDedupeReference, crate::BlobChunkDedupeAdmissionDenial> {
        registry.admit_receipt(self)
    }
}
