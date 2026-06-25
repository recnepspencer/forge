use super::domain::TopologyReadRequestReport;
use super::models::{
    TopologyAdjacentHalfEdgeEvidence, TopologyHalfEdgeRadialNeighborhoodView,
    TopologyHalfEdgeSharedVertexNeighborhoodView, TopologyLocalRewireNeighborhoodView,
    TopologyLoopCycleView, TopologyLoopNeighborEvidence, TopologyRadialCandidateEvidence,
    TopologyShellBoundaryNeighborhoodView,
};

impl TopologyAdjacentHalfEdgeEvidence {
    pub fn half_edge_identity(&self) -> &str {
        self.half_edge_identity.as_str()
    }

    pub fn edge_identity(&self) -> &str {
        self.edge_identity.as_str()
    }

    pub fn shared_vertex_identities(&self) -> &[String] {
        self.shared_vertex_identities.as_slice()
    }
}

impl TopologyRadialCandidateEvidence {
    pub fn half_edge_identity(&self) -> &str {
        self.half_edge_identity.as_str()
    }

    pub fn edge_identity(&self) -> &str {
        self.edge_identity.as_str()
    }
}

impl TopologyLoopNeighborEvidence {
    pub fn half_edge_identity(&self) -> &str {
        self.half_edge_identity.as_str()
    }

    pub fn next_half_edge_identity(&self) -> &str {
        self.next_half_edge_identity.as_str()
    }

    pub fn previous_half_edge_identity(&self) -> &str {
        self.previous_half_edge_identity.as_str()
    }

    pub fn next_relation_identity(&self) -> &str {
        self.next_relation_identity.as_str()
    }

    pub fn previous_relation_identity(&self) -> &str {
        self.previous_relation_identity.as_str()
    }
}

impl TopologyHalfEdgeSharedVertexNeighborhoodView {
    pub fn request_report(&self) -> &TopologyReadRequestReport {
        &self.request_report
    }

    pub fn source_half_edge_identity(&self) -> &str {
        self.source_half_edge_identity.as_str()
    }

    pub fn source_edge_identity(&self) -> &str {
        self.source_edge_identity.as_str()
    }

    pub fn source_vertex_identities(&self) -> &[String] {
        self.source_vertex_identities.as_slice()
    }

    pub fn vertex_adjacent_half_edge_identities(&self) -> &[String] {
        self.vertex_adjacent_half_edge_identities.as_slice()
    }

    pub fn vertex_adjacent_different_edge_half_edge_identities(&self) -> &[String] {
        self.vertex_adjacent_different_edge_half_edge_identities
            .as_slice()
    }

    pub fn vertex_adjacent_different_edge_half_edges(&self) -> &[TopologyAdjacentHalfEdgeEvidence] {
        self.vertex_adjacent_different_edge_half_edges.as_slice()
    }
}

impl TopologyHalfEdgeRadialNeighborhoodView {
    pub fn request_report(&self) -> &TopologyReadRequestReport {
        &self.request_report
    }

    pub fn source_half_edge_identity(&self) -> &str {
        self.source_half_edge_identity.as_str()
    }

    pub fn source_edge_identity(&self) -> &str {
        self.source_edge_identity.as_str()
    }

    pub fn current_target_half_edge_identity(&self) -> &str {
        self.current_target_half_edge_identity.as_str()
    }

    pub fn current_target_edge_identity(&self) -> &str {
        self.current_target_edge_identity.as_str()
    }

    pub fn source_radial_next_relation_identity(&self) -> &str {
        self.source_radial_next_relation_identity.as_str()
    }

    pub fn same_edge_half_edge_identities(&self) -> &[String] {
        self.same_edge_half_edge_identities.as_slice()
    }

    pub fn different_edge_half_edge_identities(&self) -> &[String] {
        self.different_edge_half_edge_identities.as_slice()
    }

    pub fn different_edge_half_edges(&self) -> &[TopologyRadialCandidateEvidence] {
        self.different_edge_half_edges.as_slice()
    }
}

impl TopologyShellBoundaryNeighborhoodView {
    pub fn request_report(&self) -> &TopologyReadRequestReport {
        &self.request_report
    }

    pub fn touched_shell_identity(&self) -> &str {
        self.touched_shell_identity.as_str()
    }

    pub fn touched_face_identity(&self) -> &str {
        self.touched_face_identity.as_str()
    }

    pub fn source_half_edge_identity(&self) -> &str {
        self.source_half_edge_identity.as_str()
    }

    pub fn source_edge_identity(&self) -> &str {
        self.source_edge_identity.as_str()
    }

    pub fn current_target_half_edge_identity(&self) -> &str {
        self.current_target_half_edge_identity.as_str()
    }

    pub fn current_target_edge_identity(&self) -> &str {
        self.current_target_edge_identity.as_str()
    }

    pub fn source_radial_next_relation_identity(&self) -> &str {
        self.source_radial_next_relation_identity.as_str()
    }

    pub fn same_edge_half_edge_identities(&self) -> &[String] {
        self.same_edge_half_edge_identities.as_slice()
    }

    pub fn different_edge_half_edge_identities(&self) -> &[String] {
        self.different_edge_half_edge_identities.as_slice()
    }

    pub fn different_edge_half_edges(&self) -> &[TopologyRadialCandidateEvidence] {
        self.different_edge_half_edges.as_slice()
    }
}

impl TopologyLoopCycleView {
    pub fn request_report(&self) -> &TopologyReadRequestReport {
        &self.request_report
    }

    pub fn start_half_edge_identity(&self) -> &str {
        self.start_half_edge_identity.as_str()
    }

    pub fn cycle_identities(&self) -> &[String] {
        self.cycle_identities.as_slice()
    }
}

impl TopologyLocalRewireNeighborhoodView {
    pub fn request_report(&self) -> &TopologyReadRequestReport {
        &self.request_report
    }

    pub fn moved_half_edge_identity(&self) -> &str {
        self.moved_half_edge_identity.as_str()
    }

    pub fn old_successor_identity(&self) -> &str {
        self.old_successor_identity.as_str()
    }

    pub fn old_predecessor_identity(&self) -> &str {
        self.old_predecessor_identity.as_str()
    }

    pub fn cycle_identities(&self) -> &[String] {
        self.cycle_identities.as_slice()
    }

    pub fn cycle_half_edges(&self) -> &[TopologyLoopNeighborEvidence] {
        self.cycle_half_edges.as_slice()
    }
}
