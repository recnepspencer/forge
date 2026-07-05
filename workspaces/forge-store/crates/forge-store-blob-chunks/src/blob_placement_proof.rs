use forge_store_readiness::{S6LaterMilestoneDestination, S7PlacementReadinessNonClaim};
use forge_store_security::StoreSecurityScopeIdentity;

use crate::{
    blob_lifecycle_authority::BlobLifecyclePlacementReadiness, BlobChunkSecurityMetadataWitness,
    BlobReachabilityProof, StoredChunkDigest,
};

#[derive(Debug, PartialEq, Eq)]
pub struct BlobPlacementProof {
    stored_digest: StoredChunkDigest,
    security_metadata: BlobChunkSecurityMetadataWitness,
    destination: S6LaterMilestoneDestination,
    s6_non_claims: [S7PlacementReadinessNonClaim; 3],
}

impl BlobPlacementProof {
    pub(crate) fn from_reachability_and_placement_readiness(
        reachability: &BlobReachabilityProof,
        readiness: BlobLifecyclePlacementReadiness,
    ) -> Self {
        Self {
            stored_digest: reachability.stored_digest().clone(),
            security_metadata: reachability.security_metadata(),
            destination: readiness.destination(),
            s6_non_claims: *readiness.s6_non_claims(),
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

    pub const fn destination(&self) -> S6LaterMilestoneDestination {
        self.destination
    }

    pub const fn s6_non_claims(&self) -> &[S7PlacementReadinessNonClaim; 3] {
        &self.s6_non_claims
    }
}
