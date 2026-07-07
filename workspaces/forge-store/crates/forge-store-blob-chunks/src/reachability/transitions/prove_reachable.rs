use crate::reachability::receipt_construction::proof_set::construct_proof_set;
use crate::reachability::types::{BlobChunkReachabilityProofSet, BlobChunkReachabilityRegistry};
use crate::reachability::verification::empty_proof::verify_nonempty_edge_proof;
use crate::BlobReachabilityDenial;

pub(crate) fn transition_prove_reachable_chunks(
    registry: &BlobChunkReachabilityRegistry,
) -> Result<BlobChunkReachabilityProofSet, BlobReachabilityDenial> {
    verify_nonempty_edge_proof(registry)?;
    Ok(construct_proof_set(registry))
}
