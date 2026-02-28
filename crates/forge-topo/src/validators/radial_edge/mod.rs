//! Radial-edge invariant validators (NMT core).
//!
//! DOMAIN: Radial cycle closure, uniqueness, neighbor consistency,
//! ordering determinism, and edge use-count vs state agreement.
//!
//! VALIDATORS (from validators.md §3):
//! - ValidateRadialCycleClosure
//! - ValidateRadialCycleUniqueness
//! - ValidateRadialNeighborConsistency
//! - ValidateRadialOrderingDeterminism
//! - ValidateEdgeUseCountMatchesEdgeState
//! - ValidateNoBrokenRadialSplices
//!
//! DEPENDENCIES: `arena`, `handles`, `queries::radial`

use crate::b_rep::TopologyArena;
use crate::b_rep::EntityBitset;
use forge_core::KernelError;

/// Validate radial rings: every halfedge must belong to a closed `.radial_next()` cycle.
pub(crate) fn validate_radial_rings(arena: &TopologyArena) -> Result<(), KernelError> {
    for (start_he, _) in arena.iter_half_edges() {
        let mut current_he = start_he;
        let mut count = 0;
        let limit = 100_000;
        loop {
            let data =
                arena
                    .get_half_edge(current_he)
                    .map_err(|_| KernelError::TopologyViolation {
                        err: forge_core::TopologyError::MissingTwin {
                            halfedge_index: current_he.index(),
                        },
                        context: None,
                    })?;
            current_he = data.radial_next();
            count += 1;
            if current_he == start_he {
                break;
            }
            if count > limit {
                return Err(KernelError::TopologyViolation {
                    err: forge_core::TopologyError::LoopCorruption {
                        walk_kind: "RadialRing".to_string(),
                        seed_index: start_he.index(),
                        last_visited_index: current_he.index(),
                        steps_taken: count,
                        entity_bound: limit,
                    },
                    context: Some(forge_core::ErrorContext {
                        scope: forge_core::ErrorScope::Entity {
                            entity_kind: "HalfEdge".to_string(),
                            index: start_he.index(),
                        },
                        suggested_fixes: vec![],
                        detail: "Radial ring failed to cycle back to start within limit".into(),
                    }),
                });
            }
        }
    }
    Ok(())
}

/// Validate radial ring edge-entity consistency: every halfedge in a
/// `.radial_next()` ring must reference the same `EdgeId`.
///
/// A radial ring represents all face-uses of a single geometric edge.
/// If two halfedges in the same ring disagree on which edge entity they
/// belong to, the topology is structurally corrupt — the ring conflates
/// two distinct geometric edges into one cycle.
///
/// This is a Tier 1a invariant (same level as radial ring closure).
pub fn validate_radial_edge_consistency(arena: &TopologyArena) -> Result<(), KernelError> {
    let mut checked = EntityBitset::for_half_edges(arena);

    for (start_he, start_data) in arena.iter_half_edges() {
        if checked.contains(start_he.index())? {
            continue;
        }
        checked.insert(start_he.index())?;

        let expected_edge = start_data.edge();
        let mut curr = start_data.radial_next();

        while curr != start_he {
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

            curr = curr_data.radial_next();
        }
    }
    Ok(())
}
