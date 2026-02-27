//! Tolerant vertex coalescence — the sliver prevention mechanism.
//!
//! DOMAIN: When a new intersection point lands within an existing vertex's
//! tolerance sphere, snap or coalesce them into one vertex. This prevents
//! micro-sliver edges that collapse stitching after deep boolean chains.
//!
//! Every coalescence is a `TracedDecision`. This makes the operation
//! auditable, overridable, and replayable (Doctrine D2).
//!
//! DEPENDENCIES: `forge-kernel::geometry_state` (GeometryState),
//!               `forge-kernel::core` (ModelingContext),
//!               `forge-core` (tracing types)

use forge_core::{DecisionKind, DecisionTier, ToleranceProvider};
use forge_core::policy::PolicyKind;
use forge_geom::primitives::vertex_geom::VertexGeom;
use forge_topo::handles::VertexId;

// Removed geometry_state dependency
use crate::core::ModelingContext;

/// Result of attempting to snap or coalesce a candidate vertex position
/// against an existing vertex.
#[derive(Debug, Clone)]
pub enum CoalescenceResult {
    /// Candidate fell within the existing vertex's tolerance sphere.
    /// Use the existing vertex; no new vertex needed.
    Snapped {
        /// The existing vertex that absorbs the candidate.
        existing: VertexId,
    },

    /// Candidate was near but not inside the tolerance sphere.
    /// Both positions merged into one vertex with a wider (RSS-combined)
    /// tolerance. The caller must update the existing vertex's position
    /// and tolerance in the `GeometryState`.
    Coalesced {
        /// The existing vertex (now with merged position and wider tolerance).
        merged: VertexId,
        /// The new RSS-combined tolerance for the merged vertex.
        merged_tolerance: f64,
        /// The gap that was closed by coalescence.
        gap: f64,
    },

    /// Candidate is far enough from the existing vertex to create a new one.
    NewVertex,
}

/// Classify a candidate vertex position against an existing vertex.
///
/// Decision flow:
/// 1. Compute gap = euclidean distance from candidate to existing vertex.
/// 2. If `gap < existing_tolerance`: snap (candidate is inside the tolerance
///    sphere — existing vertex absorbs it). Log `Tier::Resolved`.
/// 3. If `gap < coalescence_threshold`: coalesce (both positions merge with
///    RSS-combined tolerance). Log `Tier::PolicyApplied`.
/// 4. Otherwise: new vertex (no coalescence).
///
/// The `coalescence_threshold` should come from `ToleranceConfig`. A typical
/// value is `sliver_area_min.sqrt()` — the linear dimension below which a
/// connecting edge would create a face whose area is below the sliver threshold.
pub fn snap_or_coalesce_vertex(
    candidate_pos: [f64; 3],
    candidate_tol: f64,
    existing: VertexId,
    existing_pos: [f64; 3],
    existing_tol: f64,
    ctx: &mut ModelingContext,
    coalescence_threshold: f64,
) -> CoalescenceResult {

    let dx = candidate_pos[0] - existing_pos[0];
    let dy = candidate_pos[1] - existing_pos[1];
    let dz = candidate_pos[2] - existing_pos[2];
    let gap = (dx * dx + dy * dy + dz * dz).sqrt();

    if gap < existing_tol {
        ctx.log_decision(
            DecisionKind::PolicyApplied {
                policy: PolicyKind::CoincidentGeometry,
                default_used: true,
            },
            DecisionTier::Resolved,
            candidate_pos,
            gap,
            existing_tol,
        );
        return CoalescenceResult::Snapped { existing };
    }

    if gap < coalescence_threshold {
        let merged_tolerance = VertexGeom::coalesced_tolerance(existing_tol, candidate_tol);

        ctx.log_decision(
            DecisionKind::PolicyApplied {
                policy: PolicyKind::CoincidentGeometry,
                default_used: true,
            },
            DecisionTier::PolicyApplied,
            candidate_pos,
            gap,
            coalescence_threshold,
        );

        return CoalescenceResult::Coalesced {
            merged: existing,
            merged_tolerance,
            gap,
        };
    }

    CoalescenceResult::NewVertex
}

#[cfg(test)]
mod tests {
    use super::*;
    use forge_core::ToleranceProvider;
    use crate::geometry_state::GeometryState;

    #[test]
    fn snap_when_inside_tolerance_sphere() {
        let mut geom = GeometryState::new();
        let mut ctx = ModelingContext::new();

        let v = VertexId::from_raw_parts(0, 0);
        let existing_pos = [0.0, 0.0, 0.0];
        let existing_tol = 1e-6;

        let result = snap_or_coalesce_vertex(
            [1e-8, 0.0, 0.0],
            1e-6,
            v,
            existing_pos,
            existing_tol,
            &mut ctx,
            1e-4,
        );

        assert!(matches!(result, CoalescenceResult::Snapped { .. }));
        assert_eq!(ctx.get_decision_count(), 1);
    }

    #[test]
    fn coalesce_when_near_but_outside_tolerance() {
        let mut geom = GeometryState::new();
        let mut ctx = ModelingContext::new();

        let v = VertexId::from_raw_parts(0, 0);
        let existing_pos = [0.0, 0.0, 0.0];
        let existing_tol = 1e-10;

        let result = snap_or_coalesce_vertex(
            [1e-6, 0.0, 0.0],
            1e-10,
            v,
            existing_pos,
            existing_tol,
            &mut ctx,
            1e-4,
        );

        match result {
            CoalescenceResult::Coalesced { merged_tolerance, gap, .. } => {
                assert!(merged_tolerance > 1e-10);
                assert!((gap - 1e-6).abs() < 1e-12);
            }
            other => panic!("Expected Coalesced, got {:?}", other),
        }
        assert_eq!(ctx.get_decision_count(), 1);
    }

    #[test]
    fn new_vertex_when_far_away() {
        let mut geom = GeometryState::new();
        let mut ctx = ModelingContext::new();

        let v = VertexId::from_raw_parts(0, 0);
        let existing_pos = [0.0, 0.0, 0.0];
        let existing_tol = 1e-10;

        let result = snap_or_coalesce_vertex(
            [1.0, 0.0, 0.0],
            1e-10,
            v,
            existing_pos,
            existing_tol,
            &mut ctx,
            1e-4,
        );

        assert!(matches!(result, CoalescenceResult::NewVertex));
        assert_eq!(ctx.get_decision_count(), 0);
    }

    #[test]
    fn coalesced_tolerance_exceeds_both_inputs() {
        let mut geom = GeometryState::new();
        let mut ctx = ModelingContext::new();

        let v = VertexId::from_raw_parts(0, 0);
        let existing_pos = [0.0, 0.0, 0.0];
        let existing_tol = 5e-9;

        let result = snap_or_coalesce_vertex(
            [1e-6, 0.0, 0.0],
            5e-9,
            v,
            existing_pos,
            existing_tol,
            &mut ctx,
            1e-4,
        );

        match result {
            CoalescenceResult::Coalesced { merged_tolerance, .. } => {
                assert!(merged_tolerance > 5e-9);
            }
            other => panic!("Expected Coalesced, got {:?}", other),
        }
    }
}
