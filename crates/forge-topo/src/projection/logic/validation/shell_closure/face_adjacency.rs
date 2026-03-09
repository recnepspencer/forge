use forge_core::KernelError;

use crate::projection::data::{ProjectedEdgeId, ProjectedTopology};
use crate::projection::logic::ProjectedTopologyQueries;

use super::super::shared::vf;

pub fn validate_projected_face_adjacency(topology: &ProjectedTopology) -> Result<(), KernelError> {
    for edge_index in 0..topology.edge_count() {
        let edge = ProjectedEdgeId::new(edge_index as u32);
        let half_edges = topology.edge_half_edges(edge);
        if half_edges.len() != 2 {
            continue;
        }

        let face_a = topology.half_edge(half_edges[0]).face;
        let face_b = topology.half_edge(half_edges[1]).face;
        let shell_a = topology.face(face_a).shell;
        let shell_b = topology.face(face_b).shell;

        if shell_a != shell_b {
            return Err(vf(
                "projected_face_adjacency_consistency",
                format!(
                    "Adjacent faces {} (shell {}) and {} (shell {}) share edge {} but are in different shells",
                    face_a.raw(),
                    shell_a.raw(),
                    face_b.raw(),
                    shell_b.raw(),
                    edge.raw()
                ),
            ));
        }
    }
    Ok(())
}
