use forge_core::KernelError;

use crate::projection::data::{ProjectedFaceId, ProjectedTopology};
use crate::projection::logic::ProjectedTopologyQueries;

use super::super::shared::vf;

pub fn validate_projected_broken_boundary(
    topology: &ProjectedTopology,
) -> Result<(), KernelError> {
    for face_index in 0..topology.face_count() {
        let face = ProjectedFaceId::new(face_index as u32);
        for half_edge in topology
            .face_half_edges(face)
            .map_err(|err| vf("projected_broken_boundary", err.to_string()))?
        {
            let half_edge_data = topology.half_edge(half_edge);
            if half_edge_data.face != face {
                return Err(vf(
                    "projected_broken_boundary",
                    format!(
                        "Face {} boundary includes halfedge {} claiming face {}",
                        face.raw(),
                        half_edge.raw(),
                        half_edge_data.face.raw()
                    ),
                ));
            }
        }
    }
    Ok(())
}
