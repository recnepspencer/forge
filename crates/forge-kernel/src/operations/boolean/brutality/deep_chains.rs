//! Deep Path-Dependent Chain Tests
//!
//! DOMAIN: Cascaded boolean operations where small errors in early steps
//! can compound into catastrophic failures at later steps.
//!
//! INVARIANTS:
//! - Euler χ = 2 at every intermediate step
//! - No panics or topology corruption after N chained operations
//! - If a step fails, the test reports exactly which step
//!
//! ═══════════════════════════════════════════════════════════════
//! REQUIRED CODE/MATH CHANGES TO PASS ALL TESTS:
//! ═══════════════════════════════════════════════════════════════
//!
//! 1. **Vertex Provenance Across Chains**: When a `BooleanResult` is used
//!    as input for the next operation, vertex `VertexMatchKey` provenance
//!    must be preserved so `assign_original_vertex_provenance` in the split
//!    phase can rebuild 3-plane keys. Currently, provenance is lost between
//!    operations because the copy phase doesn't carry it forward.
//!    THIS IS THE ROOT CAUSE of all chain failures.
//!
//! 2. **Spatial Vertex Welding Tolerance**: The `VertexWelder` in
//!    `copy.rs` uses `1e-18` squared tolerance for nearest-neighbor vertex
//!    matching. This is too tight for vertices that went through floating-point
//!    arithmetic in prior boolean ops. Needs to be relaxed to ~`1e-12` squared.
//!
//! 3. **Position-Based Stitch Fallback**: When `stitch_twins` can't find a
//!    matching reverse halfedge by vertex index, try matching by geometric
//!    position of the edge endpoints. This handles the case where the copy
//!    phase created duplicate vertices at the same position.
//!
//! 4. **Coordinate Drift Prevention**: After N chained operations, vertex
//!    positions accumulate floating-point roundoff. A global re-normalizer
//!    could snap vertices to their symbolic 3-plane intersection positions
//!    after each operation, preventing drift.
//!
//! 5. **Stitching Resilience**: The current stitch implementation should
//!    handle non-manifold junctions (>2 halfedges sharing a directed edge)
//!    via radial sorting. This is partially implemented in `select_best_twin`
//!    but needs testing under chained-op conditions.

use super::super::test_helpers::{
    build_cube, execute_boolean_logged, euler_audit,
};
use super::super::schema::{BooleanInput, BooleanOp};
use forge_topo::hashing::compute_arena_topology_hash;

// ══════════════════════════════════════════════════════════════
// §DC.1  UNION CHAIN (10 STEPS)
// ══════════════════════════════════════════════════════════════

/// DC.1 — ((A ∪ B) ∪ C) ∪ ... for 10 cubes at different offsets.
///
/// After each union, the Euler characteristic must be 2.
/// Tests whether boolean results remain usable as inputs.
#[test]
fn chain_union_10_steps() {
    let (mut topo, mut geom) = build_cube([0.0, 0.0, 0.0], 1.0);

    for step in 1..=10 {
        let offset = step as f64 * 0.8;
        let (topo_tool, geom_tool) = build_cube([offset, 0.0, 0.0], 1.0);

        let input = BooleanInput::new(
            topo, geom,
            topo_tool, geom_tool,
            BooleanOp::Union,
        );

        let result = execute_boolean_logged(input)
            .into_result()
            .unwrap_or_else(|e| panic!("Union chain step {step} failed: {e}"));
        let r = result;

        let (v, e, f, chi) = euler_audit(r.topology().arena());
        assert_eq!(
            chi, 2,
            "Union chain step {step} Euler violation: V={v} E={e} F={f} χ={chi}"
        );

        let parts = r.into_topo_geom();
        topo = parts.0;
        geom = parts.1;
    }

    let final_f = topo.arena().face_count();
    assert!(
        final_f >= 6,
        "10-step union chain should produce at least 6 faces, got {final_f}"
    );
}

// ══════════════════════════════════════════════════════════════
// §DC.2  SUBTRACTION CHAIN (10 STEPS)
// ══════════════════════════════════════════════════════════════

/// DC.2 — Large cube minus 10 small cubes at different positions.
///
/// Each subtraction carves a notch. After each step, Euler must be 2.
#[test]
fn chain_subtract_10_steps() {
    let (mut topo, mut geom) = build_cube([0.0, 0.0, 0.0], 5.0);

    for step in 0..10 {
        let x = -4.0 + step as f64 * 0.9;
        let (topo_tool, geom_tool) = build_cube([x, 0.0, 4.5], 0.5);

        let input = BooleanInput::new(
            topo, geom,
            topo_tool, geom_tool,
            BooleanOp::Subtraction,
        );

        let result = execute_boolean_logged(input)
            .into_result()
            .unwrap_or_else(|e| panic!("Subtract chain step {step} failed: {e}"));
        let r = result;

        let (v, e, f, chi) = euler_audit(r.topology().arena());
        assert_eq!(
            chi, 2,
            "Subtract chain step {step} Euler violation: V={v} E={e} F={f} χ={chi}"
        );

        let parts = r.into_topo_geom();
        topo = parts.0;
        geom = parts.1;
    }

    let final_f = topo.arena().face_count();
    assert!(
        final_f >= 6,
        "10-step subtract chain should produce at least 6 faces, got {final_f}"
    );
}

// ══════════════════════════════════════════════════════════════
// §DC.3  MIXED OPS CHAIN (10 STEPS)
// ══════════════════════════════════════════════════════════════

