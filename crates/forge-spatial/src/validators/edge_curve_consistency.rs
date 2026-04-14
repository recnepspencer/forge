//! Edge-curve geometric consistency validation.
//!
//! DOMAIN: Validates that every edge's curve binding is geometrically
//! consistent with its endpoint vertex positions. Checks:
//!   1. Curve origin matches the edge's origin vertex position.
//!   2. Curve direction is aligned with the vertex displacement vector.
//!   3. Curve endpoint (via `point_at`) matches the destination vertex.
//!
//! DEPENDENCIES: worth-math (linalg), worth-geom (CurveKind),
//!               forge-topo (arena, handles, traversal),
//!               forge-core (KernelError, ToleranceProvider).
//! INVARIANTS: No topology mutation. Requires position + curve callbacks.

use forge_core::{KernelError, ToleranceProvider};
use worth_geom::facade::CurveKind;
use worth_math::linalg;
use forge_topo::b_rep::TopologyArena;
use forge_topo::handles::{EdgeId, VertexId};
use forge_topo::queries::edge_endpoint_ids;

/// Validate that every edge's curve binding is geometrically consistent
/// with its vertex endpoint positions.
///
/// **Checks performed** (per edge):
/// 1. **Origin match**: `curve.point_at(0)` ≈ origin vertex position.
/// 2. **Direction alignment**: `curve.tangent_at(0)` is collinear with
///    the displacement from origin to destination (dot ≈ 1.0).
/// 3. **Destination match**: `curve.point_at(edge_length)` ≈ destination
///    vertex position — confirms direction + length coherence.
///
/// Uses `worth_math::linalg` for all metric computations.
/// Tolerance is drawn from `ToleranceProvider`.
pub fn validate_edge_curve_consistency(
    arena: &TopologyArena,
    position_fn: &dyn Fn(VertexId) -> Option<[f64; 3]>,
    curve_fn: &dyn Fn(EdgeId) -> Option<CurveKind>,
    tolerance_provider: &dyn ToleranceProvider,
) -> Result<(), KernelError> {
    let tolerance = tolerance_provider.global_default();

    for (edge_id, edge) in arena.iter_edges() {
        let curve = match curve_fn(edge_id) {
            Some(c) => c,
            None => continue, // completeness is checked by GeometryCompleteness
        };

        let he_id = edge.half_edge();
        let (v_origin, v_dest) = edge_endpoint_ids(arena, he_id)?;

        let p_origin = match position_fn(v_origin) {
            Some(p) => p,
            None => continue,
        };
        let p_dest = match position_fn(v_dest) {
            Some(p) => p,
            None => continue,
        };

        // ── Check 1: Curve origin matches origin vertex ─────────────
        let curve_origin = curve.point_at(0.0);
        let origin_dist = linalg::norm(linalg::sub(curve_origin, p_origin));
        if origin_dist > tolerance {
            return Err(KernelError::InternalError {
                message: format!(
                    "Edge {}: curve origin deviation {:.2e} exceeds tolerance {:.2e}",
                    edge_id, origin_dist, tolerance
                ),
                context: None,
            });
        }

        // ── Check 2: Curve tangent aligned with vertex displacement ──
        let displacement = linalg::sub(p_dest, p_origin);
        let edge_length = linalg::norm(displacement);

        if edge_length > tolerance {
            let expected_dir = linalg::normalize_checked(displacement);
            let curve_tangent = curve.tangent_at(0.0);
            let tangent_dir = linalg::normalize_checked(curve_tangent);

            if let (Some(expected), Some(actual)) = (expected_dir, tangent_dir) {
                let alignment = linalg::dot(expected, actual);
                if (alignment - 1.0).abs() > tolerance {
                    return Err(KernelError::InternalError {
                        message: format!(
                            "Edge {}: curve tangent misaligned with edge displacement (dot={:.6}, expected 1.0)",
                            edge_id, alignment
                        ),
                        context: None,
                    });
                }
            }

            // ── Check 3: Curve destination matches dest vertex ───────
            let curve_dest = curve.point_at(edge_length);
            let dest_dist = linalg::norm(linalg::sub(curve_dest, p_dest));
            if dest_dist > tolerance {
                return Err(KernelError::InternalError {
                    message: format!(
                        "Edge {}: curve destination deviation {:.2e} exceeds tolerance {:.2e}",
                        edge_id, dest_dist, tolerance
                    ),
                    context: None,
                });
            }
        }
    }

    Ok(())
}
