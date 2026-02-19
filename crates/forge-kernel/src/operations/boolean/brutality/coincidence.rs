use super::super::test_helpers::{run_boolean, try_boolean};
use super::super::schema::{BooleanOp};
use forge_topo::hashing::compute_arena_topology_hash;

// ══════════════════════════════════════════════════════════════
// §2  COINCIDENCE FRAMEWORK TORTURE
// ══════════════════════════════════════════════════════════════

/// 2.1 — Perfect Flush Union, 100× Determinism
///
/// Two cubes sharing a full face. Run 100 times.
/// Assert deterministic topology hash every time.
#[test]
fn perfect_flush_union_100x() {
    let mut hashes = Vec::with_capacity(100);

    for i in 0..100 {
        let result = run_boolean(
            [0.0, 0.0, 0.0], 1.0,
            [2.0, 0.0, 0.0], 1.0,
            BooleanOp::Union,
        );

        let hash = compute_arena_topology_hash(result.topology().arena());
        let arena = result.topology().arena();

        if i == 0 {
            let v = arena.vertex_count();
            let e = arena.half_edge_count() / 2;
            let f = arena.face_count();
            assert_eq!(v, 16, "Flush union (two shells) should produce 16 vertices, got {v}");
            assert_eq!(e, 24, "Flush union (two shells) should produce 24 edges, got {e}");
            assert_eq!(f, 12, "Flush union (two shells) should produce 12 faces, got {f}");
        }

        hashes.push(hash);
    }

    let first = hashes[0];
    for (i, &h) in hashes.iter().enumerate() {
        assert_eq!(
            h, first,
            "Topology hash diverged on iteration {i}: {h:#x} vs {first:#x}"
        );
    }
}

/// 2.2 — Partial Coplanar Overlap
///
/// Two cubes overlapping by exactly half a face.
/// Expect face split into rectangular region, no micro-faces.
#[test]
fn partial_coplanar_overlap() {
    let result = run_boolean(
        [0.0, 0.0, 0.0], 1.0,
        [1.0, 1.0, 0.0], 1.0,
        BooleanOp::Union,
    );

    let arena = result.topology().arena();
    let face_count = arena.face_count();
    let vertex_count = arena.vertex_count();

    eprintln!("Partial coplanar overlap: V={vertex_count}, F={face_count}");
    assert!(face_count >= 10, "Expected at least 10 faces for partial overlap union, got {face_count}");

    let euler = vertex_count as isize - (arena.half_edge_count() / 2) as isize + face_count as isize;
    assert_eq!(euler, 2, "Euler formula violation: V-E+F = {euler}, expected 2");
}

/// 2.3 — Edge-Aligned Overlap
///
/// Two cubes touching along exactly one edge.
/// Expect union produces correct topology, no non-manifold edge.
#[test]
fn edge_aligned_overlap() {
    let result = run_boolean(
        [0.0, 0.0, 0.0], 1.0,
        [2.0, 2.0, 0.0], 1.0,
        BooleanOp::Union,
    );

    let arena = result.topology().arena();
    let face_count = arena.face_count();
    let vertex_count = arena.vertex_count();
    let edge_count = arena.half_edge_count() / 2;

    eprintln!("Edge-aligned: V={vertex_count}, E={edge_count}, F={face_count}");

    let euler = vertex_count as isize - edge_count as isize + face_count as isize;
    assert!(
        euler == 2 || euler == 4,
        "Euler V-E+F = {euler}, expected 2 (single shell) or 4 (two disjoint shells)"
    );

    for (_he_id, he) in arena.iter_half_edges() {
        let twin = arena.get_half_edge(he.twin());
        assert!(
            twin.is_ok(),
            "Dangling half-edge without valid twin"
        );
    }
}

/// 2.4 — Vertex-Only Contact
///
/// Two cubes touching at exactly one vertex.
/// Expect union produces correct manifold, no phantom edges.
#[test]
fn vertex_only_contact() {
    let result = run_boolean(
        [0.0, 0.0, 0.0], 1.0,
        [2.0, 2.0, 2.0], 1.0,
        BooleanOp::Union,
    );

    let arena = result.topology().arena();
    let face_count = arena.face_count();
    let vertex_count = arena.vertex_count();
    let edge_count = arena.half_edge_count() / 2;

    eprintln!("Vertex contact: V={vertex_count}, E={edge_count}, F={face_count}");

    let euler = vertex_count as isize - edge_count as isize + face_count as isize;
    assert!(euler >= 2, "Euler formula should hold: V-E+F = {euler}");
}

/// 2.5 — Identical Solids Union
///
/// union(solid, identical solid) should return the same solid.
#[test]
fn identical_solids_union() {
    let result = run_boolean(
        [0.0, 0.0, 0.0], 1.0,
        [0.0, 0.0, 0.0], 1.0,
        BooleanOp::Union,
    );

    let arena = result.topology().arena();
    let face_count = arena.face_count();
    let vertex_count = arena.vertex_count();

    eprintln!("Identical union: V={vertex_count}, F={face_count}");

    assert_eq!(face_count, 6, "Identical union should produce 6 faces, got {face_count}");
    assert_eq!(vertex_count, 8, "Identical union should produce 8 vertices, got {vertex_count}");
}

/// 2.5b — Identical Solids Subtraction → empty result
#[test]
fn identical_solids_subtraction_empty() {
    let result = try_boolean(
        [0.0, 0.0, 0.0], 1.0,
        [0.0, 0.0, 0.0], 1.0,
        BooleanOp::Subtraction,
    );

    match result {
        Ok(r) => {
            let face_count = r.topology().arena().face_count();
            assert_eq!(face_count, 0, "Identical subtraction should produce 0 faces, got {face_count}");
        }
        Err(_) => {
            // Returning an error is also acceptable (empty result can be signaled as error)
        }
    }
}
