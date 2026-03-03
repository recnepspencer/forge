//! Selector query tests — finding entities by geometric properties.
//!
//! DOMAIN: Verifies the selector DSL produces correct results on
//! known shapes. All geometry queries delegate to production code.

use crate::integration_tests::harness::selectors::select;
use crate::integration_tests::harness::shapes;

// ── Face selectors ───────────────────────────────────────────────────────────

/// A unit cube has exactly one face with normal pointing straight up.
#[test]
fn cube_has_one_top_face() {
    let env = shapes::unit_cube().unwrap();
    let top = select(env.get_value())
        .faces()
        .where_normal_near([0.0, 0.0, 1.0], 0.01)
        .one();
    assert!(top.is_ok(), "Cube should have exactly one +Z face");
}

/// A unit cube has exactly one face with normal pointing straight down.
#[test]
fn cube_has_one_bottom_face() {
    let env = shapes::unit_cube().unwrap();
    let bottom = select(env.get_value())
        .faces()
        .where_normal_near([0.0, 0.0, -1.0], 0.01)
        .one();
    assert!(bottom.is_ok(), "Cube should have exactly one -Z face");
}

/// A cube has 6 faces, each with a unique axis-aligned normal.
/// Querying all 6 directions should each return exactly 1 face.
#[test]
fn cube_six_axis_aligned_faces() {
    let env = shapes::unit_cube().unwrap();
    let normals: [[f64; 3]; 6] = [
        [1.0, 0.0, 0.0], [-1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0], [0.0, -1.0, 0.0],
        [0.0, 0.0, 1.0], [0.0, 0.0, -1.0],
    ];
    for normal in &normals {
        let count = select(env.get_value())
            .faces()
            .where_normal_near(*normal, 0.01)
            .count();
        assert_eq!(
            count, 1,
            "Expected exactly 1 face with normal {:?}, got {}",
            normal, count
        );
    }
}

// ── Vertex selectors ─────────────────────────────────────────────────────────

/// The tetrahedron has one face with normal [0,0,1] (its top plane).
/// This was initially written as "has no axis-aligned faces" which was wrong —
/// forge_geom::tetrahedron() explicitly defines plane 0 with normal [0,0,1].
#[test]
fn tetrahedron_has_one_top_face() {
    let env = shapes::tetrahedron().unwrap();
    let count = select(env.get_value())
        .faces()
        .where_normal_near([0.0, 0.0, 1.0], 0.01)
        .count();
    assert_eq!(count, 1, "Tetrahedron should have exactly 1 +Z face (its top)");
}


/// unit_cube (size=1.0) has vertices at ±0.5 on each axis.
/// Verify we can find the vertex at (+0.5, +0.5, +0.5).
#[test]
fn cube_corner_vertex() {
    let env = shapes::unit_cube().unwrap();
    let corner = select(env.get_value())
        .vertices()
        .where_near([0.5, 0.5, 0.5], 0.01)
        .one();
    assert!(corner.is_ok(), "Cube should have exactly one vertex near (0.5, 0.5, 0.5)");
}

/// All 8 cube vertices should be findable at known positions (±0.5).
#[test]
fn cube_all_eight_vertices_findable() {
    let env = shapes::unit_cube().unwrap();
    let corners: [[f64; 3]; 8] = [
        [ 0.5,  0.5,  0.5], [ 0.5,  0.5, -0.5],
        [ 0.5, -0.5,  0.5], [ 0.5, -0.5, -0.5],
        [-0.5,  0.5,  0.5], [-0.5,  0.5, -0.5],
        [-0.5, -0.5,  0.5], [-0.5, -0.5, -0.5],
    ];
    for corner in &corners {
        let count = select(env.get_value())
            .vertices()
            .where_near(*corner, 0.01)
            .count();
        assert_eq!(
            count, 1,
            "Expected exactly 1 vertex near {:?}, got {}",
            corner, count
        );
    }
}

// ── Edge selectors ───────────────────────────────────────────────────────────

/// unit_cube (size=1.0) has 12 edges, each of length 1.0.
/// Querying for edges longer than 0.9 should return all 12.
#[test]
fn cube_all_edges_same_length() {
    let env = shapes::unit_cube().unwrap();
    let long_edges = select(env.get_value())
        .edges()
        .where_length_above(0.9)
        .all();
    assert_eq!(long_edges.len(), 12, "Cube should have 12 edges > 0.9");
}

/// No edges of a unit cube are longer than 1.1.
#[test]
fn cube_no_edges_above_diagonal() {
    let env = shapes::unit_cube().unwrap();
    let too_long = select(env.get_value())
        .edges()
        .where_length_above(1.1)
        .all();
    assert_eq!(too_long.len(), 0, "No cube edge should be longer than 1.1");
}

// ── Cross-entity selectors ───────────────────────────────────────────────────

/// Find the top face, then query its vertices — should have 4.
#[test]
fn top_face_has_four_vertices() {
    let env = shapes::unit_cube().unwrap();
    let top = select(env.get_value())
        .faces()
        .where_normal_near([0.0, 0.0, 1.0], 0.01)
        .one()
        .unwrap();

    let verts = select(env.get_value()).vertices_of(top).all();
    assert_eq!(verts.len(), 4, "Top face should have 4 vertices");
}

/// Find the top face, then query its edges — should have 4.
#[test]
fn top_face_has_four_edges() {
    let env = shapes::unit_cube().unwrap();
    let top = select(env.get_value())
        .faces()
        .where_normal_near([0.0, 0.0, 1.0], 0.01)
        .one()
        .unwrap();

    let edges = select(env.get_value()).edges_of(top).all();
    assert_eq!(edges.len(), 4, "Top face should have 4 edges");
}
