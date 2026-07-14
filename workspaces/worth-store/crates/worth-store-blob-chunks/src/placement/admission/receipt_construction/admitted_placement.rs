use crate::BlobChunkReachabilityProofSet;

use crate::placement::admission::{
    basis::BlobPlacementReachabilityBasis, types::AdmittedBlobPlacement,
    BlobPlacementCounterSnapshot, BlobPlacementIntent, BlobPlacementNonClaim,
};

pub(crate) fn construct_admitted_placement(
    basis: BlobPlacementReachabilityBasis,
    reachability: &BlobChunkReachabilityProofSet,
    intent: BlobPlacementIntent,
    counters: BlobPlacementCounterSnapshot,
) -> AdmittedBlobPlacement {
    AdmittedBlobPlacement {
        basis,
        stored_digest: reachability.stored_digest().clone(),
        security_metadata: reachability.security_metadata(),
        class: intent.class(),
        cold_state: intent.cold_state(),
        counters,
        non_claims: BlobPlacementNonClaim::required(),
    }
}