/// DC.3 — Alternating union/subtract for 10 steps.
///
/// Odd steps: union a cube. Even steps: subtract a cube at a different offset.
#[test]
fn chain_mixed_ops_10() {
    let (mut topo, mut geom) = build_cube([0.0, 0.0, 0.0], 2.0);

    for step in 1..=10 {
        let op = if step % 2 == 1 {
            BooleanOp::Union
        } else {
            BooleanOp::Subtraction
        };

        let offset = step as f64 * 0.4;
        let half = if step % 2 == 1 { 1.0 } else { 0.3 };
        let (topo_tool, geom_tool) = build_cube([offset, 0.0, 0.0], half);

        let input = BooleanInput::new(
            topo, geom,
            topo_tool, geom_tool,
            op,
        );

        match execute_boolean_logged(input).into_result() {
            Ok(result) => {
                let r = result;
                let (v, e, f, chi) = euler_audit(r.topology().arena());
                eprintln!("Mixed chain step {step} ({op:?}): V={v} E={e} F={f} χ={chi}");
                assert_eq!(
                    chi, 2,
                    "Mixed chain step {step} Euler violation: V={v} E={e} F={f} χ={chi}"
                );
                let parts = r.into_topo_geom();
                topo = parts.0;
                geom = parts.1;
            }
            Err(e) => {
                panic!("Mixed chain step {step} ({op:?}) failed: {e}");
            }
        }
    }
}

// ══════════════════════════════════════════════════════════════
// §DC.4  CHAIN WITH STEP IDENTIFICATION
// ══════════════════════════════════════════════════════════════

/// DC.4 — Same chain pattern but with explicit step labels for diagnostics.
///
/// Validates that if a step fails, the test output clearly identifies
/// which step caused the problem and dumps the topology state.
#[test]
fn chain_identifies_failing_step() {
    let (mut topo, mut geom) = build_cube([0.0, 0.0, 0.0], 3.0);

    let operations = vec![
        ([1.0, 0.0, 0.0], 1.0, BooleanOp::Union),
        ([0.0, 1.0, 0.0], 1.0, BooleanOp::Union),
        ([0.0, 0.0, 1.0], 1.0, BooleanOp::Union),
        ([0.5, 0.5, 0.5], 0.5, BooleanOp::Subtraction),
        ([-0.5, -0.5, 0.0], 0.8, BooleanOp::Union),
        ([1.5, 0.0, 0.0], 0.5, BooleanOp::Subtraction),
        ([0.0, 1.5, 0.0], 0.5, BooleanOp::Subtraction),
        ([0.0, 0.0, 1.5], 0.5, BooleanOp::Subtraction),
    ];

    for (step, (center, half, op)) in operations.iter().enumerate() {
        let pre_state = euler_audit(topo.arena());
        let (topo_tool, geom_tool) = build_cube(*center, *half);

        let input = BooleanInput::new(
            topo, geom,
            topo_tool, geom_tool,
            *op,
        );

        match execute_boolean_logged(input).into_result() {
            Ok(result) => {
                let r = result;
                let (v, e, f, chi) = euler_audit(r.topology().arena());
                eprintln!(
                    "Step {step} ({op:?} @ {center:?} h={half}): V={v} E={e} F={f} χ={chi}"
                );
                assert_eq!(
                    chi, 2,
                    "Step {step} ({op:?}) Euler violation: V={v} E={e} F={f}"
                );
                let parts = r.into_topo_geom();
                topo = parts.0;
                geom = parts.1;
            }
            Err(e) => {
                let (v, e_count, f, chi) = pre_state;
                eprintln!(
                    "FAILURE at step {step} ({op:?}): {e}\n\
                     State BEFORE failure: V={v} E={e_count} F={f} χ={chi}"
                );
                panic!("Chain failed at step {step}");
            }
        }
    }

}

// ══════════════════════════════════════════════════════════════
// §DC.5  MINIMAL REPRODUCTIONS
// ══════════════════════════════════════════════════════════════

/// Minimal repro: two overlapping subtracted notches.
///
/// This is the exact geometry from chain_subtract_10_steps step 0+1.
/// Step 0: subtract cube at (-4.0, 0, 4.5) half=0.5 → notch at x∈[-4.5,-3.5]
/// Step 1: subtract cube at (-3.1, 0, 4.5) half=0.5 → notch at x∈[-3.6,-2.6]
/// The two notches overlap in x∈[-3.6,-3.5].
#[test]
fn minimal_overlapping_notches() {
    let (topo, geom) = build_cube([0.0, 0.0, 0.0], 5.0);

    // Step 0: first notch
    let (tool0, tool0_g) = build_cube([-4.0, 0.0, 4.5], 0.5);
    let input0 = BooleanInput::new(topo, geom, tool0, tool0_g, BooleanOp::Subtraction);
    let r0 = execute_boolean_logged(input0).into_result().expect("Step 0 failed");
    let (v, e, f, chi) = euler_audit(r0.topology().arena());
    assert_eq!(chi, 2, "Step 0 Euler: V={v} E={e} F={f} χ={chi}");
    let (topo, geom) = r0.into_topo_geom();

    // Step 1: overlapping notch
    let (tool1, tool1_g) = build_cube([-3.1, 0.0, 4.5], 0.5);
    let input1 = BooleanInput::new(topo, geom, tool1, tool1_g, BooleanOp::Subtraction);
    let r1 = execute_boolean_logged(input1).into_result().expect("Step 1 failed (overlapping notch)");
    let (v, e, f, chi) = euler_audit(r1.topology().arena());
    assert_eq!(chi, 2, "Step 1 Euler: V={v} E={e} F={f} χ={chi}");
}

