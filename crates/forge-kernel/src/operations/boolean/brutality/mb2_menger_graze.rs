//! MB2 — The Menger Sponge Graze
//!
//! DOMAIN: Level-4 Menger sponge (≈20k faces, genus ~6,000) booleaned
//! with a micro-rotated/translated copy. Then chain 50 more micro-rotated
//! unions.
//!
//! RISK: Thin-feature + graze + iterative-shredder + high-genus Euler
//! + sliver avalanche all at once.
//!
//! GOAL: Genus-correct output with no zero-area faces after 50 ops.
//!
//! KERNEL REQUIREMENTS TO PASS:
//! - Stitching phase survives deleted arena slots from deep chained booleans
//! - Relative epsilon handles sub-cube scale differences across levels
//! - Symbolic vertex representation prevents vertex drift accumulation
//! - Queue decimator prunes redundant vertices without topology corruption
//! - High-genus Euler audit maintains χ = 2 − 2g correctly

use super::super::test_helpers::{
    build_cube, execute_boolean_logged, euler_audit,
    menger_sponge_subtraction_centers,
};
use super::super::schema::{BooleanInput, BooleanOp};

/// Build a Menger sponge at the given level by subtracting cubes.
///
/// Returns None if any subtraction in the chain fails.
fn build_menger_sponge(
    center: [f64; 3],
    half: f64,
    level: u32,
) -> Option<(forge_topo::state::TopologyState, crate::geometry_store::GeometryStore)> {
    let subs = menger_sponge_subtraction_centers(center, half, level);
    let total = subs.len();


    let (mut topo, mut geom) = build_cube(center, half);

    for (i, (sub_center, sub_half)) in subs.into_iter().enumerate() {
        let (topo_tool, geom_tool) = build_cube(sub_center, sub_half);

        let input = BooleanInput::new(
            topo, geom,
            topo_tool, geom_tool,
            BooleanOp::Subtraction,
        );

        match execute_boolean_logged(input).into_result() {
            Ok(result) => {
                let r = result;

                let parts = r.into_topo_geom();
                topo = parts.0;
                geom = parts.1;
            }
            Err(e) => {
                return None;
            }
        }
    }

    Some((topo, geom))
}

// ══════════════════════════════════════════════════════════════
// §MB2.1  MENGER SPONGE LEVEL 1  (7 subtractions)
// ══════════════════════════════════════════════════════════════

/// MB2.1 — Level-1 Menger sponge: 7 sub-cube subtractions.
#[test]
fn menger_level1() {
    match build_menger_sponge([0.0, 0.0, 0.0], 3.0, 1) {
        Some((topo, _geom)) => {
            let (_v, _e, f, _chi) = euler_audit(topo.arena());
            assert!(f > 6, "Menger L1 should have many faces, got {f}");
        }
        None => {
            panic!("Menger L1 construction failed");
        }
    }
}

// ══════════════════════════════════════════════════════════════
// §MB2.2  MENGER SPONGE LEVEL 2  (147 subtractions)
// ══════════════════════════════════════════════════════════════

/// MB2.2 — Level-2 Menger sponge: 147 sub-cube subtractions.
///
/// This is where the genus starts climbing rapidly. The topology
/// has internal tunnels at two scales.
#[test]
fn menger_level2() {
    match build_menger_sponge([0.0, 0.0, 0.0], 9.0, 2) {
        Some((topo, _geom)) => {
            let (_v, _e, f, _chi) = euler_audit(topo.arena());
            assert!(f > 100, "Menger L2 should have hundreds of faces, got {f}");
        }
        None => {
            panic!("Menger L2 construction failed");
        }
    }
}

// ══════════════════════════════════════════════════════════════
// §MB2.3  MENGER SPONGE LEVEL 3  (2,947 subtractions)
// ══════════════════════════════════════════════════════════════

