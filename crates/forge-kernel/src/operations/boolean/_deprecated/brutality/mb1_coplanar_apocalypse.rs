//! MB1 — The Coplanar Overlap Apocalypse
//!
//! DOMAIN: Two complex solids whose hundreds of faces are exactly coplanar
//! in 12 separate overlapping regions (partial overlaps, nested holes,
//! figure-8 boundaries), then differenced with a third solid that grazes
//! all 12 planes at 10⁻¹⁴.
//!
//! RISK: False intersection edges + inconsistent "which face wins" +
//! sliver explosion + orientation flip cascade.
//!
//! GOAL: Clean manifold with zero false edges.
//!
//! KERNEL REQUIREMENTS TO PASS:
//! - Robust lex-tie-breaker for coplanar face arbitration
//! - Flush/coincident logic stress-tested at 800+ face scale
//! - Stitching phase handles deleted arena slots from chained booleans
//! - Coplanar face merge pass must not create T-junctions
//! - Orientation consistency across touching coplanar regions

use super::super::schema::{BooleanInput, BooleanOp};
use super::super::test_helpers::{build_cube, euler_audit, execute_boolean_ember};

/// Build a complex solid by unioning cubes in a 4×4×4 grid.
///
/// Creates a solid with many coplanar internal boundaries.
/// The cubes share faces along grid lines, producing exactly
/// coplanar face pairs throughout the volume.
fn build_coplanar_grid_solid(
    origin: [f64; 3],
    cube_half: f64,
    grid_dim: usize,
) -> Option<(
    forge_topo::transactions::TopologyState,
    crate::geometry_state::GeometryState,
)> {
    let step = cube_half * 2.0;
    let (mut topo, mut geom) = build_cube(origin, cube_half);

    let mut step_count = 1;
    let total_steps = grid_dim * grid_dim * grid_dim - 1;

    for ix in 0..grid_dim {
        for iy in 0..grid_dim {
            for iz in 0..grid_dim {
                if ix == 0 && iy == 0 && iz == 0 {
                    continue;
                }
                let center = [
                    origin[0] + ix as f64 * step,
                    origin[1] + iy as f64 * step,
                    origin[2] + iz as f64 * step,
                ];
                let (topo_tool, geom_tool) = build_cube(center, cube_half);

                let input = BooleanInput::new(topo, geom, topo_tool, geom_tool, BooleanOp::Union);

                eprintln!(
                    "\n[chain] step {}/{} — Union at [{:.1}, {:.1}, {:.1}] ({}F in)",
                    step_count,
                    total_steps,
                    center[0],
                    center[1],
                    center[2],
                    input.target_topology().arena().face_count()
                );

                match execute_boolean_ember(input).into_result() {
                    Ok(result) => {
                        let r = result;
                        let parts = r.into_states();
                        topo = parts.0;
                        geom = parts.1;
                        step_count += 1;
                    }
                    Err(_e) => {
                        eprintln!("[chain] step {} FAILED: {:?}", step_count, _e);
                        return None;
                    }
                }
            }
        }
    }

    Some((topo, geom))
}

// ══════════════════════════════════════════════════════════════
// §MB1.1  FULL COPLANAR GRID (4×4×4 = 63 unions)
// ══════════════════════════════════════════════════════════════

/// MB1.1 — 4×4×4 grid of cubes fused into one solid.
///
/// 63 sequential union operations, each creating coplanar shared faces.
/// The merged solid should have exactly 6 faces (a big box), but
/// intermediate steps create hundreds of coplanar internal boundaries
/// that must be resolved.
#[test]
fn coplanar_grid_4x4x4() {
    match build_coplanar_grid_solid([0.0, 0.0, 0.0], 0.5, 4) {
        Some((topo, _geom)) => {
            let (_v, _e, f, chi) = euler_audit(topo.arena());
            assert_eq!(chi, 2, "MB1 grid Euler violation");
            assert_eq!(
                f, 6,
                "4×4×4 flush grid should merge to 6 faces (one box), got {f}"
            );
        }
        None => {
            panic!("MB1 4×4×4 grid construction failed — chained booleans not robust enough");
        }
    }
}

// ══════════════════════════════════════════════════════════════
// §MB1.2  PARTIAL OVERLAP REGIONS
// ══════════════════════════════════════════════════════════════

