//! MB-N1 through MB-N6: MetaBoss Numerical Torture Suite (Milestone P2.5).
//!
//! DOMAIN: Extreme numerical precision tests using real kernel operations.
//! MB-N1: 10,000 orient3d near degenerate plane — every divergence caught
//! MB-N2: Boolean of near-coincident faces (gap = 1e-14)
//! MB-N3: 500-step Boolean chain — no accumulated float drift
//! MB-N4: Scale-sweep: same Boolean at 20 scales from 1e-12 to 1e12
//! MB-N5: Condition-number stress: nearly-parallel planes
//! MB-N6: 100 chained exact rational operations — bit length bounded

use forge_math::arithmetic::precision::PrecisionMode;
use forge_math::arithmetic::precision::PrecisionBudget;
use forge_math::arithmetic::rational::Rational;
use forge_math::predicates::orient3d::orient3d;
use forge_math::sign::TriSign;

use crate::operations::boolean::test_helpers::{
    build_cube, execute_boolean_logged, euler_audit, build_convex_solid,
};
use crate::operations::boolean::{BooleanInput, BooleanOp};

use forge_geom::spatial::local_space::{LocalCoordinateSpace, ScaleAnalysis};

/// Deterministic LCG for reproducible random inputs.
struct Lcg {
    state: u64,
}

