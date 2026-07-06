use crate::BlobReachabilityCounterSnapshot;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobReachabilityCanonicalSnapshot {
    reachable_chunks: Vec<String>,
    reference_edges: Vec<String>,
    protected_holds: Vec<String>,
    counters: BlobReachabilityCounterSnapshot,
}

impl BlobReachabilityCanonicalSnapshot {
    pub(crate) fn from_parts(
        reachable_chunks: Vec<String>,
        reference_edges: Vec<String>,
        protected_holds: Vec<String>,
        counters: BlobReachabilityCounterSnapshot,
    ) -> Self {
        Self {
            reachable_chunks,
            reference_edges,
            protected_holds,
            counters,
        }
    }

    pub fn reachable_chunks(&self) -> &[String] {
        &self.reachable_chunks
    }

    pub fn reference_edges(&self) -> &[String] {
        &self.reference_edges
    }

    pub fn protected_holds(&self) -> &[String] {
        &self.protected_holds
    }

    pub const fn counters(&self) -> BlobReachabilityCounterSnapshot {
        self.counters
    }
}
