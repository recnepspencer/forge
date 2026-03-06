//! Radial edge entity consistency validator.
//!
//! INVARIANT: Every halfedge in a `.radial_next()` ring must reference
//! the same `EdgeId`.

use crate::b_rep::EntityBitset;
use crate::b_rep::TopologyArena;
use crate::queries::walk::walk_radial_iter;
use forge_core::KernelError;

pub fn validate_radial_edge_consistency(arena: &TopologyArena) -> Result<(), KernelError> {
    let mut checked = EntityBitset::for_half_edges(arena);

    for (start_he, start_data) in arena.iter_half_edges() {
        if checked.contains(start_he.index())? {
            continue;
        }
        checked.insert(start_he.index())?;

        let expected_edge = start_data.edge();
        for curr_result in walk_radial_iter(arena, start_he)? {
            let curr = curr_result?;
            if curr == start_he {
                continue;
            }
            checked.insert(curr.index())?;
            let curr_data = arena.get_half_edge(curr)?;

            if curr_data.edge() != expected_edge {
                return Err(KernelError::TopologyViolation {
                    err: forge_core::TopologyError::RadialEdgeInconsistency {
                        halfedge_index: curr.index(),
                        actual_edge: curr_data.edge().index(),
                        seed_halfedge_index: start_he.index(),
                        expected_edge: expected_edge.index(),
                    },
                    context: Some(forge_core::ErrorContext {
                        scope: forge_core::ErrorScope::Entity {
                            entity_kind: "HalfEdge".to_string(),
                            index: curr.index(),
                        },
                        suggested_fixes: Vec::new(),
                        detail: format!(
                            "Radial ring edge-entity inconsistency: he[{}].edge = {} \
                             but ring seed he[{}].edge = {}. All members of a radial \
                             ring must reference the same geometric edge.",
                            curr.index(),
                            curr_data.edge().index(),
                            start_he.index(),
                            expected_edge.index(),
                        ),
                    }),
                });
            }
        }
    }
    Ok(())
}
