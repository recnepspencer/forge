//! Diagnostic and regression tests for the zero-split assembly path.
//!
//! DOMAIN: Verify that copy+stitch, pass-through, and splice
//! produce topologically valid output from both EMBER and legacy pipelines.
//! INVARIANTS: Euler χ=2 for single-shell results, vertex counts preserved.

#![allow(unused_imports)]

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

/// Helper: copy all faces from a source topology into a fresh arena and stitch.
fn copy_and_stitch(
    source_topo: &TopologyState,
    source_geom: &GeometryStore,
) -> Result<(TopologyState, GeometryStore, usize, usize), forge_core::KernelError> {
    let faces: Vec<FaceId> = source_topo.arena().iter_faces().map(|(fid, _)| fid).collect();
    let src_v = source_topo.arena().vertex_count();
    let scale = compute_disjoint_scale(source_topo.arena(), source_geom, None);

    let state = TopologyState::empty();
    let mut draft = state.into_mutation();
    let mut result_geom = GeometryStore::new();
    let mut he_ids: Vec<HalfEdgeId> = Vec::new();
    let mut vertex_map: BTreeMap<VertexMatchKey, VertexId> = BTreeMap::new();
    let mut spatial = VertexWelder::new(scale);
    let mut dedup = VertexDedup::new();

    copy_faces(
        &mut draft, &mut result_geom, &mut dedup, &mut he_ids,
        &mut vertex_map, &mut spatial,
        source_topo.arena(), source_geom, &faces, false, None,
    )?;

    let mut ctx = ModelingContext::default();
    let report = stitch_twins(&mut draft, &he_ids, &result_geom, spatial.weld_tolerance_sq(), &mut ctx)?;
    report.require_fully_paired(&draft, &result_geom, &ctx)?;

    let topo = draft.commit()?;
    let dst_v = topo.arena().vertex_count();
    Ok((topo, result_geom, src_v, dst_v))
}

/// Dump the plane histogram for a topology — how many faces share each plane.
fn plane_histogram(topo: &TopologyState, geom: &GeometryStore) -> Vec<(String, usize)> {
    let mut hist: BTreeMap<String, usize> = BTreeMap::new();
    for (fid, _) in topo.arena().iter_faces() {
        let key = if let Some(p) = geom.get_face_plane(fid) {
            let n = p.normal();
            format!("n=[{:.2},{:.2},{:.2}] d={:.2}", n[0], n[1], n[2], p.offset())
        } else {
            "no-plane".to_string()
        };
        *hist.entry(key).or_insert(0) += 1;
    }
    let mut v: Vec<_> = hist.into_iter().collect();
    v.sort_by(|a, b| b.1.cmp(&a.1));
    v
}

// ── Test 1: cube round-trip ──────────────────────────────────────────────────

/// Copy+stitch a fresh cube — baseline correctness check.
#[test]
fn copy_stitch_round_trip_cube() {
    let (topo, geom) = build_cube([0.0, 0.0, 0.0], 5.0);
    let (result_topo, _, src_v, dst_v) = copy_and_stitch(&topo, &geom).unwrap();
    let (_, _, _, chi) = euler_audit(result_topo.arena());
    assert_eq!(src_v, dst_v, "vertex count mismatch");
    assert_eq!(chi, 2, "Euler χ should be 2, got {chi}");
}

// ── Test 2: EMBER notched cube round-trip ────────────────────────────────────

/// EMBER-produced notched cube round-trips through copy+stitch.
#[test]
fn copy_stitch_round_trip_notched_cube() {
    let (topo_a, geom_a) = build_cube([0.0, 0.0, 0.0], 5.0);
    let (topo_b, geom_b) = build_cube([5.0, 0.0, -4.0], 0.3);

    let input = BooleanInput::new(topo_a, geom_a, topo_b, geom_b, BooleanOp::Subtraction);
    let result = crate::operations::boolean::execute_boolean(input)
        .into_result()
        .expect("EMBER cube - notch should succeed");

    let (topo, geom) = result.into_topo_geom();
    let (_, _, _, src_chi) = euler_audit(topo.arena());

    let (result_topo, _, src_v, dst_v) = copy_and_stitch(&topo, &geom).unwrap();
    let (_, _, _, chi) = euler_audit(result_topo.arena());
    assert_eq!(src_v, dst_v, "vertex count mismatch");
    assert_eq!(chi, src_chi, "Euler χ changed: {src_chi} → {chi}");
}

// ── Test 3: EMBER 3-step chain ───────────────────────────────────────────────

/// 3-step subtraction chain via EMBER adaptive — must not break.
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

// ── Test 4: vertex count invariant ───────────────────────────────────────────

/// Copy+stitch of EMBER double-notched cube preserves vertex count and χ.
#[test]
fn vertex_count_preserved_after_copy() {
    let (topo_a, geom_a) = build_cube([0.0, 0.0, 0.0], 5.0);
    let (topo_b, geom_b) = build_cube([5.0, 0.0, -4.0], 0.3);
    let input = BooleanInput::new(topo_a, geom_a, topo_b, geom_b, BooleanOp::Subtraction);
    let r1 = crate::operations::boolean::execute_boolean(input)
        .into_result()
        .unwrap();
    let (topo_r1, geom_r1) = r1.into_topo_geom();

    let (topo_b2, geom_b2) = build_cube([4.76, 1.55, -3.6], 0.3);
    let input2 = BooleanInput::new(topo_r1, geom_r1, topo_b2, geom_b2, BooleanOp::Subtraction);
    let r2 = crate::operations::boolean::execute_boolean(input2)
        .into_result()
        .unwrap();

    let (topo, geom) = r2.into_topo_geom();
    let (src_v, _, _, src_chi) = euler_audit(topo.arena());

    let (result_topo, _, _, dst_v) = copy_and_stitch(&topo, &geom).unwrap();
    let (v_dst, e_dst, f_dst, chi_dst) = euler_audit(result_topo.arena());
    assert_eq!(src_v, dst_v,
        "VERTEX DEDUP FAILURE: source={} → dest={} vertices (delta: +{})",
        src_v, dst_v, dst_v - src_v);
    assert_eq!(chi_dst, src_chi,
        "Euler χ changed after copy+stitch: source χ={} → dest χ={} (V={} E={} F={})",
        src_chi, chi_dst, v_dst, e_dst, f_dst);
}

