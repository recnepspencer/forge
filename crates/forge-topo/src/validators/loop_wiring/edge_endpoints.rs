//! Edge endpoint / loop vertex match validator.
//!
//! INVARIANT: For every manifold edge pair, the twin's origin must equal
//! the destination of the halfedge (i.e., he.next.origin).

use crate::b_rep::TopologyArena;
use forge_core::KernelError;

use super::vf;

pub(crate) fn validate_edge_endpoints_match_loop_vertices(arena: &TopologyArena) -> Result<(), KernelError> {
    for (he_id, he_data) in arena.iter_half_edges() {
        let next_data = arena.get_half_edge(he_data.next())?;
        let twin = he_data.radial_next();
        if twin != he_id {
            let twin_data = arena.get_half_edge(twin)?;
            if twin_data.origin() != next_data.origin() {
                return Err(vf("edge_endpoints_match", format!(
                    "HE {} twin {} origin {} != HE {}.next().origin {} (vertex wiring broken)",
                    he_id.index(), twin.index(), twin_data.origin().index(),
                    he_id.index(), next_data.origin().index()
                )));
            }
        }
    }
    Ok(())
}
