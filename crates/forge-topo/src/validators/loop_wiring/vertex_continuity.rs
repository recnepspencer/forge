//! Vertex continuity validator.
//!
//! INVARIANT: Each edge in a radial ring must have at most 2 distinct
//! endpoint vertices (1 for geometric self-loops, 2 for normal edges).

use crate::b_rep::EntityBitset;
use crate::b_rep::TopologyArena;
use crate::queries::walk::walk_radial_iter;
use forge_core::KernelError;

pub fn validate_vertex_continuity(arena: &TopologyArena) -> Result<(), KernelError> {
    let mut checked_halfedges = EntityBitset::for_half_edges(arena);

    for (he_id, he_data) in arena.iter_half_edges() {
        if checked_halfedges.contains(he_id.index())? {
            continue;
        }

        checked_halfedges.insert(he_id.index())?;

        let edge_id = he_data.edge();

        let mut endpoints = EntityBitset::for_vertices(arena);
        for curr_result in walk_radial_iter(arena, he_id)? {
            let curr = curr_result?;
            checked_halfedges.insert(curr.index())?;
            let curr_data = arena.get_half_edge(curr)?;
            let next_data = arena.get_half_edge(curr_data.next())?;
            endpoints.insert(curr_data.origin().index())?;
            endpoints.insert(next_data.origin().index())?;
        }

        if endpoints.count() > 2 {
            return Err(KernelError::TopologyViolation {
                err: forge_core::TopologyError::BrokenLoop {
                    starting_halfedge: he_id.index(),
                    face_index: he_data.face().index(),
                },
                context: Some(forge_core::ErrorContext {
                    scope: forge_core::ErrorScope::Entity {
                        entity_kind: "Edge".to_string(),
                        index: edge_id.index(),
                    },
                    suggested_fixes: Vec::new(),
                    detail: format!(
                        "Edge {} has {} distinct endpoint vertices (expected 1 or 2)",
                        edge_id.index(),
                        endpoints.count()
                    ),
                }),
            });
        }
    }
    Ok(())
}
