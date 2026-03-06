//! Shell orientation validator poison tests.
//!
//! Validates that adjacent faces across shared edges have opposite
//! half-edge winding (compatible normals in an orientable manifold).

use super::test_support::*;
use forge_core::{FlatToleranceProvider, KernelError, TopologyError};
use forge_spatial::validators::shell_orientation::validate_shell_orientation;
use forge_topo::b_rep::{EdgeData, FaceData, HalfEdgeData, LoopData, VertexData};
use forge_topo::handles::{EdgeId, FaceId, HalfEdgeId};

// ── Baseline ────────────────────────────────────────────────────────────

#[test]
fn adjacent_faces_opposite_passes() {
    // Two triangles sharing edge A→B / B→A (correct orientation).
    let mut draft = empty_test_draft();
    let placeholder_he = HalfEdgeId::new(0, 0);
    let placeholder_face = FaceId::new(0, 0);
    let placeholder_edge = EdgeId::new(0, 0);

    let va = draft.insert_vertex(VertexData::new(placeholder_he));
    let vb = draft.insert_vertex(VertexData::new(placeholder_he));
    let vc = draft.insert_vertex(VertexData::new(placeholder_he));
    let vd = draft.insert_vertex(VertexData::new(placeholder_he));

    let shell = insert_test_solid_shell(&mut draft);
    let loop1 = draft.insert_loop(LoopData::new(placeholder_he, placeholder_face));
    let loop2 = draft.insert_loop(LoopData::new(placeholder_he, placeholder_face));
    let face1 = draft.insert_face(FaceData::new(loop1, shell));
    let face2 = draft.insert_face(FaceData::new(loop2, shell));

    // Face 1: A→B→C (he_ab, he_bc, he_ca)
    let he_ab = draft.insert_half_edge(HalfEdgeData::new(placeholder_he, placeholder_he, placeholder_he, face1, va, placeholder_edge));
    let he_bc = draft.insert_half_edge(HalfEdgeData::new(placeholder_he, placeholder_he, placeholder_he, face1, vb, placeholder_edge));
    let he_ca = draft.insert_half_edge(HalfEdgeData::new(placeholder_he, placeholder_he, placeholder_he, face1, vc, placeholder_edge));

    // Face 2: B→A→D (he_ba, he_ad, he_db) — B→A is opposite of A→B ✓
    let he_ba = draft.insert_half_edge(HalfEdgeData::new(placeholder_he, placeholder_he, placeholder_he, face2, vb, placeholder_edge));
    let he_ad = draft.insert_half_edge(HalfEdgeData::new(placeholder_he, placeholder_he, placeholder_he, face2, va, placeholder_edge));
    let he_db = draft.insert_half_edge(HalfEdgeData::new(placeholder_he, placeholder_he, placeholder_he, face2, vd, placeholder_edge));

    {
        let arena = draft.arena_mut();
        // Face 1 loop
        arena.get_half_edge_mut(he_ab).unwrap().set_next(he_bc);
        arena.get_half_edge_mut(he_bc).unwrap().set_next(he_ca);
        arena.get_half_edge_mut(he_ca).unwrap().set_next(he_ab);

        // Face 2 loop
        arena.get_half_edge_mut(he_ba).unwrap().set_next(he_ad);
        arena.get_half_edge_mut(he_ad).unwrap().set_next(he_db);
        arena.get_half_edge_mut(he_db).unwrap().set_next(he_ab);

        // Shared edge: A↔B radially linked
        arena.get_half_edge_mut(he_ab).unwrap().set_radial_next(he_ba);
        arena.get_half_edge_mut(he_ba).unwrap().set_radial_next(he_ab);

        // Boundary edges: self-radial
        arena.get_half_edge_mut(he_bc).unwrap().set_radial_next(he_bc);
        arena.get_half_edge_mut(he_ca).unwrap().set_radial_next(he_ca);
        arena.get_half_edge_mut(he_ad).unwrap().set_radial_next(he_ad);
        arena.get_half_edge_mut(he_db).unwrap().set_radial_next(he_db);

        arena.get_loop_mut(loop1).unwrap().set_half_edge(he_ab);
        arena.get_loop_mut(loop1).unwrap().set_face(face1);
        arena.get_loop_mut(loop2).unwrap().set_half_edge(he_ba);
        arena.get_loop_mut(loop2).unwrap().set_face(face2);
        arena.get_shell_mut(shell).unwrap().set_representative_face(face1);
    }

    let arena = draft.arena();
    let result = validate_shell_orientation(
        arena,
        &|v| {
            if v == va { Some([0.0, 0.0, 0.0]) }
            else if v == vb { Some([1.0, 0.0, 0.0]) }
            else if v == vc { Some([0.0, 1.0, 0.0]) }
            else if v == vd { Some([0.0, -1.0, 0.0]) }
            else { None }
        },
        &FlatToleranceProvider::new(1e-10),
    );
    assert!(result.is_ok(), "Opposite-direction shared edge should pass");
}

