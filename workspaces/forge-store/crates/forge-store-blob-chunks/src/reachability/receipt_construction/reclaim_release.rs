use crate::reachability::counters::BlobReachabilityCounterSnapshot;
use crate::reachability::receipt_construction::BlobReachabilityReclaimRelease;
use crate::{BlobChunkIdentity, BlobReachabilityEdgeRelease};

pub(crate) fn construct_reclaim_release(
    chunk_identity: BlobChunkIdentity,
    released_edges: Vec<BlobReachabilityEdgeRelease>,
    counters: BlobReachabilityCounterSnapshot,
) -> BlobReachabilityReclaimRelease {
    BlobReachabilityReclaimRelease::from_released_edges(chunk_identity, released_edges, counters)
}
