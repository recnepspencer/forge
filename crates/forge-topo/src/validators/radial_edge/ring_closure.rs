//! Radial ring closure validator.
//!
//! INVARIANT: Every halfedge must belong to a closed `.radial_next()` cycle.

use crate::b_rep::TopologyArena;
use crate::queries::walk::walk_radial_iter;
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

        for he_result in walk_radial_iter(arena, start_he)? {
            he_result.map_err(|_| KernelError::TopologyViolation {
                err: forge_core::TopologyError::MissingTwin {
                    halfedge_index: start_he.index(),
                },
                context: None,
            })?;
        }
    }
    Ok(())
}