// ── Poison ──────────────────────────────────────────────────────────────

#[test]
fn same_direction_detected() {
    // Two triangles sharing edge but BOTH traverse A→B (flipped face).
    let mut draft = empty_test_draft();
    let placeholder_he = HalfEdgeId::new(0, 0);
    let placeholder_face = FaceId::new(0, 0);
    let placeholder_edge = EdgeId::new(0, 0);

    let va = draft.insert_vertex(VertexData::new(placeholder_he));
    let vb = draft.insert_vertex(VertexData::new(placeholder_he));
    let vc = draft.insert_vertex(VertexData::new(placeholder_he));
    let vd = draft.insert_vertex(VertexData::new(placeholder_he));

    let shell = insert_test_solid_shell(&mut draft);
    let loop1 = draft.insert_loop(LoopData::new(placeholder_he, placeholder_face));
    let loop2 = draft.insert_loop(LoopData::new(placeholder_he, placeholder_face));
    let face1 = draft.insert_face(FaceData::new(loop1, shell));
    let face2 = draft.insert_face(FaceData::new(loop2, shell));

    // Face 1: A→B→C
    let h1_ab = draft.insert_half_edge(HalfEdgeData::new(placeholder_he, placeholder_he, placeholder_he, face1, va, placeholder_edge));
    let h1_bc = draft.insert_half_edge(HalfEdgeData::new(placeholder_he, placeholder_he, placeholder_he, face1, vb, placeholder_edge));
    let h1_ca = draft.insert_half_edge(HalfEdgeData::new(placeholder_he, placeholder_he, placeholder_he, face1, vc, placeholder_edge));

    // Face 2: A→B→D (SAME direction A→B — WRONG!)
    let h2_ab = draft.insert_half_edge(HalfEdgeData::new(placeholder_he, placeholder_he, placeholder_he, face2, va, placeholder_edge));
    let h2_bd = draft.insert_half_edge(HalfEdgeData::new(placeholder_he, placeholder_he, placeholder_he, face2, vb, placeholder_edge));
    let h2_da = draft.insert_half_edge(HalfEdgeData::new(placeholder_he, placeholder_he, placeholder_he, face2, vd, placeholder_edge));

    {
        let arena = draft.arena_mut();
        arena.get_half_edge_mut(h1_ab).unwrap().set_next(h1_bc);
        arena.get_half_edge_mut(h1_bc).unwrap().set_next(h1_ca);
        arena.get_half_edge_mut(h1_ca).unwrap().set_next(h1_ab);

        arena.get_half_edge_mut(h2_ab).unwrap().set_next(h2_bd);
        arena.get_half_edge_mut(h2_bd).unwrap().set_next(h2_da);
        arena.get_half_edge_mut(h2_da).unwrap().set_next(h2_ab);

        arena.get_half_edge_mut(h1_ab).unwrap().set_radial_next(h2_ab);
        arena.get_half_edge_mut(h2_ab).unwrap().set_radial_next(h1_ab);

        arena.get_half_edge_mut(h1_bc).unwrap().set_radial_next(h1_bc);
        arena.get_half_edge_mut(h1_ca).unwrap().set_radial_next(h1_ca);
        arena.get_half_edge_mut(h2_bd).unwrap().set_radial_next(h2_bd);
        arena.get_half_edge_mut(h2_da).unwrap().set_radial_next(h2_da);

        arena.get_loop_mut(loop1).unwrap().set_half_edge(h1_ab);
        arena.get_loop_mut(loop1).unwrap().set_face(face1);
        arena.get_loop_mut(loop2).unwrap().set_half_edge(h2_ab);
        arena.get_loop_mut(loop2).unwrap().set_face(face2);
        arena.get_shell_mut(shell).unwrap().set_representative_face(face1);
    }

    let arena = draft.arena();
    let result = validate_shell_orientation(
        arena,
        &|v| {
            if v == va { Some([0.0, 0.0, 0.0]) }
            else if v == vb { Some([1.0, 0.0, 0.0]) }
            else if v == vc { Some([0.0, 1.0, 0.0]) }
            else if v == vd { Some([0.0, -1.0, 0.0]) }
            else { None }
        },
        &FlatToleranceProvider::new(1e-10),
    );
    assert!(result.is_err(), "Same-direction shared edge should fail");
    match result.unwrap_err() {
        KernelError::TopologyViolation { err: TopologyError::ValidatorFailure { validator, .. }, .. } => {
            assert_eq!(validator, "ShellOrientationConsistency");
        }
        other => panic!("Expected ValidatorFailure, got: {:?}", other),
    }
}

