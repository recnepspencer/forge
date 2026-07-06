use forge_store_contracts::StableDigest;

use crate::reachability::counters::BlobReachabilityCounterSnapshot;
use crate::{BlobChunkIdentity, BlobChunkSecurityMetadataWitness, BlobReachabilityEdge,
    BlobReachabilityEdgeKind,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobReachabilityEdgeRelease {
    chunk_identity: BlobChunkIdentity,
    edge_identity: StableDigest,
    kind: BlobReachabilityEdgeKind,
    security_metadata: BlobChunkSecurityMetadataWitness,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobReachabilityReclaimRelease {
    chunk_identity: BlobChunkIdentity,
    released_edges: Vec<BlobReachabilityEdgeRelease>,
    counters: BlobReachabilityCounterSnapshot,
}

impl BlobReachabilityEdgeRelease {
    pub(crate) fn from_edge(edge: &BlobReachabilityEdge) -> Self {
        Self {
            chunk_identity: edge.chunk_identity().clone(),
            edge_identity: edge.identity().clone(),
            kind: edge.kind(),
            security_metadata: edge.security_metadata(),
        }
    }

    pub const fn chunk_identity(&self) -> &BlobChunkIdentity {
        &self.chunk_identity
    }

    pub const fn edge_identity(&self) -> &StableDigest {
        &self.edge_identity
    }

    pub const fn kind(&self) -> BlobReachabilityEdgeKind {
        self.kind
    }

    pub const fn security_metadata(&self) -> BlobChunkSecurityMetadataWitness {
        self.security_metadata
    }
}

impl BlobReachabilityReclaimRelease {
    pub(crate) fn from_released_edges(
        chunk_identity: BlobChunkIdentity,
        released_edges: Vec<BlobReachabilityEdgeRelease>,
        counters: BlobReachabilityCounterSnapshot,
    ) -> Self {
        Self {
            chunk_identity,
            released_edges,
            counters,
        }
    }

    pub const fn chunk_identity(&self) -> &BlobChunkIdentity {
        &self.chunk_identity
    }

    pub fn released_edges(&self) -> &[BlobReachabilityEdgeRelease] {
        &self.released_edges
    }

    pub const fn counters(&self) -> BlobReachabilityCounterSnapshot {
        self.counters
    }
}