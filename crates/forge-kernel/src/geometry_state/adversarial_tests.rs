//! Adversarial aerospace-grade tests for the geometry hierarchy.
//!
//! These tests target the edge cases that would fail in production:
//! singularities, degenerate inputs, machine-epsilon boundaries,
//! chained operation tolerance accumulation, and post-mutation
//! invariant preservation.

use crate::geom::{classify_surface_pair, SurfaceData, SurfaceRelation};
use crate::geom::{Coedge, ParametricCurve2D};
use crate::geom::{CurveGeom, CurveKind, CurveProvenance, SpCurveApproximation};
use std::f64::consts::{FRAC_PI_2, PI, TAU};

// =====================================================================
// 1. SINGULARITIES — Points where parametric surfaces degenerate
// =====================================================================

#[test]
fn sphere_normal_at_north_pole_is_vertical() {
    let s = SurfaceData::sphere([0.0, 0.0, 0.0], 1.0);
    let n = s.normal_at(0.0, FRAC_PI_2);
    assert!(
        (n[2] - 1.0).abs() < 1e-10,
        "north pole normal z={}, expected 1.0",
        n[2]
    );
    assert!(n[0].abs() < 1e-10);
    assert!(n[1].abs() < 1e-10);
}

#[test]
fn sphere_normal_at_south_pole_is_vertical() {
    let s = SurfaceData::sphere([0.0, 0.0, 0.0], 1.0);
    let n = s.normal_at(0.0, -FRAC_PI_2);
    assert!(
        (n[2] + 1.0).abs() < 1e-10,
        "south pole normal z={}, expected -1.0",
        n[2]
    );
}

#[test]
fn sphere_all_u_values_at_pole_give_same_point() {
    let s = SurfaceData::sphere([0.0, 0.0, 0.0], 1.0);
    let p0 = s.point_at(0.0, FRAC_PI_2);
    for u_frac in 1..=20 {
        let u = TAU * u_frac as f64 / 20.0;
        let p = s.point_at(u, FRAC_PI_2);
        let dist =
            ((p[0] - p0[0]).powi(2) + (p[1] - p0[1]).powi(2) + (p[2] - p0[2]).powi(2)).sqrt();
        assert!(dist < 1e-10, "Pole is not singular: u={}, dist={}", u, dist);
    }
}

#[test]
fn cone_at_apex_all_u_values_converge() {
    let s = SurfaceData::cone([1.0, 2.0, 3.0], [0.0, 0.0, 1.0], PI / 4.0);
    let p0 = s.point_at(0.0, 0.0);
    for u_frac in 1..=12 {
        let u = TAU * u_frac as f64 / 12.0;
        let p = s.point_at(u, 0.0);
        let dist =
            ((p[0] - p0[0]).powi(2) + (p[1] - p0[1]).powi(2) + (p[2] - p0[2]).powi(2)).sqrt();
        assert!(
            dist < 1e-10,
            "Cone apex not singular: u={}, dist={}",
            u,
            dist
        );
    }
}

// =====================================================================
// 2. DEGENERATE INPUTS — Zero/near-zero parameters
// =====================================================================

