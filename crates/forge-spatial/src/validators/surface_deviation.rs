//! Vertex-on-surface deviation validation.
//!
//! DOMAIN: Checks that every vertex of each face lies on that face's
//! supporting surface plane within tolerance. Uses `Plane::signed_distance`
//! from `worth-geom` — no ad-hoc math.
//!
//! DEPENDENCIES: forge-core (KernelError, ToleranceProvider),
//!               worth-geom (Plane, signed_distance),
//!               forge-topo (TopologyArena, handles, FaceEdgeIterator)

use forge_core::{KernelError, ToleranceProvider};
use worth_geom::facade::{signed_distance, Plane};
use forge_topo::b_rep::TopologyArena;
use forge_topo::handles::{FaceId, VertexId};
use forge_topo::traverse::FaceEdgeIterator;

/// Validate that every vertex of each face lies within tolerance of the
/// face's supporting plane.
///
/// For each face that has a plane binding, walks the outer loop and checks
/// `signed_distance(plane, vertex_position)`. If the absolute deviation
/// exceeds the tolerance from `ToleranceProvider`, the face/vertex is
/// reported as a violation.
///
/// This catches:
/// - Corrupt geometry where vertices drift off their supporting plane
/// - Numerical precision loss from chained operations
/// - Import/export roundtrip errors
pub fn validate_surface_deviation(
    arena: &TopologyArena,
    position_fn: &dyn Fn(VertexId) -> Option<[f64; 3]>,
    plane_fn: &dyn Fn(FaceId) -> Option<Plane>,
    tolerance_provider: &dyn ToleranceProvider,
) -> Result<(), KernelError> {
    for (face_id, _) in arena.iter_faces() {
        let plane = match plane_fn(face_id) {
            Some(p) => p,
            None => continue, // Skip faces without plane bindings
        };

        for he_res in FaceEdgeIterator::new(arena, face_id)? {
            let he_id = he_res?;
            let he = arena.get_half_edge(he_id)?;
            let v = he.origin();

            let tol = tolerance_provider.vertex_tolerance(v.index(), v.generation());

            let pos = match position_fn(v) {
                Some(p) => p,
                None => continue, // Skip vertices without positions
            };

            let deviation = signed_distance(&plane, &pos).abs();

            if deviation > tol {
                return Err(KernelError::TopologyViolation {
                    err: forge_core::TopologyError::VertexOffSurface {
                        vertex_index: v.index(),
                        face_index: face_id.index(),
                        deviation,
                        tolerance: tol,
                    },
                    context: Some(forge_core::ErrorContext {
                        scope: forge_core::ErrorScope::Entity {
                            entity_kind: "Vertex".to_string(),
                            index: v.index(),
                        },
                        suggested_fixes: Vec::new(),
                        detail: format!(
                            "Vertex {} on face {} deviates {:.2e} from surface (tol {:.2e})",
                            v.index(),
                            face_id.index(),
                            deviation,
                            tol
                        ),
                    }),
                });
            }
        }
    }

    Ok(())
}
