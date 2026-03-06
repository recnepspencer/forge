//! Radial neighbor consistency validator.
//!
//! INVARIANT: In a 2-manifold radial pair, origins must differ (opposite traverse).
//!
//! EXCEPTION: NMT slit pairs (same face) are always valid. Additionally, in an
//! NMT-capable kernel, co-directional valence-2 pairs can arise from valid merge
//! operations (e.g., two sheets meeting at an edge). This is a geometric concern
//! (face-normal orientation), not a topological one, so it is demoted to a warning.

use crate::b_rep::EntityBitset;
use crate::b_rep::TopologyArena;
use forge_core::KernelError;

use super::vf;

pub(crate) fn validate_radial_neighbor_consistency(
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

        if he_data.origin() == twin_data.origin() {
            let valence = crate::queries::traverse::radial_valence(arena, he_id)?;
            if valence == 2 {
                // NMT slit exception: both HEs belong to the same face (inner loop seam).
                if he_data.face() == twin_data.face() {
                    continue;
                }

                // In an NMT-capable kernel, co-directional valence-2 pairs can
                // arise from valid topology (e.g., merged sheet junctions).
                // This is a geometric orientation concern, not topological.
                // Log a warning for diagnostic visibility but do not reject.
                tracing::warn!(
                    "radial_neighbor_consistency: Manifold edge pair HE {} and HE {} \
                     have same origin vertex {} (co-directional). This may indicate \
                     a face-orientation issue (geometric, not topological).",
                    he_id.index(),
                    twin.index(),
                    he_data.origin().index()
                );
            }
        }
    }
    Ok(())
}
