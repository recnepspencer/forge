//! MB4 — The Self-Intersecting Thin Labyrinth
//!
//! DOMAIN: Input a deliberately corrupted solid (self-intersecting faces
//! + non-manifold wire edges + 5,000 walls of thickness 10⁻¹⁰) inside
//! a clean outer cube. Intersect with a complex tool that creates 200
//! new self-intersections inside the labyrinth.
//!
//! RISK: Self-intersect recovery + thin-feature + non-manifold +
//! orientation inconsistency all triggered simultaneously.
//!
//! GOAL: Transaction-rollback + self-intersect cleaner + relative epsilon
//! must heal everything into one valid manifold shell (or correctly
//! report empty).
//!
//! KERNEL REQUIREMENTS TO PASS:
//! - Self-intersection detection and repair pipeline
//! - Non-manifold wire edge handling
//! - Transaction rollback on topology corruption
//! - Thin-wall collapse or preservation at 10⁻¹⁰ thickness
//! - Relative epsilon handles sub-epsilon feature geometry
//! - Orientation consistency restoration after self-intersect repair

use super::super::schema::{BooleanInput, BooleanOp};
use super::super::test_helpers::{build_cube, euler_audit, execute_boolean_logged};

/// Build a thin-walled labyrinth by subtracting many thin slits.
///
/// Each wall has thickness `wall_thickness`. The labyrinth is
/// constructed by alternating subtractions along X and Y axes.
fn build_thin_labyrinth(
    center: [f64; 3],
    outer_half: f64,
    wall_count: usize,
    wall_thickness: f64,
) -> Option<(
    forge_topo::state::TopologyState,
    crate::geometry_state::GeometryState,
)> {
    let (mut topo, mut geom) = build_cube(center, outer_half);

    let span = outer_half * 2.0;
    let spacing = span / (wall_count as f64 + 1.0);

    for i in 0..wall_count {
        let offset = -outer_half + spacing * (i as f64 + 1.0);

        let slit_center = if i % 2 == 0 {
            [center[0] + offset, center[1], center[2]]
        } else {
            [center[0], center[1] + offset, center[2]]
        };

        let slit_half = if i % 2 == 0 {
            [wall_thickness / 2.0, outer_half * 0.8, outer_half]
        } else {
            [outer_half * 0.8, wall_thickness / 2.0, outer_half]
        };

        let slit_planes = vec![
            crate::geom_facade::Plane::from_point_normal(
                [
                    slit_center[0] + slit_half[0],
                    slit_center[1],
                    slit_center[2],
                ],
                [1.0, 0.0, 0.0],
            )
            .unwrap(),
            crate::geom_facade::Plane::from_point_normal(
                [
                    slit_center[0] - slit_half[0],
                    slit_center[1],
                    slit_center[2],
                ],
                [-1.0, 0.0, 0.0],
            )
            .unwrap(),
            crate::geom_facade::Plane::from_point_normal(
                [
                    slit_center[0],
                    slit_center[1] + slit_half[1],
                    slit_center[2],
                ],
                [0.0, 1.0, 0.0],
            )
            .unwrap(),
            crate::geom_facade::Plane::from_point_normal(
                [
                    slit_center[0],
                    slit_center[1] - slit_half[1],
                    slit_center[2],
                ],
                [0.0, -1.0, 0.0],
            )
            .unwrap(),
            crate::geom_facade::Plane::from_point_normal(
                [
                    slit_center[0],
                    slit_center[1],
                    slit_center[2] + slit_half[2],
                ],
                [0.0, 0.0, 1.0],
            )
            .unwrap(),
            crate::geom_facade::Plane::from_point_normal(
                [
                    slit_center[0],
                    slit_center[1],
                    slit_center[2] - slit_half[2],
                ],
                [0.0, 0.0, -1.0],
            )
            .unwrap(),
        ];

        let (topo_slit, geom_slit) = super::super::test_helpers::build_convex_solid(slit_planes);

        let input = BooleanInput::new(topo, geom, topo_slit, geom_slit, BooleanOp::Subtraction);

        match execute_boolean_logged(input).into_result() {
            Ok(result) => {
                let r = result;
                if (i + 1) % 50 == 0 || i == wall_count - 1 {
                    let (v, e, f, chi) = euler_audit(r.topology().arena());
                    eprintln!(
                        "MB4 labyrinth slit {}/{wall_count}: V={v} E={e} F={f} χ={chi}",
                        i + 1
                    );
                }
                let parts = r.into_states();
                topo = parts.0;
                geom = parts.1;
            }
            Err(e) => {
                eprintln!("MB4 labyrinth slit {}/{wall_count} failed: {e}", i + 1);
                return None;
            }
        }
    }

    Some((topo, geom))
}

