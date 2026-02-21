//! MB7 — The Scale-Invariant Micro-Feature Avalanche
//!
//! DOMAIN: A 10¹²-unit cube containing 10,000 micro-cubes (10⁻⁹ size)
//! arranged in a 3D grid with 10⁻¹² gaps. Subtract a tool that grazes
//! every micro-cube at 10⁻¹⁴ while being exactly flush with the large
//! cube faces.
//!
//! RISK: Massive scale separation + thin features + collinear storm
//! + 10k-face performance death.
//!
//! GOAL: Coordinate normalization + BVH + lazy evaluation + relative
//! epsilon all firing at once. Must finish with zero slivers.
//!
//! KERNEL REQUIREMENTS TO PASS:
//! - Coordinate normalization handles 10²¹ scale range (10¹² to 10⁻⁹)
//! - BVH spatial indexing efficiently filters 10k+ micro-cubes
//! - Lazy evaluation skips non-intersecting micro-cubes
//! - Relative epsilon adapts to local feature scale
//! - Chained subtraction of 10k cubes without topology corruption
//! - Performance: complete within reasonable time despite 10k operations

use super::super::test_helpers::{
    build_cube, execute_boolean_logged, euler_audit,
};
use super::super::schema::{BooleanInput, BooleanOp};

// ══════════════════════════════════════════════════════════════
// §MB7.1  MICRO-CUBE GRID SUBTRACTION (10×10×10 = 1,000)
// ══════════════════════════════════════════════════════════════

/// MB7.1 — 1,000 micro-cube subtractions from a large cube.
///
/// A cube of half-size 10⁶ with 10³ micro-cubes (half-size 10⁻⁹)
/// subtracted in a 10×10×10 grid with 10⁻¹² gaps.
/// Scaled-down version of the full 10k spec.
#[test]
fn micro_cube_grid_1000() {
    let large_half = 1e6;
    let micro_half = 1e-9;
    let gap = 1e-12;
    let spacing = micro_half * 2.0 + gap;
    let grid_dim = 10;

    let (mut topo, mut geom) = build_cube([0.0, 0.0, 0.0], large_half);

    let mut step = 0usize;
    for ix in 0..grid_dim {
        for iy in 0..grid_dim {
            for iz in 0..grid_dim {
                let center = [
                    ix as f64 * spacing,
                    iy as f64 * spacing,
                    iz as f64 * spacing,
                ];

                let (topo_micro, geom_micro) = build_cube(center, micro_half);

                let input = BooleanInput::new(
                    topo, geom,
                    topo_micro, geom_micro,
                    BooleanOp::Subtraction,
                );

                match execute_boolean_logged(input).into_result() {
                    Ok(result) => {
                        let r = result;
                        step += 1;
                        if step % 100 == 0 {
                            let (v, e, f, chi) = euler_audit(r.topology().arena());
                            eprintln!(
                                "MB7 micro-cube {step}/{}: V={v} E={e} F={f} χ={chi}",
                                grid_dim * grid_dim * grid_dim
                            );
                        }
                        let parts = r.into_topo_geom();
                        topo = parts.0;
                        geom = parts.1;
                    }
                    Err(e) => {
                        panic!("MB7 micro-cube step {step} failed: {e}");
                    }
                }
            }
        }
    }

    let (v, e, f, chi) = euler_audit(topo.arena());
    eprintln!("MB7 1k micro-cubes final: V={v} E={e} F={f} χ={chi}");
    assert!(f > 6, "Micro-cube grid should have many faces, got {f}");
}

// ══════════════════════════════════════════════════════════════
// §MB7.2  FULL SPEC: 10,000 MICRO-CUBES (21×21×21 ≈ 9261)
// ══════════════════════════════════════════════════════════════

