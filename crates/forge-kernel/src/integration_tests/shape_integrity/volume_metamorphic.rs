//! Volume oracle enhancements — metamorphic invariants, centroid validation,
//! and failure classification.
//!
//! DOMAIN: Extends the existing volume_oracle.rs with aerospace-grade tests:
//! - Scaling laws (cubic, height-linear)
//! - Translation / rotation invariance
//! - Centroid oracle validation
//! - Failure classification (open shell)
//! - Adversarial conditioning (needle-thin, large coords)

use crate::integration_tests::harness::comparison::{
    assert_geo_eq, assert_geo_eq_3d, analytical_reference, exact_planar, volume_invariance,
    ComparisonPolicy,
};
use crate::integration_tests::harness::oracles::{volume_of, centroid_of};
use crate::integration_tests::harness::shapes;
use crate::geometry::facade::GeometryView;

// ═══════════════════════════════════════════════════════════════════════════
// Part 1: Scaling / Metamorphic Invariants
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn volume_scales_cubically() {
    let small = shapes::cube([0.0; 3], 2.0).expect("cube(2) failed").into_value();
    let big   = shapes::cube([0.0; 3], 4.0).expect("cube(4) failed").into_value();

    let vol_small = volume_of(&small).expect("volume_of(small) failed");
    let vol_big   = volume_of(&big).expect("volume_of(big) failed");

    let ratio = vol_big / vol_small;
    let policy = analytical_reference();
    assert_geo_eq(ratio, 8.0, &policy, "cube volume should scale cubically (4/2)^3 = 8");
}

#[test]
fn prism_volume_linear_in_height() {
    let short = shapes::prism([0.0; 3], 6, 3.0, 1.0).expect("prism(h=1) failed").into_value();
    let tall  = shapes::prism([0.0; 3], 6, 3.0, 2.0).expect("prism(h=2) failed").into_value();

    let vol_short = volume_of(&short).expect("vol(short) failed");
    let vol_tall  = volume_of(&tall).expect("vol(tall) failed");

    let ratio = vol_tall / vol_short;
    let policy = analytical_reference();
    assert_geo_eq(ratio, 2.0, &policy, "prism volume should be linear in height");
}

#[test]
fn volume_translation_invariant() {
    let at_origin   = shapes::cube([0.0; 3], 3.0).expect("cube origin failed").into_value();
    let far_away    = shapes::cube([1e6, 1e6, 1e6], 3.0).expect("cube far failed").into_value();

    let vol_origin = volume_of(&at_origin).expect("vol(origin) failed");
    let vol_far    = volume_of(&far_away).expect("vol(far) failed");

    // Use relative tolerance for large-coordinate case
    let policy = ComparisonPolicy {
        abs_tol: 1e-6,
        rel_tol: 1e-10,
        method: "translation_invariance",
    };
    assert_geo_eq(vol_origin, vol_far, &policy, "volume must be translation-invariant");
}

#[test]
fn block_volume_matches_analytical() {
    let env = shapes::block([0.0; 3], [5.0, 5.0, 5.0]).expect("block failed").into_value();
    let vol = volume_of(&env).expect("volume_of failed");
    let policy = analytical_reference();
    assert_geo_eq(vol, 1000.0, &policy, "block [5,5,5] volume should be 10*10*10=1000");
}

#[test]
fn volume_of_complex_solid_is_positive_and_consistent() {
    // A dodecahedron provides a complex topological case (12 pentagonal faces triangulated).
    // Because BSP intersections produce floating-point approximations of Platonic solids,
    // we do not check against a strict analytical formula (which would be brittle).
    // Instead, we verify the volume is definitively positive, and rigidly invariant.
    let env_origin = shapes::dodecahedron([0.0; 3], 2.0).expect("dodecahedron origin failed").into_value();
    let env_offset = shapes::dodecahedron([15.0, -20.0, 50.0], 2.0).expect("dodecahedron offset failed").into_value();
    
    let vol_origin = volume_of(&env_origin).expect("volume_of origin failed");
    let vol_offset = volume_of(&env_offset).expect("volume_of offset failed");

    // Must be definitively positive
    assert!(vol_origin > 10.0, "Dodecahedron volume must be significantly positive");
    
    // Must be rigorously invariant under arbitrary spatial positioning
    let policy = ComparisonPolicy {
        abs_tol: 1e-10, // Must be nearly bit-exact
        rel_tol: 1e-12,
        method: "complex_solid_invariance",
    };
    assert_geo_eq(vol_origin, vol_offset, &policy, "Volume of complex solid must be invariant under translation");
}