/// Control: two NON-overlapping subtracted notches.
///
/// Same as above but notches are spaced far apart (no overlap).
/// If this passes but overlapping fails, the bug is in how overlapping
/// geometry is handled (split/classify interaction with prior notch walls).
#[test]
fn minimal_nonoverlapping_notches() {
    let (topo, geom) = build_cube([0.0, 0.0, 0.0], 5.0);

    // Step 0: first notch at x=-4
    let (tool0, tool0_g) = build_cube([-4.0, 0.0, 4.5], 0.5);
    let input0 = BooleanInput::new(topo, geom, tool0, tool0_g, BooleanOp::Subtraction);
    let r0 = execute_boolean_logged(input0).into_result().expect("Step 0 failed");
    let (v, e, f, chi) = euler_audit(r0.topology().arena());
    assert_eq!(chi, 2, "Step 0 Euler: V={v} E={e} F={f} χ={chi}");
    
    // DIAGNOSTIC: print all edges that have at least one endpoint at z=5
    let arena = r0.topology().arena();
    let geom_ref = r0.geometry();
    eprintln!("=== STEP 0 RESULT: edges touching z=5 ===");
    for (he_id, _he) in arena.iter_half_edges() {
        let he_data = arena.get_half_edge(he_id).unwrap();
        let origin = he_data.origin();
        let next_data = arena.get_half_edge(he_data.next()).unwrap();
        let dest = next_data.origin();
        let p_o = geom_ref.get_vertex_position(origin).unwrap();
        let p_d = geom_ref.get_vertex_position(dest).unwrap();
        if (p_o[2] - 5.0).abs() < 1e-9 || (p_d[2] - 5.0).abs() < 1e-9 {
            let face = he_data.face();
            let twin = he_data.twin();
            let twin_face = arena.get_half_edge(twin).map(|t| t.face()).unwrap_or(face);
            eprintln!("  HE#{}: {origin}->{dest} [{:.3},{:.3},{:.3}]->[{:.3},{:.3},{:.3}] face={face} twin_face={twin_face}",
                he_id.index(), p_o[0], p_o[1], p_o[2], p_d[0], p_d[1], p_d[2]);
        }
    }
    eprintln!("=== END STEP 0 z=5 edges ===");
    
    let (topo, geom) = r0.into_topo_geom();

    // Step 1: NON-overlapping notch at x=+4 (far away)
    let (tool1, tool1_g) = build_cube([4.0, 0.0, 4.5], 0.5);
    let input1 = BooleanInput::new(topo, geom, tool1, tool1_g, BooleanOp::Subtraction);
    let r1 = execute_boolean_logged(input1).into_result().expect("Step 1 failed (non-overlapping)");
    let (v, e, f, chi) = euler_audit(r1.topology().arena());
    assert_eq!(chi, 2, "Step 1 Euler: V={v} E={e} F={f} χ={chi}");
}

/// Simplest case: single subtraction with flush z=5 boundary.
///
/// If this fails, the coplanar boundary problem exists even without chains.
#[test]
fn minimal_single_flush_subtraction() {
    let (topo, geom) = build_cube([0.0, 0.0, 0.0], 5.0);
    let (tool, tool_g) = build_cube([0.0, 0.0, 4.5], 0.5);
    let input = BooleanInput::new(topo, geom, tool, tool_g, BooleanOp::Subtraction);
    let r = execute_boolean_logged(input).into_result().expect("Single flush subtraction failed");
    let (v, e, f, chi) = euler_audit(r.topology().arena());
    assert_eq!(chi, 2, "Euler: V={v} E={e} F={f} χ={chi}");
}

/// Two non-flush subtractions (tool fully inside, no touching boundary).
///
/// If this passes, the bug is specifically about flush coplanar boundaries.
#[test]
fn minimal_two_interior_subtractions() {
    let (topo, geom) = build_cube([0.0, 0.0, 0.0], 5.0);

    // Step 0: interior subtraction
    let (tool0, tool0_g) = build_cube([-3.0, 0.0, 0.0], 0.5);
    let input0 = BooleanInput::new(topo, geom, tool0, tool0_g, BooleanOp::Subtraction);
    let r0 = execute_boolean_logged(input0).into_result().expect("Step 0 failed");
    // Interior subtraction creates a cavity: V-E+F = 4 (two shells)
    let (v, e, f, chi) = euler_audit(r0.topology().arena());
    eprintln!("Step 0: V={v} E={e} F={f} χ={chi}");
    let (topo, geom) = r0.into_topo_geom();

    // Step 1: another interior subtraction, far away
    let (tool1, tool1_g) = build_cube([3.0, 0.0, 0.0], 0.5);
    let input1 = BooleanInput::new(topo, geom, tool1, tool1_g, BooleanOp::Subtraction);
    let r1 = execute_boolean_logged(input1).into_result().expect("Step 1 failed");
    let (v, e, f, chi) = euler_audit(r1.topology().arena());
    eprintln!("Step 1: V={v} E={e} F={f} χ={chi}");
}

// ══════════════════════════════════════════════════════════════
// §DC.7  DIAGNOSTIC TESTS — ROOT CAUSE ISOLATION
// ══════════════════════════════════════════════════════════════

/// Diagnostic 1: epsilon-offset overlap.
///
/// Identical to minimal_overlapping_notches but shifts the second tool
/// by y=0.01, forcing true geometric intersection (2 cut points)
/// instead of coplanar tangent grazing.
///
/// If this PASSES but minimal_overlapping_notches FAILS:
///   → Bug is collinear/grazing handling in the split phase.
/// If this ALSO FAILS:
///   → Bug is precision drift / provenance loss in copy.rs.
#[test]
fn diagnostic_epsilon_offset_notches() {
    let (topo, geom) = build_cube([0.0, 0.0, 0.0], 5.0);

    let (tool0, tool0_g) = build_cube([-4.0, 0.0, 4.5], 0.5);
    let input0 = BooleanInput::new(topo, geom, tool0, tool0_g, BooleanOp::Subtraction);
    let r0 = execute_boolean_logged(input0).into_result().expect("Step 0 failed");
    let (topo, geom) = r0.into_topo_geom();

    let (tool1, tool1_g) = build_cube([-3.1, 0.01, 4.5], 0.5);
    let input1 = BooleanInput::new(topo, geom, tool1, tool1_g, BooleanOp::Subtraction);

    match execute_boolean_logged(input1).into_result() {
        Ok(r1) => {
            let (_v, _e, _f, chi) = euler_audit(r1.topology().arena());
            assert_eq!(chi, 2, "Passed but Euler violated");
            eprintln!("DIAGNOSTIC: Epsilon-offset PASSED → bug is collinear/grazing handling");
        }
        Err(e) => {
            panic!("DIAGNOSTIC: Epsilon-offset FAILED → bug is precision drift. Error: {e}");
        }
    }
}

