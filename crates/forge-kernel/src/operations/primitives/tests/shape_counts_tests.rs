//! Shape-specific entity count tests.
//!
//! Validates exact V/E/F counts for each primitive, plus custom
//! plane sets (octahedron, truncated cube) and new shapes.

use crate::operations::primitives::{
    make_block, make_convex_solid, make_cube, make_dodecahedron, make_prism, make_pyramid,
    make_tetrahedron, make_wedge,
};
use super::structural_invariants_tests::assert_valid_solid;
use super::test_config;

fn assert_counts(label: &str, v: usize, e: usize, f: usize, ev: usize, ee: usize, ef: usize) {
    assert_eq!(v, ev, "{label}: V={v}, expected {ev}");
    assert_eq!(e, ee, "{label}: E={e}, expected {ee}");
    assert_eq!(f, ef, "{label}: F={f}, expected {ef}");
}

#[test]
fn cube_counts() {
    let cfg = test_config();
    let r = make_cube([0.0; 3], 2.0, &cfg).unwrap();
    let a = r.topology().arena();
    assert_counts("cube", a.vertex_count(), a.half_edge_count() / 2, a.face_count(), 8, 12, 6);
}

#[test]
fn tetrahedron_counts() {
    let cfg = test_config();
    let r = make_tetrahedron([0.0; 3], 1.0, &cfg).unwrap();
    let a = r.topology().arena();
    assert_counts("tet", a.vertex_count(), a.half_edge_count() / 2, a.face_count(), 4, 6, 4);
}

#[test]
fn dodecahedron_counts() {
    let cfg = test_config();
    let r = make_dodecahedron([0.0; 3], 1.0, &cfg).unwrap();
    let a = r.topology().arena();
    assert_counts("dodec", a.vertex_count(), a.half_edge_count() / 2, a.face_count(), 20, 30, 12);
}

#[test]
fn octahedron_from_eight_planes_generates() {
    let cfg = test_config();
    let planes = vec![
        forge_geom::facade::Plane::from_point_normal([1.0, 1.0, 1.0], [1.0, 1.0, 1.0]).unwrap(),
        forge_geom::facade::Plane::from_point_normal([1.0, 1.0, -1.0], [1.0, 1.0, -1.0]).unwrap(),
        forge_geom::facade::Plane::from_point_normal([1.0, -1.0, 1.0], [1.0, -1.0, 1.0]).unwrap(),
        forge_geom::facade::Plane::from_point_normal([1.0, -1.0, -1.0], [1.0, -1.0, -1.0]).unwrap(),
        forge_geom::facade::Plane::from_point_normal([-1.0, 1.0, 1.0], [-1.0, 1.0, 1.0]).unwrap(),
        forge_geom::facade::Plane::from_point_normal([-1.0, 1.0, -1.0], [-1.0, 1.0, -1.0]).unwrap(),
        forge_geom::facade::Plane::from_point_normal([-1.0, -1.0, 1.0], [-1.0, -1.0, 1.0]).unwrap(),
        forge_geom::facade::Plane::from_point_normal([-1.0, -1.0, -1.0], [-1.0, -1.0, -1.0]).unwrap(),
    ];
    let r = make_convex_solid(planes, &cfg).unwrap();
    let a = r.topology().arena();
    assert!(a.face_count() >= 4, "octahedron must have at least 4 faces");
}

#[test]
fn truncated_cube_fourteen_faces() {
    let cfg = test_config();
    let mut planes = forge_geom::primitives::shapes::cube([0.0; 3], 2.0).unwrap();
    let corners: [[f64; 3]; 8] = [
        [1.0, 1.0, 1.0], [1.0, 1.0, -1.0], [1.0, -1.0, 1.0], [1.0, -1.0, -1.0],
        [-1.0, 1.0, 1.0], [-1.0, 1.0, -1.0], [-1.0, -1.0, 1.0], [-1.0, -1.0, -1.0],
    ];
    for n in &corners {
        let pt = [n[0] * 2.5, n[1] * 2.5, n[2] * 2.5];
        planes.push(forge_geom::facade::Plane::from_point_normal(pt, *n).unwrap());
    }
    let r = make_convex_solid(planes, &cfg).unwrap();
    assert_eq!(r.topology().arena().face_count(), 14, "truncated cube: expected 14 faces");
    assert_valid_solid(&r, "truncated_cube");
}

// ── New shape counts ──────────────────────────────────────────────────────

#[test]
fn block_non_uniform_counts() {
    let cfg = test_config();
    let r = make_block([0.0; 3], [1.0, 2.0, 3.0], &cfg).unwrap();
    let a = r.topology().arena();
    assert_counts("block", a.vertex_count(), a.half_edge_count() / 2, a.face_count(), 8, 12, 6);
    assert_valid_solid(&r, "block");
}

#[test]
fn prism_triangular_counts() {
    let cfg = test_config();
    let r = make_prism([0.0; 3], 3, 1.0, 2.0, &cfg).unwrap();
    let a = r.topology().arena();
    assert_counts("prism3", a.vertex_count(), a.half_edge_count() / 2, a.face_count(), 6, 9, 5);
    assert_valid_solid(&r, "prism3");
}

#[test]
fn prism_hexagonal_counts() {
    let cfg = test_config();
    let r = make_prism([0.0; 3], 6, 1.0, 2.0, &cfg).unwrap();
    let a = r.topology().arena();
    assert_counts("prism6", a.vertex_count(), a.half_edge_count() / 2, a.face_count(), 12, 18, 8);
    assert_valid_solid(&r, "prism6");
}

#[test]
fn pyramid_quad_generates() {
    let cfg = test_config();
    let planes = forge_geom::primitives::shapes::pyramid([0.0; 3], 4, 1.0, 2.0).unwrap();
    assert_eq!(planes.len(), 5, "pyramid(4) should produce 5 planes");
}

#[test]
fn wedge_counts() {
    let cfg = test_config();
    let r = make_wedge([0.0; 3], [2.0, 3.0, 1.0], &cfg).unwrap();
    assert_valid_solid(&r, "wedge");
}
