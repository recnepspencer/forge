use crate::runtime::{WorthUiIdentityMatchCounters, WorthUiIdentityMatchGraph};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiIdentityMatchReport {
    active_artifact_digest: u64,
    candidate_artifact_digest: u64,
    graph: WorthUiIdentityMatchGraph,
}

impl WorthUiIdentityMatchReport {
    pub(crate) fn new(
        active_artifact_digest: u64,
        candidate_artifact_digest: u64,
        graph: WorthUiIdentityMatchGraph,
    ) -> Self {
        Self {
            active_artifact_digest,
            candidate_artifact_digest,
            graph,
        }
    }

    pub fn active_artifact_digest(&self) -> u64 {
        self.active_artifact_digest
    }

    pub fn candidate_artifact_digest(&self) -> u64 {
        self.candidate_artifact_digest
    }

    pub fn graph(&self) -> &WorthUiIdentityMatchGraph {
        &self.graph
    }

    pub fn counters(&self) -> WorthUiIdentityMatchCounters {
        self.graph.counters()
    }
}
