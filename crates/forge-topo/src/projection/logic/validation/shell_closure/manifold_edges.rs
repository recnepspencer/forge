use forge_core::KernelError;

use crate::projection::data::{ProjectedEdgeId, ProjectedTopology};
use crate::projection::logic::ProjectedTopologyQueries;

use super::super::shared::vf;

pub fn validate_projected_manifold_edges(topology: &ProjectedTopology) -> Result<(), KernelError> {
    for edge_index in 0..topology.edge_count() {
        let edge = ProjectedEdgeId::new(edge_index as u32);
        let valence = topology.radial_valence(edge);
        if valence > 2 {
            return Err(vf(
                "projected_manifold_edges",
                format!(
                    "Edge {} has radial valence {} (max allowed: 2)",
                    edge.raw(),
                    valence
                ),
            ));
        }
    }
    Ok(())
}
