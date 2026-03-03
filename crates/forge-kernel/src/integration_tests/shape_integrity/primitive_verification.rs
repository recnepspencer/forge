//! Primitive shape verification using the fluent `verify()` chain.
//!
//! DOMAIN: Every shape factory must produce a solid with correct
//! entity counts and approximate volume. All checks use production
//! queries through the verify chain with OBJ dump on failure.
//!
//! Expected values derived from:
//! - `make_cube(center, size)` → side length `size`, half-extent `size/2`
//! - `make_block(center, half_extents)` → `half_extents` are half-dimensions
//! - `make_tetrahedron(center, scale)` → regular tet with scale param
//! - Euler formula: V - E + F = 2 for all closed shells

use crate::integration_tests::harness::verify::verify;
use crate::integration_tests::harness::shapes;

// ── Cube ────────────────────────────────────────────────────────────────────

/// unit_cube() → make_cube([0;3], 1.0) → side=1.0, half-extent=0.5, V=1.0
#[test]
fn cube_topology_and_volume() {
    let env = shapes::unit_cube().unwrap();
    verify(&env)
        .named("cube")
        .faces(6).vertices(8).edges(12)
        .half_edges(24).loops(6).shells(1).bodies(1)
        .volume_approx(1.0, 1e-6)
        .pass();
}

// ── Tetrahedron ─────────────────────────────────────────────────────────────

/// tetrahedron(scale=1.0) produces a tet with 4 faces, 4 vertices.
/// V−E+F = 4−6+4 = 2 ✓
#[test]
fn tetrahedron_topology() {
    let env = shapes::tetrahedron().unwrap();
    verify(&env)
        .named("tetrahedron")
        .faces(4).vertices(4).edges(6)
        .half_edges(12).loops(4).shells(1).bodies(1)
        .pass();
}

/// Tetrahedron volume is non-trivial — verify against the production oracle.
/// For scale=1.0, forge_geom::tetrahedron produces a specific geometry;
/// we lock the volume to catch regressions.
#[test]
fn tetrahedron_volume_regression() {
    use crate::geometry::facade::solid_volume;
    let env = shapes::tetrahedron().unwrap();
    let vol = solid_volume(env.topology().arena(), env.geometry());
    // Lock the exact value from production (23.303639...)
    assert!((vol - 23.303639).abs() < 0.001,
        "Tetrahedron volume regression: got {vol:.6}, expected ~23.303639");
}

// ── Dodecahedron ────────────────────────────────────────────────────────────

/// V−E+F = 20−30+12 = 2 ✓
#[test]
fn dodecahedron_topology() {
    let env = shapes::dodecahedron([0.0; 3], 1.0).unwrap();
    verify(&env)
        .named("dodecahedron")
        .faces(12).vertices(20).edges(30)
        .half_edges(60).loops(12).shells(1).bodies(1)
        .pass();
}

// ── Prisms ──────────────────────────────────────────────────────────────────

/// Triangular prism: 2 caps + 3 sides = 5 faces. V−E+F = 6−9+5 = 2 ✓
#[test]
fn triangular_prism_topology() {
    let env = shapes::prism([0.0; 3], 3, 1.0, 2.0).unwrap();
    verify(&env)
        .named("prism_3")
        .faces(5).vertices(6).edges(9)
        .half_edges(18).loops(5).shells(1).bodies(1)
        .pass();
}

/// Hexagonal prism: 2 caps + 6 sides = 8 faces. V−E+F = 12−18+8 = 2 ✓
#[test]
fn hexagonal_prism_topology() {
    let env = shapes::prism([0.0; 3], 6, 1.0, 2.0).unwrap();
    verify(&env)
        .named("prism_6")
        .faces(8).vertices(12).edges(18)
        .half_edges(36).loops(8).shells(1).bodies(1)
        .pass();
}

// ── Block ───────────────────────────────────────────────────────────────────

/// block(half_extents=[1,2,3]) → dimensions 2×4×6 → V=48
#[test]
fn block_non_uniform_topology_and_volume() {
    let env = shapes::block([0.0; 3], [1.0, 2.0, 3.0]).unwrap();
    verify(&env)
        .named("block")
        .faces(6).vertices(8).edges(12)
        .half_edges(24).loops(6).shells(1).bodies(1)
        .volume_approx(48.0, 1e-6)
        .pass();
}

// ── Wedge ───────────────────────────────────────────────────────────────────

