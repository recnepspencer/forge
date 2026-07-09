use worth_store_tiering::S7ColdPlacementState;

use crate::{BlobChunkReachabilityProofSet, BlobChunkSecurityMetadataWitness, StoredChunkDigest};

use crate::placement::admission::{
    basis::BlobPlacementReachabilityBasis, BlobPlacementClass, BlobPlacementCounterSnapshot,
    BlobPlacementNonClaim,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdmittedBlobPlacement {
    pub(crate) basis: BlobPlacementReachabilityBasis,
    pub(crate) stored_digest: StoredChunkDigest,
    pub(crate) security_metadata: BlobChunkSecurityMetadataWitness,
    pub(crate) class: BlobPlacementClass,
    pub(crate) cold_state: Option<S7ColdPlacementState>,
    pub(crate) counters: BlobPlacementCounterSnapshot,
    pub(crate) non_claims: [BlobPlacementNonClaim; 3],
}

impl AdmittedBlobPlacement {
    pub(crate) fn matches_reachability(
        &self,
        reachability: &BlobChunkReachabilityProofSet,
    ) -> bool {
        self.basis.matches_reachability(reachability)
    }

    pub const fn stored_digest(&self) -> &StoredChunkDigest {
        &self.stored_digest
    }

    pub const fn security_metadata(&self) -> BlobChunkSecurityMetadataWitness {
        self.security_metadata
    }

    pub const fn class(&self) -> BlobPlacementClass {
        self.class
    }

    pub const fn cold_state(&self) -> Option<S7ColdPlacementState> {
        self.cold_state
    }

    pub const fn counters(&self) -> BlobPlacementCounterSnapshot {
        self.counters
    }

    pub const fn non_claims(&self) -> &[BlobPlacementNonClaim; 3] {
        &self.non_claims
    }
}
