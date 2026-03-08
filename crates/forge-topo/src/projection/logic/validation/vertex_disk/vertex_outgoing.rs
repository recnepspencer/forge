use forge_core::KernelError;

use crate::projection::data::{ProjectedTopology, ProjectedVertexId};
use crate::projection::logic::ProjectedTopologyQueries;

use super::super::shared::vf;

pub fn validate_projected_vertex_outgoing(topology: &ProjectedTopology) -> Result<(), KernelError> {
    for (vertex_index, vertex) in topology.vertices().iter().enumerate() {
        let vertex_id = ProjectedVertexId::new(vertex_index as u32);
        let outgoing = topology.vertex_outgoing_half_edges(vertex_id);

        match (vertex.primary_half_edge, outgoing.is_empty()) {
            (None, true) => {}
            (None, false) => {
                return Err(vf(
                    "projected_vertex_outgoing",
                    format!(
                        "Vertex {} has {} outgoing halfedges but no primary_half_edge",
                        vertex_id.raw(),
                        outgoing.len()
                    ),
                ));
            }
            (Some(primary), _) => {
                if topology.half_edge(primary).origin != vertex_id {
                    return Err(vf(
                        "projected_vertex_outgoing",
                        format!(
                            "Vertex {} primary_half_edge {} originates at vertex {}",
                            vertex_id.raw(),
                            primary.raw(),
                            topology.half_edge(primary).origin.raw()
                        ),
                    ));
                }
            }
        }
    }

    Ok(())
}
