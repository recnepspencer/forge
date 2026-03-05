//! Radial ring closure validator.
//!
//! INVARIANT: Every halfedge must belong to a closed `.radial_next()` cycle.

use crate::b_rep::TopologyArena;
use forge_core::KernelError;

pub(crate) fn validate_radial_rings(arena: &TopologyArena) -> Result<(), KernelError> {
    for (start_he, start_data) in arena.iter_half_edges() {
        // Sentinel detection: radial_next pointing to DANGLING means
        // the halfedge was never properly wired into a radial ring.
        if start_data.radial_next() == crate::handles::HalfEdgeId::DANGLING {
            return Err(super::vf(
                "radial_ring_closure",
                format!(
                    "HE[{}].radial_next is DANGLING (u32::MAX) — halfedge was never wired. \
                     Edge: {}, Face: {}, Origin: {}",
                    start_he.index(),
                    start_data.edge().index(),
                    start_data.face().index(),
                    start_data.origin().index(),
                ),
            ));
        }

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