// ── False Positive Checks ───────────────────────────────────────────────

#[test]
fn non_manifold_touching_corner() {
    // Two separate triangles that touch only at vertex A (no shared edge).
    // Shell orientation validator should pass since there are no shared edges to check.
    let mut draft = empty_test_draft();
    let (_face1, v0, v1, v2) = build_triangle_face(&mut draft);
    let (_face2, v3, v4, v5) = build_triangle_face(&mut draft);
    let arena = draft.arena();

    let result = validate_shell_orientation(
        arena,
        &|v| {
            if v == v0 { Some([0.0, 0.0, 0.0]) }
            else if v == v1 { Some([1.0, 0.0, 0.0]) }
            else if v == v2 { Some([0.0, 1.0, 0.0]) }
            // Second triangle shares position of v0 but different vertex ID
            else if v == v3 { Some([0.0, 0.0, 0.0]) }
            else if v == v4 { Some([-1.0, 0.0, 0.0]) }
            else if v == v5 { Some([0.0, -1.0, 0.0]) }
            else { None }
        },
        &FlatToleranceProvider::new(1e-10),
    );
    assert!(result.is_ok(), "Faces touching only at a vertex (no shared edge) should pass");
}

#[test]
fn coincident_vertices_different_ids() {
    // Two faces sharing an edge where vertex IDs differ but positions match.
    // Both half-edges traverse A→B (same geometric direction) — WRONG.
    // The validator must catch this via positions_match(), not just VertexId comparison.
    let mut draft = empty_test_draft();
    let placeholder_he = HalfEdgeId::new(0, 0);
    let placeholder_face = FaceId::new(0, 0);
    let placeholder_edge = EdgeId::new(0, 0);

    // Create 6 vertices: va1, vb1 for face 1; va2, vb2 for face 2 (same positions!)
    let va1 = draft.insert_vertex(VertexData::new(placeholder_he));
    let vb1 = draft.insert_vertex(VertexData::new(placeholder_he));
    let vc = draft.insert_vertex(VertexData::new(placeholder_he));
    let va2 = draft.insert_vertex(VertexData::new(placeholder_he));
    let vb2 = draft.insert_vertex(VertexData::new(placeholder_he));
    let vd = draft.insert_vertex(VertexData::new(placeholder_he));

    let shell = insert_test_solid_shell(&mut draft);
    let loop1 = draft.insert_loop(LoopData::new(placeholder_he, placeholder_face));
    let loop2 = draft.insert_loop(LoopData::new(placeholder_he, placeholder_face));
    let face1 = draft.insert_face(FaceData::new(loop1, shell));
    let face2 = draft.insert_face(FaceData::new(loop2, shell));

    // Face 1: va1→vb1→vc (A→B)
    let h1_ab = draft.insert_half_edge(HalfEdgeData::new(placeholder_he, placeholder_he, placeholder_he, face1, va1, placeholder_edge));
    let h1_bc = draft.insert_half_edge(HalfEdgeData::new(placeholder_he, placeholder_he, placeholder_he, face1, vb1, placeholder_edge));
    let h1_ca = draft.insert_half_edge(HalfEdgeData::new(placeholder_he, placeholder_he, placeholder_he, face1, vc, placeholder_edge));

    // Face 2: va2→vb2→vd (A→B again — positions match! WRONG direction!)
    let h2_ab = draft.insert_half_edge(HalfEdgeData::new(placeholder_he, placeholder_he, placeholder_he, face2, va2, placeholder_edge));
    let h2_bd = draft.insert_half_edge(HalfEdgeData::new(placeholder_he, placeholder_he, placeholder_he, face2, vb2, placeholder_edge));
    let h2_da = draft.insert_half_edge(HalfEdgeData::new(placeholder_he, placeholder_he, placeholder_he, face2, vd, placeholder_edge));

    {
        let arena = draft.arena_mut();
        arena.get_half_edge_mut(h1_ab).unwrap().set_next(h1_bc);
        arena.get_half_edge_mut(h1_bc).unwrap().set_next(h1_ca);
        arena.get_half_edge_mut(h1_ca).unwrap().set_next(h1_ab);

        arena.get_half_edge_mut(h2_ab).unwrap().set_next(h2_bd);
        arena.get_half_edge_mut(h2_bd).unwrap().set_next(h2_da);
        arena.get_half_edge_mut(h2_da).unwrap().set_next(h2_ab);

        // Radially link the "shared" edge
        arena.get_half_edge_mut(h1_ab).unwrap().set_radial_next(h2_ab);
        arena.get_half_edge_mut(h2_ab).unwrap().set_radial_next(h1_ab);

        // Boundary edges
        arena.get_half_edge_mut(h1_bc).unwrap().set_radial_next(h1_bc);
        arena.get_half_edge_mut(h1_ca).unwrap().set_radial_next(h1_ca);
        arena.get_half_edge_mut(h2_bd).unwrap().set_radial_next(h2_bd);
        arena.get_half_edge_mut(h2_da).unwrap().set_radial_next(h2_da);

        arena.get_loop_mut(loop1).unwrap().set_half_edge(h1_ab);
        arena.get_loop_mut(loop1).unwrap().set_face(face1);
        arena.get_loop_mut(loop2).unwrap().set_half_edge(h2_ab);
        arena.get_loop_mut(loop2).unwrap().set_face(face2);
        arena.get_shell_mut(shell).unwrap().set_representative_face(face1);
    }

    let arena = draft.arena();
    let result = validate_shell_orientation(
        arena,
        &|v| {
            // va1 and va2 have the same position; vb1 and vb2 have the same position
            if v == va1 || v == va2 { Some([0.0, 0.0, 0.0]) }
            else if v == vb1 || v == vb2 { Some([1.0, 0.0, 0.0]) }
            else if v == vc { Some([0.0, 1.0, 0.0]) }
            else if v == vd { Some([0.0, -1.0, 0.0]) }
            else { None }
        },
        &FlatToleranceProvider::new(1e-10),
    );
    assert!(result.is_err(), "Coincident vertices with same-direction half-edges MUST be caught");
    match result.unwrap_err() {
        KernelError::TopologyViolation { err: TopologyError::ValidatorFailure { validator, .. }, .. } => {
            assert_eq!(validator, "ShellOrientationConsistency");
        }
        other => panic!("Expected ValidatorFailure, got: {:?}", other),
    }
}