/// Diagnostic 2: single tangent corner graze (no chaining).
///
/// A tool cube whose corner touches a target cube's corner at [5,5,5].
/// No chaining — tests whether the split logic can handle a tangent
/// vertex graze in pure isolation.
///
/// If this FAILS:
///   → Split logic fundamentally cannot handle tangent vertex grazes.
/// If this PASSES:
///   → The problem is specific to chained operations.
#[test]
fn diagnostic_manual_tangent_graze() {
    let (topo, geom) = build_cube([0.0, 0.0, 0.0], 5.0);
    let (tool, tool_g) = build_cube([6.0, 6.0, 6.0], 1.0);

    let input = BooleanInput::new(topo, geom, tool, tool_g, BooleanOp::Subtraction);
    match execute_boolean_logged(input).into_result() {
        Ok(r) => {
            let (v, e, f, chi) = euler_audit(r.topology().arena());
            eprintln!("DIAGNOSTIC: Tangent graze PASSED: V={v} E={e} F={f} χ={chi}");
        }
        Err(e) => {
            panic!("DIAGNOSTIC: Tangent graze FAILED → split logic broken. Error: {e}");
        }
    }
}

/// Diagnostic 3: vertex provenance survival audit.
///
/// Checks whether exact rational vertex positions survive the
/// boolean result → new input pipeline. If exact positions are
/// lost during copy, assign_original_vertex_provenance can't
/// build VertexMatchKeys and cross-solid vertex gluing fails.
#[test]
fn diagnostic_vertex_provenance_survival() {
    let (topo, geom) = build_cube([0.0, 0.0, 0.0], 5.0);
    let (tool0, tool0_g) = build_cube([-4.0, 0.0, 4.5], 0.5);
    let input0 = BooleanInput::new(topo, geom, tool0, tool0_g, BooleanOp::Subtraction);
    let r0 = execute_boolean_logged(input0).into_result().expect("Step 0 failed");

    let result_geom = r0.geometry();
    let result_arena = r0.topology().arena();
    let mut exact_count = 0u32;
    let mut total_count = 0u32;
    for (vid, _) in result_arena.iter_vertices() {
        total_count += 1;
        if result_geom.get_vertex_position_exact(vid).is_some() {
            exact_count += 1;
        }
    }

    eprintln!("DIAGNOSTIC: {exact_count}/{total_count} vertices have exact rational positions");
    assert!(exact_count > 0,
        "PROVENANCE LOSS: No exact vertices survived Step 0! \
         copy.rs is stripping exact positions.");
    assert_eq!(exact_count, total_count,
        "PARTIAL PROVENANCE LOSS: {exact_count}/{total_count} vertices have exact positions. \
         Some vertices lost their symbolic identity.");
}

/// Diagnostic 4: concave face detection after Step 0.
///
/// Dumps the face topology (edge count, vertex positions) for every face
/// in the Step 0 result. If the postprocessor merged coplanar fragments
/// into concave polygons (>4 edges), the split logic in Step 1 may fail
/// because it assumes convex face geometry.
#[test]
fn diagnostic_concave_face_audit() {
    let (topo, geom) = build_cube([0.0, 0.0, 0.0], 5.0);
    let (tool0, tool0_g) = build_cube([-4.0, 0.0, 4.5], 0.5);
    let input0 = BooleanInput::new(topo, geom, tool0, tool0_g, BooleanOp::Subtraction);
    let r0 = execute_boolean_logged(input0).into_result().expect("Step 0 failed");

    let arena = r0.topology().arena();
    let geom_ref = r0.geometry();

    let mut max_edges = 0usize;
    let mut concave_faces = Vec::new();

    for (fid, _) in arena.iter_faces() {
        let edges: Vec<_> = forge_topo::traverse::FaceEdgeIterator::new(arena, fid)
            .unwrap()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        let edge_count = edges.len();
        if edge_count > max_edges { max_edges = edge_count; }

        let plane = geom_ref.get_face_plane(fid);
        let plane_str = plane.map(|p| {
            let n = p.normal();
            format!("n=[{:.2},{:.2},{:.2}] d={:.2}", n[0], n[1], n[2], p.offset())
        }).unwrap_or("??".into());

        let verts: Vec<String> = edges.iter().map(|he_id| {
            let he = arena.get_half_edge(*he_id).unwrap();
            let v = he.origin();
            let pos = geom_ref.get_vertex_position(v).unwrap();
            format!("[{:.2},{:.2},{:.2}]", pos[0], pos[1], pos[2])
        }).collect();

        eprintln!("  Face#{}: {}E {} | {}", fid.index(), edge_count, plane_str, verts.join(" → "));

        if edge_count > 4 {
            concave_faces.push((fid, edge_count));
        }
    }

    eprintln!("DIAGNOSTIC: max edges per face = {max_edges}, concave faces (>4E) = {:?}",
        concave_faces.iter().map(|(f, e)| format!("F#{} ({}E)", f.index(), e)).collect::<Vec<_>>());

    if !concave_faces.is_empty() {
        eprintln!("DIAGNOSTIC: CONCAVE FACES DETECTED! The postprocessor merged coplanar fragments into >4-gons.");
        eprintln!("This is likely the root cause — the split logic handles convex faces only.");
    }
}