#[test]
fn zero_radius_cylinder_degenerates_to_line() {
    let s = SurfaceData::cylinder([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 0.0);
    for v in [0.0, 1.0, 5.0, -3.0] {
        let p = s.point_at(0.0, v);
        assert!(p[0].abs() < 1e-12, "r=0 cylinder should be on axis");
        assert!(p[1].abs() < 1e-12);
        assert!((p[2] - v).abs() < 1e-12);
    }
}

#[test]
fn zero_half_angle_cone_degenerates_to_line() {
    let s = SurfaceData::cone([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 0.0);
    for v in [0.0, 1.0, 5.0] {
        let p = s.point_at(0.0, v);
        let r = (p[0] * p[0] + p[1] * p[1]).sqrt();
        assert!(r < 1e-12, "0-angle cone should be a ray, got r={}", r);
    }
}

#[test]
fn zero_minor_radius_torus_degenerates_to_circle() {
    let major = 5.0;
    let s = SurfaceData::torus([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], major, 0.0);
    for u_frac in 0..=8 {
        let u = TAU * u_frac as f64 / 8.0;
        let p = s.point_at(u, 0.0);
        let r = (p[0] * p[0] + p[1] * p[1]).sqrt();
        assert!(
            (r - major).abs() < 1e-10,
            "Minor_r=0 torus not a circle: r={}",
            r
        );
        assert!(p[2].abs() < 1e-10);
    }
}

#[test]
fn sp_curve_empty_control_points_returns_origin() {
    let kind = CurveKind::SurfaceIntersection {
        surface_a: 0,
        surface_b: 1,
        sp_curve_cache: crate::geom::SpCurveApproximation {
            control_points: vec![],
            knots: vec![],
            error_bound: 0.0,
            domain: (0.0, 1.0),
        },
    };
    let p = kind.point_at(0.5);
    assert_eq!(p, [0.0, 0.0, 0.0]);
}

#[test]
fn sp_curve_single_control_point_is_constant() {
    let kind = CurveKind::SurfaceIntersection {
        surface_a: 0,
        surface_b: 1,
        sp_curve_cache: crate::geom::SpCurveApproximation {
            control_points: vec![[3.0, 4.0, 5.0]],
            knots: vec![0.0, 1.0],
            error_bound: 0.0,
            domain: (0.0, 1.0),
        },
    };
    for t in [0.0, 0.3, 0.7, 1.0] {
        let p = kind.point_at(t);
        assert_eq!(p, [3.0, 4.0, 5.0], "Single point at t={}", t);
    }
}

// =====================================================================
// 3. CLASSIFICATION AT MACHINE EPSILON — Boundary decisions
// =====================================================================

#[test]
fn planes_one_ulp_apart_are_still_disjoint() {
    let a = SurfaceData::plane([0.0, 0.0, 1.0], 0.0);
    let b = SurfaceData::plane([0.0, 0.0, 1.0], 1e-10);
    assert_eq!(
        classify_surface_pair(&a, &b, 1e-12, 10.0)
            .into_result_strict()
            .unwrap(),
        SurfaceRelation::Disjoint
    );
}

#[test]
fn spheres_just_touching_are_general() {
    let a = SurfaceData::sphere([0.0, 0.0, 0.0], 5.0);
    let b = SurfaceData::sphere([10.0, 0.0, 0.0], 5.0);
    assert_eq!(
        classify_surface_pair(&a, &b, 1e-12, 10.0)
            .into_result_strict()
            .unwrap(),
        SurfaceRelation::General,
        "Spheres exactly touching should be General (tangent intersection)"
    );
}

#[test]
fn spheres_barely_overlapping_are_general() {
    let a = SurfaceData::sphere([0.0, 0.0, 0.0], 5.0);
    let b = SurfaceData::sphere([9.999, 0.0, 0.0], 5.0);
    assert_eq!(
        classify_surface_pair(&a, &b, 1e-12, 10.0)
            .into_result_strict()
            .unwrap(),
        SurfaceRelation::General
    );
}

#[test]
fn spheres_barely_separated_are_disjoint() {
    let a = SurfaceData::sphere([0.0, 0.0, 0.0], 5.0);
    let b = SurfaceData::sphere([10.001, 0.0, 0.0], 5.0);
    assert_eq!(
        classify_surface_pair(&a, &b, 1e-12, 10.0)
            .into_result_strict()
            .unwrap(),
        SurfaceRelation::Disjoint
    );
}

#[test]
fn different_surface_types_always_general() {
    let plane = SurfaceData::plane([0.0, 0.0, 1.0], 0.0);
    let sphere = SurfaceData::sphere([0.0, 0.0, 0.0], 1.0);
    let cylinder = SurfaceData::cylinder([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 1.0);
    let cone = SurfaceData::cone([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], PI / 4.0);
    let torus = SurfaceData::torus([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 5.0, 1.0);

    assert_eq!(
        classify_surface_pair(&plane, &sphere, 1e-12, 10.0)
            .into_result_strict()
            .unwrap(),
        SurfaceRelation::General
    );
    assert_eq!(
        classify_surface_pair(&plane, &cylinder, 1e-12, 10.0)
            .into_result_strict()
            .unwrap(),
        SurfaceRelation::General
    );
    assert_eq!(
        classify_surface_pair(&sphere, &cylinder, 1e-12, 10.0)
            .into_result_strict()
            .unwrap(),
        SurfaceRelation::General
    );
    assert_eq!(
        classify_surface_pair(&cone, &torus, 1e-12, 10.0)
            .into_result_strict()
            .unwrap(),
        SurfaceRelation::General
    );
    assert_eq!(
        classify_surface_pair(&plane, &torus, 1e-12, 10.0)
            .into_result_strict()
            .unwrap(),
        SurfaceRelation::General
    );
}

#[test]
fn cylinders_parallel_different_radius_are_general() {
    let a = SurfaceData::cylinder([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 3.0);
    let b = SurfaceData::cylinder([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 5.0);
    assert_eq!(
        classify_surface_pair(&a, &b, 1e-12, 10.0)
            .into_result_strict()
            .unwrap(),
        SurfaceRelation::General
    );
}

#[test]
fn cylinders_skew_axes_are_general() {
    let a = SurfaceData::cylinder([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], 3.0);
    let b = SurfaceData::cylinder([0.0, 0.0, 0.0], [0.0, 1.0, 0.0], 3.0);
    assert_eq!(
        classify_surface_pair(&a, &b, 1e-12, 10.0)
            .into_result_strict()
            .unwrap(),
        SurfaceRelation::General
    );
}

// =====================================================================
// 4. CHAINED COALESCENCE — Tolerance accumulation over N operations
// =====================================================================

#[test]
fn chained_coalescence_tolerance_grows_monotonically() {
    use crate::geom::VertexGeom;

    let initial = 1e-10;
    let mut tolerance = initial;
    for step in 0..20 {
        tolerance = VertexGeom::coalesced_tolerance(tolerance, initial);
        assert!(
            tolerance > initial,
            "Step {}: tolerance {} must exceed initial {}",
            step,
            tolerance,
            initial
        );
        assert!(
            tolerance.is_finite(),
            "Step {}: tolerance went to infinity",
            step
        );
    }
}

#[test]
fn chained_coalescence_20_steps_stays_below_1e_6() {
    use crate::geom::VertexGeom;

    let initial = 1e-10;
    let mut tolerance = initial;
    for _ in 0..20 {
        tolerance = VertexGeom::coalesced_tolerance(tolerance, initial);
    }
    assert!(
        tolerance < 1e-6,
        "20 coalescence steps from 1e-10 grew to {} — would fail classification",
        tolerance
    );
}

#[test]
fn chained_snap_decisions_are_all_logged() {
    use crate::core::ModelingContext;
    use crate::geometry_state::{snap_or_coalesce_vertex, CoalescenceResult, GeometryState};
    use forge_topo::handles::VertexId;

    let mut geom = GeometryState::new();
    let mut ctx = ModelingContext::new();
    let v = VertexId::from_raw_parts(0, 0);
    let existing_pos = [0.0, 0.0, 0.0];
    let existing_tol = 1e-5;

    for i in 0..10 {
        let offset = (i as f64 + 1.0) * 1e-8;
        let result = snap_or_coalesce_vertex(
            [offset, 0.0, 0.0],
            1e-10,
            v,
            existing_pos,
            existing_tol,
            &mut ctx,
            1e-4,
        );
        assert!(
            matches!(result, CoalescenceResult::Snapped { .. }),
            "Iteration {} should snap (offset={} < tolerance=1e-5)",
            i,
            offset
        );
    }
    assert_eq!(
        ctx.get_decision_count(),
        10,
        "Every snap must produce a TracedDecision"
    );
}

// =====================================================================
// 5. POST-SPLIT CURVE EVALUATION CONSISTENCY
// =====================================================================

#[test]
fn split_line_segments_are_continuous_at_split_point() {
    use crate::geometry_state::{propagate_curve_on_split, GeometryState};
    use forge_topo::handles::EdgeId;

    let mut geom = GeometryState::new();
    let old_edge = EdgeId::from_raw_parts(0, 0);
    let new_edge = EdgeId::from_raw_parts(1, 0);
    let t_split = 0.4;

    let curve = CurveGeom::from_analytic(
        CurveKind::Line {
            origin: [0.0, 0.0, 0.0],
            direction: [10.0, 0.0, 0.0],
        },
        [0, 1],
    );
    let cr = geom.insert_curve(curve);
    geom.attach_curve_to_edge(old_edge, cr);

    propagate_curve_on_split(old_edge, new_edge, t_split, &mut geom).unwrap();

    let ref_b = geom.get_edge_curve(new_edge).unwrap();
    let curve_b = geom.get_curve(ref_b).unwrap();

    let start_of_segment_b = curve_b.kind.point_at(0.0);
    let expected_split_point = [t_split * 10.0, 0.0, 0.0];

    let dist = ((start_of_segment_b[0] - expected_split_point[0]).powi(2)
        + (start_of_segment_b[1] - expected_split_point[1]).powi(2)
        + (start_of_segment_b[2] - expected_split_point[2]).powi(2))
    .sqrt();
    assert!(
        dist < 1e-10,
        "Line split at t={}: segment B starts at {:?}, expected {:?}, dist={}",
        t_split,
        start_of_segment_b,
        expected_split_point,
        dist
    );
}

#[test]
fn split_sp_curve_domains_partition_original() {
    use crate::geometry_state::{propagate_curve_on_split, GeometryState};
    use forge_topo::handles::EdgeId;

    let mut geom = GeometryState::new();
    let old_edge = EdgeId::from_raw_parts(0, 0);
    let new_edge = EdgeId::from_raw_parts(1, 0);

    let curve = CurveGeom {
        kind: CurveKind::SurfaceIntersection {
            surface_a: 0,
            surface_b: 1,
            sp_curve_cache: SpCurveApproximation {
                control_points: vec![
                    [0.0, 0.0, 0.0],
                    [0.25, 0.5, 0.0],
                    [0.5, 0.5, 0.0],
                    [0.75, 0.5, 0.0],
                    [1.0, 0.0, 0.0],
                ],
                knots: vec![0.0, 0.25, 0.5, 0.75, 1.0],
                error_bound: 1e-8,
                domain: (0.0, 1.0),
            },
        },
        tolerance: 1e-8,
        provenance: CurveProvenance::SsiSolver {
            residual: 1e-8,
            iterations: 5,
        },
    };
    let cr = geom.insert_curve(curve);
    geom.attach_curve_to_edge(old_edge, cr);

    propagate_curve_on_split(old_edge, new_edge, 0.6, &mut geom).unwrap();

    let ref_a = geom.get_edge_curve(old_edge).unwrap();
    let ref_b = geom.get_edge_curve(new_edge).unwrap();
    let curve_a = geom.get_curve(ref_a).unwrap();
    let curve_b = geom.get_curve(ref_b).unwrap();

    match (&curve_a.kind, &curve_b.kind) {
        (
            CurveKind::SurfaceIntersection {
                sp_curve_cache: ca, ..
            },
            CurveKind::SurfaceIntersection {
                sp_curve_cache: cb, ..
            },
        ) => {
            assert!((ca.domain.0 - 0.0).abs() < 1e-12, "Segment A domain start");
            assert!((ca.domain.1 - 0.6).abs() < 1e-12, "Segment A domain end");
            assert!((cb.domain.0 - 0.6).abs() < 1e-12, "Segment B domain start");
            assert!((cb.domain.1 - 1.0).abs() < 1e-12, "Segment B domain end");
            assert!(
                !ca.control_points.is_empty(),
                "Segment A should have points"
            );
            assert!(
                !cb.control_points.is_empty(),
                "Segment B should have points"
            );
        }
        _ => panic!("Expected SurfaceIntersection for both segments"),
    }
}

// =====================================================================
// 6. ANTI-DRIFT AFTER SPLIT — The final boss
// =====================================================================

#[test]
fn anti_drift_survives_curve_split_on_cylinder() {
    use crate::geometry_state::{propagate_curve_on_split, GeometryState};
    use forge_topo::handles::EdgeId;

    let radius = 3.0;
    let surface = SurfaceData::cylinder([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], radius);

    let mut geom = GeometryState::new();
    let old_edge = EdgeId::from_raw_parts(0, 0);
    let new_edge = EdgeId::from_raw_parts(1, 0);

    let sp_cache = SpCurveApproximation {
        control_points: (0..=20)
            .map(|i| {
                let frac = i as f64 / 20.0;
                let u = frac * PI;
                let v = frac * 5.0;
                surface.point_at(u, v)
            })
            .collect(),
        knots: (0..=20).map(|i| i as f64 / 20.0).collect(),
        error_bound: 1e-12,
        domain: (0.0, 1.0),
    };

    let curve = CurveGeom {
        kind: CurveKind::SurfaceIntersection {
            surface_a: 0,
            surface_b: 1,
            sp_curve_cache: sp_cache,
        },
        tolerance: 1e-12,
        provenance: CurveProvenance::SsiSolver {
            residual: 1e-12,
            iterations: 10,
        },
    };
    let cr = geom.insert_curve(curve);
    geom.attach_curve_to_edge(old_edge, cr);

    propagate_curve_on_split(old_edge, new_edge, 0.5, &mut geom).unwrap();

    let ref_a = geom.get_edge_curve(old_edge).unwrap();
    let ref_b = geom.get_edge_curve(new_edge).unwrap();
    let curve_a = geom.get_curve(ref_a).unwrap();
    let curve_b = geom.get_curve(ref_b).unwrap();

    let check_on_cylinder = |curve: &CurveGeom, label: &str| {
        for i in 0..=10 {
            let t = i as f64 / 10.0;
            let p = curve.kind.point_at(t);
            let r = (p[0] * p[0] + p[1] * p[1]).sqrt();
            assert!(
                (r - radius).abs() < 0.1,
                "{}: point at t={} has r={}, expected {} (drift = {})",
                label,
                t,
                r,
                radius,
                (r - radius).abs()
            );
        }
    };

    check_on_cylinder(curve_a, "Segment A after split");
    check_on_cylinder(curve_b, "Segment B after split");
}

// =====================================================================
// 7. NORMAL/TANGENT ORTHOGONALITY — surface normal ⊥ to surface
// =====================================================================

#[test]
fn normal_dot_tangent_is_zero_for_all_surfaces() {
    let surfaces = vec![
        (
            "cylinder",
            SurfaceData::cylinder([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 3.0),
        ),
        ("sphere", SurfaceData::sphere([0.0, 0.0, 0.0], 5.0)),
        (
            "torus",
            SurfaceData::torus([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 5.0, 1.0),
        ),
        (
            "cone",
            SurfaceData::cone([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], PI / 6.0),
        ),
    ];

    let dt = 1e-7;
    for (name, s) in &surfaces {
        for &(u, v) in &[(0.5, 0.3), (1.0, 1.0), (2.0, -0.5_f64.max(0.01))] {
            let v = if *name == "cone" { v.max(0.1) } else { v };
            let n = s.normal_at(u, v);

            let du = [
                (s.point_at(u + dt, v)[0] - s.point_at(u - dt, v)[0]) / (2.0 * dt),
                (s.point_at(u + dt, v)[1] - s.point_at(u - dt, v)[1]) / (2.0 * dt),
                (s.point_at(u + dt, v)[2] - s.point_at(u - dt, v)[2]) / (2.0 * dt),
            ];
            let dv = [
                (s.point_at(u, v + dt)[0] - s.point_at(u, v - dt)[0]) / (2.0 * dt),
                (s.point_at(u, v + dt)[1] - s.point_at(u, v - dt)[1]) / (2.0 * dt),
                (s.point_at(u, v + dt)[2] - s.point_at(u, v - dt)[2]) / (2.0 * dt),
            ];

            let n_dot_du = n[0] * du[0] + n[1] * du[1] + n[2] * du[2];
            let n_dot_dv = n[0] * dv[0] + n[1] * dv[1] + n[2] * dv[2];

            assert!(
                n_dot_du.abs() < 1e-4,
                "{}: normal·∂S/∂u = {} at u={}, v={}",
                name,
                n_dot_du,
                u,
                v
            );
            assert!(
                n_dot_dv.abs() < 1e-4,
                "{}: normal·∂S/∂v = {} at u={}, v={}",
                name,
                n_dot_dv,
                u,
                v
            );
        }
    }
}

// =====================================================================
// 8. CURVE TANGENT CONSISTENCY — tangent must match derivative
// =====================================================================

#[test]
fn curve_tangent_matches_finite_difference_for_all_types() {
    let curves: Vec<(&str, CurveKind)> = vec![
        (
            "line",
            CurveKind::Line {
                origin: [1.0, 2.0, 3.0],
                direction: [0.0, 0.0, 1.0],
            },
        ),
        (
            "circle",
            CurveKind::Circle {
                center: [0.0, 0.0, 0.0],
                normal: [0.0, 0.0, 1.0],
                radius: 5.0,
            },
        ),
        (
            "ellipse",
            CurveKind::Ellipse {
                center: [0.0, 0.0, 0.0],
                major: [3.0, 0.0, 0.0],
                minor: [0.0, 2.0, 0.0],
            },
        ),
    ];

    let dt = 1e-7;
    for (name, c) in &curves {
        for &t in &[0.0, 0.5, 1.0, 2.0] {
            let analytic = c.tangent_at(t);
            let p0 = c.point_at(t - dt);
            let p1 = c.point_at(t + dt);
            let numerical = [
                (p1[0] - p0[0]) / (2.0 * dt),
                (p1[1] - p0[1]) / (2.0 * dt),
                (p1[2] - p0[2]) / (2.0 * dt),
            ];
            let num_len =
                (numerical[0].powi(2) + numerical[1].powi(2) + numerical[2].powi(2)).sqrt();
            if num_len < 1e-10 {
                continue;
            }
            let normalized = [
                numerical[0] / num_len,
                numerical[1] / num_len,
                numerical[2] / num_len,
            ];

            let dot = analytic[0] * normalized[0]
                + analytic[1] * normalized[1]
                + analytic[2] * normalized[2];
            assert!(
                dot.abs() > 0.999,
                "{}: tangent mismatch at t={}: analytic={:?} numerical={:?} dot={}",
                name,
                t,
                analytic,
                normalized,
                dot
            );
        }
    }
}
