use crate::reachability::receipt_construction::BlobReachabilityEdgeRelease;
use crate::BlobReachabilityEdge;

pub(crate) fn construct_edge_release(edge: &BlobReachabilityEdge) -> BlobReachabilityEdgeRelease {
    BlobReachabilityEdgeRelease::from_edge(edge)
}