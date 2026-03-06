//! Normal correctness — inside/outside classification for all primitives.
//!
//! DOMAIN: Validates that every face normal points outward by probing
//! both sides of the face with `classify_point_in_solid`. This is the
//! aerospace-grade replacement for "normal points away from centroid"
//! which assumes convexity.
//!
//! For face point `p` with normal `n`:
//!   - `p + εn` must classify as `Outside`
//!   - `p - εn` must classify as `Inside`
//!
//! Works for convex, concave, and multi-shell solids.

use crate::integration_tests::harness::comparison::{assert_geo_eq, unit_normal};
use crate::integration_tests::harness::oracles::{classify_normal_outward, NormalClassification};
use crate::integration_tests::harness::shapes;

/// Probe epsilon for inside/outside classification.
/// Must be large enough to clear the boundary tolerance (1e-10) but
/// small enough to stay within thin features.
const NORMAL_EPSILON: f64 = 1e-4;

// ═══════════════════════════════════════════════════════════════════════════
// Part 1: All primitives — every face normal confirmed outward
// ═══════════════════════════════════════════════════════════════════════════

fn assert_all_normals_outward(env: &crate::engine::facade::SolidEnvelope, shape_name: &str) {
    let faces: Vec<_> = env.faces().to_vec();
    for (i, &face_id) in faces.iter().enumerate() {
        let classification = classify_normal_outward(env, face_id, NORMAL_EPSILON)
            .unwrap_or_else(|e| panic!("{shape_name} face {i}: oracle error: {e}"));
        assert_eq!(
            classification,
            NormalClassification::OutwardConfirmed,
            "{shape_name} face {i}/{}: expected OutwardConfirmed, got {:?}",
            faces.len(),
            classification,
        );
    }
}

#[test]
fn cube_all_6_normals_outward() {
    let env = shapes::cube([0.0; 3], 2.0)
        .expect("cube failed")
        .into_value();
    assert_eq!(env.faces().len(), 6, "cube should have 6 faces");
    assert_all_normals_outward(&env, "cube");
}

#[test]
fn tetrahedron_all_4_normals_outward() {
    let env = shapes::tetrahedron()
        .expect("tetrahedron failed")
        .into_value();
    assert_eq!(env.faces().len(), 4, "tetrahedron should have 4 faces");
    assert_all_normals_outward(&env, "tetrahedron");
}

#[test]
fn dodecahedron_all_12_normals_outward() {
    let env = shapes::dodecahedron([0.0; 3], 2.0)
        .expect("dodecahedron failed")
        .into_value();
    assert_eq!(env.faces().len(), 12, "dodecahedron should have 12 faces");
    assert_all_normals_outward(&env, "dodecahedron");
}

#[test]
fn hexagonal_prism_all_8_normals_outward() {
    let env = shapes::prism([0.0; 3], 6, 3.0, 4.0)
        .expect("prism failed")
        .into_value();
    assert_eq!(env.faces().len(), 8, "hex prism should have 8 faces");
    assert_all_normals_outward(&env, "hex_prism");
}

#[test]
fn pyramid_all_5_normals_outward() {
    let env = shapes::pyramid([0.0; 3], 4, 3.0, 4.0)
        .expect("pyramid failed")
        .into_value();
    assert_eq!(env.faces().len(), 5, "pyramid should have 5 faces");
    assert_all_normals_outward(&env, "pyramid");
}

#[test]
fn wedge_all_normals_outward() {
    let env = shapes::wedge([0.0; 3], [2.0, 3.0, 4.0])
        .expect("wedge failed")
        .into_value();
    // Wedge from BSP has 6 faces (two triangles + two quads + two rects)
    assert_all_normals_outward(&env, "wedge");
}

#[test]
fn triangular_prism_all_5_normals_outward() {
    let env = shapes::prism([0.0; 3], 3, 3.0, 5.0)
        .expect("triangular prism failed")
        .into_value();
    assert_eq!(env.faces().len(), 5, "triangular prism should have 5 faces");
    assert_all_normals_outward(&env, "tri_prism");
}

// ═══════════════════════════════════════════════════════════════════════════
// Part 2: Robustness
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn normal_magnitudes_are_unit() {
    use crate::geometry::facade::GeometryView;
    use forge_spatial::operations::facade::face_normal_from_outer_loop;
    use forge_topo::handles::VertexId;

    let policy = unit_normal();
    let env = shapes::cube([0.0; 3], 2.0)
        .expect("cube failed")
        .into_value();

    for &face_id in env.faces() {
        let pos_fn = |v: VertexId| env.geometry().get_vertex_position(v).copied();
        let normal = face_normal_from_outer_loop(env.topology().arena(), &pos_fn, face_id)
            .expect("topology error")
            .expect("face has no normal");

        let magnitude =
            (normal[0] * normal[0] + normal[1] * normal[1] + normal[2] * normal[2]).sqrt();
        assert_geo_eq(
            magnitude,
            1.0,
            &policy,
            &format!("normal magnitude face {:?}", face_id),
        );
    }
}

#[test]
fn normal_query_is_idempotent() {
    let env = shapes::cube([0.0; 3], 2.0)
        .expect("cube failed")
        .into_value();
    let face = env.faces()[0];

    let r1 = classify_normal_outward(&env, face, NORMAL_EPSILON).unwrap();
    let r2 = classify_normal_outward(&env, face, NORMAL_EPSILON).unwrap();
    assert_eq!(r1, r2, "normal classification must be idempotent");
}

// ═══════════════════════════════════════════════════════════════════════════
// Part 3: Geometric stress
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn offset_cube_normals_still_outward() {
    // Cube far from origin — tests numerical stability of classification
    let env = shapes::cube([1e6, 1e6, 1e6], 2.0)
        .expect("offset cube failed")
        .into_value();
    assert_all_normals_outward(&env, "offset_cube");
}

#[test]
fn large_cube_normals_outward() {
    let env = shapes::cube([0.0; 3], 1000.0)
        .expect("large cube failed")
        .into_value();
    // Large cube needs larger epsilon to clear the boundary
    let faces: Vec<_> = env.faces().to_vec();
    for (i, &face_id) in faces.iter().enumerate() {
        let classification = classify_normal_outward(&env, face_id, 1.0)
            .unwrap_or_else(|e| panic!("large_cube face {i}: oracle error: {e}"));
        assert_eq!(
            classification,
            NormalClassification::OutwardConfirmed,
            "large_cube face {i}/{}: expected OutwardConfirmed, got {:?}",
            faces.len(),
            classification,
        );
    }
}

#[test]
fn small_cube_normals_outward() {
    let env = shapes::cube([0.0; 3], 0.001)
        .expect("small cube failed")
        .into_value();
    // Use smaller epsilon for small geometry
    let faces: Vec<_> = env.faces().to_vec();
    for (i, &face_id) in faces.iter().enumerate() {
        let classification = classify_normal_outward(&env, face_id, 1e-5)
            .unwrap_or_else(|e| panic!("small_cube face {i}: oracle error: {e}"));
        assert_eq!(
            classification,
            NormalClassification::OutwardConfirmed,
            "small_cube face {i}/{}: expected OutwardConfirmed, got {:?}",
            faces.len(),
            classification,
        );
    }
}