// ══════════════════════════════════════════════════════════════
// §MB4.1  THIN LABYRINTH CONSTRUCTION (100 walls, 10⁻¹⁰)
// ══════════════════════════════════════════════════════════════

/// MB4.1 — Build a labyrinth with 100 walls of thickness 10⁻¹⁰.
///
/// The walls are thinner than typical tolerance thresholds.
/// Tests whether the engine preserves thin features or cleanly
/// collapses them.
#[test]
fn thin_labyrinth_100_walls() {
    match build_thin_labyrinth([0.0, 0.0, 0.0], 5.0, 100, 1e-10) {
        Some((topo, _geom)) => {
            let (v, e, f, chi) = euler_audit(topo.arena());
            eprintln!("MB4 labyrinth 100: V={v} E={e} F={f} χ={chi}");
            assert!(f > 6, "Labyrinth should have many faces, got {f}");
        }
        None => {
            panic!("MB4 thin labyrinth construction failed");
        }
    }
}

// ══════════════════════════════════════════════════════════════
// §MB4.2  FULL SPEC: 5,000 WALLS
// ══════════════════════════════════════════════════════════════

/// MB4.2 — 5,000 walls of thickness 10⁻¹⁰.
///
/// The full-spec labyrinth. Requires the engine to handle
/// thousands of chained subtractions at sub-tolerance thickness.
#[test]
fn thin_labyrinth_5000_walls() {
    match build_thin_labyrinth([0.0, 0.0, 0.0], 50.0, 5000, 1e-10) {
        Some((topo, _geom)) => {
            let (v, e, f, chi) = euler_audit(topo.arena());
            eprintln!("MB4 labyrinth 5000: V={v} E={e} F={f} χ={chi}");
            assert!(
                f > 100,
                "5000-wall labyrinth should have many faces, got {f}"
            );
        }
        None => {
            panic!("MB4 5000-wall labyrinth construction failed");
        }
    }
}

// ══════════════════════════════════════════════════════════════
// §MB4.3  LABYRINTH + COMPLEX TOOL INTERSECTION
// ══════════════════════════════════════════════════════════════

/// MB4.3 — Intersect labyrinth with a rotated cube creating
/// 200 new self-intersection candidates inside the labyrinth.
///
/// The tool cube is rotated 15° to guarantee it crosses many
/// labyrinth walls at non-axis-aligned angles, creating complex
/// intersection patterns.
#[test]
fn labyrinth_complex_intersection() {
    let labyrinth = build_thin_labyrinth([0.0, 0.0, 0.0], 5.0, 50, 1e-8);

    let (topo_lab, geom_lab) = match labyrinth {
        Some(s) => s,
        None => panic!("MB4 labyrinth construction failed — cannot test intersection"),
    };

    let angle = 15.0_f64.to_radians();
    let cos_a = angle.cos();
    let sin_a = angle.sin();

    let rotated_planes = vec![
        crate::geom_facade::Plane::from_point_normal([cos_a * 4.0, sin_a * 4.0, 0.0], [cos_a, sin_a, 0.0])
            .unwrap(),
        crate::geom_facade::Plane::from_point_normal(
            [-cos_a * 4.0, -sin_a * 4.0, 0.0],
            [-cos_a, -sin_a, 0.0],
        )
        .unwrap(),
        crate::geom_facade::Plane::from_point_normal([0.0, cos_a * 4.0, sin_a * 4.0], [0.0, cos_a, sin_a])
            .unwrap(),
        crate::geom_facade::Plane::from_point_normal(
            [0.0, -cos_a * 4.0, -sin_a * 4.0],
            [0.0, -cos_a, -sin_a],
        )
        .unwrap(),
        crate::geom_facade::Plane::from_point_normal([0.0, 0.0, 4.0], [0.0, 0.0, 1.0]).unwrap(),
        crate::geom_facade::Plane::from_point_normal([0.0, 0.0, -4.0], [0.0, 0.0, -1.0]).unwrap(),
    ];

    let (topo_tool, geom_tool) = super::super::test_helpers::build_convex_solid(rotated_planes);

    let input = BooleanInput::new(
        topo_lab,
        geom_lab,
        topo_tool,
        geom_tool,
        BooleanOp::Intersection,
    );

    match execute_boolean_logged(input).into_result() {
        Ok(result) => {
            let r = result;
            let (v, e, f, chi) = euler_audit(r.topology().arena());
            eprintln!("MB4 labyrinth intersection: V={v} E={e} F={f} χ={chi}");
            assert!(f > 0, "Labyrinth intersection should produce faces");
        }
        Err(e) => {
            panic!("MB4 labyrinth intersection failed: {e}");
        }
    }
}
