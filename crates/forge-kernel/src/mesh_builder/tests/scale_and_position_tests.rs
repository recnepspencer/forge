//! Scale and position stress tests.
//!
//! Validates that mesh_builder produces valid output across extreme
//! scales and arbitrary translations.

use crate::mesh_builder::{make_cube, make_tetrahedron, make_dodecahedron};
use super::structural_invariants_tests::assert_valid_solid;
use super::test_config;

#[test]
fn cube_far_from_origin() {
    let cfg = test_config();
    assert_valid_solid(&make_cube([1e8, -1e8, 5e7], 2.0, &cfg).unwrap(), "cube_far");
}

#[test]
fn cube_tiny_scale() {
    let cfg = test_config();
    assert_valid_solid(&make_cube([0.0; 3], 1e-6, &cfg).unwrap(), "cube_tiny");
}

#[test]
fn cube_large_scale() {
    let cfg = test_config();
    assert_valid_solid(&make_cube([0.0; 3], 1e6, &cfg).unwrap(), "cube_large");
}

#[test]
fn cube_non_origin_center() {
    let cfg = test_config();
    assert_valid_solid(&make_cube([3.7, -2.1, 8.9], 1.5, &cfg).unwrap(), "cube_offset");
}

#[test]
fn tetrahedron_far_from_origin() {
    let cfg = test_config();
    assert_valid_solid(&make_tetrahedron([1e6, 1e6, 1e6], 1.0, &cfg).unwrap(), "tet_far");
}

#[test]
fn dodecahedron_scaled_large() {
    let cfg = test_config();
    assert_valid_solid(&make_dodecahedron([0.0; 3], 100.0, &cfg).unwrap(), "dodec_large");
}
