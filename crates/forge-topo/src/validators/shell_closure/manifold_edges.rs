//! Manifold edge valence validator (Doctrine D8).
//!
//! INVARIANT: Every edge must have radial valence ≤ 2.

use crate::b_rep::TopologyArena;
use crate::b_rep::EntityBitset;
use forge_core::KernelError;

pub fn validate_manifold_edges(arena: &TopologyArena) -> Result<(), KernelError> {
    let mut checked_halfedges = EntityBitset::for_half_edges(arena);

    for (he_id, he_data) in arena.iter_half_edges() {
        if checked_halfedges.contains(he_id.index())? {
            continue;
        }

        checked_halfedges.insert(he_id.index())?;

        let edge_id = he_data.edge();
        let valence = crate::queries::traverse::radial_valence(arena, he_id)?;

        let mut curr = he_data.radial_next();
        while curr != he_id {
            checked_halfedges.insert(curr.index())?;
            curr = arena.get_half_edge(curr)?.radial_next();
        }

        if valence <= 2 {
            continue;
        }

        return Err(KernelError::TopologyViolation {
            err: forge_core::TopologyError::NonManifoldEdge {
                edge_index: edge_id.index(),
                valence,
            },
            context: Some(forge_core::ErrorContext {
                scope: forge_core::ErrorScope::Entity {
                    entity_kind: "Edge".to_string(),
                    index: edge_id.index(),
                },
                suggested_fixes: Vec::new(),
                detail: format!(
                    "Edge {} has radial valence {} (max allowed: 2). \
                     Doctrine D8 requires 2-manifold topology at commit time.",
                    edge_id.index(),
                    valence
                ),
            }),
        });
    }
    Ok(())
}
