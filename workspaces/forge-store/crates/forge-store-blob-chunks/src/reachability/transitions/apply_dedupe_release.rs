use crate::reachability::receipt_construction::edge_release::construct_edge_release;
use crate::reachability::types::BlobChunkReachabilityRegistry;
use crate::{BlobChunkDedupeReferenceRelease, BlobReachabilityEdgeKind};

pub(crate) fn transition_apply_dedupe_reference_release(
    registry: &mut BlobChunkReachabilityRegistry,
    release: &BlobChunkDedupeReferenceRelease,
) {
    let mut removed = Vec::new();
    registry.edges_mut().retain(|edge| {
        let remove = edge.kind() == BlobReachabilityEdgeKind::DedupeSharedReference
            && edge.security_metadata() == release.security_metadata()
            && edge
                .dedupe_reference_identity()
                .is_some_and(|identity| release.contains_reference_identity(identity));
        if remove {
            removed.push(construct_edge_release(edge));
        }
        !remove
    });
    registry.released_edges_mut().extend(removed);
    registry.sort_released_edges();
}