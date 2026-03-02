//! Edge case tests — degenerate inputs that must be cleanly rejected.

use crate::operations::primitives::{make_convex_solid, make_cube, make_block, make_prism, make_pyramid};
use super::{test_config, OperationScope};

#[test]
fn two_planes_rejected() {
    let cfg = test_config();
    let mut null = OperationScope::null_sink();
    let mut scope = OperationScope::new(&cfg, &mut null);
    let planes = vec![
        forge_geom::facade::Plane::try_new([1.0, 0.0, 0.0], 1.0).unwrap(),
        forge_geom::facade::Plane::try_new([-1.0, 0.0, 0.0], 1.0).unwrap(),
    ];
    assert!(make_convex_solid(planes, &mut scope).is_err(), "2 planes cannot form a polyhedron");
}

#[test]
fn three_planes_rejected() {
    let cfg = test_config();
    let mut null = OperationScope::null_sink();
    let mut scope = OperationScope::new(&cfg, &mut null);
    let planes = vec![
        forge_geom::facade::Plane::try_new([1.0, 0.0, 0.0], 1.0).unwrap(),
        forge_geom::facade::Plane::try_new([-1.0, 0.0, 0.0], 1.0).unwrap(),
        forge_geom::facade::Plane::try_new([0.0, 1.0, 0.0], 1.0).unwrap(),
    ];
    assert!(make_convex_solid(planes, &mut scope).is_err(), "3 planes cannot form a closed polyhedron");
}

// ── Input validation rejection tests ──────────────────────────────────────

#[test]
fn negative_cube_size_rejected() {
    let cfg = test_config();
    let mut null = OperationScope::null_sink();
    let mut scope = OperationScope::new(&cfg, &mut null);
    assert!(make_cube([0.0; 3], -1.0, &mut scope).is_err());
}

#[test]
fn zero_cube_size_rejected() {
    let cfg = test_config();
    let mut null = OperationScope::null_sink();
    let mut scope = OperationScope::new(&cfg, &mut null);
    assert!(make_cube([0.0; 3], 0.0, &mut scope).is_err());
}

#[test]
fn nan_cube_size_rejected() {
    let cfg = test_config();
    let mut null = OperationScope::null_sink();
    let mut scope = OperationScope::new(&cfg, &mut null);
    assert!(make_cube([0.0; 3], f64::NAN, &mut scope).is_err());
}

#[test]
fn inf_cube_size_rejected() {
    let cfg = test_config();
    let mut null = OperationScope::null_sink();
    let mut scope = OperationScope::new(&cfg, &mut null);
    assert!(make_cube([0.0; 3], f64::INFINITY, &mut scope).is_err());
}

#[test]
fn nan_center_rejected() {
    let cfg = test_config();
    let mut null = OperationScope::null_sink();
    let mut scope = OperationScope::new(&cfg, &mut null);
    assert!(make_cube([f64::NAN, 0.0, 0.0], 1.0, &mut scope).is_err());
}

#[test]
fn inf_center_rejected() {
    let cfg = test_config();
    let mut null = OperationScope::null_sink();
    let mut scope = OperationScope::new(&cfg, &mut null);
    assert!(make_cube([0.0, f64::INFINITY, 0.0], 1.0, &mut scope).is_err());
}

#[test]
fn negative_block_extent_rejected() {
    let cfg = test_config();
    let mut null = OperationScope::null_sink();
    let mut scope = OperationScope::new(&cfg, &mut null);
    assert!(make_block([0.0; 3], [1.0, -1.0, 1.0], &mut scope).is_err());
}

#[test]
fn prism_two_sides_rejected() {
    let cfg = test_config();
    let mut null = OperationScope::null_sink();
    let mut scope = OperationScope::new(&cfg, &mut null);
    assert!(make_prism([0.0; 3], 2, 1.0, 1.0, &mut scope).is_err());
}

#[test]
fn pyramid_two_sides_rejected() {
    let cfg = test_config();
    let mut null = OperationScope::null_sink();
    let mut scope = OperationScope::new(&cfg, &mut null);
    assert!(make_pyramid([0.0; 3], 2, 1.0, 1.0, &mut scope).is_err());
}
