//! Read-only hierarchy queries for face/region/shell relationships.
//!
//! DOMAIN: Deterministic topology hierarchy accessors that centralize
//! common scans and parent-child traversal logic.
//!
//! INVARIANTS:
//! - Ordering is deterministic (slot iteration order)
//! - No topology mutation occurs
//! - Queries validate handles through arena accessors

use forge_core::KernelError;

use crate::b_rep::TopologyArena;
use crate::handles::{FaceId, RegionId, ShellId};

/// Return all faces that belong to a shell.
///
/// This performs an arena scan keyed by `FaceData::shell`.
pub fn shell_faces(arena: &TopologyArena, shell: ShellId) -> Result<Vec<FaceId>, KernelError> {
    arena.get_shell(shell)?;

    let mut faces = Vec::new();
    for (face_id, face_data) in arena.iter_faces() {
        if face_data.shell() == shell {
            faces.push(face_id);
        }
    }

    Ok(faces)
}

/// Return the shells of a region in canonical order (outer shell first, then inner shells).
pub fn region_shells(arena: &TopologyArena, region: RegionId) -> Result<Vec<ShellId>, KernelError> {
    let region_data = arena.get_region(region)?;
    let mut shells = Vec::with_capacity(region_data.shell_count());

    if let Some(outer_shell) = region_data.outer_shell() {
        shells.push(outer_shell);
    }

    shells.extend_from_slice(region_data.inner_shells());
    Ok(shells)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity_lifecycle::make_vertex_face::MakeVertexFace;
    use crate::transactions::TopologyState;

    #[test]
    fn seed_region_and_shell_queries_return_seed_face() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();
        let mvf = draft.execute(MakeVertexFace).unwrap().into_value();
        let state = draft.commit().unwrap();

        let shells = region_shells(state.arena(), mvf.region).unwrap();
        let faces = shell_faces(state.arena(), mvf.shell).unwrap();

        assert_eq!(shells, vec![mvf.shell]);
        assert_eq!(faces, vec![mvf.face]);
    }
}
