//! Face adjacency consistency validator.
//!
//! INVARIANT: Adjacent faces sharing an edge must belong to the same shell.

use crate::b_rep::EntityBitset;
use crate::b_rep::TopologyArena;
use forge_core::KernelError;

use super::vf;

pub(crate) fn validate_face_adjacency_consistency(
    arena: &TopologyArena,
) -> Result<(), KernelError> {
    let mut checked = EntityBitset::for_half_edges(arena);

    for (he_id, he_data) in arena.iter_half_edges() {
        if checked.contains(he_id.index())? {
            continue;
        }
        checked.insert(he_id.index())?;

        let twin = he_data.radial_next();
        if twin == he_id {
            continue;
        }
        checked.insert(twin.index())?;

        let twin_data = arena.get_half_edge(twin)?;
        let face_a = arena.get_face(he_data.face())?;
        let face_b = arena.get_face(twin_data.face())?;

        if face_a.shell() != face_b.shell() {
            return Err(vf("face_adjacency_consistency", format!(
                "Adjacent faces {} (shell {}) and {} (shell {}) share edge {} but are in different shells",
                he_data.face().index(), face_a.shell().index(),
                twin_data.face().index(), face_b.shell().index(),
                he_data.edge().index()
            )));
        }
    }
    Ok(())
}
