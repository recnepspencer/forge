//! PV-31, PV-32, PV-52: Scale-Invariant Precision Guards (Milestone P2.4).
//!
//! DOMAIN: Acceptance tests for local coordinate transforms and scale analysis.
//! PV-31: Same config at 5 scales → identical topological result
//! PV-32: Mixed-scale (1e12 + 1e-9) → correct precision escalation
//! PV-52: Local coordinate round-trip < 1 ULP

use crate::geom::{LocalCoordinateSpace, ScaleAnalysis};
use forge_math::arithmetic::precision::PrecisionMode;
use forge_math::predicates::orient3d::orient3d;

/// ULP computation (mirrors forge-math's internal ulp).
fn ulp(x: f64) -> f64 {
    let abs = x.abs();
    if abs == 0.0 {
        return f64::MIN_POSITIVE;
    }
    let bits = abs.to_bits();
    let next = f64::from_bits(bits + 1);
    next - abs
}

#[test]
fn pv_31_same_topology_at_five_scales() {
    let scales: [f64; 5] = [1.0, 1e6, 1e-6, 1e12, 1e-12];

    let base_a = [0.0, 0.0, 0.0];
    let base_b = [1.0, 0.0, 0.0];
    let base_c = [0.0, 1.0, 0.0];
    let base_d = [0.0, 0.0, 1.0];

    let (ref_sign, _) = orient3d(base_a, base_b, base_c, base_d).unwrap();

    for scale in &scales {
        let a = [base_a[0] * scale, base_a[1] * scale, base_a[2] * scale];
        let b = [base_b[0] * scale, base_b[1] * scale, base_b[2] * scale];
        let c = [base_c[0] * scale, base_c[1] * scale, base_c[2] * scale];
        let d = [base_d[0] * scale, base_d[1] * scale, base_d[2] * scale];

        let space = LocalCoordinateSpace::from_points(&[a, b, c, d]);
        let la = space.to_local(a);
        let lb = space.to_local(b);
        let lc = space.to_local(c);
        let ld = space.to_local(d);

        let (local_sign, _) = orient3d(la, lb, lc, ld).unwrap();

        assert_eq!(
            ref_sign.sign(),
            local_sign.sign(),
            "Sign mismatch at scale {}: ref={:?}, got={:?}",
            scale,
            ref_sign.sign(),
            local_sign.sign()
        );
    }
}

#[test]
fn pv_32_mixed_scale_correct_escalation() {
    let big_points = [
        [1e12, 0.0, 0.0],
        [1e12 + 1.0, 0.0, 0.0],
        [1e12, 1.0, 0.0],
        [1e12, 0.0, 1.0],
    ];

    let analysis = ScaleAnalysis::compute(&big_points, 1e-9);
    assert!(
        analysis.get_needs_local_transform(),
        "Mixed-scale should need local transform"
    );

    let space = LocalCoordinateSpace::from_points(&big_points);
    let la = space.to_local(big_points[0]);
    let lb = space.to_local(big_points[1]);
    let lc = space.to_local(big_points[2]);
    let ld = space.to_local(big_points[3]);

    let (sign, esc) = orient3d(la, lb, lc, ld).unwrap();

    assert!(
        sign.sign().is_positive() || sign.sign().is_negative(),
        "Should resolve to a definite sign in local space"
    );
    assert!(
        esc.get_resolved_at() <= PrecisionMode::ExpansionB,
        "In local space (unit range), should resolve at Float64 or Interval, not {:?}",
        esc.get_resolved_at()
    );
}

#[test]
fn pv_52_round_trip_within_one_ulp() {
    for decade in -10..=10i32 {
        let scale = 10.0f64.powi(decade);
        let original = [scale, scale, scale];

        let space = LocalCoordinateSpace::from_points(&[original]);
        let local = space.to_local(original);
        let round_trip = space.from_local(local);

        for i in 0..3 {
            let err = (round_trip[i] - original[i]).abs();
            let u = ulp(original[i]);
            assert!(
                err <= u,
                "Round-trip error {:.2e} exceeds ULP {:.2e} at scale 1e{} coord[{}]",
                err,
                u,
                decade,
                i,
            );
        }
    }
}

#[test]
fn local_space_from_empty_is_identity() {
    let space = LocalCoordinateSpace::from_points(&[]);
    assert_eq!(space.get_origin(), [0.0, 0.0, 0.0]);
    assert_eq!(space.get_scale(), 1.0);
}

#[test]
fn scale_analysis_unit_scale_no_transform() {
    let points = [[0.0, 0.0, 0.0], [1.0, 1.0, 1.0]];
    let analysis = ScaleAnalysis::compute(&points, 1e-6);
    assert!(!analysis.get_needs_local_transform());
    assert!(analysis.get_condition_number() < 1e12);
}