/// MB1.2 — 12 overlapping regions with partial coplanar overlaps.
///
/// Cubes overlap by 50%, creating figure-8-like boundary intersections
/// where coplanar faces partially overlap. The merge pass must handle
/// partial-overlap correctly without creating false edges.
#[test]
fn coplanar_partial_overlap_12_regions() {
    let (mut topo, mut geom) = build_cube([0.0, 0.0, 0.0], 1.0);

    let positions: Vec<[f64; 3]> = (0..12)
        .map(|i| {
            let angle = (i as f64) * std::f64::consts::PI / 6.0;
            let r = 1.0;
            [r * angle.cos(), r * angle.sin(), 0.0]
        })
        .collect();

    for (i, pos) in positions.iter().enumerate() {
        let (topo_tool, geom_tool) = build_cube(*pos, 1.0);

        let input = BooleanInput::new(topo, geom, topo_tool, geom_tool, BooleanOp::Union);

        eprintln!(
            "\n[chain] step {}/12 — Union at [{:.1}, {:.1}, {:.1}] ({}F in)",
            i + 1,
            pos[0],
            pos[1],
            pos[2],
            input.target_topology().arena().face_count()
        );

        match execute_boolean_ember(input).into_result() {
            Ok(result) => {
                let r = result;
                let (_v, _e, _f, chi) = euler_audit(r.topology().arena());
                assert_eq!(chi, 2, "MB1 partial step {i} Euler violation");
                let parts = r.into_states();
                topo = parts.0;
                geom = parts.1;
            }
            Err(e) => {
                panic!("MB1 partial-overlap step {i} failed: {e}");
            }
        }
    }

    let (_v, _e, _f, _chi) = euler_audit(topo.arena());
}

// ══════════════════════════════════════════════════════════════
// §MB1.3  GRAZE ALL 12 COPLANAR PLANES
// ══════════════════════════════════════════════════════════════

/// MB1.3 — Build coplanar grid, then subtract a graze solid at 10⁻¹⁴.
///
/// The graze solid's faces are offset by 10⁻¹⁴ from the grid's coplanar
/// planes, triggering the most dangerous near-coincident classification.
#[test]
fn coplanar_graze_at_1e14() {
    let solid = build_coplanar_grid_solid([0.0, 0.0, 0.0], 0.5, 3);

    match solid {
        Some((topo, geom)) => {
            let epsilon = 1e-14;
            let (topo_graze, geom_graze) = build_cube([1.5, 1.5, epsilon], 3.0);

            let input =
                BooleanInput::new(topo, geom, topo_graze, geom_graze, BooleanOp::Subtraction);

            match execute_boolean_ember(input).into_result() {
                Ok(result) => {
                    let r = result;
                    let (_v, _e, f, chi) = euler_audit(r.topology().arena());
                    assert_eq!(chi, 2, "MB1 graze Euler violation");
                    assert!(f >= 6, "MB1 graze should produce faces");
                }
                Err(e) => {
                    panic!("MB1 graze failed: {e}");
                }
            }
        }
        None => {
            panic!("MB1 grid construction failed — cannot test graze");
        }
    }
}

/// MB1.4 — Dense collinear edge storm: 50 points spaced at 10⁻¹⁵.
///
/// 50 cubes with faces separated by 10⁻¹⁵. Creates nearly-identical
/// intersection edges that must be collapsed or handled without
/// orientation flips.
#[test]
fn collinear_point_storm_50() {
    let (mut topo, mut geom) = build_cube([0.0, 0.0, 0.0], 1.0);
    let epsilon = 1e-15;

    for i in 0..50 {
        let offset = i as f64 * epsilon;
        let (topo_tool, geom_tool) = build_cube([1.0 + offset, 0.0, 0.0], 1.0);

        let input = BooleanInput::new(topo, geom, topo_tool, geom_tool, BooleanOp::Union);

        match execute_boolean_ember(input).into_result() {
            Ok(result) => {
                let r = result;
                let parts = r.into_states();
                topo = parts.0;
                geom = parts.1;
            }
            Err(e) => {
                panic!("MB1 collinear step {i} failed: {e}");
            }
        }
    }

    let (_v, _e, _f, chi) = euler_audit(topo.arena());
    assert_eq!(chi, 2, "MB1 collinear Euler violation");
}
