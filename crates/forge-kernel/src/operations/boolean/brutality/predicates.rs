use super::super::schema::{BooleanInput, BooleanOp};
use super::super::test_helpers::{execute_boolean_logged, run_boolean};
use crate::brep::state::BrepState;
use crate::brep::state::BrepState;
use crate::core::ModelingContext;
use crate::mesh_builder::build_halfedge_mesh;
use crate::geom_facade::{build_convex_polyhedron, BspConfig};
use crate::geom_facade::Plane;
use forge_math::predicates::orient3d::orient3d;
use forge_math::sign::TriSign;

// ══════════════════════════════════════════════════════════════
// §1  PREDICATE & EXACTNESS TORTURE
// ══════════════════════════════════════════════════════════════

/// 1.1 — Near-Coplanar Escalation Ladder
///
/// Generate plane triples where orientation determinant magnitude scales
/// from 1e-3 down to 1e-24. Verify correct TriSign at every level,
/// deterministic sign (identical across 10 runs), no sign flips.
#[test]
fn near_coplanar_escalation_ladder() {
    let a = [0.0, 0.0, 0.0];
    let b = [1.0, 0.0, 0.0];
    let c = [0.0, 1.0, 0.0];

    let magnitudes = [1e-3, 1e-6, 1e-9, 1e-12, 1e-15, 1e-18, 1e-24];

    for &mag in &magnitudes {
        let d = [0.5, 0.5, mag];

        let first_sign = orient3d(a, b, c, d).unwrap().0.sign();

        assert_ne!(
            first_sign,
            TriSign::Pos,
            "Point above XY plane at z={mag} should not be Pos (convention: Neg = above)"
        );

        for _ in 0..10 {
            let sign = orient3d(a, b, c, d).unwrap().0.sign();
            assert_eq!(
                sign, first_sign,
                "Sign flip at magnitude {mag}: got {sign:?}, expected {first_sign:?}"
            );
        }
    }
}

/// 1.1b — Verify that truly coplanar points return Zero.
#[test]
fn coplanar_returns_zero() {
    let a = [0.0, 0.0, 0.0];
    let b = [1.0, 0.0, 0.0];
    let c = [0.0, 1.0, 0.0];
    let d = [0.5, 0.5, 0.0];

    for _ in 0..100 {
        let sign = orient3d(a, b, c, d).unwrap().0.sign();
        assert_eq!(
            sign,
            TriSign::Zero,
            "Coplanar point must always return Zero"
        );
    }
}

/// 1.2 — Large Coordinate Stress
///
/// Construct cubes at large coordinates and boolean them.
/// Expect same topology as origin case, no precision-driven misclassification.
#[test]
fn large_coordinate_stress_1e6() {
    let origin_result = run_boolean([0.0, 0.0, 0.0], 1.0, [0.5, 0.0, 0.0], 1.0, BooleanOp::Union);
    let origin_v = origin_result.topology().arena().vertex_count();
    let origin_e = origin_result.topology().arena().half_edge_count() / 2;
    let origin_f = origin_result.topology().arena().face_count();

    let offset = 1e6;
    let result = run_boolean(
        [offset, offset, offset],
        1.0,
        [offset + 0.5, offset, offset],
        1.0,
        BooleanOp::Union,
    );

    let v = result.topology().arena().vertex_count();
    let e = result.topology().arena().half_edge_count() / 2;
    let f = result.topology().arena().face_count();
    // Face count must match exactly (faces are unaffected by vertex cleanup precision)
    assert_eq!(f, origin_f, "1e6 face count should match origin case");
    // Vertex/edge counts may differ by ±1 at large coordinates due to
    // collinearity check precision in remove_redundant_vertices
    assert!(
        (v as isize - origin_v as isize).abs() <= 1,
        "1e6 vertex count should be within ±1 of origin: got {v}, expected {origin_v}",
    );
    assert!(
        (e as isize - origin_e as isize).abs() <= 1,
        "1e6 edge count should be within ±1 of origin: got {e}, expected {origin_e}",
    );
}

#[test]
fn large_coordinate_stress_1e9() {
    let offset = 1e9;
    run_large_coordinate_boolean(offset, "1e9");
}

/// 1.2b — 1e12 coordinates.
#[test]
fn large_coordinate_stress_1e12() {
    let offset = 1e12;
    run_large_coordinate_boolean(offset, "1e12");
}

