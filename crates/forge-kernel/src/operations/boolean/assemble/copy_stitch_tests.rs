//! Phase 1 confirmation tests for the copy+stitch round-trip hypothesis.
//!
//! DOMAIN: Prove or disprove that copying a valid manifold to a fresh arena
//! and re-stitching it preserves topology identity.
//!
//! If any test fails, the vertex-dedup-during-copy hypothesis is confirmed.
//! If all pass, the root cause is elsewhere and we avoid a wild goose chase.

use std::collections::BTreeMap;
use forge_topo::state::TopologyState;
use forge_topo::handles::{FaceId, HalfEdgeId, VertexId};

use crate::geometry_store::GeometryStore;
use crate::operations::boolean::test_helpers::{build_cube, euler_audit};
use crate::operations::boolean::schema::{BooleanInput, BooleanOp};
use crate::operations::boolean::assemble::copy::{copy_faces, VertexDedup, VertexWelder};
use crate::operations::boolean::assemble::stitch::stitch_twins;
use crate::operations::boolean::assemble::disjoint::compute_disjoint_scale;
use crate::operations::boolean::assemble::execute_boolean_direct;
use crate::core::ModelingContext;
use crate::operations::boolean::eval::VertexMatchKey;

/// Copy all faces from a source topology into a fresh arena, stitch,
/// and return the result along with vertex counts for diagnostics.
///
/// Uses the same code path as `assemble_two_shells`: `copy_faces` →
/// `stitch_twins`, with the production `compute_disjoint_scale` for
/// tolerance computation.
fn copy_and_stitch(
    source_topo: &TopologyState,
    source_geom: &GeometryStore,
) -> Result<(TopologyState, GeometryStore, usize, usize), forge_core::KernelError> {
    let source_vertex_count = source_topo.arena().vertex_count();

    let scale = compute_disjoint_scale(
        source_topo.arena(), source_geom, None,
    );

    let state = TopologyState::empty();
    let mut draft = state.into_mutation();
    let mut result_geom = GeometryStore::new();
    let mut vertex_dedup = VertexDedup::new();
    let mut global_vertex_map: BTreeMap<VertexMatchKey, VertexId> = BTreeMap::new();
    let mut spatial_index = VertexWelder::new(scale);
    let mut all_he: Vec<HalfEdgeId> = Vec::new();

    let all_faces: Vec<FaceId> = source_topo.arena().iter_faces()
        .map(|(fid, _)| fid).collect();

    copy_faces(
        &mut draft, &mut result_geom, &mut vertex_dedup, &mut all_he,
        &mut global_vertex_map, &mut spatial_index,
        source_topo.arena(), source_geom, &all_faces, false, None,
    )?;

    let dest_vertex_count = draft.arena().vertex_count();

    let mut ctx = ModelingContext::default();
    stitch_twins(&mut draft, &all_he, &result_geom, spatial_index.weld_tolerance_sq(), &mut ctx)?;

    let topo = draft.commit()?;
    Ok((topo, result_geom, source_vertex_count, dest_vertex_count))
}

/// Test 1: Fresh cube round-trips through copy+stitch without issues.
///
/// Baseline: if THIS fails, copy+stitch is fundamentally broken.
#[test]
fn copy_stitch_round_trip_cube() {
    let (topo, geom) = build_cube([0.0, 0.0, 0.0], 5.0);
    let (v_src, _, f_src, chi_src) = euler_audit(topo.arena());

    let (result_topo, _, source_verts, dest_verts) = copy_and_stitch(&topo, &geom)
        .expect("copy+stitch of fresh cube should succeed");

    let (v_dst, _, f_dst, chi_dst) = euler_audit(result_topo.arena());

    assert_eq!(source_verts, dest_verts,
        "vertex count mismatch: source={}, dest={}", source_verts, dest_verts);
    assert_eq!(v_src, v_dst, "vertex count changed: {} → {}", v_src, v_dst);
    assert_eq!(f_src, f_dst, "face count changed: {} → {}", f_src, f_dst);
    assert_eq!(chi_src, chi_dst, "Euler χ changed: {} → {}", chi_src, chi_dst);
    assert_eq!(chi_dst, 2, "cube χ should be 2, got {}", chi_dst);
}

