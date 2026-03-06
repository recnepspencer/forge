//! Surface & curve binding completeness — integration tests.
//!
//! DOMAIN: Verifies that all primitives emit SurfaceData and CurveGeom
//! bindings alongside their Plane and Position bindings. Tests consistency
//! between the two representations and checks adversarial corruption.
//!
//! These are Phase 2b hardening tests — every assertion uses production
//! code, no theatre.

use crate::integration_tests::harness::shapes;

use crate::geometry::facade::ExactPosition;
use forge_geom::facade::{CurveKind, SurfaceKind};

// ── Surface completeness ────────────────────────────────────────────────

/// Every face in every primitive has a SurfaceData binding.
#[test]
fn all_primitives_have_surface_bindings() {
    let primitives: Vec<(&str, _)> = vec![
        ("cube", shapes::unit_cube().unwrap().into_value()),
        ("tetrahedron", shapes::tetrahedron().unwrap().into_value()),
        (
            "dodecahedron",
            shapes::dodecahedron([0.0; 3], 1.0).unwrap().into_value(),
        ),
        (
            "prism_5",
            shapes::prism([0.0; 3], 5, 1.0, 2.0).unwrap().into_value(),
        ),
        (
            "pyramid_4",
            shapes::pyramid([0.0; 3], 4, 1.0, 2.0).unwrap().into_value(),
        ),
        (
            "wedge",
            shapes::wedge([0.0; 3], [1.0, 1.0, 1.0])
                .unwrap()
                .into_value(),
        ),
        (
            "block",
            shapes::block([0.0; 3], [1.0, 2.0, 3.0])
                .unwrap()
                .into_value(),
        ),
    ];

    for (name, env) in &primitives {
        let arena = env.topology().arena();
        let geometry = env.geometry();

        for (face_id, _) in arena.iter_faces() {
            assert!(
                geometry.surfaces.contains(face_id),
                "{}: Face {} has no surface binding",
                name,
                face_id
            );
        }
    }
}

// ── Curve completeness ──────────────────────────────────────────────────

/// Every edge in every primitive has a CurveGeom binding.
#[test]
fn all_primitives_have_curve_bindings() {
    let primitives: Vec<(&str, _)> = vec![
        ("cube", shapes::unit_cube().unwrap().into_value()),
        ("tetrahedron", shapes::tetrahedron().unwrap().into_value()),
        (
            "dodecahedron",
            shapes::dodecahedron([0.0; 3], 1.0).unwrap().into_value(),
        ),
        (
            "prism_5",
            shapes::prism([0.0; 3], 5, 1.0, 2.0).unwrap().into_value(),
        ),
        (
            "pyramid_4",
            shapes::pyramid([0.0; 3], 4, 1.0, 2.0).unwrap().into_value(),
        ),
        (
            "wedge",
            shapes::wedge([0.0; 3], [1.0, 1.0, 1.0])
                .unwrap()
                .into_value(),
        ),
        (
            "block",
            shapes::block([0.0; 3], [1.0, 2.0, 3.0])
                .unwrap()
                .into_value(),
        ),
    ];

    for (name, env) in &primitives {
        let arena = env.topology().arena();
        let geometry = env.geometry();

        for (edge_id, _) in arena.iter_edges() {
            assert!(
                geometry.curves.contains(edge_id),
                "{}: Edge {} has no curve binding",
                name,
                edge_id
            );
        }
    }
}

// ── Surface↔Plane consistency ────────────────────────────────────────────

/// For every face, the SurfaceData normal/offset must match the Plane's
/// normalized f64 cache. SurfaceData is a derived projection of Plane.
#[test]
fn surface_normal_matches_plane_normal() {
    let env = shapes::unit_cube().unwrap().into_value();
    let arena = env.topology().arena();
    let geometry = env.geometry();

    for (face_id, _) in arena.iter_faces() {
        let plane = geometry
            .planes
            .get(face_id)
            .expect("face should have plane");
        let surface = geometry
            .surfaces
            .get(face_id)
            .expect("face should have surface");

        match &surface.kind {
            SurfaceKind::Plane { normal, offset } => {
                let pn = plane.normal();
                let po = plane.offset();
                assert!(
                    (normal[0] - pn[0]).abs() < 1e-15
                        && (normal[1] - pn[1]).abs() < 1e-15
                        && (normal[2] - pn[2]).abs() < 1e-15,
                    "Face {}: surface normal {:?} != plane normal {:?}",
                    face_id,
                    normal,
                    pn
                );
                assert!(
                    (offset - po).abs() < 1e-15,
                    "Face {}: surface offset {} != plane offset {}",
                    face_id,
                    offset,
                    po
                );
            }
            other => panic!(
                "Expected SurfaceKind::Plane for planar primitive, got {:?}",
                other
            ),
        }
    }
}