// ══════════════════════════════════════════════════════════════
// §DC.8  SYMBOLIC PLANE PRESERVATION
// ══════════════════════════════════════════════════════════════

/// DC.8a — Cube vertices must carry symbolic plane triples at birth.
///
/// Each vertex of a cube is the intersection of exactly 3 planes.
/// After `make_cube`, every vertex's `get_vertex_symbolic_planes`
/// must return `Some([p0, p1, p2])`.
#[test]
fn cube_vertices_born_with_symbolic_planes() {
    let (topo, geom) = build_cube([0.0, 0.0, 0.0], 1.0);
    let arena = topo.arena();

    let mut with_planes = 0u32;
    let mut without_planes = 0u32;

    for (vid, _) in arena.iter_vertices() {
        if geom.get_vertex_symbolic_planes(vid).is_some() {
            with_planes += 1;
        } else {
            without_planes += 1;
        }
    }

    let total = with_planes + without_planes;
    eprintln!(
        "Cube symbolic planes: {with_planes}/{total} vertices have plane triples"
    );
    assert_eq!(
        without_planes, 0,
        "All {total} cube vertices should have symbolic planes, but {without_planes} are missing them"
    );
    assert_eq!(total, 8, "A cube should have 8 vertices, got {total}");
}

/// DC.8b — Symbolic plane triples must survive a boolean chain.
///
/// After A ∪ B, vertices that were original cube corners should still
/// carry their symbolic plane indices. This tests the copy phase in
/// `copy.rs` which must propagate `get_vertex_symbolic_planes`.
#[test]
fn symbolic_planes_survive_boolean_chain() {
    let (topo_a, geom_a) = build_cube([0.0, 0.0, 0.0], 1.0);
    let (topo_b, geom_b) = build_cube([3.0, 0.0, 0.0], 1.0);

    let input = BooleanInput::new(
        topo_a, geom_a,
        topo_b, geom_b,
        BooleanOp::Union,
    );

    let result = execute_boolean_logged(input)
        .into_result()
        .expect("Disjoint union failed");

    let arena = result.topology().arena();
    let geom = result.geometry();

    let mut with_planes = 0u32;
    let mut without_planes = 0u32;

    for (vid, _) in arena.iter_vertices() {
        if geom.get_vertex_symbolic_planes(vid).is_some() {
            with_planes += 1;
        } else {
            without_planes += 1;
        }
    }

    let total = with_planes + without_planes;
    eprintln!(
        "Post-union symbolic planes: {with_planes}/{total} vertices have plane triples"
    );

    assert_eq!(
        without_planes, 0,
        "After disjoint union of two cubes, all {total} vertices should preserve \
         symbolic planes (got {with_planes}/{total}). \
         The copy phase is dropping plane triples."
    );
    assert_eq!(total, 16, "Disjoint union of 2 cubes should have 16 vertices, got {total}");
}

// ══════════════════════════════════════════════════════════════
// §DC.9  IDEMPOTENCE TESTS (DRIFT DETECTOR)
// ══════════════════════════════════════════════════════════════

/// DC.9a — Self-Union Idempotence.
///
/// result1 = union(A, B), result2 = union(result1, B).
/// Since result1 already contains B, the second union should produce
/// topologically identical output (same V, E, F counts and hash).
///
/// If this fails → classification drift, merge instability, or
/// nondeterministic ordering.
#[test]
fn idempotence_self_union() {
    let (topo_a, geom_a) = build_cube([0.0, 0.0, 0.0], 1.0);
    let (topo_b, geom_b) = build_cube([0.8, 0.0, 0.0], 1.0);

    let input1 = BooleanInput::new(
        topo_a, geom_a,
        topo_b.clone(), geom_b.clone(),
        BooleanOp::Union,
    );

    let r1 = execute_boolean_logged(input1)
        .into_result()
        .expect("First union(A,B) failed");

    let (v1, e1, f1, chi1) = euler_audit(r1.topology().arena());
    assert_eq!(chi1, 2, "First union Euler violation: V={v1} E={e1} F={f1} χ={chi1}");
    let hash1 = compute_arena_topology_hash(r1.topology().arena());

    let (topo_r1, geom_r1) = r1.into_topo_geom();

    let input2 = BooleanInput::new(
        topo_r1, geom_r1,
        topo_b, geom_b,
        BooleanOp::Union,
    );

    let r2 = execute_boolean_logged(input2)
        .into_result()
        .expect("Second union(result1, B) failed");

    let (v2, e2, f2, chi2) = euler_audit(r2.topology().arena());
    assert_eq!(chi2, 2, "Second union Euler violation: V={v2} E={e2} F={f2} χ={chi2}");

    assert_eq!(
        v1, v2,
        "IDEMPOTENCE: vertex count changed: {v1} → {v2}"
    );
    assert_eq!(
        e1, e2,
        "IDEMPOTENCE: edge count changed: {e1} → {e2}"
    );
    assert_eq!(
        f1, f2,
        "IDEMPOTENCE: face count changed: {f1} → {f2}"
    );

    let hash2 = compute_arena_topology_hash(r2.topology().arena());
    assert_eq!(
        hash1, hash2,
        "IDEMPOTENCE: topology hash changed: {hash1:#x} → {hash2:#x}"
    );
}

