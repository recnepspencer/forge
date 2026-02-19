//! PV-25, PV-26, PV-27: Precision Escalation Pipeline (Milestone P2.2).
//!
//! DOMAIN: Acceptance tests for the automatic float→interval→double→rational
//! pipeline. Tests exercise real orient3d and orient2d predicates, not mocks.
//! PV-25: Standard well-separated geometry stays at Float64.
//! PV-26: Crafted near-degenerate geometry escalates to Interval and resolves.
//! PV-27: Exactly-degenerate geometry cascades all the way to Rational.

use forge_math::arithmetic::filter::PrecisionMode;
use forge_math::predicates::orient3d::orient3d;
use forge_math::predicates::orient2d::orient2d;
use forge_math::sign::TriSign;

/// PV-25: Standard cases resolve at Float64 — no unnecessary escalation.
///
/// Uses well-separated tetrahedra and triangles that should never need
/// higher precision. Every resolved_at must be Float64.
#[test]
fn pv_25_standard_cases_resolve_at_float64() {
    let test_cases: Vec<([f64;3], [f64;3], [f64;3], [f64;3])> = vec![
        ([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]),
        ([0.0, 0.0, 0.0], [10.0, 0.0, 0.0], [0.0, 10.0, 0.0], [0.0, 0.0, -5.0]),
        ([1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 10.0], [0.0, 0.0, 0.0]),
        ([-1.0, -1.0, -1.0], [1.0, -1.0, -1.0], [-1.0, 1.0, -1.0], [-1.0, -1.0, 1.0]),
        ([100.0, 200.0, 300.0], [101.0, 200.0, 300.0], [100.0, 201.0, 300.0], [100.0, 200.0, 301.0]),
    ];

    for (i, (a, b, c, d)) in test_cases.iter().enumerate() {
        let (sign, esc) = orient3d(*a, *b, *c, *d).unwrap();
        assert_eq!(
            esc.get_resolved_at(), PrecisionMode::Float64,
            "Case {} should resolve at Float64, got {:?} (sign={:?})",
            i, esc.get_resolved_at(), sign.sign()
        );
        assert!(esc.get_float_agreed(), "Case {} Float64 should agree", i);
    }

    let orient2d_cases: Vec<([f64;2], [f64;2], [f64;2])> = vec![
        ([0.0, 0.0], [1.0, 0.0], [0.0, 1.0]),
        ([-5.0, -5.0], [5.0, -5.0], [0.0, 5.0]),
        ([100.0, 100.0], [200.0, 100.0], [100.0, 200.0]),
    ];

    for (i, (a, b, c)) in orient2d_cases.iter().enumerate() {
        let (sign, esc) = orient2d(*a, *b, *c).unwrap();
        assert_eq!(
            esc.get_resolved_at(), PrecisionMode::Float64,
            "orient2d case {} should resolve at Float64, got {:?} (sign={:?})",
            i, esc.get_resolved_at(), sign.sign()
        );
    }
}

/// PV-26: Crafted near-degenerate case escalates beyond Float64.
///
/// Uses large coordinates (M=1e6) to inflate the f64 error bound.
/// The determinant is small relative to the permanent, forcing
/// escalation to Interval or Double.
#[test]
fn pv_26_near_degenerate_escalates_beyond_float64() {
    let m = 1e8;
    let a = [m, 0.0, 0.0];
    let b = [0.0, m, 0.0];
    let c = [0.0, 0.0, m];

    let mut escalation_count = 0u32;

    let near_degenerate_offsets = [5e-8, 4e-8, 3e-8];

    for offset in &near_degenerate_offsets {
        let third = m / 3.0;
        let d_plus = [third, third, third + *offset];
        let d_minus = [third, third, third - *offset];

        let (sign_plus, esc_plus) = orient3d(a, b, c, d_plus).unwrap();
        let (sign_minus, esc_minus) = orient3d(a, b, c, d_minus).unwrap();

        assert!(
            !sign_plus.sign().is_zero(),
            "Non-zero offset +{} must produce definite sign", offset
        );
        assert_ne!(
            sign_plus.sign(), sign_minus.sign(),
            "Flipping offset {} must flip sign: +={:?}, -={:?}",
            offset, sign_plus.sign(), sign_minus.sign()
        );

        if esc_plus.get_resolved_at() > PrecisionMode::Float64 {
            escalation_count += 1;
        }
        if esc_minus.get_resolved_at() > PrecisionMode::Float64 {
            escalation_count += 1;
        }
    }

    assert!(
        escalation_count > 0,
        "At least one large-coordinate near-degenerate case should escalate beyond Float64"
    );

    eprintln!("PV-26: {} escalations out of {} calls", escalation_count, near_degenerate_offsets.len() * 2);
}

/// PV-27: Exactly-degenerate at large scale forces full escalation chain.
///
/// Four points at large coordinates (M=1e8) that are exactly coplanar.
/// f64 cannot certify zero because the error bound is huge (~M^3 * ERR_BOUND_A).
/// The cascade must escalate past Float64. Whether it resolves at Double
/// or Rational depends on whether Double-double arithmetic is exact for
/// these particular coordinates.
#[test]
fn pv_27_exactly_degenerate_reaches_beyond_float64() {
    let exactly_degenerate_cases: Vec<(f64, [f64;3], [f64;3], [f64;3], [f64;3])> = vec![
        (1e12,
         [1e12, 0.0, 0.0], [0.0, 1e12, 0.0], [0.0, 0.0, 0.0],
         [5e11, 5e11, 0.0]),
        (1e8,
         [1e8, 0.0, 0.0], [0.0, 1e8, 0.0], [0.0, 0.0, 0.0],
         [5e7, 5e7, 0.0]),
    ];

    for (i, (scale, a, b, c, d)) in exactly_degenerate_cases.iter().enumerate() {
        let (sign, esc) = orient3d(*a, *b, *c, *d).unwrap();
        assert_eq!(
            sign.sign(), TriSign::Zero,
            "Case {} (scale={}) should be exactly Zero, got {:?} (resolved_at={:?})",
            i, scale, sign.sign(), esc.get_resolved_at()
        );
        assert!(
            esc.get_resolved_at() > PrecisionMode::Float64,
            "Case {} (scale={}) must escalate past Float64, got {:?}",
            i, scale, esc.get_resolved_at()
        );
    }
}

/// PV-27 supplemental: Verify orient2d also escalates for exactly-collinear
/// at large coordinates.
#[test]
fn pv_27_orient2d_collinear_at_large_scale() {
    let m = 1e8;
    let a = [m, m];
    let b = [m + 1.0, m + 1.0];
    let c = [m + 0.5, m + 0.5];

    let (sign, esc) = orient2d(a, b, c).unwrap();
    assert_eq!(sign.sign(), TriSign::Zero);
    assert!(
        esc.get_resolved_at() > PrecisionMode::Float64,
        "Collinear at M={} must escalate past Float64, got {:?}",
        m, esc.get_resolved_at()
    );
}
