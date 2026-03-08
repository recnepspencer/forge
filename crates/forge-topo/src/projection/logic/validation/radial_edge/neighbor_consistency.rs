use std::collections::BTreeSet;

use forge_core::KernelError;

use crate::projection::data::{ProjectedHalfEdgeId, ProjectedTopology};
use crate::projection::logic::ProjectedTopologyQueries;

pub fn validate_projected_radial_neighbor_consistency(
    topology: &ProjectedTopology,
) -> Result<(), KernelError> {
    let mut checked = BTreeSet::new();
    for (he_index, half_edge) in topology.half_edges().iter().enumerate() {
        let he_id = ProjectedHalfEdgeId::new(he_index as u32);
        if checked.contains(&he_id.raw()) {
            continue;
        }
        checked.insert(he_id.raw());
        let neighbor_id = half_edge.radial_next;
        if neighbor_id == he_id {
            continue;
        }
        checked.insert(neighbor_id.raw());
        let neighbor = topology.half_edge(neighbor_id);
        if half_edge.origin == neighbor.origin {
            let valence = topology.edge_half_edges(half_edge.edge).len();
            if valence == 2 && half_edge.face != neighbor.face {
                tracing::warn!(
                    "projected_radial_neighbor_consistency: HE {} and HE {} are co-directional at vertex {}",
                    he_id.raw(),
                    neighbor_id.raw(),
                    half_edge.origin.raw()
                );
            }
        }
    }
    Ok(())
}
