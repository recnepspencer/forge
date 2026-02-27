//! Zero-length edge detection.
//!
//! DOMAIN: Validate that no edge has length below its endpoint vertex tolerances.

use forge_core::{KernelError, ToleranceProvider};
use forge_math::linalg::{sub, norm};
use forge_topo::arena::TopologyArena;
use forge_topo::handles::VertexId;
use forge_topo::topology::bitset::EntityBitset;

/// Validate that no edge has length below the max of its endpoint tolerances.
pub fn validate_zero_length_edges(
    arena: &TopologyArena,
    position_fn: &dyn Fn(VertexId) -> Option<[f64; 3]>,
    tolerance_provider: &dyn ToleranceProvider,
) -> Result<(), KernelError> {
    let mut checked_edges = EntityBitset::for_edges(arena);

    for (he_id, he_data) in arena.iter_half_edges() {
        let edge_id = he_data.edge();
        if !checked_edges.insert(edge_id.index())? {
            continue;
        }

        let origin = he_data.origin();
        let next_data = arena.get_half_edge(he_data.next())?;
        let target = next_data.origin();

        if origin == target {
            continue;
        }

        let origin_pos = position_fn(origin);
        let target_pos = position_fn(target);

        if let (Some(p0), Some(p1)) = (origin_pos, target_pos) {
            let length = norm(sub(p1, p0));
            let edge_length_threshold = tolerance_provider
                .vertex_tolerance(origin.index(), origin.generation())
                .max(tolerance_provider.vertex_tolerance(target.index(), target.generation()));

            if length < edge_length_threshold {
                return Err(KernelError::TopologyViolation {
                    err: forge_core::TopologyError::ZeroLengthEdge {
                        halfedge_index: he_id.index(),
                        computed_length: length,
                        threshold: edge_length_threshold,
                    },
                    context: Some(forge_core::ErrorContext {
                        scope: forge_core::ErrorScope::Entity {
                            entity_kind: "HalfEdge".to_string(),
                            index: he_id.index(),
                        },
                        suggested_fixes: Vec::new(),
                        detail: format!(
                            "Edge {} length {:.2e} is below per-entity threshold {:.2e}",
                            he_id.index(), length, edge_length_threshold
                        ),
                    }),
                });
            }
        }
    }
    Ok(())
}
