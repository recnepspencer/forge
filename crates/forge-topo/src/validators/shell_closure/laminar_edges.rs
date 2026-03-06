//! Laminar edge validator for sheet shells.
//!
//! INVARIANT: Sheet shells must not contain edges with radial valence > 2.

use crate::b_rep::TopologyArena;
use crate::queries::traverse::FaceEdgeIterator;
use forge_core::KernelError;

use super::vf;

pub(crate) fn validate_boundary_edges_laminar_only(
    arena: &TopologyArena,
) -> Result<(), KernelError> {
    for (shell_id, shell_data) in arena.iter_shells() {
        if !matches!(shell_data.kind(), crate::b_rep::ShellKind::Sheet) {
            continue;
        }

        for (face_id, face_data) in arena.iter_faces() {
            if face_data.shell() != shell_id {
                continue;
            }
            let iter = FaceEdgeIterator::new(arena, face_id)?;
            for he_res in iter {
                let he_id = he_res?;
                let valence = crate::queries::traverse::radial_valence(arena, he_id)?;
                if valence > 2 {
                    return Err(vf("boundary_edges_laminar", format!(
                        "Sheet shell {} face {} has edge {} with radial valence {} (max 2 for sheets)",
                        shell_id.index(), face_id.index(),
                        arena.get_half_edge(he_id)?.edge().index(), valence
                    )));
                }
            }
        }
    }
    Ok(())
}
