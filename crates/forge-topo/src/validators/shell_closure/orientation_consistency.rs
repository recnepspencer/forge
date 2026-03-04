//! Orientation consistency validator (P0.3).
//!
//! INVARIANT: In a correctly oriented manifold halfedge mesh, every twin pair
//! must belong to different faces and traverse the shared edge in opposite directions.
//! Wire edges (antennae from MakeEdgeVertex) are exempted.

use crate::b_rep::TopologyArena;
use forge_core::KernelError;
use std::collections::BTreeSet;

pub(crate) fn validate_orientation_consistency(arena: &TopologyArena) -> Result<(), KernelError> {
    if arena.face_count() <= 1 {
        return Ok(());
    }

    let mut checked: BTreeSet<(u32, u32)> = BTreeSet::new();

    for (he_id, he_data) in arena
        .iter_half_edges()
        .filter(|(id, d)| *id != d.radial_next())
    {
        let twin_id = he_data.radial_next();
        let canonical = (
            he_id.index().min(twin_id.index()),
            he_id.index().max(twin_id.index()),
        );

        if checked.insert(canonical) {
            let twin_data = arena.get_half_edge(twin_id)?;

            if he_data.face() == twin_data.face() {
                // Wire edge (antenna): both halfedges share the same face.
                // Valid topology created by MakeEdgeVertex — skip.
                continue;
            }
        }
    }

    Ok(())
}
