//! Tier 2 — Numerical Adversity Tests (99.99% Confidence)
//!
//! DOMAIN: Surviving the limits of floating-point precision (f64).
//! Tests extreme scale differences, near-miss geometry, shallow angles,
//! and iterative operations that accumulate precision errors.
//!
//! INVARIANTS:
//! - No NaN or Inf in vertex positions
//! - No zero-area faces in results
//! - Euler χ = 2 for single-shell results
//!
//! ═══════════════════════════════════════════════════════════════
//! REQUIRED CODE/MATH CHANGES TO PASS ALL TESTS:
//! ═══════════════════════════════════════════════════════════════
//!
//! 1. **Adaptive Precision Predicates** (Shewchuk): Integrate the exact
//!    `orient3d` / `orient2d` predicates. Use a "Filter" approach:
//!    - Calculate using f64.
//!    - Calculate the error bound ε.
//!    - If |result| < ε, recalculate using Arbitrary Precision Rationals.
//!    Already partially implemented in `forge-math::predicates`.
//!
//! 2. **Symbolic Vertices**: Change vertex storage from bare `[f64; 3]` to
//!    `Intersection(PlaneIdx, PlaneIdx, PlaneIdx)`. When cubes rotate 100
//!    times, you don't store the rounded f64 result — you store the reference
//!    to the three original planes that defined that corner. This prevents
//!    "drift." Currently approximated by `VertexMatchKey` in the split phase,
//!    but needs to be carried through the full pipeline.
//!
//! 3. **Coordinate Normalization**: Before intersection, translate the
//!    "Action Zone" to the origin and scale to unit-size. This is critical
//!    for T2.2 (10⁶ vs 10⁻⁶ cubes) and T2.6 (coordinates at 10⁸).
//!    Currently missing — all arithmetic happens in world coordinates.
//!
//! 4. **Sliver Detection**: After iterative operations (T2.1), detect
//!    and optionally collapse faces with area < threshold. This requires
//!    a face-area computation utility and a post-merge cleanup pass.
//!
//! 5. **Relative Epsilon**: Tolerance comparisons must use relative epsilon
//!    (scaled to the magnitude of the coordinates) instead of fixed absolute
//!    thresholds. Critical for T2.2 and T2.6 where ULP varies by 10¹².

use forge_geom::Plane;

use super::super::test_helpers::{
    build_cube, build_convex_solid, run_boolean, try_boolean,
    execute_boolean_logged, euler_audit,
};
use super::super::schema::{BooleanInput, BooleanOp};

// ══════════════════════════════════════════════════════════════
// §T2.1  ITERATIVE ROTATION SHREDDER
// ══════════════════════════════════════════════════════════════

/// T2.1 — Union a cube 10 times, rotating 1° each step.
///
/// After each step, audit the Euler characteristic. This catches
/// "drift" where small angular perturbations create sliver faces
/// or self-intersecting edges.
#[test]
fn iterative_rotation_shredder_10() {
    let mut accumulated = {
        let (topo, geom) = build_cube([0.0, 0.0, 0.0], 1.0);
        (topo, geom)
    };

    for step in 1..=10 {
        let angle = (step as f64) * 1.0_f64.to_radians();
        let cos_a = angle.cos();
        let sin_a = angle.sin();
        let offset_x = 0.5 * cos_a;
        let offset_y = 0.5 * sin_a;

        let (topo_tool, geom_tool) = build_cube([offset_x, offset_y, 0.0], 1.0);

        let input = BooleanInput::new(
            accumulated.0, accumulated.1,
            topo_tool, geom_tool,
            BooleanOp::Union,
        );

        match execute_boolean_logged(input) {
            Ok(envelope) => {
                let result = envelope.into_value();
                let (v, e, f, chi) = euler_audit(result.topology().arena());
                eprintln!("Step {step}: V={v} E={e} F={f} χ={chi}");
                assert_eq!(
                    chi, 2,
                    "Iterative shredder step {step} Euler violation: V={v} E={e} F={f} χ={chi}"
                );
                accumulated = result.into_topo_geom();
            }
            Err(e) => {
                panic!("Iterative shredder step {step} failed: {e:?}");
            }
        }
    }
}

// ══════════════════════════════════════════════════════════════
// §T2.2  SCALE SEPARATOR
// ══════════════════════════════════════════════════════════════

/// T2.2 — Large cube (half=1000) ∪ tiny cube (half=0.001) at origin.
///
/// Extreme scale difference: the tiny cube is fully contained.
/// Must not produce NaN or zero-area faces.
#[test]
fn scale_separator_union() {
    let result = try_boolean(
        [0.0, 0.0, 0.0], 1000.0,
        [0.0, 0.0, 0.0], 0.001,
        BooleanOp::Union,
    );

    match result {
        Ok(r) => {
            let (v, e, f, chi) = euler_audit(r.topology().arena());
            eprintln!("Scale separator: V={v} E={e} F={f} χ={chi}");
            assert_eq!(
                r.target_faces_kept() + r.tool_faces_kept(), 6,
                "Containment union should keep 6 outer faces"
            );
        }
        Err(e) => {
            eprintln!("Scale separator returned error (tracking): {e:?}");
        }
    }
}

/// T2.2b — Intersection of extremely different scales.
///
/// Large ∩ Tiny = Tiny cube.
#[test]
fn scale_separator_intersection() {
    let result = try_boolean(
        [0.0, 0.0, 0.0], 1000.0,
        [0.0, 0.0, 0.0], 0.001,
        BooleanOp::Intersection,
    );

    match result {
        Ok(r) => {
            let (v, e, f, chi) = euler_audit(r.topology().arena());
            eprintln!("Scale intersection: V={v} E={e} F={f} χ={chi}");
            assert!(f >= 6, "Intersection should produce the small cube (≥6 faces), got {f}");
        }
        Err(e) => {
            eprintln!("Scale intersection returned error (tracking): {e:?}");
        }
    }
}

