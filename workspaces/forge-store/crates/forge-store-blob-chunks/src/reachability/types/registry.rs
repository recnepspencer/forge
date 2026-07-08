use crate::reachability::counters::BlobReachabilityCounterSnapshot;
use crate::reachability::edges::BlobReachabilityAuthorityKey;
use crate::reachability::receipt_construction::BlobReachabilityEdgeRelease;
use crate::{BlobReachabilityEdge, BlobReachabilityProtectedHold};

#[derive(Debug, Default, PartialEq, Eq)]
pub struct BlobChunkReachabilityRegistry {
    authority: Option<BlobReachabilityAuthorityKey>,
    edges: Vec<BlobReachabilityEdge>,
    holds: Vec<BlobReachabilityProtectedHold>,
    released_edges: Vec<BlobReachabilityEdgeRelease>,
    counters: BlobReachabilityCounterSnapshot,
}

impl BlobChunkReachabilityRegistry {
    pub fn new_store_owned() -> Self {
        Self::default()
    }

    pub(crate) fn authority(&self) -> Option<BlobReachabilityAuthorityKey> {
        self.authority.clone()
    }

    pub(crate) fn set_authority(&mut self, authority: BlobReachabilityAuthorityKey) {
        self.authority = Some(authority);
    }

    pub(crate) fn edges(&self) -> &[BlobReachabilityEdge] {
        &self.edges
    }

    pub(crate) fn edges_mut(&mut self) -> &mut Vec<BlobReachabilityEdge> {
        &mut self.edges
    }

    pub(crate) fn holds(&self) -> &[BlobReachabilityProtectedHold] {
        &self.holds
    }

    pub(crate) fn holds_mut(&mut self) -> &mut Vec<BlobReachabilityProtectedHold> {
        &mut self.holds
    }

    pub(crate) fn released_edges(&self) -> &[BlobReachabilityEdgeRelease] {
        &self.released_edges
    }

    pub(crate) fn released_edges_mut(&mut self) -> &mut Vec<BlobReachabilityEdgeRelease> {
        &mut self.released_edges
    }

    pub(crate) const fn stored_counters(&self) -> BlobReachabilityCounterSnapshot {
        self.counters
    }

    pub(crate) fn set_stored_counters(&mut self, counters: BlobReachabilityCounterSnapshot) {
        self.counters = counters;
    }

    pub(crate) fn sort_edges(&mut self) {
        self.edges
            .sort_by(|left, right| left.identity().as_str().cmp(right.identity().as_str()));
    }

    pub(crate) fn sort_released_edges(&mut self) {
        self.released_edges.sort_by(|left, right| {
            left.edge_identity()
                .as_str()
                .cmp(right.edge_identity().as_str())
        });
    }
}
