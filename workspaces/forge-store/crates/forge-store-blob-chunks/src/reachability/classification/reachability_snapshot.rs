use crate::reachability::types::BlobChunkReachabilityRegistry;
use crate::{BlobChunkIdentity, BlobReachabilityEdge, BlobReachabilityProtectedHold};

pub(crate) struct ReachabilityRegistryView<'a> {
    edges: &'a [BlobReachabilityEdge],
    holds: &'a [BlobReachabilityProtectedHold],
}

impl<'a> ReachabilityRegistryView<'a> {
    pub(crate) fn from_registry(registry: &'a BlobChunkReachabilityRegistry) -> Self {
        Self {
            edges: registry.edges(),
            holds: registry.holds(),
        }
    }

    pub(crate) const fn edges(&self) -> &[BlobReachabilityEdge] {
        self.edges
    }

    pub(crate) const fn holds(&self) -> &[BlobReachabilityProtectedHold] {
        self.holds
    }

    pub(crate) fn has_live_edge_for(&self, identity: &BlobChunkIdentity) -> bool {
        self.edges
            .iter()
            .any(|edge| edge.chunk_identity() == identity)
    }

    pub(crate) fn has_any_hold(&self) -> bool {
        !self.holds.is_empty()
    }
}