// ── Test 5: Legacy chain bisection with vertex identity diagnostics ──────────

/// Bisect the legacy 4-step chain to find which step introduces broken topology.
///
/// Dumps face count, Euler χ, plane histogram, and vertex identity diagnostics
/// (suggestion #5: count duplicate-position vertex clusters and directed edge
/// pairs with same geometry but different vertex IDs).
#[test]
fn bisect_legacy_chain_face_shatter() {
    let base_half = 5.0;
    let (mut topo, mut geom) = build_cube([0.0, 0.0, 0.0], base_half);

    for i in 0..4 {
        let angle = (i as f64) * std::f64::consts::TAU / 20.0;
        let x = base_half * angle.cos();
        let y = base_half * angle.sin();
        let z = -4.0 + (i as f64) * 0.4;

        eprintln!("--- Step {} tool at ({:.3}, {:.3}, {:.3}) ---", i, x, y, z);

        let (topo_b, geom_b) = build_cube([x, y, z], 0.3);
        let input = BooleanInput::new(topo, geom, topo_b, geom_b, BooleanOp::Subtraction);

        match execute_boolean_direct(input).into_result() {
            Ok(result) => {
                let (v, e, f, chi) = euler_audit(result.topology().arena());
                eprintln!("  result: V={} E={} F={} χ={}", v, e, f, chi);

                let hist = plane_histogram(result.topology(), result.geometry());
                for (plane, count) in &hist {
                    if *count > 1 {
                        eprintln!("  ⚠ {}× {}", count, plane);
                    }
                }

                dump_vertex_identity(result.topology(), result.geometry());

                let (t, g) = result.into_topo_geom();
                topo = t;
                geom = g;
            }
            Err(e) => {
                eprintln!("  FAILED at step {}: {:?}", i, e);
                break;
            }
        }
    }
}

/// Dump vertex identity diagnostics for a topology.
///
/// Counts: (a) vertex clusters where multiple VertexIds share the same
/// quantized position, (b) directed edge pairs where the geometry matches
/// but vertex IDs differ.
fn dump_vertex_identity(topo: &TopologyState, geom: &GeometryStore) {
    let tol = 1e-6;
    let mut pos_clusters: BTreeMap<(i64, i64, i64), Vec<VertexId>> = BTreeMap::new();

    for (vid, _) in topo.arena().iter_vertices() {
        if let Some(pos) = geom.get_vertex_position(vid) {
            let key = (
                (pos[0] / tol).round() as i64,
                (pos[1] / tol).round() as i64,
                (pos[2] / tol).round() as i64,
            );
            pos_clusters.entry(key).or_default().push(vid);
        }
    }

    let dup_clusters: Vec<_> = pos_clusters.values()
        .filter(|c| c.len() > 1)
        .collect();

    if !dup_clusters.is_empty() {
        eprintln!("  🔴 {} duplicate-position vertex clusters:", dup_clusters.len());
        for cluster in &dup_clusters {
            let pos = geom.get_vertex_position(cluster[0]).unwrap();
            let ids: Vec<String> = cluster.iter().map(|v| format!("V{}", v.index())).collect();
            eprintln!("    pos=[{:.6},{:.6},{:.6}] ids=[{}]",
                pos[0], pos[1], pos[2], ids.join(", "));
        }
    } else {
        eprintln!("  ✅ No duplicate-position vertices");
    }
}

// ── Tests 6-7: Known legacy defect (ignored) ────────────────────────────────

/// KNOWN DEFECT: Legacy output can't survive copy+stitch round-trip.
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
    let (_, _, _, chi_src) = euler_audit(topo.arena());

    let (result_topo, _, source_verts, dest_verts) = copy_and_stitch(&topo, &geom)
        .unwrap_or_else(|e| panic!("copy+stitch FAILED: {:?}", e));

    let (_, _, _, chi_dst) = euler_audit(result_topo.arena());
    assert_eq!(source_verts, dest_verts);
    assert_eq!(chi_dst, chi_src);
}

/// KNOWN DEFECT: Same vertex-identity issue, amplified.
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
            .unwrap_or_else(|e| panic!("legacy step {} failed: {:?}", i, e));
        let (t, g) = result.into_topo_geom();
        topo = t;
        geom = g;
    }

    let (_, _, _, src_chi) = euler_audit(topo.arena());
    let (result_topo, _, src_v, dst_v) = copy_and_stitch(&topo, &geom)
        .unwrap_or_else(|e| panic!("copy+stitch FAILED: {:?}", e));

    let (_, _, _, dst_chi) = euler_audit(result_topo.arena());
    assert_eq!(src_v, dst_v);
    assert_eq!(dst_chi, src_chi);
}
