use crate::projection::TopologyDomainQueryRequestReport;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TopologyAdjacentHalfEdgeEvidence {
    pub(crate) half_edge_identity: String,
    pub(crate) edge_identity: String,
    pub(crate) shared_vertex_identities: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TopologyRadialCandidateEvidence {
    pub(crate) half_edge_identity: String,
    pub(crate) edge_identity: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TopologyLoopNeighborEvidence {
    pub(crate) half_edge_identity: String,
    pub(crate) next_half_edge_identity: String,
    pub(crate) previous_half_edge_identity: String,
    pub(crate) next_relation_identity: String,
    pub(crate) previous_relation_identity: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TopologyHalfEdgeSharedVertexNeighborhoodView {
    pub(crate) request_report: TopologyDomainQueryRequestReport,
    pub(crate) source_half_edge_identity: String,
    pub(crate) source_edge_identity: String,
    pub(crate) source_vertex_identities: Vec<String>,
    pub(crate) vertex_adjacent_half_edge_identities: Vec<String>,
    pub(crate) vertex_adjacent_different_edge_half_edge_identities: Vec<String>,
    pub(crate) vertex_adjacent_different_edge_half_edges: Vec<TopologyAdjacentHalfEdgeEvidence>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TopologyHalfEdgeRadialNeighborhoodView {
    pub(crate) request_report: TopologyDomainQueryRequestReport,
    pub(crate) source_half_edge_identity: String,
    pub(crate) source_edge_identity: String,
    pub(crate) current_target_half_edge_identity: String,
    pub(crate) current_target_edge_identity: String,
    pub(crate) source_radial_next_relation_identity: String,
    pub(crate) same_edge_half_edge_identities: Vec<String>,
    pub(crate) different_edge_half_edge_identities: Vec<String>,
    pub(crate) different_edge_half_edges: Vec<TopologyRadialCandidateEvidence>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TopologyLoopCycleView {
    pub(crate) request_report: TopologyDomainQueryRequestReport,
    pub(crate) start_half_edge_identity: String,
    pub(crate) cycle_identities: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TopologyLocalRewireNeighborhoodView {
    pub(crate) request_report: TopologyDomainQueryRequestReport,
    pub(crate) moved_half_edge_identity: String,
    pub(crate) old_successor_identity: String,
    pub(crate) old_predecessor_identity: String,
    pub(crate) cycle_identities: Vec<String>,
    pub(crate) cycle_half_edges: Vec<TopologyLoopNeighborEvidence>,
}




