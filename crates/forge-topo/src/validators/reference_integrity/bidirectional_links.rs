//! Bidirectional link validator.
//!
//! INVARIANT: Every representative pointer (Edge→HE, Loop→HE, Shell→Face)
//! must be reciprocated by the target entity.

use crate::b_rep::TopologyArena;
use forge_core::KernelError;

use super::vf;

pub(crate) fn validate_bidirectional_links(arena: &TopologyArena) -> Result<(), KernelError> {
    // Vertex primary-disk → HE → Vertex
    for (vertex_id, vertex_data) in arena.iter_vertices() {
        let rep_he = vertex_data.primary_disk();
        let he_data = arena.get_half_edge(rep_he).map_err(|_| {
            vf("bidirectional_links", format!(
                "Vertex {} primary-disk HE {} is deleted", vertex_id.index(), rep_he.index()
            ))
        })?;
        if he_data.origin() != vertex_id {
            return Err(vf("bidirectional_links", format!(
                "Vertex {} primary-disk HE {} originates from vertex {} instead",
                vertex_id.index(), rep_he.index(), he_data.origin().index()
            )));
        }
    }

    // Edge → HE → Edge
    for (edge_id, edge_data) in arena.iter_edges() {
        let rep_he = edge_data.half_edge();
        let he_data = arena.get_half_edge(rep_he).map_err(|_| {
            vf("bidirectional_links", format!(
                "Edge {} representative HE {} is deleted", edge_id.index(), rep_he.index()
            ))
        })?;
        if he_data.edge() != edge_id {
            return Err(vf("bidirectional_links", format!(
                "Edge {} representative HE {} references edge {} instead",
                edge_id.index(), rep_he.index(), he_data.edge().index()
            )));
        }
    }

    // Loop → HE → Face matches Loop → Face
    for (loop_id, loop_data) in arena.iter_loops() {
        let rep_he = loop_data.half_edge();
        let he_data = arena.get_half_edge(rep_he).map_err(|_| {
            vf("bidirectional_links", format!(
                "Loop {} representative HE {} is deleted", loop_id.index(), rep_he.index()
            ))
        })?;
        if he_data.face() != loop_data.face() {
            return Err(vf("bidirectional_links", format!(
                "Loop {} (face {}) representative HE {} is on face {} instead",
                loop_id.index(), loop_data.face().index(),
                rep_he.index(), he_data.face().index()
            )));
        }
    }

    // Shell → Face → Shell
    for (shell_id, shell_data) in arena.iter_shells() {
        let rep_face = shell_data.representative_face();
        let face_data = arena.get_face(rep_face).map_err(|_| {
            vf("bidirectional_links", format!(
                "Shell {} representative face {} is deleted", shell_id.index(), rep_face.index()
            ))
        })?;
        if face_data.shell() != shell_id {
            return Err(vf("bidirectional_links", format!(
                "Shell {} representative face {} references shell {} instead",
                shell_id.index(), rep_face.index(), face_data.shell().index()
            )));
        }
    }

    Ok(())
}