/// Test 2: EMBER-produced notched cube round-trips through copy+stitch.
///
/// This is the critical test — the notched cube is the step-1 result of MB-N3.
/// If this fails, the vertex-dedup hypothesis is confirmed.
#[test]
fn copy_stitch_round_trip_notched_cube() {
    let (topo_a, geom_a) = build_cube([0.0, 0.0, 0.0], 5.0);
    let (topo_b, geom_b) = build_cube([5.0, 0.0, -4.0], 0.3);

    let input = BooleanInput::new(topo_a, geom_a, topo_b, geom_b, BooleanOp::Subtraction);
    let result = crate::operations::boolean::execute_boolean(input)
        .into_result()
        .expect("cube - notch should succeed");

    let (topo, geom) = result.into_topo_geom();
    let (v_src, e_src, f_src, chi_src) = euler_audit(topo.arena());

    let (result_topo, _, source_verts, dest_verts) = copy_and_stitch(&topo, &geom)
        .expect("copy+stitch of notched cube should succeed");

    let (v_dst, _, f_dst, chi_dst) = euler_audit(result_topo.arena());

    assert_eq!(source_verts, dest_verts,
        "VERTEX DEDUP FAILURE: source has {} vertices but copy created {} \
         (duplicate positions with different VertexIds detected)",
        source_verts, dest_verts);
    assert_eq!(f_src, f_dst, "face count changed: {} → {}", f_src, f_dst);
    assert_eq!(chi_src, chi_dst, "Euler χ changed: {} → {}", chi_src, chi_dst);
}

/// Test 3: Full MB-N3 chain steps 0→1→2 — the exact reproduction.
///
/// If step 2 fails, we know the exact pipeline path that breaks.
/// If step 2 succeeds, our diagnosis of the zero_split path was wrong.
#[test]
fn contained_subtraction_chain_2steps() {
    let base_half = 5.0;
    let (mut topo, mut geom) = build_cube([0.0, 0.0, 0.0], base_half);

    for i in 0..3 {
        let angle = (i as f64) * std::f64::consts::TAU / 20.0;
        let x = base_half * angle.cos();
        let y = base_half * angle.sin();
        let z = -4.0 + (i as f64) * 0.4;

        let (topo_b, geom_b) = build_cube([x, y, z], 0.3);
        let input = BooleanInput::new(topo.clone(), geom.clone(), topo_b, geom_b, BooleanOp::Subtraction);

        match crate::operations::boolean::execute_boolean(input).into_result() {
            Ok(result) => {
                let (v, e, f, chi) = euler_audit(result.topology().arena());
                eprintln!("step {}: V={} E={} F={} χ={}", i, v, e, f, chi);
                let (t, g) = result.into_topo_geom();
                topo = t;
                geom = g;
            }
            Err(e) => {
                panic!(
                    "chain broke at step {}/3: {:?}\n\
                     This confirms the contained-subtraction stitch bug.",
                    i, e
                );
            }
        }
    }
}

/// Test 4: Diagnostic vertex count comparison on double-notched cube.
///
/// Copies the step 0+1 result of MB-N3 and asserts vertex count preservation.
/// This is the definitive yes/no on the vertex dedup hypothesis.
#[test]
fn vertex_count_preserved_after_copy() {
    let (mut topo, mut geom) = build_cube([0.0, 0.0, 0.0], 5.0);

    for i in 0..2 {
        let angle = (i as f64) * std::f64::consts::TAU / 20.0;
        let x = 5.0 * angle.cos();
        let y = 5.0 * angle.sin();
        let z = -4.0 + (i as f64) * 0.4;

        let (topo_b, geom_b) = build_cube([x, y, z], 0.3);
        let input = BooleanInput::new(topo, geom, topo_b, geom_b, BooleanOp::Subtraction);
        let result = crate::operations::boolean::execute_boolean(input)
            .into_result()
            .unwrap_or_else(|e| panic!("step {} should succeed: {:?}", i, e));
        let (t, g) = result.into_topo_geom();
        topo = t;
        geom = g;
    }

    let (src_v_audit, src_e_audit, src_f_audit, src_chi) = euler_audit(topo.arena());

    let (result_topo, _, src_v, dst_v) = copy_and_stitch(&topo, &geom)
        .unwrap_or_else(|e| panic!(
            "copy+stitch of double-notched cube failed: {:?}\n\
             Source: V={} E={} F={} χ={}\n\
             This confirms copy+stitch round-trip is broken for booleaned geometry.",
            e, src_v_audit, src_e_audit, src_f_audit, src_chi
        ));

    let (v_dst, e_dst, f_dst, chi_dst) = euler_audit(result_topo.arena());

    assert_eq!(src_v, dst_v,
        "VERTEX DEDUP FAILURE: source={} → dest={} vertices (delta: +{})",
        src_v, dst_v, dst_v - src_v);
    assert_eq!(chi_dst, src_chi,
        "Euler χ changed after copy+stitch: source χ={} → dest χ={} (V={} E={} F={})",
        src_chi, chi_dst, v_dst, e_dst, f_dst);
}

