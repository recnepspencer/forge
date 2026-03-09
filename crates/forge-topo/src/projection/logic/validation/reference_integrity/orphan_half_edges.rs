use std::collections::BTreeSet;

use forge_core::KernelError;

use crate::projection::data::{ProjectedLoopId, ProjectedTopology};
use crate::projection::logic::ProjectedTopologyQueries;

use super::super::shared::vf;

pub fn validate_projected_no_orphan_half_edges(
    topology: &ProjectedTopology,
) -> Result<(), KernelError> {
    let mut reachable = BTreeSet::new();
    for (loop_index, _) in topology.loops().iter().enumerate() {
        let loop_id = ProjectedLoopId::new(loop_index as u32);
        for half_edge in topology
            .loop_half_edges(loop_id)
            .map_err(|err| vf("projected_no_orphan_half_edges", err.to_string()))?
        {
            reachable.insert(half_edge.raw());
        }
    }

    for (half_edge_index, _) in topology.half_edges().iter().enumerate() {
        if !reachable.contains(&(half_edge_index as u32)) {
            return Err(vf(
                "projected_no_orphan_half_edges",
                format!(
                    "HalfEdge {} is not reachable from any loop",
                    half_edge_index
                ),
            ));
        }
    }
    Ok(())
}