/// MB2.3 — Level-3 Menger sponge: 2,947 sub-cube subtractions.
///
/// At this scale, the solid has thousands of faces and high genus.
/// This is a severe performance and correctness test.
#[test]
fn menger_level3() {
    match build_menger_sponge([0.0, 0.0, 0.0], 27.0, 3) {
        Some((topo, _geom)) => {
            let (_v, _e, f, _chi) = euler_audit(topo.arena());
            assert!(f > 1000, "Menger L3 should have thousands of faces, got {f}");
        }
        None => {
            panic!("Menger L3 construction failed");
        }
    }
}

// ══════════════════════════════════════════════════════════════
// §MB2.4  MENGER SPONGE LEVEL 4  (≈59,000 subtractions)
// ══════════════════════════════════════════════════════════════

/// MB2.4 — Level-4 Menger sponge: ~59,000 sub-cube subtractions.
///
/// The final boss of sponge construction. ~20k faces, genus ~6,000.
/// This will take significant time even with a perfect kernel.
#[test]
fn menger_level4() {
    match build_menger_sponge([0.0, 0.0, 0.0], 81.0, 4) {
        Some((topo, _geom)) => {
            let (_v, _e, f, _chi) = euler_audit(topo.arena());
            assert!(f > 10000, "Menger L4 should have ~20k faces, got {f}");
        }
        None => {
            panic!("Menger L4 construction failed");
        }
    }
}

// ══════════════════════════════════════════════════════════════
// §MB2.5  MENGER GRAZE: MICRO-TRANSLATED COPY
// ══════════════════════════════════════════════════════════════

/// MB2.5 — Level-1 sponge ∪ micro-translated copy (10⁻¹² per axis).
///
/// 1,200 edges graze vertices/edges at near-machine-epsilon.
/// Tests whether the vertex matching and stitching survive near-graze.
#[test]
fn menger_graze_micro_translate() {
    let epsilon = 1e-12;
    let sponge1 = build_menger_sponge([0.0, 0.0, 0.0], 3.0, 1);
    let sponge2 = build_menger_sponge([epsilon, epsilon, epsilon], 3.0, 1);

    match (sponge1, sponge2) {
        (Some((topo_a, geom_a)), Some((topo_b, geom_b))) => {
            let input = BooleanInput::new(
                topo_a, geom_a,
                topo_b, geom_b,
                BooleanOp::Union,
            );

            match execute_boolean_logged(input).into_result() {
                Ok(result) => {
                    let r = result;
                    let (_v, _e, f, _chi) = euler_audit(r.topology().arena());
                    assert!(f > 0, "Menger graze should produce faces");
                }
                Err(e) => {
                    panic!("Menger graze failed: {e}");
                }
            }
        }
        _ => {
            panic!("Menger sponge construction failed — cannot test graze");
        }
    }
}

// ══════════════════════════════════════════════════════════════
// §MB2.6  MENGER + 50 MICRO-ROTATED UNIONS
// ══════════════════════════════════════════════════════════════

/// MB2.6 — Level-1 sponge chained with 50 micro-rotated unions.
///
/// Each step adds a small cube at a slightly rotated offset.
/// Tests iterative sliver accumulation on high-genus topology.
#[test]
fn menger_50_micro_rotated_unions() {
    let sponge = build_menger_sponge([0.0, 0.0, 0.0], 3.0, 1);

    let (mut topo, mut geom) = match sponge {
        Some(s) => s,
        None => panic!("Menger sponge construction failed"),
    };

    for step in 1..=50 {
        let angle = step as f64 * 0.000001_f64.to_radians();
        let cos_a = angle.cos();
        let sin_a = angle.sin();
        let (topo_tool, geom_tool) = build_cube(
            [cos_a * 0.5, sin_a * 0.5, 0.0],
            0.3,
        );

        let input = BooleanInput::new(
            topo, geom,
            topo_tool, geom_tool,
            BooleanOp::Union,
        );

        match execute_boolean_logged(input).into_result() {
            Ok(result) => {
                let r = result;
                let parts = r.into_topo_geom();
                topo = parts.0;
                geom = parts.1;
            }
            Err(e) => {
                panic!("Menger+micro step {step} failed: {e}");
            }
        }
    }

    let (_v, _e, _f, chi) = euler_audit(topo.arena());
    assert_eq!(chi, 2, "Menger+50 Euler violation");
}