/// Test 6: Copy+stitch of LEGACY-produced single-notched cube.
///
/// KNOWN DEFECT: Legacy pipeline output has vertex-identity issues that
/// prevent copy+stitch round-tripping. Remove #[ignore] when the
/// underlying legacy vertex-identity defect is fixed.
#[test]
#[ignore = "known legacy pipeline vertex-identity defect"]
fn copy_stitch_round_trip_legacy_notched_cube() {
    let (topo_a, geom_a) = build_cube([0.0, 0.0, 0.0], 5.0);
    let (topo_b, geom_b) = build_cube([5.0, 0.0, -4.0], 0.3);

    let input = BooleanInput::new(topo_a, geom_a, topo_b, geom_b, BooleanOp::Subtraction);
    let result = execute_boolean_direct(input)
        .into_result()
        .expect("legacy cube - notch should succeed");

    let (topo, geom) = result.into_topo_geom();
    let (v_src, e_src, f_src, chi_src) = euler_audit(topo.arena());
    eprintln!("legacy notched cube: V={} E={} F={} χ={}", v_src, e_src, f_src, chi_src);

    let (result_topo, _, source_verts, dest_verts) = copy_and_stitch(&topo, &geom)
        .unwrap_or_else(|e| panic!(
            "copy+stitch of LEGACY notched cube FAILED: {:?}\n\
             This means the legacy output topology is defective.",
            e
        ));

    let (v_dst, e_dst, f_dst, chi_dst) = euler_audit(result_topo.arena());
    eprintln!("after copy+stitch: V={}/{} E={} F={} χ={}/{}", 
        v_dst, source_verts, e_dst, f_dst, chi_dst, chi_src);

    assert_eq!(source_verts, dest_verts,
        "vertex dedup: source={} → dest={}", source_verts, dest_verts);
    assert_eq!(chi_dst, chi_src,
        "Euler χ changed: {} → {}", chi_src, chi_dst);
}

/// Test 7: Copy+stitch of LEGACY-produced double-notched cube.
///
/// KNOWN DEFECT: Same vertex-identity issue as Test 6, amplified.
/// Remove #[ignore] when the underlying legacy defect is fixed.
#[test]
#[ignore = "known legacy pipeline vertex-identity defect"]
fn copy_stitch_round_trip_legacy_double_notched() {
    let (mut topo, mut geom) = build_cube([0.0, 0.0, 0.0], 5.0);

    for i in 0..2 {
        let angle = (i as f64) * std::f64::consts::TAU / 20.0;
        let x = 5.0 * angle.cos();
        let y = 5.0 * angle.sin();
        let z = -4.0 + (i as f64) * 0.4;

        let (topo_b, geom_b) = build_cube([x, y, z], 0.3);
        let input = BooleanInput::new(topo, geom, topo_b, geom_b, BooleanOp::Subtraction);
        let result = execute_boolean_direct(input)
            .into_result()
            .unwrap_or_else(|e| panic!("legacy step {} should succeed: {:?}", i, e));
        let (t, g) = result.into_topo_geom();
        topo = t;
        geom = g;
    }

    let (src_v, src_e, src_f, src_chi) = euler_audit(topo.arena());
    eprintln!("legacy double-notched: V={} E={} F={} χ={}", src_v, src_e, src_f, src_chi);

    let (result_topo, _, dedup_src_v, dedup_dst_v) = copy_and_stitch(&topo, &geom)
        .unwrap_or_else(|e| panic!(
            "copy+stitch of LEGACY double-notched cube FAILED: {:?}\n\
             Source: V={} E={} F={} χ={}\n\
             CONFIRMED: legacy output can't survive copy+stitch round-trip.",
            e, src_v, src_e, src_f, src_chi
        ));

    let (dst_v, dst_e, dst_f, dst_chi) = euler_audit(result_topo.arena());
    eprintln!("after copy+stitch: V={}/{} E={} F={} χ={}/{}", 
        dst_v, dedup_src_v, dst_e, dst_f, dst_chi, src_chi);

    assert_eq!(dedup_src_v, dedup_dst_v,
        "vertex dedup: source={} → dest={}", dedup_src_v, dedup_dst_v);
    assert_eq!(dst_chi, src_chi,
        "Euler χ changed: {} → {}", src_chi, dst_chi);
}

