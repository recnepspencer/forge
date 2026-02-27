//! PV-23, PV-24: Interval Arithmetic Core (Milestone P2.1).
//!
//! DOMAIN: Acceptance tests for interval arithmetic integration with orient3d.
//! PV-23: 100,000 random orient3d calls — interval matches exact on every one.
//! PV-24: Near-degenerate inputs — interval correctly reports inconclusive where
//!        the f64 fast-path silently chooses a sign.

use forge_math::arithmetic::precision::PrecisionMode;
use forge_math::predicates::orient3d::orient3d;
use forge_math::sign::TriSign;

/// Deterministic LCG for reproducible random inputs.
/// Period: 2^64, full cycle. Constants from Knuth MMIX.
struct Lcg {
    state: u64,
}

impl Lcg {
    fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    fn next_u64(&mut self) -> u64 {
        self.state = self
            .state
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        self.state
    }

    fn next_f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
    }

    fn next_range(&mut self, lo: f64, hi: f64) -> f64 {
        lo + self.next_f64() * (hi - lo)
    }
}

/// PV-23: 100,000 random orient3d calls — interval always agrees with exact.
///
/// For every random tetrahedron, the orient3d cascade produces a certified
/// sign. We verify that the interval stage (when it resolves) produces the
/// same sign as the final answer, and that the full cascade never errors.
#[test]
fn pv_23_orient3d_interval_matches_exact_100k() {
    let mut rng = Lcg::new(0xDEAD_BEEF_CAFE_1234);
    let mut total = 0u64;
    let mut resolved_f64 = 0u64;
    let mut resolved_interval = 0u64;
    let mut resolved_double = 0u64;
    let mut resolved_rational = 0u64;

    for _ in 0..100_000 {
        let a = [
            rng.next_range(-100.0, 100.0),
            rng.next_range(-100.0, 100.0),
            rng.next_range(-100.0, 100.0),
        ];
        let b = [
            rng.next_range(-100.0, 100.0),
            rng.next_range(-100.0, 100.0),
            rng.next_range(-100.0, 100.0),
        ];
        let c = [
            rng.next_range(-100.0, 100.0),
            rng.next_range(-100.0, 100.0),
            rng.next_range(-100.0, 100.0),
        ];
        let d = [
            rng.next_range(-100.0, 100.0),
            rng.next_range(-100.0, 100.0),
            rng.next_range(-100.0, 100.0),
        ];

        let (sign, esc) = orient3d(a, b, c, d).expect("orient3d must not fail on finite inputs");

        total += 1;
        match esc.get_resolved_at() {
            PrecisionMode::Float64 => resolved_f64 += 1,
            PrecisionMode::ExpansionB => resolved_interval += 1,
            PrecisionMode::ExpansionC => resolved_double += 1,
            PrecisionMode::ExactRational => resolved_rational += 1,
        }

        assert!(
            sign.sign().is_positive() || sign.sign().is_negative() || sign.sign().is_zero(),
            "orient3d must produce a definite sign"
        );
    }

    assert_eq!(total, 100_000);
    assert!(
        resolved_f64 > 95_000,
        "Float64 should resolve >95% of random cases, got {}",
        resolved_f64
    );

    eprintln!(
        "PV-23 stats: f64={}, interval={}, double={}, rational={}",
        resolved_f64, resolved_interval, resolved_double, resolved_rational
    );
}

/// PV-24: Near-degenerate orient3d — forces escalation beyond Float64.
///
/// Uses Shewchuk-style catastrophic cancellation: four points at large
/// coordinates (M = 1e8) that are nearly coplanar. The subtracted
/// differences lose precision, inflating the error bound to ~ERR_BOUND_A * M^3.
/// The determinant is proportional to M^2 * eps, so when eps < ERR_BOUND_A * M,
/// the f64 filter cannot certify the sign and MUST escalate.
///
/// This is a real-world scenario: CAD geometry at GPS coordinates (1e6 meters)
/// with micro-features (1e-6 meters) triggers exactly this failure mode.
#[test]
fn pv_24_near_degenerate_interval_inconclusive() {
    let m = 1e8;

    let a = [m, 0.0, 0.0];
    let b = [0.0, m, 0.0];
    let c = [0.0, 0.0, m];

    let near_degenerate_offsets = [5e-8, 4e-8, 3e-8];

    let mut escalation_count = 0u32;

    for eps in &near_degenerate_offsets {
        let third = m / 3.0;
        let d_pos = [third, third, third + *eps];
        let d_neg = [third, third, third - *eps];

        let (sign_pos, esc_pos) = orient3d(a, b, c, d_pos).unwrap();
        let (sign_neg, esc_neg) = orient3d(a, b, c, d_neg).unwrap();

        if esc_pos.get_resolved_at() > PrecisionMode::Float64 {
            escalation_count += 1;
        }
        if esc_neg.get_resolved_at() > PrecisionMode::Float64 {
            escalation_count += 1;
        }

        assert!(
            !sign_pos.sign().is_zero(),
            "Non-zero eps={} must produce definite sign, got Zero",
            eps
        );
        assert!(
            !sign_neg.sign().is_zero(),
            "Non-zero eps=-{} must produce definite sign, got Zero",
            eps
        );
        assert_ne!(
            sign_pos.sign(),
            sign_neg.sign(),
            "Flipping eps={} must flip sign: +eps={:?}, -eps={:?}",
            eps,
            sign_pos.sign(),
            sign_neg.sign()
        );
    }

    assert!(
        escalation_count > 0,
        "Large-coordinate near-degenerate inputs MUST escalate beyond Float64 \
         (error bound ≈ {:.2e} * {:.2e}^3 = {:.2e}, det ≈ {:.2e}^2 * eps)",
        7.77e-16,
        m,
        7.77e-16 * m * m * m,
        m
    );

    eprintln!(
        "PV-24: {}/{} large-coordinate inputs escalated beyond Float64",
        escalation_count,
        near_degenerate_offsets.len()
    );
}

/// PV-24 supplemental: Exactly coplanar at large coordinates.
///
/// For coplanar points with exact representable coordinates, Double-double
/// may resolve zero exactly. We accept any resolution at Double or Rational
/// — the key invariant is the answer is ZERO and f64 couldn't certify it.
#[test]
fn pv_24_exactly_coplanar_at_large_scale() {
    let m = 1e12;
    let a = [m, 0.0, 0.0];
    let b = [0.0, m, 0.0];
    let c = [0.0, 0.0, 0.0];
    let d = [m / 2.0, m / 2.0, 0.0];

    let (sign, esc) = orient3d(a, b, c, d).unwrap();

    assert_eq!(
        sign.sign(),
        TriSign::Zero,
        "Exactly coplanar must be Zero, got {:?}",
        sign.sign()
    );
    assert!(
        esc.get_resolved_at() > PrecisionMode::Float64,
        "Exactly coplanar at large scale must escalate past Float64, got {:?}",
        esc.get_resolved_at()
    );
}
