use std::collections::BTreeMap;

use forge_core::KernelError;

use crate::projection::data::{ProjectedEdgeId, ProjectedTopology};

use super::{vf, walk_radial_ring};

pub fn validate_projected_no_broken_radial_splices(
    topology: &ProjectedTopology,
) -> Result<(), KernelError> {
    let mut edge_half_edge_counts = BTreeMap::new();
    for half_edge in topology.half_edges() {
        *edge_half_edge_counts
            .entry(half_edge.edge.raw())
            .or_insert(0usize) += 1;
    }

    for (edge_index, edge) in topology.edges().iter().enumerate() {
        let edge_id = ProjectedEdgeId::new(edge_index as u32);
        let ring = walk_radial_ring(topology, edge.half_edge)
            .map_err(|err| vf("projected_no_broken_radial_splices", err))?;
        let expected = edge_half_edge_counts
            .get(&edge_id.raw())
            .copied()
            .unwrap_or(0);
        if ring.len() != expected {
            return Err(vf(
                "projected_no_broken_radial_splices",
                format!(
                    "Edge {} has {} halfedges referencing it but radial ring from {} only reaches {}",
                    edge_id.raw(),
                    expected,
                    edge.half_edge.raw(),
                    ring.len()
                ),
            ));
        }
    }
    Ok(())
}