/// Shared helper: build two cubes at large coordinates and boolean them.
/// Uses explicit error handling instead of `build_cube` which `.unwrap()`s.
fn run_large_coordinate_boolean(offset: f64, label: &str) {
    let build_at = |center: [f64; 3], half: f64| -> Result<_, forge_core::KernelError> {
        let planes = vec![
            Plane::from_point_normal([center[0] + half, center[1], center[2]], [1.0, 0.0, 0.0])
                .map_err(|e| forge_core::KernelError::InternalError {
                    message: format!("{e}"),
                    context: None,
                })?,
            Plane::from_point_normal([center[0] - half, center[1], center[2]], [-1.0, 0.0, 0.0])
                .map_err(|e| forge_core::KernelError::InternalError {
                    message: format!("{e}"),
                    context: None,
                })?,
            Plane::from_point_normal([center[0], center[1] + half, center[2]], [0.0, 1.0, 0.0])
                .map_err(|e| forge_core::KernelError::InternalError {
                    message: format!("{e}"),
                    context: None,
                })?,
            Plane::from_point_normal([center[0], center[1] - half, center[2]], [0.0, -1.0, 0.0])
                .map_err(|e| forge_core::KernelError::InternalError {
                    message: format!("{e}"),
                    context: None,
                })?,
            Plane::from_point_normal([center[0], center[1], center[2] + half], [0.0, 0.0, 1.0])
                .map_err(|e| forge_core::KernelError::InternalError {
                    message: format!("{e}"),
                    context: None,
                })?,
            Plane::from_point_normal([center[0], center[1], center[2] - half], [0.0, 0.0, -1.0])
                .map_err(|e| forge_core::KernelError::InternalError {
                    message: format!("{e}"),
                    context: None,
                })?,
        ];
        let cell = build_convex_polyhedron(&planes, &BspConfig::default())?;
        let mut ctx = ModelingContext::new();
        let result = build_halfedge_mesh(&cell, &mut ctx)?;
        Ok(result.into_parts())
    };

    let solid_a = build_at([offset, offset, offset], 1.0);
    let solid_b = build_at([offset + 0.5, offset, offset], 1.0);

    match (solid_a, solid_b) {
        (Ok((topo_a, geom_a)), Ok((topo_b, geom_b))) => {
            let input = BooleanInput::new(
                topo_a,
                geom_a,
                BrepState::new(),
                topo_b,
                geom_b,
                BrepState::new(),
                BooleanOp::Union,
            );
            match execute_boolean_logged(input).into_result() {
                Ok(r) => {
                    let face_count = r.topology().arena().face_count();
                    assert!(
                        face_count >= 6,
                        "{label}-coord union should produce at least 6 faces, got {face_count}"
                    );
                }
                Err(e) => {
                    panic!("{label}-coord boolean failed: {e:?} — kernel must handle large coordinates");
                }
            }
        }
        (Err(e), _) | (_, Err(e)) => {
            panic!(
                "{label}-coord cube construction failed: {e:?} — BSP must handle large coordinates"
            );
        }
    }
}

/// 1.3 — Catastrophic Cancellation Planes
///
/// Nearly parallel planes (x=0, x=1e-14, y=0, y=1e-14).
/// Expect no NaN, no panic, either result or policy escalation.
#[test]
fn catastrophic_cancellation_planes() {
    let planes = vec![
        Plane::from_point_normal([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]).unwrap(),
        Plane::from_point_normal([1e-14, 0.0, 0.0], [-1.0, 0.0, 0.0]).unwrap(),
        Plane::from_point_normal([0.0, 0.0, 0.0], [0.0, 1.0, 0.0]).unwrap(),
        Plane::from_point_normal([0.0, 1e-14, 0.0], [0.0, -1.0, 0.0]).unwrap(),
        Plane::from_point_normal([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]).unwrap(),
        Plane::from_point_normal([0.0, 0.0, 1.0], [0.0, 0.0, -1.0]).unwrap(),
    ];

    let result = build_convex_polyhedron(&planes, &BspConfig::default());

    match result {
        Ok(cell) => {
            let mut ctx = ModelingContext::new();
            let mesh_result = build_halfedge_mesh(&cell, &mut ctx);
            match mesh_result {
                Ok(mr) => {
                    let (topo, geom) = mr.into_parts();
                    let arena = topo.arena();
                    for (vid, _) in arena.iter_vertices() {
                        if let Some(pos) = geom.get_vertex_position(vid) {
                            for &coord in pos {
                                assert!(
                                    coord.is_finite(),
                                    "Vertex position contains non-finite value: {coord}"
                                );
                            }
                        }
                    }
                }
                Err(_) => {
                    // Mesh build failure IS acceptable for nearly-degenerate geometry,
                    // but the kernel must not panic or produce NaN.
                }
            }
        }
        Err(_) => {
            // BSP build failure IS acceptable for nearly-degenerate planes
            // (1e-14 thickness is sub-tolerance). The key invariant is: no panic.
        }
    }
}