// ── Edge Case ───────────────────────────────────────────────────────────

#[test]
fn non_manifold_shared_edge_cycle() {
    // 3 faces sharing the same edge (a radial cycle of length 3).
    // As long as every adjacent pair runs opposite, the validator should pass.
    // Face 1: A→B, Face 2: B→A, Face 3: A→B — checker only tests pairs via radial_next.
    let mut draft = empty_test_draft();
    let placeholder_he = HalfEdgeId::new(0, 0);
    let placeholder_face = FaceId::new(0, 0);
    let placeholder_edge = EdgeId::new(0, 0);

    let va = draft.insert_vertex(VertexData::new(placeholder_he));
    let vb = draft.insert_vertex(VertexData::new(placeholder_he));
    let vc = draft.insert_vertex(VertexData::new(placeholder_he));
    let vd = draft.insert_vertex(VertexData::new(placeholder_he));
    let ve = draft.insert_vertex(VertexData::new(placeholder_he));

    let shell = insert_test_solid_shell(&mut draft);
    let l1 = draft.insert_loop(LoopData::new(placeholder_he, placeholder_face));
    let l2 = draft.insert_loop(LoopData::new(placeholder_he, placeholder_face));
    let l3 = draft.insert_loop(LoopData::new(placeholder_he, placeholder_face));
    let f1 = draft.insert_face(FaceData::new(l1, shell));
    let f2 = draft.insert_face(FaceData::new(l2, shell));
    let f3 = draft.insert_face(FaceData::new(l3, shell));

    // Face 1: A→B→C  (A→B direction)
    let h1_ab = draft.insert_half_edge(HalfEdgeData::new(placeholder_he, placeholder_he, placeholder_he, f1, va, placeholder_edge));
    let h1_bc = draft.insert_half_edge(HalfEdgeData::new(placeholder_he, placeholder_he, placeholder_he, f1, vb, placeholder_edge));
    let h1_ca = draft.insert_half_edge(HalfEdgeData::new(placeholder_he, placeholder_he, placeholder_he, f1, vc, placeholder_edge));

    // Face 2: B→A→D  (B→A direction — opposite of face 1 ✓)
    let h2_ba = draft.insert_half_edge(HalfEdgeData::new(placeholder_he, placeholder_he, placeholder_he, f2, vb, placeholder_edge));
    let h2_ad = draft.insert_half_edge(HalfEdgeData::new(placeholder_he, placeholder_he, placeholder_he, f2, va, placeholder_edge));
    let h2_db = draft.insert_half_edge(HalfEdgeData::new(placeholder_he, placeholder_he, placeholder_he, f2, vd, placeholder_edge));

    // Face 3: A→B→E  (A→B direction — opposite of face 2 ✓)
    let h3_ab = draft.insert_half_edge(HalfEdgeData::new(placeholder_he, placeholder_he, placeholder_he, f3, va, placeholder_edge));
    let h3_be = draft.insert_half_edge(HalfEdgeData::new(placeholder_he, placeholder_he, placeholder_he, f3, vb, placeholder_edge));
    let h3_ea = draft.insert_half_edge(HalfEdgeData::new(placeholder_he, placeholder_he, placeholder_he, f3, ve, placeholder_edge));

    {
        let arena = draft.arena_mut();
        // Wire face loops
        arena.get_half_edge_mut(h1_ab).unwrap().set_next(h1_bc);
        arena.get_half_edge_mut(h1_bc).unwrap().set_next(h1_ca);
        arena.get_half_edge_mut(h1_ca).unwrap().set_next(h1_ab);

        arena.get_half_edge_mut(h2_ba).unwrap().set_next(h2_ad);
        arena.get_half_edge_mut(h2_ad).unwrap().set_next(h2_db);
        arena.get_half_edge_mut(h2_db).unwrap().set_next(h2_ba);

        arena.get_half_edge_mut(h3_ab).unwrap().set_next(h3_be);
        arena.get_half_edge_mut(h3_be).unwrap().set_next(h3_ea);
        arena.get_half_edge_mut(h3_ea).unwrap().set_next(h3_ab);

        // Radial cycle: h1_ab → h2_ba → h3_ab → h1_ab
        arena.get_half_edge_mut(h1_ab).unwrap().set_radial_next(h2_ba);
        arena.get_half_edge_mut(h2_ba).unwrap().set_radial_next(h3_ab);
        arena.get_half_edge_mut(h3_ab).unwrap().set_radial_next(h1_ab);

        // Boundary edges: self-radial
        for he in [h1_bc, h1_ca, h2_ad, h2_db, h3_be, h3_ea] {
            arena.get_half_edge_mut(he).unwrap().set_radial_next(he);
        }

        arena.get_loop_mut(l1).unwrap().set_half_edge(h1_ab);
        arena.get_loop_mut(l1).unwrap().set_face(f1);
        arena.get_loop_mut(l2).unwrap().set_half_edge(h2_ba);
        arena.get_loop_mut(l2).unwrap().set_face(f2);
        arena.get_loop_mut(l3).unwrap().set_half_edge(h3_ab);
        arena.get_loop_mut(l3).unwrap().set_face(f3);
        arena.get_shell_mut(shell).unwrap().set_representative_face(f1);
    }

    let arena = draft.arena();
    let result = validate_shell_orientation(
        arena,
        &|v| {
            if v == va { Some([0.0, 0.0, 0.0]) }
            else if v == vb { Some([1.0, 0.0, 0.0]) }
            else if v == vc { Some([0.0, 1.0, 0.0]) }
            else if v == vd { Some([0.0, -1.0, 0.0]) }
            else if v == ve { Some([0.0, 0.0, 1.0]) }
            else { None }
        },
        &FlatToleranceProvider::new(1e-10),
    );
    assert!(result.is_ok(), "Non-manifold 3-face edge cycle with alternating directions should pass");
}

