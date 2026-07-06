use crate::reachability::receipt_construction::edge_release::construct_edge_release;
use crate::reachability::receipt_construction::BlobReachabilityEdgeRelease;
use crate::reachability::types::BlobChunkReachabilityRegistry;
use crate::reachability::verification::release_evidence::verify_edge_present;
use crate::{BlobReachabilityDenial, BlobReachabilityEdge};

pub(crate) fn transition_release_edge(
    registry: &mut BlobChunkReachabilityRegistry,
    edge: &BlobReachabilityEdge,
) -> Result<BlobReachabilityEdgeRelease, BlobReachabilityDenial> {
    let Some(position) = verify_edge_present(registry, edge) else {
        return Err(BlobReachabilityDenial::MissingReclaimReleaseEvidence {
            counters: registry
                .stored_counters()
                .with_classified_reclaim_outcome(
                    &crate::reachability::classification::ReachabilityReclaimCase::DeniedMissingRelease,
                ),
        });
    };
    let removed = registry.edges_mut().remove(position);
    let release = construct_edge_release(&removed);
    registry.released_edges_mut().push(release.clone());
    registry.sort_released_edges();
    Ok(release)
}