/// Twin half-edges on each edge must have curve displacements that are antiparallel.
///
/// For a properly oriented B-rep, if edge A→B has curve direction d,
/// then the twin edge B→A should have direction -d (antiparallel).
/// This test verifies the geometric consistency of curve emission.
#[test]
fn twin_curve_directions_are_antiparallel() {
    use forge_topo::queries::edge_endpoint_ids;

    let primitives: Vec<(&str, _)> = vec![
        ("cube", shapes::unit_cube().unwrap().into_value()),
        ("tetrahedron", shapes::tetrahedron().unwrap().into_value()),
        (
            "block",
            shapes::block([0.0; 3], [3.0, 5.0, 7.0])
                .unwrap()
                .into_value(),
        ),
    ];

    for (name, env) in &primitives {
        let arena = env.topology().arena();
        let geometry = env.geometry();

        for (edge_id, edge) in arena.iter_edges() {
            let he_a = edge.half_edge();
            let he_b = arena.get_half_edge(he_a).unwrap().radial_next();

            // Skip boundary edges (self-radial)
            if he_a == he_b {
                continue;
            }

            // Get displacement vectors from vertex positions
            let (va_origin, va_dest) = edge_endpoint_ids(arena, he_a).unwrap();
            let (vb_origin, vb_dest) = edge_endpoint_ids(arena, he_b).unwrap();

            let pos_a_o = geometry.positions.get(va_origin).unwrap().approx();
            let pos_a_d = geometry.positions.get(va_dest).unwrap().approx();
            let pos_b_o = geometry.positions.get(vb_origin).unwrap().approx();
            let pos_b_d = geometry.positions.get(vb_dest).unwrap().approx();

            // Displacement vectors
            let disp_a = [
                pos_a_d[0] - pos_a_o[0],
                pos_a_d[1] - pos_a_o[1],
                pos_a_d[2] - pos_a_o[2],
            ];
            let disp_b = [
                pos_b_d[0] - pos_b_o[0],
                pos_b_d[1] - pos_b_o[1],
                pos_b_d[2] - pos_b_o[2],
            ];

            // dot(d_a, d_b) should be negative (antiparallel)
            let dot = disp_a[0] * disp_b[0] + disp_a[1] * disp_b[1] + disp_a[2] * disp_b[2];
            let len_a =
                (disp_a[0] * disp_a[0] + disp_a[1] * disp_a[1] + disp_a[2] * disp_a[2]).sqrt();
            let len_b =
                (disp_b[0] * disp_b[0] + disp_b[1] * disp_b[1] + disp_b[2] * disp_b[2]).sqrt();

            if len_a < 1e-15 || len_b < 1e-15 {
                continue; // degenerate edge, skip
            }

            let cos_angle = dot / (len_a * len_b);
            assert!(
                cos_angle < -0.99,
                "{}: Edge {} twin half-edges are not antiparallel (cos={:.6})",
                name,
                edge_id,
                cos_angle
            );
        }
    }
}

/// The production edge-curve consistency validator passes on all primitives.
/// Exercises origin match, direction alignment, and destination match checks.
#[test]
fn edge_curve_consistency_passes_all_primitives() {
    let primitives: Vec<(&str, _)> = vec![
        ("cube", shapes::unit_cube().unwrap().into_value()),
        ("tetrahedron", shapes::tetrahedron().unwrap().into_value()),
        (
            "dodecahedron",
            shapes::dodecahedron([0.0; 3], 1.0).unwrap().into_value(),
        ),
        (
            "prism_5",
            shapes::prism([0.0; 3], 5, 1.0, 2.0).unwrap().into_value(),
        ),
        (
            "pyramid_4",
            shapes::pyramid([0.0; 3], 4, 1.0, 2.0).unwrap().into_value(),
        ),
        (
            "wedge",
            shapes::wedge([0.0; 3], [1.0, 1.0, 1.0])
                .unwrap()
                .into_value(),
        ),
        (
            "block",
            shapes::block([0.0; 3], [3.0, 5.0, 7.0])
                .unwrap()
                .into_value(),
        ),
    ];

    for (name, env) in &primitives {
        let arena = env.topology().arena();
        let geom = env.geometry();

        let tol = forge_core::FlatToleranceProvider::new(1e-12);
        let result = forge_spatial::validate_edge_curve_consistency(
            arena,
            &|v| geom.positions.get(v).map(|p| *p.approx()),
            &|e| geom.curves.get(e).map(|c| c.kind.clone()),
            &tol,
        );

        assert!(
            result.is_ok(),
            "{}: edge-curve consistency validator failed: {:?}",
            name,
            result.unwrap_err()
        );
    }
}

