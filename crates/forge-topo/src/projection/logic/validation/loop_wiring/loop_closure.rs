use forge_core::KernelError;

use crate::projection::data::{ProjectedFaceId, ProjectedTopology};
use crate::projection::logic::ProjectedTopologyQueries;

use super::vf;

pub fn validate_projected_loops(topology: &ProjectedTopology) -> Result<(), KernelError> {
    for (face_index, _) in topology.faces().iter().enumerate() {
        let face_id = ProjectedFaceId::new(face_index as u32);
        for half_edge in topology
            .face_half_edges(face_id)
            .map_err(|err| vf("projected_loop_closure", err.to_string()))?
        {
            let half_edge_data = topology.half_edge(half_edge);
            if half_edge_data.face != face_id {
                return Err(vf(
                    "projected_loop_closure",
                    format!(
                        "HE {} is reachable from face {} loops but claims face {}",
                        half_edge.raw(),
                        face_id.raw(),
                        half_edge_data.face.raw()
                    ),
                ));
            }
        }
    }
    Ok(())
}
