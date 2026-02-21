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
