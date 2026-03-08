use std::collections::BTreeSet;

use forge_core::KernelError;

use crate::projection::data::{ProjectedHalfEdgeId, ProjectedTopology};

use super::{vf, walk_radial_ring};

pub fn validate_projected_radial_edge_consistency(
    topology: &ProjectedTopology,
) -> Result<(), KernelError> {
    let mut checked = BTreeSet::new();
    for (start_index, start_half_edge) in topology.half_edges().iter().enumerate() {
        let start_id = ProjectedHalfEdgeId::new(start_index as u32);
        if !checked.insert(start_id.raw()) {
            continue;
        }
        let expected_edge = start_half_edge.edge;
        for current in walk_radial_ring(topology, start_id)
            .map_err(|err| vf("projected_radial_edge_consistency", err))?
        {
            checked.insert(current.raw());
            if topology.half_edge(current).edge != expected_edge {
                return Err(vf(
                    "projected_radial_edge_consistency",
                    format!(
                        "HE {} claims edge {} but ring seed {} claims edge {}",
                        current.raw(),
                        topology.half_edge(current).edge.raw(),
                        start_id.raw(),
                        expected_edge.raw()
                    ),
                ));
            }
        }
    }
    Ok(())
}
