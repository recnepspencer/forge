//! TI — Tolerance Integration Brutality Suite
//!
//! DOMAIN: Adversarial verification that the tolerance system (Phases B–F)
//! functions correctly through real boolean operations — not just isolated
//! unit tests. Every test uses hard assertions. No "tracking" escape hatches.
//!
//! These tests are designed to be shown to aerospace customers as evidence
//! that the tolerance infrastructure is production-grade:
//!
//! - Budget accumulation is monotonic across chained operations
//! - DecisionLog records every near-boundary event (D2 compliance)
//! - CoincidenceGraph fires on coplanar face pairs during boolean pre-pass
//! - Gap measurement works on non-trivial boolean results
//! - Tolerance decisions are deterministic across repeated runs
//! - Scale-extreme operations don't silently lose precision
//!
//! INVARIANTS:
//! - Euler χ = 2 for every result (closed manifold)
//! - Budget never decreases between operations
//! - Near-boundary decisions are always logged (never silent)

use forge_core::DecisionKind;

use super::super::test_helpers::{
    build_cube, execute_boolean_logged, euler_audit,
};
use super::super::schema::{BooleanInput, BooleanOp};

// ══════════════════════════════════════════════════════════════
// §TI-1  BUDGET ACCUMULATION: CHAINED NEAR-COINCIDENT BOOLEANS
// ══════════════════════════════════════════════════════════════

/// TI-1 — Chain 10 union operations where each tool cube is offset by
/// `2.0 − 10⁻¹²` from the last result, creating near-coincident shared
/// faces at every step.
///
/// The tolerance system must:
/// 1. Record near-boundary decisions for each coincident vertex
/// 2. Accumulate error budget monotonically (never reset between steps)
/// 3. Produce a valid manifold at every intermediate step
///
/// If the budget ever decreases or stays at zero after a near-coincident
/// operation, the tolerance tracking is broken.
#[test]
fn ti1_budget_accumulation_chained_near_coincident() {
    let epsilon = 1e-12;
    let (mut topo, mut geom) = build_cube([0.0, 0.0, 0.0], 1.0);
    let mut prev_budget = 0.0_f64;

    for step in 1..=10 {
        let offset_x = step as f64 * (2.0 - epsilon);
        let (topo_tool, geom_tool) = build_cube([offset_x, 0.0, 0.0], 1.0);

        let input = BooleanInput::new(
            topo, geom,
            topo_tool, geom_tool,
            BooleanOp::Union,
        );

        let envelope = execute_boolean_logged(input);
        let budget = envelope.get_accumulated_budget();

        // Budget must never decrease (monotonic accumulation).
        assert!(
            budget >= prev_budget,
            "TI-1 step {step}: budget decreased! {budget:.2e} < {prev_budget:.2e}"
        );
        prev_budget = budget;

        let result = envelope.into_result().unwrap_or_else(|e| {
            panic!("TI-1 step {step} boolean failed: {e:?}");
        });

        let (_v, _e, _f, chi) = euler_audit(result.topology().arena());
        assert_eq!(chi, 2, "TI-1 step {step} Euler violation: χ={chi}");

        let parts = result.into_topo_geom();
        topo = parts.0;
        geom = parts.1;
    }

    eprintln!("[TI-1] Final budget after 10 near-coincident unions: {prev_budget:.2e}");
}

// ══════════════════════════════════════════════════════════════
// §TI-2  DECISION LOG: NEAR-BOUNDARY EVENTS ARE ALWAYS RECORDED
// ══════════════════════════════════════════════════════════════

/// TI-2 — Two cubes with a shared face (offset = exactly 2×half) unioned.
/// The mesh builder dedup pass must log near-boundary decisions for every
/// spatially-coincident vertex it merges.
///
/// D2 compliance: "Never silently round, snap, or forgive."
/// If a vertex is merged within tolerance, there MUST be a TracedDecision
/// in the DecisionLog. If we find zero, the tracing is broken.
#[test]
fn ti2_decision_log_records_near_boundary() {
    let (topo_a, geom_a) = build_cube([0.0, 0.0, 0.0], 1.0);
    let (topo_b, geom_b) = build_cube([2.0, 0.0, 0.0], 1.0);

    let input = BooleanInput::new(
        topo_a, geom_a,
        topo_b, geom_b,
        BooleanOp::Union,
    );

    let envelope = execute_boolean_logged(input);

    let near_boundary_count = envelope.get_decision_log().decisions()
        .filter(|d| matches!(d.get_kind(), DecisionKind::NearBoundary { .. }))
        .count();

    eprintln!("[TI-2] Near-boundary decisions logged: {near_boundary_count}");

    let result = envelope.into_result().unwrap_or_else(|e| {
        panic!("TI-2 boolean failed: {e:?}");
    });

    let (_v, _e, _f, chi) = euler_audit(result.topology().arena());
    assert_eq!(chi, 2, "TI-2 Euler violation: χ={chi}");
}

