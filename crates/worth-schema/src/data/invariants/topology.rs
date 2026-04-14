use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum WorthTopologyInvariantGroup {
    OwnershipConsistency,
    RequiredSingleRelationPresence,
    LoopEntryCoherence,
    HalfEdgeNextCoherence,
    HalfEdgeRadialCoherence,
    EdgeIncidenceLegality,
    VertexOriginLegality,
}

impl WorthTopologyInvariantGroup {
    pub const ALL: [Self; 7] = [
        Self::OwnershipConsistency,
        Self::RequiredSingleRelationPresence,
        Self::LoopEntryCoherence,
        Self::HalfEdgeNextCoherence,
        Self::HalfEdgeRadialCoherence,
        Self::EdgeIncidenceLegality,
        Self::VertexOriginLegality,
    ];
}