/// DC.9b — Double Application Neutrality.
///
/// result1 = union(A, B), result2 = subtract(result1, B).
/// The result should topologically approximate A (Euler=2, face count ≥ 6).
///
/// If this fails → boundary classification inconsistency or
/// split bookkeeping error.
#[test]
fn idempotence_double_application_neutrality() {
    let (topo_a, geom_a) = build_cube([0.0, 0.0, 0.0], 2.0);
    let (topo_b, geom_b) = build_cube([1.5, 0.0, 0.0], 1.0);

    let input1 = BooleanInput::new(
        topo_a, geom_a,
        topo_b.clone(), geom_b.clone(),
        BooleanOp::Union,
    );

    let r1 = execute_boolean_logged(input1)
        .into_result()
        .expect("union(A,B) failed");

    let (v1, e1, f1, chi1) = euler_audit(r1.topology().arena());
    assert_eq!(chi1, 2, "Union Euler violation: V={v1} E={e1} F={f1} χ={chi1}");

    let (topo_r1, geom_r1) = r1.into_topo_geom();

    let input2 = BooleanInput::new(
        topo_r1, geom_r1,
        topo_b, geom_b,
        BooleanOp::Subtraction,
    );

    let r2 = execute_boolean_logged(input2)
        .into_result()
        .expect("subtract(union(A,B), B) failed");

    let (v2, e2, f2, chi2) = euler_audit(r2.topology().arena());
    eprintln!(
        "NEUTRALITY: union(A,B) had V={v1} E={e1} F={f1} | \
         subtract back → V={v2} E={e2} F={f2} χ={chi2}"
    );

    assert_eq!(chi2, 2, "Neutrality Euler violation: V={v2} E={e2} F={f2} χ={chi2}");
    assert!(
        f2 >= 6,
        "NEUTRALITY: subtract(union(A,B), B) should produce at least 6 faces \
         (original cube shape), got {f2}"
    );
}

// ══════════════════════════════════════════════════════════════
// §DC.10  COMMUTATIVITY + ASSOCIATIVITY TESTS
// ══════════════════════════════════════════════════════════════

/// DC.10a — Order Stability (Commutativity).
///
/// union(A,B) vs union(B,A) for overlapping grid-aligned cubes.
/// Topology counts and hash must match.
///
/// If this fails → nondeterministic split ordering, hash iteration
/// instability, or BSP asymmetry.
#[test]
fn commutativity_order_stability() {
    let configs: Vec<([f64; 3], [f64; 3], f64)> = vec![
        ([0.0, 0.0, 0.0], [0.8, 0.0, 0.0], 1.0),
        ([0.0, 0.0, 0.0], [0.5, 0.5, 0.0], 1.0),
        ([0.0, 0.0, 0.0], [1.0, 1.0, 1.0], 1.5),
    ];

    for (case_idx, (ca, cb, half)) in configs.iter().enumerate() {
        let (ta1, ga1) = build_cube(*ca, *half);
        let (tb1, gb1) = build_cube(*cb, *half);
        let (ta2, ga2) = build_cube(*ca, *half);
        let (tb2, gb2) = build_cube(*cb, *half);

        let input_ab = BooleanInput::new(ta1, ga1, tb1, gb1, BooleanOp::Union);
        let input_ba = BooleanInput::new(tb2, gb2, ta2, ga2, BooleanOp::Union);

        let r_ab = execute_boolean_logged(input_ab)
            .into_result()
            .unwrap_or_else(|e| panic!("Case {case_idx} union(A,B) failed: {e}"));
        let r_ba = execute_boolean_logged(input_ba)
            .into_result()
            .unwrap_or_else(|e| panic!("Case {case_idx} union(B,A) failed: {e}"));

        let (v_ab, e_ab, f_ab, _) = euler_audit(r_ab.topology().arena());
        let (v_ba, e_ba, f_ba, _) = euler_audit(r_ba.topology().arena());

        assert_eq!(
            (v_ab, e_ab, f_ab), (v_ba, e_ba, f_ba),
            "COMMUTATIVITY case {case_idx}: \
             union(A,B) → V={v_ab} E={e_ab} F={f_ab} vs \
             union(B,A) → V={v_ba} E={e_ba} F={f_ba}"
        );

        let hash_ab = compute_arena_topology_hash(r_ab.topology().arena());
        let hash_ba = compute_arena_topology_hash(r_ba.topology().arena());
        assert_eq!(
            hash_ab, hash_ba,
            "COMMUTATIVITY case {case_idx}: hash {hash_ab:#x} vs {hash_ba:#x}"
        );
    }
}

/// DC.10b — Small Associativity Chain.
///
/// ((A ∪ B) ∪ C) ∪ D  vs  A ∪ (B ∪ (C ∪ D))
/// Topology counts must match for 4 axis-aligned cubes.
///
/// If this fails at just 4 shapes, you don't need 64 to debug.
#[test]
fn associativity_small_chain() {
    let cubes: Vec<([f64; 3], f64)> = vec![
        ([0.0, 0.0, 0.0], 1.0),
        ([1.5, 0.0, 0.0], 1.0),
        ([0.0, 1.5, 0.0], 1.0),
        ([0.0, 0.0, 1.5], 1.0),
    ];

    let left_fold = {
        let (mut topo, mut geom) = build_cube(cubes[0].0, cubes[0].1);
        for (step, (center, half)) in cubes[1..].iter().enumerate() {
            let (t_tool, g_tool) = build_cube(*center, *half);
            let input = BooleanInput::new(topo, geom, t_tool, g_tool, BooleanOp::Union);
            let r = execute_boolean_logged(input)
                .into_result()
                .unwrap_or_else(|e| panic!("Left-fold step {step} failed: {e}"));
            let parts = r.into_topo_geom();
            topo = parts.0;
            geom = parts.1;
        }
        euler_audit(topo.arena())
    };

    let right_fold = {
        let last = cubes.len() - 1;
        let (mut topo, mut geom) = build_cube(cubes[last].0, cubes[last].1);
        for step in (0..last).rev() {
            let (t_tool, g_tool) = build_cube(cubes[step].0, cubes[step].1);
            let input = BooleanInput::new(t_tool, g_tool, topo, geom, BooleanOp::Union);
            let r = execute_boolean_logged(input)
                .into_result()
                .unwrap_or_else(|e| panic!("Right-fold step {} failed: {e}", last - 1 - step));
            let parts = r.into_topo_geom();
            topo = parts.0;
            geom = parts.1;
        }
        euler_audit(topo.arena())
    };

    let (v_l, e_l, f_l, chi_l) = left_fold;
    let (v_r, e_r, f_r, chi_r) = right_fold;

    eprintln!(
        "ASSOCIATIVITY: left-fold V={v_l} E={e_l} F={f_l} χ={chi_l} | \
         right-fold V={v_r} E={e_r} F={f_r} χ={chi_r}"
    );

    assert_eq!(chi_l, 2, "Left-fold Euler violation: χ={chi_l}");
    assert_eq!(chi_r, 2, "Right-fold Euler violation: χ={chi_r}");

    assert_eq!(
        (v_l, e_l, f_l), (v_r, e_r, f_r),
        "ASSOCIATIVITY: left-fold and right-fold produce different topology counts"
    );
}