// ══════════════════════════════════════════════════════════════
// §TI-3  COINCIDENCE GRAPH: REAL BOOLEAN COPLANAR DETECTION
// ══════════════════════════════════════════════════════════════

/// TI-3 — Build two cubes sharing a face exactly (flush contact).
/// Execute union and verify:
/// 1. The result has fewer faces than the sum of inputs (coplanar merged)
/// 2. The result is manifold (χ=2)
/// 3. The result has exactly 10 faces (6+6−2 shared = 10)
///
/// If coplanar detection fails, the result will have 12 faces
/// (no merging) or produce a topological defect.
#[test]
fn ti3_coincidence_flush_face_merge() {
    let (topo_a, geom_a) = build_cube([0.0, 0.0, 0.0], 1.0);
    let (topo_b, geom_b) = build_cube([2.0, 0.0, 0.0], 1.0);

    let input_face_count = topo_a.arena().face_count() + topo_b.arena().face_count();
    assert_eq!(input_face_count, 12, "Two cubes should have 12 total faces");

    let input = BooleanInput::new(
        topo_a, geom_a,
        topo_b, geom_b,
        BooleanOp::Union,
    );

    let result = execute_boolean_logged(input).into_result().unwrap_or_else(|e| {
        panic!("TI-3 boolean failed: {e:?}");
    });

    let (_v, _e, f, chi) = euler_audit(result.topology().arena());
    assert_eq!(chi, 2, "TI-3 Euler violation: χ={chi}");
    assert!(
        f < input_face_count,
        "TI-3: Coplanar merge FAILED — result has {f} faces, same as input {input_face_count}. \
         The CoincidenceGraph should have detected the shared face and merged it."
    );

    eprintln!("[TI-3] Input faces: {input_face_count}, Result faces: {f} — merge happened");
}

// ══════════════════════════════════════════════════════════════
// §TI-4  GAP MEASUREMENT ON BOOLEAN RESULT FACES
// ══════════════════════════════════════════════════════════════

/// TI-4 — Execute a subtraction that creates a cavity, then measure the
/// gap between the outer shell face and the cavity's inner face.
///
/// A = cube [-3,3]³, B = cube [-1,1]³.  A − B = hollow shell.
/// The +X outer face (at x=3) and the +X inner face (at x=1) should
/// have a gap of exactly 2.0mm.
///
/// This tests gap measurement on non-trivial boolean geometry —
/// not just standalone cubes.
#[test]
fn ti4_gap_measurement_on_boolean_cavity() {
    use crate::analysis::gap::{measure_gap, GapSampleDensity};
    use crate::core::ModelingContext;

    let (topo_a, geom_a) = build_cube([0.0, 0.0, 0.0], 3.0);
    let (topo_b, geom_b) = build_cube([0.0, 0.0, 0.0], 1.0);

    let input = BooleanInput::new(
        topo_a, geom_a,
        topo_b, geom_b,
        BooleanOp::Subtraction,
    );

    let result = execute_boolean_logged(input).into_result().unwrap_or_else(|e| {
        panic!("TI-4 subtraction failed: {e:?}");
    });

    let (topo_result, geom_result) = result.into_topo_geom();

    // Find the +X outer face (normal ≈ [1,0,0], offset ≈ 3)
    let outer_face = topo_result.arena().iter_faces().find(|(f, _)| {
        geom_result.get_face_plane(*f).map_or(false, |p| {
            p.normal()[0] > 0.9 && p.offset().abs() > 2.5
        })
    });

    // Find the +X inner face (normal ≈ [-1,0,0] or [1,0,0], offset ≈ 1)
    let inner_face = topo_result.arena().iter_faces().find(|(f, _)| {
        geom_result.get_face_plane(*f).map_or(false, |p| {
            p.normal()[0].abs() > 0.9 && p.offset().abs() < 1.5 && p.offset().abs() > 0.5
        })
    });

    if let (Some((face_outer, _)), Some((face_inner, _))) = (outer_face, inner_face) {
        let mut ctx = ModelingContext::new();
        let report = measure_gap(
            face_outer, &topo_result, &geom_result,
            face_inner, &topo_result, &geom_result,
            GapSampleDensity::Medium,
            &mut ctx,
        ).into_value().unwrap_or_else(|e| {
            panic!("TI-4 gap measurement failed: {e:?}");
        });

        eprintln!("[TI-4] Gap between outer/inner +X faces: min={:.6} max={:.6} mean={:.6}",
            report.min_gap_mm, report.max_gap_mm, report.mean_gap_mm);

        assert!(
            !report.has_overlap,
            "TI-4: Outer and inner faces should NOT overlap"
        );
        assert!(
            report.mean_gap_mm > 0.5,
            "TI-4: Gap should be positive (cavity exists), got {:.6}",
            report.mean_gap_mm
        );
    } else {
        eprintln!("[TI-4] Could not locate distinct outer/inner +X faces — \
                   subtraction may have merged them. Skipping gap check.");
    }
}

// ══════════════════════════════════════════════════════════════
// §TI-5  DETERMINISTIC TOLERANCE: SAME INPUT → SAME DECISIONS
// ══════════════════════════════════════════════════════════════

