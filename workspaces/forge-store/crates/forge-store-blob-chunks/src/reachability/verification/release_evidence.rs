use crate::reachability::receipt_construction::BlobReachabilityEdgeRelease;
use crate::reachability::types::BlobChunkReachabilityRegistry;
use crate::{BlobChunkIdentity, BlobReachabilityEdge};

pub(crate) fn collect_released_edges_for(
    registry: &BlobChunkReachabilityRegistry,
    identity: &BlobChunkIdentity,
) -> Vec<BlobReachabilityEdgeRelease> {
    registry
        .released_edges()
        .iter()
        .filter(|release| release.chunk_identity() == identity)
        .cloned()
        .collect()
}

pub(crate) fn verify_edge_present(
    registry: &BlobChunkReachabilityRegistry,
    edge: &BlobReachabilityEdge,
) -> Option<usize> {
    registry
        .edges()
        .iter()
        .position(|candidate| candidate.identity() == edge.identity())
}