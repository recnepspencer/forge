use super::report::WorthTopologyDomainQueryRequestReport;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct WorthTopologyHalfEdgeSharedVertexNeighborhoodView {
    pub(crate) request_report: WorthTopologyDomainQueryRequestReport,
    pub(crate) source_half_edge_identity: String,
    pub(crate) source_edge_identity: String,
    pub(crate) source_vertex_identities: Vec<String>,
    pub(crate) vertex_adjacent_half_edge_identities: Vec<String>,
    pub(crate) vertex_adjacent_different_edge_half_edge_identities: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct WorthTopologyHalfEdgeRadialNeighborhoodView {
    pub(crate) request_report: WorthTopologyDomainQueryRequestReport,
    pub(crate) source_half_edge_identity: String,
    pub(crate) source_edge_identity: String,
    pub(crate) current_target_half_edge_identity: String,
    pub(crate) current_target_edge_identity: String,
    pub(crate) same_edge_half_edge_identities: Vec<String>,
    pub(crate) different_edge_half_edge_identities: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct WorthTopologyLoopCycleView {
    pub(crate) request_report: WorthTopologyDomainQueryRequestReport,
    pub(crate) start_half_edge_identity: String,
    pub(crate) cycle_identities: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct WorthTopologyLocalRewireNeighborhoodView {
    pub(crate) request_report: WorthTopologyDomainQueryRequestReport,
    pub(crate) moved_half_edge_identity: String,
    pub(crate) old_successor_identity: String,
    pub(crate) old_predecessor_identity: String,
    pub(crate) cycle_identities: Vec<String>,
}
