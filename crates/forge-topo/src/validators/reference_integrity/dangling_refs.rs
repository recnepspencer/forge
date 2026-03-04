//! Dangling half-edge reference validator.
//!
//! INVARIANT: Every halfedge's referenced entities (origin, face, edge,
//! next, prev, radial_next) must exist in the arena.

use crate::b_rep::TopologyArena;
use forge_core::KernelError;

use super::vf;

pub(crate) fn validate_no_dangling_half_edge_refs(arena: &TopologyArena) -> Result<(), KernelError> {
    for (he_id, he_data) in arena.iter_half_edges() {
        arena.get_vertex(he_data.origin()).map_err(|_| {
            vf("no_dangling_half_edge_refs", format!(
                "HE {} references deleted vertex {}", he_id.index(), he_data.origin().index()
            ))
        })?;
        arena.get_face(he_data.face()).map_err(|_| {
            vf("no_dangling_half_edge_refs", format!(
                "HE {} references deleted face {}", he_id.index(), he_data.face().index()
            ))
        })?;
        arena.get_edge(he_data.edge()).map_err(|_| {
            vf("no_dangling_half_edge_refs", format!(
                "HE {} references deleted edge {}", he_id.index(), he_data.edge().index()
            ))
        })?;
        arena.get_half_edge(he_data.next()).map_err(|_| {
            vf("no_dangling_half_edge_refs", format!(
                "HE {} .next references deleted HE {}", he_id.index(), he_data.next().index()
            ))
        })?;
        arena.get_half_edge(he_data.prev()).map_err(|_| {
            vf("no_dangling_half_edge_refs", format!(
                "HE {} .prev references deleted HE {}", he_id.index(), he_data.prev().index()
            ))
        })?;
        arena.get_half_edge(he_data.radial_next()).map_err(|_| {
            vf("no_dangling_half_edge_refs", format!(
                "HE {} .radial_next references deleted HE {}", he_id.index(), he_data.radial_next().index()
            ))
        })?;
    }
    Ok(())
}
