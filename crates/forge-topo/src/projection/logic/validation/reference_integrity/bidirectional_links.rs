use forge_core::KernelError;

use crate::projection::data::{ProjectedHalfEdgeId, ProjectedTopology};

use super::super::shared::vf;

pub fn validate_projected_bidirectional_links(
    topology: &ProjectedTopology,
) -> Result<(), KernelError> {
    for (vertex_index, vertex) in topology.vertices().iter().enumerate() {
        if let Some(half_edge) = vertex.primary_half_edge {
            let half_edge_data =
                half_edge_checked(topology, half_edge, "vertex", vertex_index as u32)?;
            if half_edge_data.origin.raw() != vertex_index as u32 {
                return Err(vf(
                    "projected_bidirectional_links",
                    format!(
                        "Vertex {} primary halfedge {} originates from vertex {} instead",
                        vertex_index,
                        half_edge.raw(),
                        half_edge_data.origin.raw()
                    ),
                ));
            }
        }
    }

    for (edge_index, edge) in topology.edges().iter().enumerate() {
        let half_edge_data =
            half_edge_checked(topology, edge.half_edge, "edge", edge_index as u32)?;
        if half_edge_data.edge.raw() != edge_index as u32 {
            return Err(vf(
                "projected_bidirectional_links",
                format!(
                    "Edge {} representative halfedge {} references edge {} instead",
                    edge_index,
                    edge.half_edge.raw(),
                    half_edge_data.edge.raw()
                ),
            ));
        }
    }

    for (loop_index, loop_data) in topology.loops().iter().enumerate() {
        let half_edge_data =
            half_edge_checked(topology, loop_data.half_edge, "loop", loop_index as u32)?;
        if half_edge_data.face != loop_data.face {
            return Err(vf(
                "projected_bidirectional_links",
                format!(
                    "Loop {} representative halfedge {} is on face {} instead of {}",
                    loop_index,
                    loop_data.half_edge.raw(),
                    half_edge_data.face.raw(),
                    loop_data.face.raw()
                ),
            ));
        }
    }

    Ok(())
}

fn half_edge_checked<'a>(
    topology: &'a ProjectedTopology,
    id: ProjectedHalfEdgeId,
    owner_kind: &str,
    owner_index: u32,
) -> Result<&'a crate::projection::data::ProjectedHalfEdgeData, KernelError> {
    topology.half_edges().get(id.index()).ok_or_else(|| {
        vf(
            "projected_bidirectional_links",
            format!(
                "{} {} references missing halfedge {}",
                owner_kind,
                owner_index,
                id.raw()
            ),
        )
    })
}
