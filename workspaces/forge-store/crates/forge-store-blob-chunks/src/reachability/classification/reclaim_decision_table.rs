use crate::reachability::classification::reachability_snapshot::ReachabilityRegistryView;
use crate::reachability::classification::reclaim_case::ReachabilityReclaimCase;
use crate::reachability::types::BlobChunkReachabilityRegistry;
use crate::reachability::verification::release_evidence::collect_released_edges_for;
use crate::BlobChunkIdentity;

pub(crate) fn classify_reclaim_eligibility(
    registry: &BlobChunkReachabilityRegistry,
    identity: &BlobChunkIdentity,
) -> ReachabilityReclaimCase {
    let view = ReachabilityRegistryView::from_registry(registry);
    if view.has_live_edge_for(identity) {
        return ReachabilityReclaimCase::Reachable;
    }
    if view.has_any_hold() {
        return ReachabilityReclaimCase::Held;
    }
    let released_edges = collect_released_edges_for(registry, identity);
    if released_edges.is_empty() {
        return ReachabilityReclaimCase::DeniedMissingRelease;
    }
    ReachabilityReclaimCase::Reclaimable { released_edges }
}
