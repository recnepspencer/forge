use crate::capability::CapabilitySnapshot;
use crate::graph::{UiGraphGeneration, UiGraphSnapshot};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiGraphFactIndexBasis {
    graph_generation: UiGraphGeneration,
    graph_authority_digest: u64,
    capability_snapshot_digest: u64,
}

impl UiGraphFactIndexBasis {
    pub(crate) fn from_generation(
        snapshot: &UiGraphSnapshot,
        capabilities: &CapabilitySnapshot,
    ) -> Self {
        Self {
            graph_generation: snapshot.generation(),
            graph_authority_digest: snapshot.authority_digest(),
            capability_snapshot_digest: capabilities.digest().as_u64(),
        }
    }

    pub const fn graph_generation(self) -> UiGraphGeneration {
        self.graph_generation
    }

    pub const fn graph_authority_digest(self) -> u64 {
        self.graph_authority_digest
    }

    pub const fn capability_snapshot_digest(self) -> u64 {
        self.capability_snapshot_digest
    }
}
