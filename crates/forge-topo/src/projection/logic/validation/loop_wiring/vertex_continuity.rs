use std::collections::BTreeSet;

use forge_core::KernelError;

use crate::projection::data::{ProjectedEdgeId, ProjectedTopology};
use crate::projection::logic::ProjectedTopologyQueries;

use super::vf;

pub fn validate_projected_vertex_continuity(
    topology: &ProjectedTopology,
) -> Result<(), KernelError> {
    for (edge_index, edge) in topology.edges().iter().enumerate() {
        let edge_id = ProjectedEdgeId::new(edge_index as u32);
        let mut endpoints = BTreeSet::new();
        for half_edge_id in topology.radial_half_edges(edge.half_edge) {
            let half_edge = topology.half_edge(half_edge_id);
            let next = topology.half_edge(half_edge.next);
            endpoints.insert(half_edge.origin.raw());
            endpoints.insert(next.origin.raw());
        }
        if endpoints.len() > 2 {
            return Err(vf(
                "projected_vertex_continuity",
                format!(
                    "Edge {} has {} distinct endpoint vertices; expected 1 or 2",
                    edge_id.raw(),
                    endpoints.len()
                ),
            ));
        }
    }
    Ok(())
}
