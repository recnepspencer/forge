//! P3.2 acceptance tests — Minimal Region Extractor & Delta-Debug.
//!
//! PV-35: Extract 3-ring neighborhood of a face → valid, serializable sub-mesh.
//! PV-35b: Extract region from corrupted solid → region captures corruption site.
//! PV-36: Delta-debug on 100-step chain with injected failure at step 73
//!        → finds step 73 automatically.

use forge_core::tracing::delta_debug::delta_debug;
use forge_topo::validate::{validate_topology, ValidationLevel};
use forge_topo::state::DraftConfig;
use crate::analysis::region_extractor::{extract_n_ring, ExtractedRegion};
use crate::mesh_builder::make_cube;

/// PV-35: Extract 3-ring neighborhood → produces valid, serializable sub-mesh.
///
/// Builds a cube (6 faces), picks one face, extracts 1-ring and 3-ring.
/// Asserts correct topology expansion and that the result roundtrips
/// through JSON serialization identically.
#[test]
fn pv_35_extract_n_ring_valid_serializable() {
    let cube = make_cube([0.0, 0.0, 0.0], 2.0).unwrap();
    let arena = cube.topology().arena();
    let geometry = cube.geometry();

    let all_faces: Vec<_> = arena.iter_faces().map(|(id, _)| id).collect();
    assert_eq!(all_faces.len(), 6, "Cube should have 6 faces");

    let seed = all_faces[0];

    let ring_0 = extract_n_ring(arena, geometry, seed, 0).unwrap();
    assert_eq!(ring_0.face_count(), 1, "Ring-0 should be just the seed face");
    assert!(!ring_0.is_empty());
    assert_eq!(ring_0.get_ring_depth(), 0);
    assert_eq!(ring_0.get_seed_face(), seed);

    let ring_1 = extract_n_ring(arena, geometry, seed, 1).unwrap();
    assert!(
        ring_1.face_count() >= 4,
        "Ring-1 on a cube face should reach at least 4 adjacent faces, got {}",
        ring_1.face_count()
    );

    let ring_3 = extract_n_ring(arena, geometry, seed, 3).unwrap();
    assert_eq!(
        ring_3.face_count(),
        6,
        "Ring-3 should cover all 6 faces of the cube"
    );
    assert!(ring_3.vertex_count() > 0, "Should have extracted vertices");
    assert!(ring_3.half_edge_count() > 0, "Should have extracted halfedges");
    assert!(
        !ring_3.get_face_planes().is_empty(),
        "Should have extracted plane geometry"
    );
    assert!(
        !ring_3.get_vertex_positions().is_empty(),
        "Should have extracted vertex positions"
    );

    let json = ring_3.to_json().expect("Production to_json should succeed");
    assert!(!json.is_empty(), "JSON output should not be empty");

    let deserialized = ExtractedRegion::from_json(&json)
        .expect("Production from_json should succeed");

    assert_eq!(deserialized.face_count(), ring_3.face_count());
    assert_eq!(deserialized.vertex_count(), ring_3.vertex_count());
    assert_eq!(deserialized.half_edge_count(), ring_3.half_edge_count());
    assert_eq!(deserialized.get_ring_depth(), ring_3.get_ring_depth());
    assert_eq!(deserialized.get_seed_face(), ring_3.get_seed_face());
}

/// PV-35b: Extracted region from corrupted solid captures corruption site.
///
/// Strategy:
/// 1. Build valid cube, confirm it passes validation
/// 2. Corrupt: break twin reciprocity on one edge (set twin to self)
/// 3. Confirm full arena fails validation
/// 4. Extract 3-ring from the corrupted face
/// 5. Verify region contains the corruption face and has geometry
/// 6. Serialize → roundtrip → byte-identical
#[test]
fn pv_35b_extracted_region_captures_corruption() {
    let result = make_cube([0.0, 0.0, 0.0], 2.0).unwrap();
    let (topo, geom) = result.into_parts();

    let valid_check = validate_topology(topo.arena(), ValidationLevel::Full);
    assert!(valid_check.is_ok(), "Cube should be valid before corruption");

    let mut config = DraftConfig::default();
    config.validation_level = ValidationLevel::None;

    let mut draft = topo.into_mutation_with(config);
    let arena = draft.arena_mut();

    let (he_to_corrupt, corrupted_face) = {
        let (he_id, he_data) = arena.iter_half_edges()
            .filter(|(id, d)| *id != d.radial_next())
            .next()
            .unwrap();
        (he_id, he_data.face())
    };

    arena.get_half_edge_mut(he_to_corrupt).unwrap().set_radial_next(he_to_corrupt);

    let corruption_check = validate_topology(arena, ValidationLevel::Full);
    assert!(
        corruption_check.is_err(),
        "Arena with broken twin should fail validation"
    );

    let region = extract_n_ring(arena, &geom, corrupted_face, 3).unwrap();

    assert!(
        region.face_count() >= 5,
        "3-ring from corrupted face on a 6-face cube should reach at least 5 faces, got {}",
        region.face_count()
    );

    let region_faces = region.get_faces();
    let contains_corrupted = region_faces.contains(corrupted_face.index()).unwrap_or(false);
    assert!(
        contains_corrupted,
        "Extracted region must contain the corrupted face"
    );

    assert!(
        !region.get_face_planes().is_empty(),
        "Extracted region should include geometry for the corruption site"
    );

    let json_1 = region.to_json().expect("Production serialization");
    let roundtrip = ExtractedRegion::from_json(&json_1).expect("Production deserialization");
    let json_2 = roundtrip.to_json().expect("Second serialization");

    assert_eq!(
        json_1, json_2,
        "Double roundtrip should be byte-identical"
    );

    let reconstructed_arena = roundtrip.to_arena()
        .expect("Region should reconstruct into a TopologyArena");

    let region_validation = validate_topology(
        &reconstructed_arena,
        ValidationLevel::Full,
    );

    assert!(
        region_validation.is_err(),
        "Reconstructed region must reproduce the twin corruption"
    );

    let region_err = region_validation.unwrap_err();
    assert!(
        matches!(region_err, forge_core::KernelError::TopologyViolation { .. }),
        "Must be a TopologyViolation, got: {:?}",
        region_err
    );
}

/// PV-36: Delta-debug on a 100-step chain with injected failure at step 73
/// → binary search finds step 73 automatically.
///
/// Uses the generic `delta_debug()` function with a closure that simulates
/// failure at step >= 73. Verifies the result is exactly 73 and that the
/// search used O(log N) probes.
#[test]
fn pv_36_delta_debug_finds_step_73() {
    let result = delta_debug(100, |step| Ok(step >= 73)).expect("delta_debug should succeed");

    assert_eq!(
        result.get_failing_step(),
        73,
        "Should find the injected failure at step 73"
    );

    assert_eq!(result.get_total_steps(), 100);

    assert!(
        result.get_probes_used() <= 8,
        "Binary search on 100 steps should use ≤ 8 probes (log2(100) ≈ 6.6), got {}",
        result.get_probes_used()
    );
}
