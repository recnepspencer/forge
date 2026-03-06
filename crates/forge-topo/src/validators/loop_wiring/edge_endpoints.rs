//! Edge endpoint / loop vertex match validator.
//!
//! INVARIANT: For every manifold edge pair, the twin's origin must equal
//! the destination of the halfedge (i.e., he.next.origin).

use crate::b_rep::TopologyArena;
use forge_core::KernelError;

use super::vf;

pub(crate) fn validate_edge_endpoints_match_loop_vertices(
    arena: &TopologyArena,
) -> Result<(), KernelError> {
    for (he_id, he_data) in arena.iter_half_edges() {
        let next_data = arena.get_half_edge(he_data.next())?;
        let dest = next_data.origin();

        let twin = he_data.radial_next();
        if twin != he_id {
            let twin_data = arena.get_half_edge(twin)?;
            let twin_dest = arena.get_half_edge(twin_data.next())?.origin();

            let is_opposite = twin_data.origin() == dest && twin_dest == he_data.origin();
            let is_same = twin_data.origin() == he_data.origin() && twin_dest == dest;

            if !is_opposite && !is_same {
                return Err(vf(
                    "edge_endpoints_match",
                    format!(
                        "HE {} ({}->{}) and twin {} ({}->{}) do not span the same vertices",
                        he_id.index(),
                        he_data.origin().index(),
                        dest.index(),
                        twin.index(),
                        twin_data.origin().index(),
                        twin_dest.index()
                    ),
                ));
            }
        }
    }
    Ok(())
}
