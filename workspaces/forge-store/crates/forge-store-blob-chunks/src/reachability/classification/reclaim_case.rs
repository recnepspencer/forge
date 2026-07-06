use crate::reachability::receipt_construction::BlobReachabilityEdgeRelease;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ReachabilityReclaimCase {
    Reachable,
    Held,
    DeniedMissingRelease,
    Reclaimable {
        released_edges: Vec<BlobReachabilityEdgeRelease>,
    },
}