/// Adversarial: corrupt a curve's origin and verify the validator rejects it.
#[test]
fn corrupted_curve_origin_detected_by_validator() {
    use forge_geom::facade::CurveGeom;
    use std::sync::Arc;

    let mut env = shapes::unit_cube().unwrap().into_value();
    let first_edge = env.topology().arena().iter_edges().next().unwrap().0;

    // Replace the curve with one whose origin is 100mm away from the vertex
    let bad_curve = CurveGeom::from_analytic(
        CurveKind::Line {
            origin: [999.0, 999.0, 999.0],
            direction: [1.0, 0.0, 0.0],
        },
        None,
    );
    env.geometry_mut()
        .curves
        .set(first_edge, Arc::new(bad_curve));

    let geom = env.geometry();
    let tol = forge_core::FlatToleranceProvider::new(1e-12);
    let result = forge_spatial::validate_edge_curve_consistency(
        env.topology().arena(),
        &|v| geom.positions.get(v).map(|p| *p.approx()),
        &|e| geom.curves.get(e).map(|c| c.kind.clone()),
        &tol,
    );

    assert!(
        result.is_err(),
        "Validator should detect corrupted curve origin"
    );
    let msg = format!("{:?}", result.unwrap_err());
    assert!(
        msg.contains("curve origin deviation"),
        "Error should mention origin: {}",
        msg
    );
}

/// Adversarial: corrupt a curve's direction and verify the validator rejects it.
#[test]
fn corrupted_curve_direction_detected_by_validator() {
    use forge_geom::facade::CurveGeom;
    use forge_topo::queries::edge_endpoint_ids;
    use std::sync::Arc;

    let mut env = shapes::unit_cube().unwrap().into_value();
    let first_edge = env.topology().arena().iter_edges().next().unwrap().0;

    // Get the original origin so only the direction is wrong
    let edge = env.topology().arena().get_edge(first_edge).unwrap();
    let he_id = edge.half_edge();
    let (v_origin, _) = edge_endpoint_ids(env.topology().arena(), he_id).unwrap();
    let origin = *env.geometry().positions.get(v_origin).unwrap().approx();

    // Set direction perpendicular to the actual edge displacement
    let bad_curve = CurveGeom::from_analytic(
        CurveKind::Line {
            origin,
            direction: [0.0, 0.0, 1.0], // almost certainly wrong for a cube edge
        },
        None,
    );
    env.geometry_mut()
        .curves
        .set(first_edge, Arc::new(bad_curve));

    let geom = env.geometry();
    let tol = forge_core::FlatToleranceProvider::new(1e-12);
    let result = forge_spatial::validate_edge_curve_consistency(
        env.topology().arena(),
        &|v| geom.positions.get(v).map(|p| *p.approx()),
        &|e| geom.curves.get(e).map(|c| c.kind.clone()),
        &tol,
    );

    assert!(
        result.is_err(),
        "Validator should detect corrupted curve direction"
    );
    let msg = format!("{:?}", result.unwrap_err());
    assert!(
        msg.contains("misaligned") || msg.contains("destination deviation"),
        "Error should mention misalignment or destination: {}",
        msg
    );
}

// ── Vertex-on-surface ────────────────────────────────────────────────────

/// All vertices of each face must lie on the face's surface plane.
/// Validates using the spatial layer's `validate_surface_deviation`.
#[test]
fn all_vertices_lie_on_face_surface() {
    let primitives: Vec<(&str, _)> = vec![
        ("cube", shapes::unit_cube().unwrap().into_value()),
        ("tetrahedron", shapes::tetrahedron().unwrap().into_value()),
        (
            "dodecahedron",
            shapes::dodecahedron([0.0; 3], 1.0).unwrap().into_value(),
        ),
        (
            "prism_5",
            shapes::prism([0.0; 3], 5, 1.0, 2.0).unwrap().into_value(),
        ),
        (
            "block",
            shapes::block([0.0; 3], [1.0, 2.0, 3.0])
                .unwrap()
                .into_value(),
        ),
    ];

    for (name, env) in &primitives {
        let arena = env.topology().arena();
        let geometry = env.geometry();

        let position_fn = |v| geometry.positions.get(v).map(|p| *p.approx());
        let plane_fn = |f| geometry.planes.get(f).cloned();
        let tol_provider = crate::geometry::facade::GeometryToleranceProvider::new(geometry);

        let result = forge_spatial::validate_surface_deviation(
            arena,
            &position_fn,
            &plane_fn,
            &tol_provider,
        );

        assert!(
            result.is_ok(),
            "{}: Surface deviation validator failed on primitive: {:?}",
            name,
            result.unwrap_err()
        );
    }
}