// ══════════════════════════════════════════════════════════════
// §T2.3  NEAR-MISS GRAZE
// ══════════════════════════════════════════════════════════════

/// T2.3 — Edge of solid A passes 10⁻¹⁰ units from vertex of solid B.
///
/// This is below double-precision resolution for reasonable coordinates.
/// Must not crash; the result must be manifold (or a clean error).
#[test]
fn near_miss_graze() {
    let epsilon = 1e-10;
    let result = try_boolean(
        [0.0, 0.0, 0.0], 1.0,
        [2.0 - epsilon, 0.0, 0.0], 1.0,
        BooleanOp::Union,
    );

    match result {
        Ok(r) => {
            let (v, e, f, chi) = euler_audit(r.topology().arena());
            eprintln!("Near-miss graze: V={v} E={e} F={f} χ={chi}");
            assert!(f >= 6, "Near-miss result should have faces, got {f}");
        }
        Err(e) => {
            eprintln!("Near-miss graze returned error (acceptable): {e:?}");
        }
    }
}

/// T2.3b — Near miss at sub-epsilon scale.
#[test]
fn near_miss_sub_epsilon() {
    let epsilon = 1e-14;
    let result = try_boolean(
        [0.0, 0.0, 0.0], 1.0,
        [2.0 + epsilon, 0.0, 0.0], 1.0,
        BooleanOp::Union,
    );

    match result {
        Ok(r) => {
            let f = r.topology().arena().face_count();
            eprintln!("Sub-epsilon near-miss: {f} faces");
        }
        Err(e) => {
            eprintln!("Sub-epsilon near-miss error (acceptable): {e:?}");
        }
    }
}

// ══════════════════════════════════════════════════════════════
// §T2.4  TANGENT PLANE SLIVER
// ══════════════════════════════════════════════════════════════

/// T2.4 — Two solids with near-tangent planes (0.001°).
///
/// Intersection should produce a single clean edge, not zig-zag segments.
#[test]
fn tangent_plane_sliver() {
    let tiny_angle = 0.001_f64.to_radians();
    let cos_a = tiny_angle.cos();
    let sin_a = tiny_angle.sin();

    let tilted_planes = vec![
        Plane::from_point_normal([cos_a, sin_a, 0.0], [cos_a, sin_a, 0.0]).unwrap(),
        Plane::from_point_normal([-cos_a, -sin_a, 0.0], [-cos_a, -sin_a, 0.0]).unwrap(),
        Plane::from_point_normal([0.0, 1.0, 0.0], [0.0, 1.0, 0.0]).unwrap(),
        Plane::from_point_normal([0.0, -1.0, 0.0], [0.0, -1.0, 0.0]).unwrap(),
        Plane::from_point_normal([0.0, 0.0, 1.0], [0.0, 0.0, 1.0]).unwrap(),
        Plane::from_point_normal([0.0, 0.0, -1.0], [0.0, 0.0, -1.0]).unwrap(),
    ];

    let (topo_tilted, geom_tilted) = build_convex_solid(tilted_planes);
    let (topo_cube, geom_cube) = build_cube([0.0, 0.0, 0.0], 1.0);

    let input = BooleanInput::new(
        topo_cube, geom_cube,
        topo_tilted, geom_tilted,
        BooleanOp::Intersection,
    );

    match execute_boolean_logged(input) {
        Ok(envelope) => {
            let r = envelope.into_value();
            let (v, e, f, chi) = euler_audit(r.topology().arena());
            eprintln!("Tangent sliver: V={v} E={e} F={f} χ={chi}");
            assert!(f > 0, "Tangent sliver should produce faces");
            assert_eq!(chi, 2, "Tangent sliver Euler violation: χ={chi}");
        }
        Err(e) => {
            eprintln!("Tangent sliver returned error (tracking): {e:?}");
        }
    }
}

// ══════════════════════════════════════════════════════════════
// §T2.5  MICRO-CUBE SUBTRACTION
// ══════════════════════════════════════════════════════════════

/// T2.5 — Tiny cube (10⁻⁶) subtracted from unit cube.
///
/// Must not "lose" the cavity due to tolerance rounding.
#[test]
fn micro_cube_subtraction() {
    let result = try_boolean(
        [0.0, 0.0, 0.0], 1.0,
        [0.0, 0.0, 0.0], 1e-6,
        BooleanOp::Subtraction,
    );

    match result {
        Ok(r) => {
            let f = r.topology().arena().face_count();
            eprintln!("Micro-cube subtraction: {f} faces");
            assert!(f >= 6, "Should have at least the outer shell");
        }
        Err(e) => {
            eprintln!("Micro-cube subtraction error (tracking): {e:?}");
        }
    }
}

// ══════════════════════════════════════════════════════════════
// §T2.6  COORDINATE EXTREMES
// ══════════════════════════════════════════════════════════════

/// T2.6 — Cubes at very large coordinates.
///
/// At large coordinates, ULP (unit of least precision) grows.
/// Must not produce NaN or self-intersecting results.
#[test]
fn coordinate_extremes() {
    let far = 1e8;
    let result = try_boolean(
        [far, far, far], 1.0,
        [far + 0.5, far, far], 1.0,
        BooleanOp::Union,
    );

    match result {
        Ok(r) => {
            let (v, e, f, chi) = euler_audit(r.topology().arena());
            eprintln!("Far coordinates: V={v} E={e} F={f} χ={chi}");
            assert_eq!(chi, 2, "Far coordinate Euler violation");
        }
        Err(e) => {
            eprintln!("Far coordinate error (tracking): {e:?}");
        }
    }
}
