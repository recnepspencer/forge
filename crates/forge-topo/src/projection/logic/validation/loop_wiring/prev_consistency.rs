use forge_core::KernelError;

use crate::projection::data::{ProjectedHalfEdgeId, ProjectedTopology};

use super::vf;

pub fn validate_projected_prev_consistency(
    topology: &ProjectedTopology,
) -> Result<(), KernelError> {
    for (index, half_edge) in topology.half_edges().iter().enumerate() {
        let he_id = ProjectedHalfEdgeId::new(index as u32);
        let prev_data = topology.half_edge(half_edge.prev);
        if prev_data.next != he_id {
            return Err(vf(
                "projected_prev_consistency",
                format!(
                    "HE[{}].prev = {} but HE[{}].next = {} (expected {})",
                    he_id.raw(),
                    half_edge.prev.raw(),
                    half_edge.prev.raw(),
                    prev_data.next.raw(),
                    he_id.raw()
                ),
            ));
        }
    }
    Ok(())
}