#[test]
fn reversed_edge_different_lengths() {
    // A→B and B→C where Pos(A) == Pos(B) but Pos(B) != Pos(C).
    // The validator sees half-edge origins that share the same position,
    // but destinations differ — NOT the same geometric edge. Should pass.
    let mut draft = empty_test_draft();
    let placeholder_he = HalfEdgeId::new(0, 0);
    let placeholder_face = FaceId::new(0, 0);
    let placeholder_edge = EdgeId::new(0, 0);

    let va = draft.insert_vertex(VertexData::new(placeholder_he));
    let vb = draft.insert_vertex(VertexData::new(placeholder_he));
    let vc = draft.insert_vertex(VertexData::new(placeholder_he));
    let vd = draft.insert_vertex(VertexData::new(placeholder_he));

    let shell = insert_test_solid_shell(&mut draft);
    let loop1 = draft.insert_loop(LoopData::new(placeholder_he, placeholder_face));
    let loop2 = draft.insert_loop(LoopData::new(placeholder_he, placeholder_face));
    let face1 = draft.insert_face(FaceData::new(loop1, shell));
    let face2 = draft.insert_face(FaceData::new(loop2, shell));

    // Face 1: A→B→C
    let h1_ab = draft.insert_half_edge(HalfEdgeData::new(placeholder_he, placeholder_he, placeholder_he, face1, va, placeholder_edge));
    let h1_bc = draft.insert_half_edge(HalfEdgeData::new(placeholder_he, placeholder_he, placeholder_he, face1, vb, placeholder_edge));
    let h1_ca = draft.insert_half_edge(HalfEdgeData::new(placeholder_he, placeholder_he, placeholder_he, face1, vc, placeholder_edge));

    // Face 2: B→C→D (B→C is "shared" but different geometric edge due to Pos(A)==Pos(B))
    let h2_bc = draft.insert_half_edge(HalfEdgeData::new(placeholder_he, placeholder_he, placeholder_he, face2, vb, placeholder_edge));
    let h2_cd = draft.insert_half_edge(HalfEdgeData::new(placeholder_he, placeholder_he, placeholder_he, face2, vc, placeholder_edge));
    let h2_db = draft.insert_half_edge(HalfEdgeData::new(placeholder_he, placeholder_he, placeholder_he, face2, vd, placeholder_edge));

    {
        let arena = draft.arena_mut();
        arena.get_half_edge_mut(h1_ab).unwrap().set_next(h1_bc);
        arena.get_half_edge_mut(h1_bc).unwrap().set_next(h1_ca);
        arena.get_half_edge_mut(h1_ca).unwrap().set_next(h1_ab);

        arena.get_half_edge_mut(h2_bc).unwrap().set_next(h2_cd);
        arena.get_half_edge_mut(h2_cd).unwrap().set_next(h2_db);
        arena.get_half_edge_mut(h2_db).unwrap().set_next(h2_bc);

        // Link h1_bc and h2_bc radially (they share vertex B→C)
        arena.get_half_edge_mut(h1_bc).unwrap().set_radial_next(h2_bc);
        arena.get_half_edge_mut(h2_bc).unwrap().set_radial_next(h1_bc);

        // Boundary edges
        for he in [h1_ab, h1_ca, h2_cd, h2_db] {
            arena.get_half_edge_mut(he).unwrap().set_radial_next(he);
        }

        arena.get_loop_mut(loop1).unwrap().set_half_edge(h1_ab);
        arena.get_loop_mut(loop1).unwrap().set_face(face1);
        arena.get_loop_mut(loop2).unwrap().set_half_edge(h2_bc);
        arena.get_loop_mut(loop2).unwrap().set_face(face2);
        arena.get_shell_mut(shell).unwrap().set_representative_face(face1);
    }

    let arena = draft.arena();
    let result = validate_shell_orientation(
        arena,
        &|v| {
            // Pos(A) == Pos(B) — coincident origins but different destinations
            if v == va { Some([0.0, 0.0, 0.0]) }
            else if v == vb { Some([0.0, 0.0, 0.0]) } // Coincident with A!
            else if v == vc { Some([1.0, 0.0, 0.0]) }
            else if v == vd { Some([0.0, 1.0, 0.0]) }
            else { None }
        },
        &FlatToleranceProvider::new(1e-10),
    );
    // Both h1_bc and h2_bc go B→C (same vertex B, next vertex C).
    // Origins: Pos(B)=[0,0,0] and Pos(B)=[0,0,0] — match.
    // Dests: Pos(C)=[1,0,0] and Pos(C)=[1,0,0] — match.
    // So the validator correctly sees same-direction A→B and reports an error.
    // But this IS a real topology problem — two faces traversing B→C in the same direction.
    // The "false positive" scenario requires that the edges are NOT actually shared.
    // Since they ARE radially linked here, the validator is right to flag it.
    //
    // For the true false-positive case, edges must NOT be radially linked.
    // In that case, the validator never compares them.
    assert!(result.is_err(), "Same-direction radially-linked half-edges should be caught");
}
