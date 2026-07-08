use crate::reachability::denial::BlobReachabilityDenial;
use crate::reachability::types::BlobChunkReachabilityRegistry;

pub(crate) fn verify_nonempty_edge_proof(
    registry: &BlobChunkReachabilityRegistry,
) -> Result<(), BlobReachabilityDenial> {
    if registry.edges().is_empty() {
        return Err(BlobReachabilityDenial::EmptyReferenceProofRejected {
            counters: registry.stored_counters().record_empty_proof_denial(),
        });
    }
    Ok(())
}
