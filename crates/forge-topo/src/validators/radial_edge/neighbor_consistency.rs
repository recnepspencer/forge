//! Radial neighbor consistency validator.
//!
//! INVARIANT: In a 2-manifold radial pair, origins must differ (opposite traverse).

use crate::b_rep::TopologyArena;
use crate::b_rep::EntityBitset;
use forge_core::KernelError;

use super::vf;

pub(crate) fn validate_radial_neighbor_consistency(arena: &TopologyArena) -> Result<(), KernelError> {
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

        if he_data.origin() == twin_data.origin() {
            let valence = crate::queries::traverse::radial_valence(arena, he_id)?;
            if valence == 2 {
                return Err(vf("radial_neighbor_consistency", format!(
                    "Manifold edge pair HE {} and HE {} have same origin vertex {} \
                     (co-edges, not twins). True twins must have opposite orientations.",
                    he_id.index(), twin.index(), he_data.origin().index()
                )));
            }
        }
    }
    Ok(())
}