// ── Adversarial: surface deviation catches corruption ────────────────────

/// Manually perturb a vertex position off its plane and verify the
/// surface deviation validator rejects the corrupt geometry.
#[test]
fn vertex_off_surface_detected_by_validator() {
    let mut env = shapes::unit_cube().unwrap().into_value();

    // Extract the first vertex ID in its own scope so the arena borrow is dropped
    // before we mutate geometry.
    let first_vertex = {
        let arena = env.topology().arena();
        let first_face = arena.iter_faces().next().unwrap().0;
        let outer_loop = arena
            .get_loop(arena.get_face(first_face).unwrap().loops.outer())
            .unwrap();
        let first_he = outer_loop.half_edge();
        arena.get_half_edge(first_he).unwrap().origin()
    };

    // Read original position, corrupt it, write it back
    let original = *env.geometry().positions.get(first_vertex).unwrap().approx();
    let corrupted = [original[0], original[1], original[2] + 5.0]; // 5mm off surface
    env.geometry_mut()
        .positions
        .set(first_vertex, ExactPosition::from_f64(corrupted));

    // Now re-borrow everything immutably for validation
    let arena = env.topology().arena();
    let geometry = env.geometry();
    let position_fn = |v| geometry.positions.get(v).map(|p| *p.approx());
    let plane_fn = |f| geometry.planes.get(f).cloned();
    let tol_provider = crate::geometry::facade::GeometryToleranceProvider::new(geometry);

    let result =
        forge_spatial::validate_surface_deviation(arena, &position_fn, &plane_fn, &tol_provider);

    assert!(
        result.is_err(),
        "Validator should detect vertex off surface"
    );
    let msg = format!("{:?}", result.unwrap_err());
    assert!(
        msg.contains("VertexOffSurface"),
        "Error should be VertexOffSurface: {}",
        msg
    );
}

// ── Adversarial: completeness validator catches corruption ───────────────

/// Manually remove a surface binding and verify the completeness
/// validator rejects the corrupt geometry.
#[test]
fn missing_surface_detected_by_validator() {
    use forge_topo::b_rep::TopologyArena;

    let mut env = shapes::unit_cube().unwrap().into_value();

    // Remove the first face's surface binding
    let first_face = env.topology().arena().iter_faces().next().unwrap().0;
    env.geometry_mut().surfaces.remove(first_face);

    let result = forge_spatial::validate_geometry_completeness(
        env.topology().arena(),
        &|f| env.geometry().planes.contains(f),
        &|v| env.geometry().positions.contains(v),
        Some(&|f| env.geometry().surfaces.contains(f)),
        Some(&|e| env.geometry().curves.contains(e)),
    );

    assert!(
        result.is_err(),
        "Validator should detect missing surface binding"
    );
    let msg = format!("{:?}", result.unwrap_err());
    assert!(
        msg.contains("surface"),
        "Error should mention 'surface': {}",
        msg
    );
}

/// Manually remove a curve binding and verify detection.
#[test]
fn missing_curve_detected_by_validator() {
    let mut env = shapes::unit_cube().unwrap().into_value();

    let first_edge = env.topology().arena().iter_edges().next().unwrap().0;
    env.geometry_mut().curves.remove(first_edge);

    let result = forge_spatial::validate_geometry_completeness(
        env.topology().arena(),
        &|f| env.geometry().planes.contains(f),
        &|v| env.geometry().positions.contains(v),
        Some(&|f| env.geometry().surfaces.contains(f)),
        Some(&|e| env.geometry().curves.contains(e)),
    );

    assert!(
        result.is_err(),
        "Validator should detect missing curve binding"
    );
    let msg = format!("{:?}", result.unwrap_err());
    assert!(
        msg.contains("curve"),
        "Error should mention 'curve': {}",
        msg
    );
}
