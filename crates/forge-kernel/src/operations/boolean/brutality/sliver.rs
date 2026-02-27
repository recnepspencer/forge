use super::super::schema::{BooleanInput, BooleanOp};
use super::super::test_helpers::{build_cube, execute_boolean_logged};
use crate::core::ModelingContext;
use crate::mesh_builder::build_halfedge_mesh;
use crate::geom_facade::{build_convex_polyhedron, BspConfig};
use crate::geom_facade::Plane;

// ══════════════════════════════════════════════════════════════
// §7  SLIVER BUDGET TORTURE
// ══════════════════════════════════════════════════════════════

/// 7.1 — Shallow-Angle Intersection
///
/// Two solids intersecting at extremely shallow angle (~1°).
#[test]
fn shallow_angle_intersection() {
    let angle_rad = 1.0_f64.to_radians();
    let cos_a = angle_rad.cos();
    let sin_a = angle_rad.sin();

    let tilted_planes = vec![
        Plane::from_point_normal([cos_a, sin_a, 0.0], [cos_a, sin_a, 0.0]).unwrap(),
        Plane::from_point_normal([-cos_a, -sin_a, 0.0], [-cos_a, -sin_a, 0.0]).unwrap(),
        Plane::from_point_normal([0.0, 1.0, 0.0], [0.0, 1.0, 0.0]).unwrap(),
        Plane::from_point_normal([0.0, -1.0, 0.0], [0.0, -1.0, 0.0]).unwrap(),
        Plane::from_point_normal([0.0, 0.0, 1.0], [0.0, 0.0, 1.0]).unwrap(),
        Plane::from_point_normal([0.0, 0.0, -1.0], [0.0, 0.0, -1.0]).unwrap(),
    ];

    let cell_b = build_convex_polyhedron(&tilted_planes, &BspConfig::default());

    match cell_b {
        Ok(cell) => {
            let mut ctx = ModelingContext::new();
            let mesh_result = build_halfedge_mesh(&cell, &mut ctx);
            let mr = mesh_result.expect("Mesh build for tilted solid must not fail");
            let (topo_b, geom_b) = mr.into_parts();
            let (topo_a, geom_a) = build_cube([0.0, 0.0, 0.0], 1.0);

            let input = BooleanInput::new(topo_a, geom_a, topo_b, geom_b, BooleanOp::Intersection);
            let result = execute_boolean_logged(input);
            let r = result
                .into_result()
                .expect("Shallow-angle boolean must not fail");
            let arena = r.topology().arena();
            let v = arena.vertex_count() as isize;
            let e = (arena.half_edge_count() / 2) as isize;
            let f = arena.face_count() as isize;
            assert!(f > 0, "Shallow-angle intersection should produce faces");
            assert_eq!(
                v - e + f,
                2,
                "Shallow-angle Euler violation: V={v} E={e} F={f}"
            );
        }
        Err(e) => panic!("BSP for tilted solid must not fail: {e}"),
    }
}