// ══════════════════════════════════════════════════════════════
// §DC.11  PLANE INTERN STABILITY TEST
// ══════════════════════════════════════════════════════════════

/// DC.11 — Plane Intern Replay.
///
/// After a boolean operation, extract all unique face planes,
/// reconstruct them via Plane::new with the same normal/offset,
/// and verify the reconstructed plane has identical coefficients.
///
/// If any coefficient changes at the bit level → the canonicalization
/// pipeline is not closed (idempotent).
#[test]
fn plane_intern_stability() {
    use forge_geom::Plane;

    let (topo_a, geom_a) = build_cube([0.0, 0.0, 0.0], 2.0);
    let (topo_b, geom_b) = build_cube([1.0, 0.0, 0.0], 1.0);

    let input = BooleanInput::new(topo_a, geom_a, topo_b, geom_b, BooleanOp::Union);
    let r = execute_boolean_logged(input)
        .into_result()
        .expect("Union for plane intern test failed");

    let arena = r.topology().arena();
    let geom = r.geometry();

    let mut planes_checked = 0u32;
    let mut planes_drifted = 0u32;

    for (fid, _) in arena.iter_faces() {
        let plane = match geom.get_face_plane(fid) {
            Some(p) => p,
            None => continue,
        };

        let (a, b, c, d) = plane.exact_coefficients();

        let reconstructed = match Plane::from_rationals(
            a.clone(), b.clone(), c.clone(), d.clone(),
        ) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("INTERN STABILITY: Face#{} plane reconstruction failed: {e}", fid.index());
                planes_drifted += 1;
                planes_checked += 1;
                continue;
            }
        };

        let n_orig = plane.normal();
        let d_orig = plane.offset();
        let n_recon = reconstructed.normal();
        let d_recon = reconstructed.offset();

        planes_checked += 1;

        let n_match = n_orig[0].to_bits() == n_recon[0].to_bits()
            && n_orig[1].to_bits() == n_recon[1].to_bits()
            && n_orig[2].to_bits() == n_recon[2].to_bits();
        let d_match = d_orig.to_bits() == d_recon.to_bits();

        if !n_match || !d_match {
            eprintln!(
                "INTERN DRIFT: Face#{} original n=[{:.17e},{:.17e},{:.17e}] d={:.17e} \
                 → reconstructed n=[{:.17e},{:.17e},{:.17e}] d={:.17e}",
                fid.index(),
                n_orig[0], n_orig[1], n_orig[2], d_orig,
                n_recon[0], n_recon[1], n_recon[2], d_recon
            );
            planes_drifted += 1;
        }
    }

    eprintln!(
        "INTERN STABILITY: {planes_checked} planes checked, {planes_drifted} drifted"
    );
    assert_eq!(
        planes_drifted, 0,
        "INTERN STABILITY: {planes_drifted}/{planes_checked} planes had bit-level drift \
         after reconstruction. Canonicalization pipeline is not closed."
    );
}

// ══════════════════════════════════════════════════════════════
// §DC.12  BOUNDARY CLASSIFICATION AUDIT
// ══════════════════════════════════════════════════════════════

/// DC.12 — Zero Classification Stability.
///
/// For every vertex in the result, compute signed distance to every
/// face plane twice. The sign classification must be identical both times.
///
/// If classification changes between passes → epsilon drift,
/// non-deterministic sign logic, or uninitialized state reuse.
#[test]
fn zero_classification_stability() {
    use forge_geom::primitives::plane::signed_distance;

    let (topo_a, geom_a) = build_cube([0.0, 0.0, 0.0], 2.0);
    let (topo_b, geom_b) = build_cube([1.0, 0.0, 0.0], 1.0);

    let input = BooleanInput::new(topo_a, geom_a, topo_b, geom_b, BooleanOp::Union);
    let r = execute_boolean_logged(input)
        .into_result()
        .expect("Union for classification test failed");

    let arena = r.topology().arena();
    let geom = r.geometry();

    let planes: Vec<_> = arena
        .iter_faces()
        .filter_map(|(fid, _)| geom.get_face_plane(fid).cloned())
        .collect();

    let vertices: Vec<_> = arena
        .iter_vertices()
        .map(|(vid, _)| (vid, geom.get_vertex_position(vid).unwrap()))
        .collect();

    let mut mismatches = 0u32;
    let mut total = 0u32;

    for plane in &planes {
        for (vid, pos) in &vertices {
            let d1 = signed_distance(plane, pos);
            let d2 = signed_distance(plane, pos);
            total += 1;

            let sign1 = d1.partial_cmp(&0.0);
            let sign2 = d2.partial_cmp(&0.0);

            if sign1 != sign2 {
                eprintln!(
                    "CLASSIFICATION DRIFT: vertex {} pos=[{:.6},{:.6},{:.6}] \
                     pass1={:?} ({:.2e}) pass2={:?} ({:.2e})",
                    vid.index(), pos[0], pos[1], pos[2], sign1, d1, sign2, d2
                );
                mismatches += 1;
            }
        }
    }

    eprintln!(
        "CLASSIFICATION STABILITY: {total} classifications, {mismatches} mismatches"
    );
    assert_eq!(
        mismatches, 0,
        "CLASSIFICATION DRIFT: {mismatches}/{total} vertex-plane classifications \
         changed between identical passes. Sign logic is non-deterministic."
    );
}