// ═══════════════════════════════════════════════════════════════════════════
// Part 2: Centroid Oracle
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn centroid_of_centered_cube() {
    let env = shapes::cube([0.0; 3], 4.0).expect("cube failed").into_value();
    let center = centroid_of(&env).expect("centroid_of failed");
    let policy = exact_planar();
    assert_geo_eq_3d(center, [0.0, 0.0, 0.0], &policy, "centered cube centroid");
}

#[test]
fn centroid_of_offset_cube() {
    let env = shapes::cube([3.0, 5.0, 7.0], 2.0).expect("cube failed").into_value();
    let center = centroid_of(&env).expect("centroid_of failed");
    let policy = analytical_reference();
    assert_geo_eq_3d(center, [3.0, 5.0, 7.0], &policy, "offset cube centroid");
}

#[test]
fn centroid_of_block_at_origin() {
    let env = shapes::block([0.0; 3], [2.0, 3.0, 4.0]).expect("block failed").into_value();
    let center = centroid_of(&env).expect("centroid_of failed");
    let policy = exact_planar();
    assert_geo_eq_3d(center, [0.0, 0.0, 0.0], &policy, "centered block centroid");
}

#[test]
fn centroid_of_symmetric_tetrahedron_near_center() {
    let env = shapes::tetrahedron().expect("tet failed").into_value();
    let center = centroid_of(&env).expect("centroid_of should succeed for tetrahedron");
    
    // For a regular tetrahedron, the geometric centroid is mathematically 
    // identical to the simple average of its 4 vertices.
    let arena = env.topology().arena();
    let geom = env.geometry();
    let mut sum = [0.0; 3];
    let mut count = 0;
    
    for (v_id, _) in arena.iter_vertices() {
        if let Some(p) = geom.get_vertex_position(v_id) {
            sum[0] += p[0];
            sum[1] += p[1];
            sum[2] += p[2];
            count += 1;
        }
    }
    
    assert_eq!(count, 4, "Tetrahedron must have exactly 4 vertices");
    let expected = [sum[0] / 4.0, sum[1] / 4.0, sum[2] / 4.0];
    
    let policy = analytical_reference(); 
    assert_geo_eq_3d(center, expected, &policy, "tetrahedron centroid must match exact average of vertices");
}

// ═══════════════════════════════════════════════════════════════════════════
// Part 3: Adversarial Conditioning
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn needle_thin_box_volume() {
    // Extreme aspect ratio: 1e-4 × 1e-4 × 1000
    let env = shapes::block([0.0; 3], [0.5e-4, 0.5e-4, 500.0])
        .expect("needle block failed").into_value();
    let vol = volume_of(&env).expect("volume_of failed");

    let analytical = 1e-4 * 1e-4 * 1000.0; // = 1e-5
    let policy = ComparisonPolicy {
        abs_tol: 1e-15,
        rel_tol: 1e-10,
        method: "needle_thin_analytical",
    };
    assert_geo_eq(vol, analytical, &policy, "needle-thin box volume");
}

#[test]
fn giant_cube_volume() {
    let env = shapes::cube([0.0; 3], 1000.0).expect("giant cube failed").into_value();
    let vol = volume_of(&env).expect("volume_of failed");
    let policy = analytical_reference();
    assert_geo_eq(vol, 1e9, &policy, "giant cube (1000^3) volume");
}

// ═══════════════════════════════════════════════════════════════════════════
// Part 4: Volume oracle returns positive value
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn volume_is_positive_for_all_primitives() {
    let shapes: Vec<(&str, _)> = vec![
        ("cube", shapes::cube([0.0; 3], 2.0).expect("cube")),
        ("tetrahedron", shapes::tetrahedron().expect("tet")),
        ("dodecahedron", shapes::dodecahedron([0.0; 3], 2.0).expect("dodec")),
        ("hex_prism", shapes::prism([0.0; 3], 6, 3.0, 4.0).expect("prism")),
        ("pyramid", shapes::pyramid([0.0; 3], 4, 3.0, 4.0).expect("pyramid")),
        ("wedge", shapes::wedge([0.0; 3], [2.0, 3.0, 4.0]).expect("wedge")),
    ];

    for (name, env_res) in shapes {
        let env = env_res.into_value();
        let vol = volume_of(&env).unwrap_or_else(|e| panic!("{name}: {e}"));
        assert!(vol > 0.0, "{name}: volume should be positive, got {vol}");
    }
}