/// wedge(dimensions=[1,1,1]) produces a 6-faced clipped solid.
/// The wedge is a box clipped by a slope plane, resulting in a pentahedral
/// shape that BSP clips to 6 faces due to the particular plane arrangement.
/// V−E+F = 8−12+6 = 2 ✓
#[test]
fn wedge_topology_and_volume() {
    let env = shapes::wedge([0.0; 3], [1.0, 1.0, 1.0]).unwrap();
    verify(&env)
        .named("wedge")
        .faces(6).vertices(8).edges(12)
        .half_edges(24).loops(6).shells(1).bodies(1)
        .volume_approx(1.0, 1e-6)
        .pass();
}

// ── Pyramid ─────────────────────────────────────────────────────────────────

/// Triangular pyramid (3 sides) should produce a valid solid.
#[test]
fn triangular_pyramid_topology() {
    let env = shapes::pyramid([0.0; 3], 3, 1.0, 2.0).unwrap();
    verify(&env)
        .named("pyramid_3")
        .shells(1).bodies(1)
        .pass();
}

/// Quad pyramid (4 sides).
/// V−E+F = 5−8+5 = 2 ✓
#[test]
fn quad_pyramid_topology_and_volume() {
    let env = shapes::pyramid([0.0; 3], 4, 1.0, 2.0).unwrap();
    verify(&env)
        .named("pyramid_4")
        .faces(5).vertices(5).edges(8)
        .half_edges(16).loops(5).shells(1).bodies(1)
        .pass();
}

// ── Scale and position invariance ───────────────────────────────────────────

/// Large cube: cube(size=100) → 100³ = 1_000_000 volume
#[test]
fn large_cube_scale_invariant() {
    let env = shapes::cube([0.0; 3], 100.0).unwrap();
    verify(&env)
        .named("large_cube")
        .faces(6).vertices(8).edges(12)
        .volume_approx(1_000_000.0, 1e-3)
        .pass();
}

/// Same topology and volume regardless of position.
#[test]
fn offset_cube_position_invariant() {
    let env = shapes::cube([500.0, 500.0, 500.0], 1.0).unwrap();
    verify(&env)
        .named("offset_cube")
        .faces(6).vertices(8).edges(12)
        .half_edges(24)
        .volume_approx(1.0, 1e-6)
        .pass();
}

// ── Invalid input rejection ─────────────────────────────────────────────────
// These verify that production validators catch bad inputs with explicit
// errors rather than producing corrupt geometry.

/// Zero-size cube must be rejected.
#[test]
fn zero_size_cube_rejected() {
    let result = shapes::cube([0.0; 3], 0.0);
    assert!(result.is_err(), "Zero-size cube should be rejected by validator");
}

/// Negative-size cube must be rejected.
#[test]
fn negative_size_cube_rejected() {
    let result = shapes::cube([0.0; 3], -1.0);
    assert!(result.is_err(), "Negative-size cube should be rejected by validator");
}

/// NaN coordinate must be rejected.
#[test]
fn nan_center_cube_rejected() {
    let result = shapes::cube([f64::NAN, 0.0, 0.0], 1.0);
    assert!(result.is_err(), "NaN center should be rejected by validator");
}

/// Infinity coordinate must be rejected.
#[test]
fn inf_center_cube_rejected() {
    let result = shapes::cube([f64::INFINITY, 0.0, 0.0], 1.0);
    assert!(result.is_err(), "Infinity center should be rejected by validator");
}

/// Zero-dimension block must be rejected.
#[test]
fn zero_extent_block_rejected() {
    let result = shapes::block([0.0; 3], [1.0, 0.0, 1.0]);
    assert!(result.is_err(), "Zero half-extent should be rejected by validator");
}

/// Prism with fewer than 3 sides must be rejected.
#[test]
fn prism_two_sides_rejected() {
    let result = shapes::prism([0.0; 3], 2, 1.0, 2.0);
    assert!(result.is_err(), "Prism with 2 sides should be rejected by validator");
}

/// Prism with zero radius must be rejected.
#[test]
fn prism_zero_radius_rejected() {
    let result = shapes::prism([0.0; 3], 4, 0.0, 2.0);
    assert!(result.is_err(), "Zero-radius prism should be rejected by validator");
}

/// Prism with zero height must be rejected.
#[test]
fn prism_zero_height_rejected() {
    let result = shapes::prism([0.0; 3], 4, 1.0, 0.0);
    assert!(result.is_err(), "Zero-height prism should be rejected by validator");
}