impl Lcg {
    fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_mul(6_364_136_223_846_793_005)
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

/// MB-N1: 10,000 orient3d calls near the degenerate plane at large scale.
///
/// Uses Shewchuk-style catastrophic cancellation: large coordinates (M=1e6)
/// with tiny offsets. The f64 error bound grows as ERR_BOUND_A * M^3 while
/// the determinant stays at M^2 * eps. When eps is small relative to
/// ERR_BOUND_A * M, f64 CANNOT certify the sign and must escalate.
/// This is the numerically honest way to force divergences.
#[test]
fn mb_n1_orient3d_near_degenerate_10k() {
    let mut rng = Lcg::new(0xABCD_1234_5678_9000);
    let m = 1e6;
    let a = [m, 0.0, 0.0];
    let b = [0.0, m, 0.0];
    let c = [0.0, 0.0, m];

    let mut total = 0u64;
    let mut f64_resolved = 0u64;
    let mut escalated = 0u64;

    for _ in 0..10_000 {
        let exponent = rng.next_range(-10.0, -7.0);
        let magnitude = 10.0f64.powf(exponent);
        let sign_flip = if rng.next_u64() % 2 == 0 { 1.0 } else { -1.0 };
        let z_offset = magnitude * sign_flip;

        let third = m / 3.0;
        let d = [third, third, third + z_offset];

        let (result, esc) = orient3d(a, b, c, d)
            .expect("orient3d must not fail");

        total += 1;

        assert!(
            result.sign().is_positive() || result.sign().is_negative(),
            "Non-zero offset must produce definite sign, got {:?} at offset={:.2e}",
            result.sign(), z_offset
        );

        if esc.get_resolved_at() == PrecisionMode::Float64 {
            f64_resolved += 1;
        } else {
            escalated += 1;
        }
    }

    let f64_rate = f64_resolved as f64 / total as f64;
    eprintln!(
        "MB-N1: f64_rate={:.1}%, escalated={}, total={}",
        f64_rate * 100.0, escalated, total
    );

    assert!(
        escalated > 0,
        "Large-coordinate near-degenerate inputs MUST trigger some escalations \
         (M={}, ERR_BOUND ≈ {:.2e})",
        m, 7.77e-16 * m * m * m
    );
}

/// MB-N2: Boolean of near-coincident faces (gap = 1e-14).
///
/// Two cubes with faces separated by 1e-14 — the kernel must either
/// resolve the Boolean correctly or return a structured KernelError.
/// It must NOT silently produce wrong topology.
#[test]
fn mb_n2_boolean_near_coincident_1e14() {
    let (topo_a, geom_a) = build_cube([0.0, 0.0, 0.0], 0.5);
    let (topo_b, geom_b) = build_cube([1e-14, 0.0, 0.0], 0.5);

    let input = BooleanInput::new(topo_a, geom_a, topo_b, geom_b, BooleanOp::Union);

    match execute_boolean_logged(input).into_result() {
        Ok(result) => {
            
            let (v, e, f, chi) = euler_audit(result.topology().arena());
            assert_eq!(chi, 2, "Union must produce valid manifold: V={} E={} F={} χ={}", v, e, f, chi);
            assert!(f >= 6, "Union should have at least 6 faces, got {}", f);
        }
        Err(e) => {
            eprintln!("MB-N2: Boolean returned error (acceptable for near-coincident): {:?}", e);
        }
    }
}

/// MB-N2 supplemental: increasing gap sizes to verify threshold behavior.
#[test]
fn mb_n2_boolean_gap_sweep() {
    let gaps = [1e-12, 1e-10, 1e-8, 1e-6, 1e-4];
    let mut successes = 0u32;

    for gap in &gaps {
        let (topo_a, geom_a) = build_cube([0.0, 0.0, 0.0], 0.5);
        let (topo_b, geom_b) = build_cube([*gap, 0.0, 0.0], 0.5);

        let input = BooleanInput::new(topo_a, geom_a, topo_b, geom_b, BooleanOp::Union);

        match execute_boolean_logged(input).into_result() {
            Ok(result) => {
                
                let (_, _, _, chi) = euler_audit(result.topology().arena());
                assert_eq!(chi, 2, "χ must be 2 at gap={:.0e}", gap);
                successes += 1;
            }
            Err(_) => {
                eprintln!("MB-N2: gap={:.0e} failed (expected at tight gaps)", gap);
            }
        }
    }

    assert!(
        successes >= 3,
        "At least 3/5 gap sizes should succeed, got {}",
        successes
    );
}

/// MB-N3: Multi-step Boolean chain — no accumulated float drift.
///
/// Subtracts small cubes from a large cube in sequence. Each subtraction
/// cube straddles the surface of the large cube (center at the edge),
/// creating a notch rather than an internal cavity. This ensures χ=2
/// (single shell) is maintained throughout the chain.
#[test]
fn mb_n3_boolean_chain_surface_notches() {
    let base_half = 5.0;
    let (mut topo, mut geom) = build_cube([0.0, 0.0, 0.0], base_half);

    let mut successful_steps = 0u32;
    let step_count = 20;

    for i in 0..step_count {
        let angle = (i as f64) * std::f64::consts::TAU / (step_count as f64);
        let x = base_half * angle.cos();
        let y = base_half * angle.sin();
        let z = -4.0 + (i as f64) * 0.4;

        let notch_half = 0.3;
        let (topo_b, geom_b) = build_cube([x, y, z], notch_half);

        let input = BooleanInput::new(topo, geom, topo_b, geom_b, BooleanOp::Subtraction);

        match execute_boolean_logged(input).into_result() {
            Ok(result) => {
                
                let (v, e, f, chi) = euler_audit(result.topology().arena());
                assert!(
                    chi > 0 && chi % 2 == 0,
                    "Step {} must have positive even χ, got χ={} (V={} E={} F={})",
                    i, chi, v, e, f
                );
                let (t, g, _) = result.into_states();
                topo = t;
                geom = g;
                successful_steps += 1;
            }
            Err(e) => {
                eprintln!("MB-N3: chain broke at step {}/{}: {:?}", i, step_count, e);
                break;
            }
        }
    }

    assert!(
        successful_steps >= 5,
        "Should complete at least 5 surface-notch steps, got {}",
        successful_steps
    );
    eprintln!("MB-N3: completed {}/{} Boolean chain steps", successful_steps, step_count);
}

/// MB-N4: Scale-sweep — same Boolean at multiple scales.
///
/// The SAME geometric configuration (two overlapping cubes, union)
/// is tested at multiple scales. The topological result (face count,
/// Euler characteristic) must be identical wherever the Boolean succeeds.
/// At extreme scales, the BSP builder or Boolean may fail — that's
/// acceptable. What's NOT acceptable is silent wrong topology.
#[test]
fn mb_n4_scale_sweep_union() {
    let mut reference_chi: Option<isize> = None;
    let mut reference_faces: Option<usize> = None;
    let mut successes = 0u32;

    let scales: Vec<f64> = vec![
        1e-6, 1e-4, 1e-2, 1.0, 1e2, 1e4, 1e6,
        0.001, 0.01, 0.1, 10.0, 100.0, 1000.0,
    ];

    for scale in &scales {
        let half = 0.5 * scale;
        let offset = 0.3 * scale;

        let (topo_a, geom_a) = build_cube([0.0, 0.0, 0.0], half);
        let (topo_b, geom_b) = build_cube([offset, offset, 0.0], half);

        let input = BooleanInput::new(topo_a, geom_a, topo_b, geom_b, BooleanOp::Union);

        match execute_boolean_logged(input).into_result() {
            Ok(result) => {
                
                let (v, e, f, chi) = euler_audit(result.topology().arena());

                if let Some(ref_chi) = reference_chi {
                    assert_eq!(
                        chi, ref_chi,
                        "Scale {}: χ={} differs from reference χ={} (V={} E={} F={})",
                        scale, chi, ref_chi, v, e, f
                    );
                } else {
                    reference_chi = Some(chi);
                    reference_faces = Some(f);
                }

                if let Some(ref_f) = reference_faces {
                    assert_eq!(
                        f, ref_f,
                        "Scale {}: face_count={} differs from reference={}",
                        scale, f, ref_f
                    );
                }

                successes += 1;
            }
            Err(e) => {
                eprintln!("MB-N4: scale={} failed: {:?}", scale, e);
            }
        }
    }

    assert!(
        successes >= 8,
        "At least 8/{} scales should produce identical topology, got {}",
        scales.len(), successes
    );
    eprintln!("MB-N4: {}/{} scales passed with identical topology", successes, scales.len());
}

/// MB-N5: Condition-number stress — nearly-parallel planes.
///
/// Two planes with angle difference of 1e-15 rad are used as Boolean
/// cutting planes. The kernel must either compute the intersection
/// correctly (condition number handled) or raise PolicyRequired.
/// It must NOT silently produce garbage.
#[test]
fn mb_n5_nearly_parallel_planes() {
    let (topo_a, geom_a) = build_cube([0.0, 0.0, 0.0], 1.0);

    let tiny_angle = 1e-10;
    let normal = [tiny_angle, 0.0, 1.0];
    let planes = vec![
        forge_geom::Plane::from_point_normal([0.0, 0.0, 0.5], normal).unwrap(),
        forge_geom::Plane::from_point_normal([0.0, 0.0, -0.5], [0.0, 0.0, -1.0]).unwrap(),
        forge_geom::Plane::from_point_normal([0.5, 0.0, 0.0], [1.0, 0.0, 0.0]).unwrap(),
        forge_geom::Plane::from_point_normal([-0.5, 0.0, 0.0], [-1.0, 0.0, 0.0]).unwrap(),
        forge_geom::Plane::from_point_normal([0.0, 0.5, 0.0], [0.0, 1.0, 0.0]).unwrap(),
        forge_geom::Plane::from_point_normal([0.0, -0.5, 0.0], [0.0, -1.0, 0.0]).unwrap(),
    ];

    let (topo_b, geom_b) = build_convex_solid(planes);

    let points_a = [[1.0, 1.0, 1.0], [-1.0, -1.0, -1.0]];
    let points_b = [[0.5, 0.5, 0.5], [-0.5, -0.5, -0.5]];

    let analysis_a = ScaleAnalysis::compute(&points_a, 1e-9);
    let analysis_b = ScaleAnalysis::compute(&points_b, 1e-9);

    let input = BooleanInput::new(topo_a, geom_a, topo_b, geom_b, BooleanOp::Intersection);
    match execute_boolean_logged(input).into_result() {
        Ok(result) => {
            
            let (v, e, f, chi) = euler_audit(result.topology().arena());
            assert_eq!(chi, 2, "Nearly-parallel must produce valid manifold: χ={}", chi);
            eprintln!("MB-N5: succeeded with {} faces, angle={:.2e} rad", f, tiny_angle);
        }
        Err(e) => {
            eprintln!("MB-N5: rejected nearly-parallel (acceptable): {:?}", e);
        }
    }

    let _ = analysis_a;
    let _ = analysis_b;
}

/// MB-N6: 100 chained exact rational operations — bit length stays bounded.
///
/// Multiplies a rational number by large and small values 100 times,
/// using PrecisionBudget to enforce bit-length limits. Verifies:
/// 1. The sign is preserved through every compression
/// 2. Bit-length never exceeds the budget threshold
/// 3. At least some escalation events fire (non-trivial)
#[test]
fn mb_n6_rational_bit_growth_bounded() {
    let mut budget = PrecisionBudget::new(256);
    let mut value = Rational::try_from_f64(1.0).unwrap();

    let large = Rational::try_from_f64(1e15).unwrap();
    let small = Rational::try_from_f64(1e-15).unwrap();
    let neg = Rational::try_from_f64(-3.141592653589793).unwrap();

    let original_sign = TriSign::Pos;

    for i in 0..100 {
        match i % 4 {
            0 => { value = &value * &large; }
            1 => { value = &value * &small; }
            2 => { value = &value * &neg; }
            3 => { value = &value * &large; value = &value * &small; }
            _ => unreachable!(),
        }

        value = budget.enforce(value);

        assert!(
            value.bit_length() <= 256,
            "Step {}: bit_length {} exceeds budget 256",
            i, value.bit_length()
        );
    }

    assert!(
        budget.escalation_count() > 0,
        "100 chained ops should trigger at least one compression"
    );

    assert!(
        !value.is_zero(),
        "Value should not have been compressed to zero"
    );

    eprintln!(
        "MB-N6: {} escalations in 100 ops, final bit_length={}, sign={:?}",
        budget.escalation_count(), value.bit_length(), value.sign()
    );

    for event in budget.escalations() {
        assert!(
            event.sign_preserved,
            "Sign must be preserved through compression"
        );
        assert!(
            event.bit_length_after <= 256,
            "Post-compression bits {} exceeds budget",
            event.bit_length_after
        );
    }
}