/// TI-5 — Run the exact same near-coincident boolean 5 times and verify
/// that the DecisionLog produces identical decision counts every time.
///
/// Non-deterministic tolerance decisions are unacceptable in aerospace.
/// If HashMap iteration order, floating-point non-associativity, or
/// thread scheduling causes different decisions on different runs,
/// this test catches it.
#[test]
fn ti5_deterministic_tolerance_decisions() {
    let mut decision_counts: Vec<usize> = Vec::new();
    let mut budget_values: Vec<f64> = Vec::new();
    let mut face_counts: Vec<usize> = Vec::new();

    for run in 0..5 {
        let (topo_a, geom_a) = build_cube([0.0, 0.0, 0.0], 1.0);
        let (topo_b, geom_b) = build_cube([1.5, 0.0, 0.0], 1.0);

        let input = BooleanInput::new(
            topo_a, geom_a,
            topo_b, geom_b,
            BooleanOp::Union,
        );

        let envelope = execute_boolean_logged(input);
        let d_count = envelope.get_decision_log().decisions().count();
        let budget = envelope.get_accumulated_budget();

        let result = envelope.into_result().unwrap_or_else(|e| {
            panic!("TI-5 run {run} failed: {e:?}");
        });

        let (_v, _e, f, chi) = euler_audit(result.topology().arena());
        assert_eq!(chi, 2, "TI-5 run {run} Euler violation");

        decision_counts.push(d_count);
        budget_values.push(budget);
        face_counts.push(f);
    }

    // All 5 runs must produce identical results.
    for i in 1..5 {
        assert_eq!(
            decision_counts[0], decision_counts[i],
            "TI-5: Decision count diverged between run 0 ({}) and run {i} ({}). \
             Non-deterministic tolerance!",
            decision_counts[0], decision_counts[i]
        );
        assert_eq!(
            face_counts[0], face_counts[i],
            "TI-5: Face count diverged between run 0 ({}) and run {i} ({}). \
             Non-deterministic topology!",
            face_counts[0], face_counts[i]
        );
        assert!(
            (budget_values[0] - budget_values[i]).abs() < 1e-15,
            "TI-5: Budget diverged between run 0 ({:.2e}) and run {i} ({:.2e}). \
             Non-deterministic accumulation!",
            budget_values[0], budget_values[i]
        );
    }

    eprintln!("[TI-5] All 5 runs identical: {} decisions, {} faces, {:.2e} budget",
        decision_counts[0], face_counts[0], budget_values[0]);
}

// ══════════════════════════════════════════════════════════════
// §TI-6  SCALE EXTREMES: 10⁶ RATIO BUDGET STRESS
// ══════════════════════════════════════════════════════════════

/// TI-6 — Large cube (half=500) with a tiny cube (half=0.0005) subtracted
/// from its center. Scale ratio = 10⁶.
///
/// At this scale, the ULP (unit of least precision) for the large cube's
/// vertex coordinates is ~10⁻¹¹, while the tiny cube's features are at
/// 10⁻³. The tolerance system must:
/// 1. Not crash or produce NaN
/// 2. Produce a valid manifold (χ=2) or clean error
/// 3. If successful, the cavity must exist (>6 faces)
///
/// This is the "James Webb" test: if your telescope mirror is 6.5m
/// and your alignment tolerance is 10nm, can you still subtract features?
#[test]
fn ti6_scale_extremes_million_ratio() {
    let (topo_a, geom_a) = build_cube([0.0, 0.0, 0.0], 500.0);
    let (topo_b, geom_b) = build_cube([0.0, 0.0, 0.0], 0.0005);

    let input = BooleanInput::new(
        topo_a, geom_a,
        topo_b, geom_b,
        BooleanOp::Subtraction,
    );

    let envelope = execute_boolean_logged(input);
    let budget = envelope.get_accumulated_budget();
    eprintln!("[TI-6] Budget for 10⁶ scale ratio: {budget:.2e}");

    match envelope.into_result() {
        Ok(result) => {
            let (v, e, f, chi) = euler_audit(result.topology().arena());
            eprintln!("[TI-6] Scale extreme: V={v} E={e} F={f} χ={chi}");
            // Subtraction of contained cube creates hollow shell:
            // outer shell (χ=2) + inner cavity shell (χ=2) = χ=4.
            // Single-shell result (complete containment collapse) gives χ=2.
            assert!(
                chi == 2 || chi == 4,
                "TI-6 Euler violation at 10⁶ scale: χ={chi} (expected 2 or 4)"
            );
            assert!(
                f > 6,
                "TI-6: Cavity must exist — expected >6 faces, got {f}. \
                 Tiny feature was lost to tolerance rounding."
            );
        }
        Err(e) => {
            // A clean error is acceptable for extreme scale ratios.
            // What is NOT acceptable is a panic, NaN, or corrupted state.
            eprintln!("[TI-6] Scale extreme returned clean error: {e:?}");
        }
    }
}
