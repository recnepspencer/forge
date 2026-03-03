//! Geometric data assertions for spatial correctness.
//!
//! DOMAIN: Validates that the geometric data attached to a B-Rep solid
//! is complete and correct. Delegates all computation to production
//! algorithms in `geometry::logic::measurements`.

use crate::engine::facade::SolidEnvelope;
use crate::geometry::facade::{
    self as geom, GeometryView, face_area, solid_volume, edge_length, bounding_box,
};

/// Assert every face has a plane and every vertex has a position.
///
/// This is the most common geometry bug: an operator creates topology
/// but forgets to populate the corresponding geometry entries.
pub fn assert_geometry_complete(env: &SolidEnvelope) {
    let arena = env.topology().arena();
    let geom = env.geometry();

    for (face_id, _) in arena.iter_faces() {
        assert!(
            geom.get_face_plane(face_id).is_some(),
            "Face F#{} missing plane in GeometryStore",
            face_id.index()
        );
    }
    for (vert_id, _) in arena.iter_vertices() {
        assert!(
            geom.get_vertex_position(vert_id).is_some(),
            "Vertex V#{} missing position in GeometryStore",
            vert_id.index()
        );
    }
}

/// Assert all faces have positive area above a minimum threshold.
///
/// Delegates to `geometry::logic::measurements::face_area`.
pub fn assert_positive_face_areas(env: &SolidEnvelope, min_area: f64) {
    let arena = env.topology().arena();
    let geom = env.geometry();

    for (face_id, _) in arena.iter_faces() {
        let area = face_area(arena, geom, face_id);
        assert!(
            area > min_area,
            "Face F#{} has degenerate area {:.2e} (minimum: {:.2e})",
            face_id.index(), area, min_area
        );
    }
}

/// Assert all vertex positions lie within an expected bounding box.
///
/// Delegates to `geometry::logic::measurements::bounding_box` for
/// the actual computation, then checks against expected bounds.
pub fn assert_bounds(
    env: &SolidEnvelope,
    expected_min: [f64; 3],
    expected_max: [f64; 3],
    tol: f64,
) {
    let arena = env.topology().arena();
    let geom = env.geometry();

    for (vid, _) in arena.iter_vertices() {
        if let Some(pos) = geom.get_vertex_position(vid) {
            for axis in 0..3 {
                assert!(
                    pos[axis] >= expected_min[axis] - tol
                        && pos[axis] <= expected_max[axis] + tol,
                    "Vertex V#{} position[{}] = {:.6} is outside bounds [{:.6}, {:.6}] ± {:.2e}",
                    vid.index(), axis, pos[axis], expected_min[axis], expected_max[axis], tol
                );
            }
        }
    }
}

/// Assert the signed volume of a closed shell matches an expected value.
///
/// Delegates to `geometry::logic::measurements::solid_volume`.
pub fn assert_volume(env: &SolidEnvelope, expected: f64, tol: f64) {
    let volume = solid_volume(env.topology().arena(), env.geometry());
    assert!(
        (volume - expected).abs() < tol,
        "Volume {:.6} does not match expected {:.6} (tolerance: {:.2e}, diff: {:.2e})",
        volume, expected, tol, (volume - expected).abs()
    );
}

/// Assert a specific face's plane normal matches an expected direction.
pub fn assert_face_plane(
    env: &SolidEnvelope,
    face: forge_topo::handles::FaceId,
    expected_normal: [f64; 3],
    tol: f64,
) {
    let geom = env.geometry();
    let plane = geom.get_face_plane(face)
        .unwrap_or_else(|| panic!("Face F#{} has no plane", face.index()));
    let n = plane.normal();
    let dot = n[0] * expected_normal[0] + n[1] * expected_normal[1] + n[2] * expected_normal[2];
    assert!(
        (dot - 1.0).abs() < tol,
        "Face F#{} normal [{:.4}, {:.4}, {:.4}] does not match expected [{:.4}, {:.4}, {:.4}] (dot={:.6})",
        face.index(), n[0], n[1], n[2],
        expected_normal[0], expected_normal[1], expected_normal[2], dot
    );
}

/// Assert all edges have lengths within a given range.
///
/// Delegates to `geometry::logic::measurements::edge_length`.
pub fn assert_edge_lengths(env: &SolidEnvelope, min: f64, max: f64) {
    let arena = env.topology().arena();
    let geom = env.geometry();

    for (eid, _) in arena.iter_edges() {
        if let Some(len) = edge_length(arena, geom, eid) {
            assert!(
                len >= min && len <= max,
                "Edge E#{} has length {:.6} outside range [{:.6}, {:.6}]",
                eid.index(), len, min, max
            );
        }
    }
}
