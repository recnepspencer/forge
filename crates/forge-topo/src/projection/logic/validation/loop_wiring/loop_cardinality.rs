use forge_core::KernelError;

use crate::projection::data::{ProjectedLoopId, ProjectedTopology};
use crate::projection::logic::ProjectedTopologyQueries;

use super::vf;

pub fn validate_projected_loop_minimum_cardinality(
    topology: &ProjectedTopology,
) -> Result<(), KernelError> {
    for (loop_index, _) in topology.loops().iter().enumerate() {
        let loop_id = ProjectedLoopId::new(loop_index as u32);
        let half_edges = topology
            .loop_half_edges(loop_id)
            .map_err(|err| vf("projected_loop_minimum_cardinality", err.to_string()))?;
        if half_edges.len() < 2 {
            return Err(vf(
                "projected_loop_minimum_cardinality",
                format!(
                    "Loop {} contains {} halfedge(s); expected at least 2",
                    loop_id.raw(),
                    half_edges.len()
                ),
            ));
        }
    }
    Ok(())
}
