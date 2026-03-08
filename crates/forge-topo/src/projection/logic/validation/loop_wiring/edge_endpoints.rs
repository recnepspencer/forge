use forge_core::KernelError;

use crate::projection::data::{ProjectedHalfEdgeId, ProjectedTopology};

use super::vf;

pub fn validate_projected_edge_endpoints_match_loop_vertices(
    topology: &ProjectedTopology,
) -> Result<(), KernelError> {
    for (index, half_edge) in topology.half_edges().iter().enumerate() {
        let he_id = ProjectedHalfEdgeId::new(index as u32);
        let dest = topology.half_edge(half_edge.next).origin;
        let twin_id = half_edge.radial_next;
        if twin_id == he_id {
            continue;
        }

        let twin = topology.half_edge(twin_id);
        let twin_dest = topology.half_edge(twin.next).origin;
        let is_opposite = twin.origin == dest && twin_dest == half_edge.origin;
        let is_same = twin.origin == half_edge.origin && twin_dest == dest;

        if !is_opposite && !is_same {
            return Err(vf(
                "projected_edge_endpoints_match",
                format!(
                    "HE {} ({}->{}) and radial neighbor {} ({}->{}) do not span the same vertices",
                    he_id.raw(),
                    half_edge.origin.raw(),
                    dest.raw(),
                    twin_id.raw(),
                    twin.origin.raw(),
                    twin_dest.raw()
                ),
            ));
        }
    }
    Ok(())
}
