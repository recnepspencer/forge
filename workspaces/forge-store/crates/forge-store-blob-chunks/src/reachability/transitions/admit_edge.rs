use crate::reachability::types::BlobChunkReachabilityRegistry;
use crate::reachability::verification::authority_match::verify_edge_authority;
use crate::{BlobReachabilityDenial, BlobReachabilityEdge};

pub(crate) fn transition_admit_edge(
    registry: &mut BlobChunkReachabilityRegistry,
    edge: BlobReachabilityEdge,
) -> Result<(), BlobReachabilityDenial> {
    verify_edge_authority(registry, &edge)?;
    if registry.authority().is_none() {
        registry.set_authority(edge.authority_key());
    }
    if registry
        .edges()
        .iter()
        .any(|existing| existing.identity() == edge.identity())
    {
        return Ok(());
    }
    registry.set_stored_counters(registry.stored_counters().with_edge(edge.is_dedupe()));
    registry.edges_mut().push(edge);
    registry.sort_edges();
    Ok(())
}