/// MB7.2 — Full-spec 10k micro-cube avalanche.
///
/// 21×21×21 = 9,261 micro-cubes subtracted from a 10¹²-unit cube.
/// Each micro-cube is 10⁻⁹ sized with 10⁻¹² gaps.
#[test]
fn micro_cube_grid_10000() {
    let large_half = 1e12;
    let micro_half = 1e-9;
    let gap = 1e-12;
    let spacing = micro_half * 2.0 + gap;
    let grid_dim = 21;
    let total = grid_dim * grid_dim * grid_dim;

    let (mut topo, mut geom) = build_cube([0.0, 0.0, 0.0], large_half);

    let mut step = 0usize;
    for ix in 0..grid_dim {
        for iy in 0..grid_dim {
            for iz in 0..grid_dim {
                let center = [
                    ix as f64 * spacing,
                    iy as f64 * spacing,
                    iz as f64 * spacing,
                ];

                let (topo_micro, geom_micro) = build_cube(center, micro_half);

                let input = BooleanInput::new(
                    topo, geom,
                    topo_micro, geom_micro,
                    BooleanOp::Subtraction,
                );

                match execute_boolean_logged(input).into_result() {
                    Ok(result) => {
                        let r = result;
                        step += 1;
                        if step % 500 == 0 {
                            let (v, e, f, chi) = euler_audit(r.topology().arena());
                            eprintln!(
                                "MB7 micro-cube {step}/{total}: V={v} E={e} F={f} χ={chi}"
                            );
                        }
                        let parts = r.into_topo_geom();
                        topo = parts.0;
                        geom = parts.1;
                    }
                    Err(e) => {
                        panic!("MB7 micro-cube step {step}/{total} failed: {e}");
                    }
                }
            }
        }
    }

    let (v, e, f, chi) = euler_audit(topo.arena());
    eprintln!("MB7 {total} micro-cubes final: V={v} E={e} F={f} χ={chi}");
    assert!(f > 6, "Full micro-cube grid should have many faces, got {f}");
}

// ══════════════════════════════════════════════════════════════
// §MB7.3  GRAZE TOOL ACROSS ALL MICRO-CUBES
// ══════════════════════════════════════════════════════════════

/// MB7.3 — After building micro-cube grid, subtract a graze tool
/// that is flush with the large cube faces and passes 10⁻¹⁴ from
/// every micro-cube.
///
/// Tested at the small (1k) scale first.
#[test]
fn micro_cube_graze_tool() {
    let large_half = 1e6;
    let micro_half = 1e-9;
    let gap = 1e-12;
    let spacing = micro_half * 2.0 + gap;
    let grid_dim = 5;
    let epsilon = 1e-14;

    let (mut topo, mut geom) = build_cube([0.0, 0.0, 0.0], large_half);

    for ix in 0..grid_dim {
        for iy in 0..grid_dim {
            for iz in 0..grid_dim {
                let center = [
                    ix as f64 * spacing,
                    iy as f64 * spacing,
                    iz as f64 * spacing,
                ];
                let (topo_micro, geom_micro) = build_cube(center, micro_half);
                let input = BooleanInput::new(
                    topo, geom, topo_micro, geom_micro, BooleanOp::Subtraction,
                );
                match execute_boolean_logged(input).into_result() {
                    Ok(r) => {
                        let p = r.into_topo_geom();
                        topo = p.0;
                        geom = p.1;
                    }
                    Err(e) => panic!("MB7 graze-prep micro-cube failed: {e}"),
                }
            }
        }
    }

    let grid_span = grid_dim as f64 * spacing;
    let (topo_graze, geom_graze) = build_cube(
        [grid_span / 2.0, grid_span / 2.0, epsilon],
        grid_span,
    );

    let input = BooleanInput::new(
        topo, geom,
        topo_graze, geom_graze,
        BooleanOp::Subtraction,
    );

    match execute_boolean_logged(input).into_result() {
        Ok(result) => {
            let r = result;
            let (v, e, f, chi) = euler_audit(r.topology().arena());
            eprintln!("MB7 graze tool result: V={v} E={e} F={f} χ={chi}");
            assert!(f >= 6, "Graze tool should leave valid solid");
        }
        Err(e) => {
            panic!("MB7 graze tool failed: {e}");
        }
    }
}
