use std::collections::BTreeSet;

use forge_core::KernelError;

use crate::projection::data::{ProjectedLoopId, ProjectedTopology};
use crate::projection::logic::ProjectedTopologyQueries;

use super::vf;

pub fn validate_projected_no_duplicate_coedges_in_loop(
    topology: &ProjectedTopology,
) -> Result<(), KernelError> {
    for (loop_index, _) in topology.loops().iter().enumerate() {
        let loop_id = ProjectedLoopId::new(loop_index as u32);
        let mut seen = BTreeSet::new();
        for half_edge in topology
            .loop_half_edges(loop_id)
            .map_err(|err| vf("projected_duplicate_coedges", err.to_string()))?
        {
            if !seen.insert(half_edge.raw()) {
                return Err(vf(
                    "projected_duplicate_coedges",
                    format!(
                        "Loop {} contains duplicate halfedge {}",
                        loop_id.raw(),
                        half_edge.raw()
                    ),
                ));
            }
        }
    }
    Ok(())
}
