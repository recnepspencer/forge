//! Edge case tests — degenerate inputs that must be cleanly rejected.

// use crate::context::ModelingContext;
use crate::operations::primitives::{make_convex_solid, make_cube, make_block, make_prism, make_pyramid};
use super::test_config;

#[test]
fn two_planes_rejected() {
    let cfg = test_config();
    let planes = vec![
        forge_geom::Plane::try_new([1.0, 0.0, 0.0], 1.0).unwrap(),
        forge_geom::Plane::try_new([-1.0, 0.0, 0.0], 1.0).unwrap(),
    ];
    assert!(make_convex_solid(planes, &cfg).is_err(), "2 planes cannot form a polyhedron");
}

#[test]
fn three_planes_rejected() {
    let cfg = test_config();
    let planes = vec![
        forge_geom::Plane::try_new([1.0, 0.0, 0.0], 1.0).unwrap(),
        forge_geom::Plane::try_new([-1.0, 0.0, 0.0], 1.0).unwrap(),
        forge_geom::Plane::try_new([0.0, 1.0, 0.0], 1.0).unwrap(),
    ];
    assert!(make_convex_solid(planes, &cfg).is_err(), "3 planes cannot form a closed polyhedron");
}

// ── Input validation rejection tests ──────────────────────────────────────

#[test]
fn negative_cube_size_rejected() {
    let cfg = test_config();
    assert!(make_cube([0.0; 3], -1.0, &cfg).is_err());
}

#[test]
fn zero_cube_size_rejected() {
    let cfg = test_config();
    assert!(make_cube([0.0; 3], 0.0, &cfg).is_err());
}

#[test]
fn nan_cube_size_rejected() {
    let cfg = test_config();
    assert!(make_cube([0.0; 3], f64::NAN, &cfg).is_err());
}

#[test]
fn inf_cube_size_rejected() {
    let cfg = test_config();
    assert!(make_cube([0.0; 3], f64::INFINITY, &cfg).is_err());
}

#[test]
fn nan_center_rejected() {
    let cfg = test_config();
    assert!(make_cube([f64::NAN, 0.0, 0.0], 1.0, &cfg).is_err());
}

#[test]
fn inf_center_rejected() {
    let cfg = test_config();
    assert!(make_cube([0.0, f64::INFINITY, 0.0], 1.0, &cfg).is_err());
}

#[test]
fn negative_block_extent_rejected() {
    let cfg = test_config();
    assert!(make_block([0.0; 3], [1.0, -1.0, 1.0], &cfg).is_err());
}

#[test]
fn prism_two_sides_rejected() {
    let cfg = test_config();
    assert!(make_prism([0.0; 3], 2, 1.0, 1.0, &cfg).is_err());
}

#[test]
fn pyramid_two_sides_rejected() {
    let cfg = test_config();
    assert!(make_pyramid([0.0; 3], 2, 1.0, 1.0, &cfg).is_err());
}