// ══════════════════════════════════════════════════════════════
// §DC.13  DETERMINISTIC REPLAY HARNESS
// ══════════════════════════════════════════════════════════════

/// DC.13 — Step-by-Step Invariant Harness.
///
/// For N=8 union steps, at each step assert:
///   1. Euler χ = 2
///   2. No orphan halfedges (twin reciprocity)
///   3. No duplicate coplanar faces with identical normals at same offset
///   4. Topology hash is stable across 2 replays of the same step
///
/// When failure occurs, reports exact step + invariant + diff.
#[test]
fn deterministic_replay_harness() {
    let operations: Vec<([f64; 3], f64, BooleanOp)> = vec![
        ([0.8, 0.0, 0.0], 1.0, BooleanOp::Union),
        ([0.0, 0.8, 0.0], 1.0, BooleanOp::Union),
        ([0.0, 0.0, 0.8], 1.0, BooleanOp::Union),
        ([1.6, 0.0, 0.0], 1.0, BooleanOp::Union),
        ([0.0, 1.6, 0.0], 1.0, BooleanOp::Union),
        ([0.0, 0.0, 1.6], 1.0, BooleanOp::Union),
        ([0.8, 0.8, 0.0], 1.0, BooleanOp::Union),
        ([0.0, 0.8, 0.8], 1.0, BooleanOp::Union),
    ];

    let mut run_chain = |run_label: &str| -> Vec<(usize, usize, usize, isize, u128)> {
        let (mut topo, mut geom) = build_cube([0.0, 0.0, 0.0], 1.0);
        let mut snapshots = Vec::new();

        for (step, (center, half, op)) in operations.iter().enumerate() {
            let (t_tool, g_tool) = build_cube(*center, *half);
            let input = BooleanInput::new(topo, geom, t_tool, g_tool, *op);

            let result = match execute_boolean_logged(input).into_result() {
                Ok(r) => r,
                Err(e) => {
                    eprintln!(
                        "REPLAY HARNESS [{run_label}]: step {step} ({op:?} @ {center:?}) FAILED: {e}"
                    );
                    panic!("Chain failed at step {step} in run '{run_label}'");
                }
            };

            let arena = result.topology().arena();

            let (v, e, f, chi) = euler_audit(arena);
            assert_eq!(
                chi, 2,
                "REPLAY [{run_label}] step {step}: Euler violation V={v} E={e} F={f} χ={chi}"
            );

            for (he_id, he_data) in arena.iter_half_edges() {
                let twin_id = he_data.twin();
                assert_ne!(
                    he_id, twin_id,
                    "REPLAY [{run_label}] step {step}: orphan halfedge {} (self-twin)",
                    he_id.index()
                );
                let twin_data = arena.get_half_edge(twin_id).unwrap_or_else(|_| {
                    panic!(
                        "REPLAY [{run_label}] step {step}: halfedge {} twin {} is stale",
                        he_id.index(),
                        twin_id.index()
                    );
                });
                assert_eq!(
                    twin_data.twin(), he_id,
                    "REPLAY [{run_label}] step {step}: twin reciprocity violated \
                     he[{}].twin={}, he[{}].twin={} (expected {})",
                    he_id.index(), twin_id.index(),
                    twin_id.index(), twin_data.twin().index(), he_id.index()
                );
            }

            let result_geom = result.geometry();
            let mut plane_keys: Vec<(i64, i64, i64, i64)> = Vec::new();
            for (fid, _) in arena.iter_faces() {
                if let Some(plane) = result_geom.get_face_plane(fid) {
                    let n = plane.normal();
                    let d = plane.offset();
                    let key = (
                        (n[0] * 1e9).round() as i64,
                        (n[1] * 1e9).round() as i64,
                        (n[2] * 1e9).round() as i64,
                        (d * 1e9).round() as i64,
                    );
                    plane_keys.push(key);
                }
            }
            plane_keys.sort();
            let dup_count = plane_keys.windows(2).filter(|w| w[0] == w[1]).count();
            if dup_count > 0 {
                eprintln!(
                    "REPLAY [{run_label}] step {step}: WARNING — {dup_count} duplicate plane pairs \
                     (may indicate un-merged coplanar fragments)"
                );
            }

            let hash = compute_arena_topology_hash(arena);
            snapshots.push((v, e, f, chi, hash));

            eprintln!(
                "REPLAY [{run_label}] step {step}: V={v} E={e} F={f} χ={chi} hash={hash:#x}"
            );

            let parts = result.into_topo_geom();
            topo = parts.0;
            geom = parts.1;
        }

        snapshots
    };

    let run1 = run_chain("run1");
    let run2 = run_chain("run2");

    for (step, (s1, s2)) in run1.iter().zip(run2.iter()).enumerate() {
        assert_eq!(
            s1, s2,
            "DETERMINISM FAILURE at step {step}: \
             run1=(V={},E={},F={},χ={},hash={:#x}) vs \
             run2=(V={},E={},F={},χ={},hash={:#x})",
            s1.0, s1.1, s1.2, s1.3, s1.4,
            s2.0, s2.1, s2.2, s2.3, s2.4
        );
    }
}
