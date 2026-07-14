use worth_store_security::StoreSecurityScopeIdentity;

use crate::{
    AdmittedBlobPlacement, BlobChunkSecurityMetadataWitness, BlobPlacementClass,
    BlobPlacementCounterSnapshot, BlobPlacementNonClaim, StoredChunkDigest,
};

#[derive(Debug, PartialEq, Eq)]
pub struct BlobPlacementProof {
    stored_digest: StoredChunkDigest,
    security_metadata: BlobChunkSecurityMetadataWitness,
    class: BlobPlacementClass,
    counters: BlobPlacementCounterSnapshot,
    non_claims: [BlobPlacementNonClaim; 3],
}

impl BlobPlacementProof {
    pub(crate) fn from_admitted_placement(placement: &AdmittedBlobPlacement) -> Self {
        Self {
            stored_digest: placement.stored_digest().clone(),
            security_metadata: placement.security_metadata(),
            class: placement.class(),
            counters: placement.counters(),
            non_claims: *placement.non_claims(),
        }
    }

    pub const fn stored_digest(&self) -> &StoredChunkDigest {
        &self.stored_digest
    }

    pub const fn security_scope(&self) -> StoreSecurityScopeIdentity {
        self.security_metadata.identity()
    }

    pub const fn security_metadata(&self) -> BlobChunkSecurityMetadataWitness {
        self.security_metadata
    }

    pub const fn placement_class(&self) -> BlobPlacementClass {
        self.class
    }

    pub const fn counters(&self) -> BlobPlacementCounterSnapshot {
        self.counters
    }

    pub const fn non_claims(&self) -> &[BlobPlacementNonClaim; 3] {
        &self.non_claims
    }
}
