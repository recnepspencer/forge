//! Tolerant vertex coalescence — the sliver prevention mechanism.
//!
//! DOMAIN: When a new intersection point lands within an existing vertex's
//! tolerance sphere, snap or coalesce them into one vertex. This prevents
//! micro-sliver edges that collapse stitching after deep boolean chains.
//!
//! Every coalescence is recorded via `DecisionSink`. This makes the operation
//! auditable, overridable, and replayable (Doctrine D2).
//!
//! DEPENDENCIES: `forge-geom` (VertexGeom),
//!               `forge-core` (DecisionSink, DecisionTier)

use forge_core::tracing::sink::DecisionSink;
use forge_core::DecisionTier;
use forge_geom::facade::VertexGeom;
use forge_topo::handles::VertexId;

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
    /// and tolerance in the geometry store.
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
/// Uses generics (`<S: DecisionSink>`) for zero-cost monomorphization on
/// this hot path. The sink records typed tolerance decisions — callers
/// describe what happened, the sink handles ID assignment and storage.
///
/// Decision flow:
/// 1. Compute gap = euclidean distance from candidate to existing vertex.
/// 2. If `gap < existing_tolerance`: snap (candidate is inside the tolerance
///    sphere — existing vertex absorbs it). Record `record_tolerance_snap`.
/// 3. If `gap < coalescence_threshold`: coalesce (both positions merge with
///    RSS-combined tolerance). Record `record_tolerance_snap` with `PolicyApplied`.
/// 4. Otherwise: new vertex (no recording — deterministic, zero ambiguity).
pub fn snap_or_coalesce_vertex<S: DecisionSink>(
    candidate_pos: [f64; 3],
    candidate_tol: f64,
    existing: VertexId,
    existing_pos: [f64; 3],
    existing_tol: f64,
    coalescence_threshold: f64,
    sink: &mut S,
) -> CoalescenceResult {
    let gap = forge_geom::facade::distance(&candidate_pos, &existing_pos);

    if gap < existing_tol {
        sink.record_tolerance_snap(existing.index(), gap, existing_tol, DecisionTier::Resolved);
        return CoalescenceResult::Snapped { existing };
    }

    if gap < coalescence_threshold {
        let merged_tolerance = VertexGeom::coalesced_tolerance(existing_tol, candidate_tol);

        sink.record_tolerance_snap(
            existing.index(),
            gap,
            coalescence_threshold,
            DecisionTier::PolicyApplied,
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
    use crate::context::ModelingContext;

    #[test]
    fn snap_when_inside_tolerance_sphere() {
        let mut ctx = ModelingContext::new();
        let v = VertexId::new(0, 0);
        let existing_pos = [0.0, 0.0, 0.0];
        let existing_tol = 1e-6;

        let result = snap_or_coalesce_vertex(
            [1e-8, 0.0, 0.0],
            1e-6,
            v,
            existing_pos,
            existing_tol,
            1e-4,
            &mut ctx,
        );

        assert!(matches!(result, CoalescenceResult::Snapped { .. }));
        assert_eq!(ctx.get_decision_count(), 1);
    }

    #[test]
    fn coalesce_when_near_but_outside_tolerance() {
        let mut ctx = ModelingContext::new();
        let v = VertexId::new(0, 0);
        let existing_pos = [0.0, 0.0, 0.0];
        let existing_tol = 1e-10;

        let result = snap_or_coalesce_vertex(
            [1e-6, 0.0, 0.0],
            1e-10,
            v,
            existing_pos,
            existing_tol,
            1e-4,
            &mut ctx,
        );

        match result {
            CoalescenceResult::Coalesced {
                merged_tolerance,
                gap,
                ..
            } => {
                assert!(merged_tolerance > 1e-10);
                assert!((gap - 1e-6).abs() < 1e-12);
            }
            other => panic!("Expected Coalesced, got {:?}", other),
        }
        assert_eq!(ctx.get_decision_count(), 1);
    }

    #[test]
    fn new_vertex_when_far_away() {
        let mut ctx = ModelingContext::new();
        let v = VertexId::new(0, 0);
        let existing_pos = [0.0, 0.0, 0.0];
        let existing_tol = 1e-10;

        let result = snap_or_coalesce_vertex(
            [1.0, 0.0, 0.0],
            1e-10,
            v,
            existing_pos,
            existing_tol,
            1e-4,
            &mut ctx,
        );

        assert!(matches!(result, CoalescenceResult::NewVertex));
        assert_eq!(ctx.get_decision_count(), 0);
    }

    #[test]
    fn coalesced_tolerance_exceeds_both_inputs() {
        let mut ctx = ModelingContext::new();
        let v = VertexId::new(0, 0);
        let existing_pos = [0.0, 0.0, 0.0];
        let existing_tol = 5e-9;

        let result = snap_or_coalesce_vertex(
            [1e-6, 0.0, 0.0],
            5e-9,
            v,
            existing_pos,
            existing_tol,
            1e-4,
            &mut ctx,
        );

        match result {
            CoalescenceResult::Coalesced {
                merged_tolerance, ..
            } => {
                assert!(merged_tolerance > 5e-9);
            }
            other => panic!("Expected Coalesced, got {:?}", other),
        }
    }
}
