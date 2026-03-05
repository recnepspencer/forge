//! Adversarial tests for JoinFacesNmt — NMT-compatible face merge operator.
//!
//! These tests are designed to be HARD and to surface bugs and edge cases,
//! not just verify the happy path. Each test targets a specific invariant from
//! REGION_MERGE_SPEC.md §5.8.

use crate::b_rep::{EdgeData, FaceData, HalfEdgeData, LoopData, VertexData};
use crate::boundary_editing::join_faces_nmt::JoinFacesNmt;
use crate::handles::HalfEdgeId;
use crate::transactions::TopologyState;
use forge_core::KernelError;
use crate::b_rep::ShellKind;

fn ph() -> HalfEdgeId {
    HalfEdgeId::DANGLING
}

fn placeholder_loop() -> crate::handles::LoopId {
    crate::handles::LoopId::DANGLING
}

fn placeholder_shell() -> crate::handles::ShellId {
    crate::handles::ShellId::DANGLING
}

/// Build a valence-N radial ring on a single shared edge.
/// Each face is a 2-gon (lune) between v1 and v2.
/// Returns the N forward halfedges in ring order.
fn setup_valence_n_edge(draft: &mut crate::transactions::MutableDraft, n: usize) -> Vec<HalfEdgeId> {
    let v1 = draft.insert_vertex(VertexData::new(ph()));
    let v2 = draft.insert_vertex(VertexData::new(ph()));
    let shared_edge = draft.insert_edge(EdgeData::new(ph()));

    let mut fwd_hes = Vec::with_capacity(n);

    for i in 0..n {
        let f = draft.insert_face(FaceData::new(placeholder_loop(), placeholder_shell()));
        let ret_edge = draft.insert_edge(EdgeData::new(ph()));

        let h_fwd = draft.insert_half_edge(HalfEdgeData::new(ph(), ph(), ph(), f, v1, shared_edge));
        let h_ret = draft.insert_half_edge(HalfEdgeData::new(h_fwd, h_fwd, h_fwd, f, v2, ret_edge));

        draft
            .arena_mut()
            .get_half_edge_mut(h_fwd)
            .unwrap()
            .set_next(h_ret);
        draft
            .arena_mut()
            .get_half_edge_mut(h_fwd)
            .unwrap()
            .set_prev(h_ret);
        draft
            .arena_mut()
            .get_half_edge_mut(h_ret)
            .unwrap()
            .set_next(h_fwd);
        draft
            .arena_mut()
            .get_half_edge_mut(h_ret)
            .unwrap()
            .set_prev(h_fwd);
        draft
            .arena_mut()
            .get_half_edge_mut(h_ret)
            .unwrap()
            .set_radial_next(h_ret);
        draft
            .arena_mut()
            .get_edge_mut(ret_edge)
            .unwrap()
            .set_half_edge(h_ret);

        let l = draft.insert_loop(LoopData::new(h_fwd, f));
        draft.arena_mut().get_face_mut(f).unwrap().set_outer_loop(l);

        if i == 0 {
            draft
                .arena_mut()
                .get_vertex_mut(v1)
                .unwrap()
                .set_primary_disk(h_fwd);
            draft
                .arena_mut()
                .get_vertex_mut(v2)
                .unwrap()
                .set_primary_disk(h_ret);
            draft
                .arena_mut()
                .get_edge_mut(shared_edge)
                .unwrap()
                .set_half_edge(h_fwd);
        }

        fwd_hes.push(h_fwd);
    }

    for i in 0..n {
        let next = (i + 1) % n;
        draft
            .arena_mut()
            .get_half_edge_mut(fwd_hes[i])
            .unwrap()
            .set_radial_next(fwd_hes[next]);
    }

    fwd_hes
}

// ===========================================================================
// Rejection tests
// ===========================================================================

/// The operator MUST reject valence == 2 (manifold edge).
/// Standard JoinFaces should be used instead.
#[test]
fn rejects_manifold_edge() {
    let state = TopologyState::empty();
    let mut draft = state.into_mutation();
    let hes = setup_valence_n_edge(&mut draft, 2);

    let err = draft.execute(
        JoinFacesNmt {
            he_survive: hes[0],
            he_kill: hes[1],
        },
    )
    .unwrap_err();
    assert!(
        err.to_string().contains("must be > 2"),
        "Expected valence rejection, got: {err}",
    );
}

/// The operator MUST reject when both halfedges belong to the same face.
/// This would create a self-slit which is topologically meaningless.
#[test]
fn rejects_same_face() {
    let state = TopologyState::empty();
    let mut draft = state.into_mutation();
    let hes = setup_valence_n_edge(&mut draft, 4);

    let err = draft.execute(
        JoinFacesNmt {
            he_survive: hes[0],
            he_kill: hes[0],
        },
    )
    .unwrap_err();
    assert!(
        err.to_string().contains("same face"),
        "Expected same-face rejection, got: {err}",
    );
}

/// The operator MUST reject when halfedges don't share the same EdgeId.
/// This catches accidentally passing halfedges from different geometric edges.
#[test]
fn rejects_different_edge_ids() {
    let state = TopologyState::empty();
    let mut draft = state.into_mutation();

    // Build TWO separate valence-3 edges. Take one halfedge from each.
    // They are on different faces AND different edges.
    let v1 = draft.insert_vertex(VertexData::new(ph()));
    let v2 = draft.insert_vertex(VertexData::new(ph()));
    let v3 = draft.insert_vertex(VertexData::new(ph()));

    let edge_a = draft.insert_edge(EdgeData::new(ph()));
    let edge_b = draft.insert_edge(EdgeData::new(ph()));

    let f1 = draft.insert_face(FaceData::new(placeholder_loop(), placeholder_shell()));
    let f2 = draft.insert_face(FaceData::new(placeholder_loop(), placeholder_shell()));

    // he_a: v1→v2 on edge_a, face f1
    let he_a = draft.insert_half_edge(HalfEdgeData::new(ph(), ph(), ph(), f1, v1, edge_a));
    // he_b: v2→v3 on edge_b, face f2
    let he_b = draft.insert_half_edge(HalfEdgeData::new(ph(), ph(), ph(), f2, v2, edge_b));

    // Wire minimal self-loops for validity.
    draft
        .arena_mut()
        .get_half_edge_mut(he_a)
        .unwrap()
        .set_next(he_a);
    draft
        .arena_mut()
        .get_half_edge_mut(he_a)
        .unwrap()
        .set_prev(he_a);
    draft
        .arena_mut()
        .get_half_edge_mut(he_a)
        .unwrap()
        .set_radial_next(he_a);
    draft
        .arena_mut()
        .get_half_edge_mut(he_b)
        .unwrap()
        .set_next(he_b);
    draft
        .arena_mut()
        .get_half_edge_mut(he_b)
        .unwrap()
        .set_prev(he_b);
    draft
        .arena_mut()
        .get_half_edge_mut(he_b)
        .unwrap()
        .set_radial_next(he_b);

    let err = draft.execute(
        JoinFacesNmt {
            he_survive: he_a,
            he_kill: he_b,
        },
    )
    .unwrap_err();
    assert!(
        err.to_string().contains("same geometric edge"),
        "Expected edge-mismatch rejection, got: {err}",
    );
}

// ===========================================================================
// Valence-3 (minimal NMT) — the hardest single-step case
// ===========================================================================

/// Valence-3 is the minimal NMT case. After merge:
/// - Protected ring shrinks to exactly 1 element (self-loop radial).
/// - Slit pair is a 2-element ring.
/// - EdgeData.half_edge must point to the lone protected halfedge.
#[test]
fn valence_3_protected_ring_becomes_single_element() {
    let state = TopologyState::empty();
    let mut draft = state.into_mutation();
    let hes = setup_valence_n_edge(&mut draft, 3);

    let he_s = hes[0];
    let he_k = hes[1];
    let he_protected = hes[2];

    draft.execute(
        JoinFacesNmt {
            he_survive: he_s,
            he_kill: he_k,
        },
    )
    .unwrap();

    // Protected ring is now a single-element self-loop.
    assert_eq!(
        draft
            .arena()
            .get_half_edge(he_protected)
            .unwrap()
            .radial_next(),
        he_protected,
        "Valence-3: sole protected halfedge must radial-self-loop",
    );

    // Slit pair is intact.
    assert_eq!(
        draft.arena().get_half_edge(he_s).unwrap().radial_next(),
        he_k,
    );
    assert_eq!(
        draft.arena().get_half_edge(he_k).unwrap().radial_next(),
        he_s,
    );

    // EdgeData.half_edge must point to the protected halfedge, NOT the slit.
    let shared_edge = draft.arena().get_half_edge(he_s).unwrap().edge();
    let edge_entry = draft.arena().get_edge(shared_edge).unwrap().half_edge();
    assert_eq!(
        edge_entry, he_protected,
        "EdgeData.half_edge must point to protected ring, not slit",
    );
}

// ===========================================================================
// Slit structural invariants (§5.8 postconditions 1-3)
// ===========================================================================

/// The slit must form a 2-element closed inner loop that is properly
/// registered on the surviving face.
#[test]
fn slit_forms_registered_inner_loop() {
    let state = TopologyState::empty();
    let mut draft = state.into_mutation();
    let hes = setup_valence_n_edge(&mut draft, 4);

    let he_s = hes[1];
    let he_k = hes[2];

    let out = draft.execute(
        JoinFacesNmt {
            he_survive: he_s,
            he_kill: he_k,
        },
    )
    .unwrap()
    .into_value();

    // Slit forms a closed 2-element loop.
    let s_data = draft.arena().get_half_edge(he_s).unwrap();
    let k_data = draft.arena().get_half_edge(he_k).unwrap();

    assert_eq!(s_data.next(), he_k);
    assert_eq!(k_data.next(), he_s);
    assert_eq!(s_data.prev(), he_k);
    assert_eq!(k_data.prev(), he_s);

    // Both halfedges belong to the surviving face.
    assert_eq!(s_data.face(), out.surviving_face);
    assert_eq!(k_data.face(), out.surviving_face);

    // The slit inner loop is registered.
    let inner_loops = draft
        .arena()
        .get_face(out.surviving_face)
        .unwrap()
        .inner_loops();
    assert!(
        !inner_loops.is_empty(),
        "Slit inner loop must be registered on surviving face",
    );

    // Walk the inner loop from its registered seed and verify it has exactly 2 elements.
    let slit_loop_id = inner_loops[inner_loops.len() - 1]; // last added
    let seed = draft.arena().get_loop(slit_loop_id).unwrap().half_edge();
    let mut cur = seed;
    let mut count = 0;
    loop {
        count += 1;
        cur = draft.arena().get_half_edge(cur).unwrap().next();
        if cur == seed {
            break;
        }
        assert!(count < 10, "Slit inner loop is not properly closed");
    }
    assert_eq!(count, 2, "Slit inner loop must have exactly 2 halfedges");
}

/// §5.8(3): Protected ring must preserve cyclic adjacency.
/// Test with valence-4: merge non-adjacent pair to verify ring ordering.
#[test]
fn protected_ring_preserves_cyclic_order() {
    let state = TopologyState::empty();
    let mut draft = state.into_mutation();
    let hes = setup_valence_n_edge(&mut draft, 4);

    // Ring order: 0→1→2→3→0. Merge 1 and 2.
    // Protected ring should become 0→3→0 (NOT 3→0→3).
    draft.execute(
        JoinFacesNmt {
            he_survive: hes[1],
            he_kill: hes[2],
        },
    )
    .unwrap();

    assert_eq!(
        draft.arena().get_half_edge(hes[0]).unwrap().radial_next(),
        hes[3],
        "Protected: 0 -> 3",
    );
    assert_eq!(
        draft.arena().get_half_edge(hes[3]).unwrap().radial_next(),
        hes[0],
        "Protected: 3 -> 0",
    );
}

/// Merge the FIRST and LAST elements in the ring (wrap-around).
/// This tests that the protected ring surgery handles the cyclic boundary.
#[test]
fn merge_wraps_around_ring_boundary() {
    let state = TopologyState::empty();
    let mut draft = state.into_mutation();
    let hes = setup_valence_n_edge(&mut draft, 4);

    // Ring: 0→1→2→3→0. Merge 0 and 3 (wrap-around).
    draft.execute(
        JoinFacesNmt {
            he_survive: hes[0],
            he_kill: hes[3],
        },
    )
    .unwrap();

    // Protected ring: 1→2→1
    assert_eq!(
        draft.arena().get_half_edge(hes[1]).unwrap().radial_next(),
        hes[2],
    );
    assert_eq!(
        draft.arena().get_half_edge(hes[2]).unwrap().radial_next(),
        hes[1],
    );
}

// ===========================================================================
// §5.8(4): No dangling references
// ===========================================================================

/// After merge, killed face and its outer loop must be completely gone.
/// No halfedge in the arena should point to the killed FaceId.
#[test]
fn no_dangling_face_references() {
    let state = TopologyState::empty();
    let mut draft = state.into_mutation();
    let hes = setup_valence_n_edge(&mut draft, 3);

    let killed_face = draft.arena().get_half_edge(hes[1]).unwrap().face();
    let killed_loop = draft.arena().get_face(killed_face).unwrap().outer_loop();

    draft.execute(
        JoinFacesNmt {
            he_survive: hes[0],
            he_kill: hes[1],
        },
    )
    .unwrap();

    // Arena-level removal.
    assert!(draft.arena().get_face(killed_face).is_err());
    assert!(draft.arena().get_loop(killed_loop).is_err());

    // No live halfedge points to the killed face.
    for (he_id, he_data) in draft.arena().iter_half_edges() {
        assert_ne!(
            he_data.face(),
            killed_face,
            "Halfedge {} still points to killed face {}",
            he_id.index(),
            killed_face.index(),
        );
    }
}

// ===========================================================================
// EdgeData pointer correctness (bug we found during audit)
// ===========================================================================

/// After merge, EdgeData.half_edge() must NOT point into the slit ring.
/// If it does, any code walking the radial ring from EdgeData (e.g. continuity
/// queries) will only see the 2-element slit, missing all protected uses.
#[test]
fn edge_data_points_to_protected_ring_not_slit() {
    let state = TopologyState::empty();
    let mut draft = state.into_mutation();
    let hes = setup_valence_n_edge(&mut draft, 5);

    let he_s = hes[1];
    let he_k = hes[3];

    draft.execute(
        JoinFacesNmt {
            he_survive: he_s,
            he_kill: he_k,
        },
    )
    .unwrap();

    let shared_edge = draft.arena().get_half_edge(he_s).unwrap().edge();
    let entry = draft.arena().get_edge(shared_edge).unwrap().half_edge();

    // The entry point must NOT be he_s or he_k (they're in the slit).
    assert_ne!(entry, he_s, "EdgeData entry must not be in slit");
    assert_ne!(entry, he_k, "EdgeData entry must not be in slit");

    // Walk from entry via radial_next and verify we see all 3 protected uses.
    let mut count = 0;
    let mut cur = entry;
    loop {
        count += 1;
        cur = draft.arena().get_half_edge(cur).unwrap().radial_next();
        if cur == entry {
            break;
        }
        assert!(count < 20, "Protected ring is not closed");
    }
    assert_eq!(
        count, 3,
        "Protected ring should have 3 elements after valence-5 merge"
    );
}

// ===========================================================================
// Outer loop integrity after merge
// ===========================================================================

/// Walk the surviving face's outer loop and verify:
/// - All halfedges point to the surviving face.
/// - The outer loop is properly closed.
/// - The slit halfedges are NOT in the outer loop.
#[test]
fn outer_loop_excludes_slit_and_is_closed() {
    let state = TopologyState::empty();
    let mut draft = state.into_mutation();
    let hes = setup_valence_n_edge(&mut draft, 4);

    let he_s = hes[1];
    let he_k = hes[2];

    let out = draft.execute(
        JoinFacesNmt {
            he_survive: he_s,
            he_kill: he_k,
        },
    )
    .unwrap()
    .into_value();

    let outer_loop = draft
        .arena()
        .get_face(out.surviving_face)
        .unwrap()
        .outer_loop();
    let seed = draft.arena().get_loop(outer_loop).unwrap().half_edge();
    let mut cur = seed;
    let mut steps = 0;

    loop {
        let data = draft.arena().get_half_edge(cur).unwrap();
        assert_eq!(
            data.face(),
            out.surviving_face,
            "Outer loop halfedge {} belongs to wrong face",
            cur.index(),
        );
        assert_ne!(cur, he_s, "Slit halfedge he_s must NOT be in outer loop");
        assert_ne!(cur, he_k, "Slit halfedge he_k must NOT be in outer loop");

        cur = data.next();
        steps += 1;
        if cur == seed {
            break;
        }
        assert!(steps < 100, "Outer loop is not properly closed");
    }

    // Original: 2 halfedges from FaceS + 2 from FaceK = 4 total,
    // minus the 2 slit halfedges = 2 remaining in outer loop.
    assert_eq!(
        steps, 2,
        "Outer loop should have 2 halfedges for merged 2-gon lunes"
    );
}

// ===========================================================================
// Vertex outgoing pointer stability
// ===========================================================================

/// After merge, vertex outgoing pointers must NOT point to slit halfedges.
/// If they do, vertex-ring traversal would start inside the slit.
///
/// In our 2-gon setup, ALL forward halfedges share v1 as origin. So both
/// he_s.origin() and he_k.origin() are v1. We test that v1's outgoing is
/// fixed. For v2, we set it to a return halfedge and verify it's unaffected
/// (return hes are NOT in the slit — only the shared-edge fwd hes are).
#[test]
fn vertex_outgoing_avoids_slit() {
    let state = TopologyState::empty();
    let mut draft = state.into_mutation();
    let hes = setup_valence_n_edge(&mut draft, 3);

    let he_s = hes[0];
    let he_k = hes[1];

    // Both he_s and he_k have the same origin (v1 in setup_valence_n_edge).
    let v1 = draft.arena().get_half_edge(he_s).unwrap().origin();
    assert_eq!(
        v1,
        draft.arena().get_half_edge(he_k).unwrap().origin(),
        "Test assumption: both slit halfedges share origin vertex",
    );

    // Force v1 outgoing to point to he_s (a slit halfedge).
    draft
        .arena_mut()
        .get_vertex_mut(v1)
        .unwrap()
        .set_primary_disk(he_s);

    draft.execute(
        JoinFacesNmt {
            he_survive: he_s,
            he_kill: he_k,
        },
    )
    .unwrap();

    let v1_out = draft.arena().get_vertex(v1).unwrap().primary_disk();
    assert_ne!(
        v1_out, he_s,
        "Vertex v1 outgoing must not point to slit he_s"
    );
    assert_ne!(
        v1_out, he_k,
        "Vertex v1 outgoing must not point to slit he_k"
    );

    // Verify the replacement has correct origin.
    assert_eq!(
        draft.arena().get_half_edge(v1_out).unwrap().origin(),
        v1,
        "Vertex v1 outgoing must point to a halfedge originating at v1",
    );
}

/// QA regression: when vertex_s == vertex_k (shared origin) and the
/// outgoing pointer is he_k (NOT he_s), the old code skipped the fix
/// entirely because: branch 1 checked `== he_s` (miss), branch 2 had
/// a `vertex_k != vertex_s` guard (skipped). Now both branches check
/// against both slit halfedges.
#[test]
fn vertex_outgoing_shared_origin_he_kill_regression() {
    let state = TopologyState::empty();
    let mut draft = state.into_mutation();
    let hes = setup_valence_n_edge(&mut draft, 3);

    let he_s = hes[0];
    let he_k = hes[1];

    let v1 = draft.arena().get_half_edge(he_s).unwrap().origin();
    assert_eq!(
        v1,
        draft.arena().get_half_edge(he_k).unwrap().origin(),
        "Test precondition: shared origin vertex",
    );

    // Force outgoing to he_k (NOT he_s) — this is the exact QA bug.
    draft
        .arena_mut()
        .get_vertex_mut(v1)
        .unwrap()
        .set_primary_disk(he_k);

    draft.execute(
        JoinFacesNmt {
            he_survive: he_s,
            he_kill: he_k,
        },
    )
    .unwrap();

    let v1_out = draft.arena().get_vertex(v1).unwrap().primary_disk();
    assert_ne!(v1_out, he_s, "Must not point to slit he_s");
    assert_ne!(v1_out, he_k, "Must not point to slit he_k");
    assert_eq!(
        draft.arena().get_half_edge(v1_out).unwrap().origin(),
        v1,
        "Replacement must originate at v1",
    );
}

// ===========================================================================
// Inner loop transfer from killed face
// ===========================================================================

/// If the killed face has inner loops (holes), they must be transferred
/// to the surviving face with correct face assignments on all halfedges.
#[test]
fn inner_loops_transferred_from_killed_face() {
    let state = TopologyState::empty();
    let mut draft = state.into_mutation();
    let hes = setup_valence_n_edge(&mut draft, 3);

    // Add an inner loop to face 1 (the face we will kill).
    let killed_face = draft.arena().get_half_edge(hes[1]).unwrap().face();
    let surviving_face = draft.arena().get_half_edge(hes[0]).unwrap().face();

    // Create a 1-edge inner loop (degenerate but valid for structural test).
    let inner_edge = draft.insert_edge(EdgeData::new(ph()));
    let ih1 = draft.insert_half_edge(HalfEdgeData::new(
        ph(),
        ph(),
        ph(),
        killed_face,
        draft.arena().get_half_edge(hes[1]).unwrap().origin(),
        inner_edge,
    ));
    draft
        .arena_mut()
        .get_half_edge_mut(ih1)
        .unwrap()
        .set_next(ih1);
    draft
        .arena_mut()
        .get_half_edge_mut(ih1)
        .unwrap()
        .set_prev(ih1);
    draft
        .arena_mut()
        .get_half_edge_mut(ih1)
        .unwrap()
        .set_radial_next(ih1);

    let inner_loop = draft.insert_loop(LoopData::new(ih1, killed_face));
    draft
        .arena_mut()
        .get_face_mut(killed_face)
        .unwrap()
        .add_inner_loop(inner_loop);

    // Now merge.
    draft.execute(
        JoinFacesNmt {
            he_survive: hes[0],
            he_kill: hes[1],
        },
    )
    .unwrap();

    // The inner loop's halfedge must now point to the surviving face.
    let ih1_data = draft.arena().get_half_edge(ih1).unwrap();
    assert_eq!(
        ih1_data.face(),
        surviving_face,
        "Transferred inner loop halfedge must point to surviving face",
    );

    // The inner loop entity must belong to the surviving face.
    let loop_data = draft.arena().get_loop(inner_loop).unwrap();
    assert_eq!(
        loop_data.face(),
        surviving_face,
        "Transferred inner loop entity must reference surviving face",
    );

    // The surviving face must list the transferred inner loop.
    let face_data = draft.arena().get_face(surviving_face).unwrap();
    assert!(
        face_data.inner_loops().contains(&inner_loop),
        "Surviving face must contain the transferred inner loop",
    );
}

// ===========================================================================
// Sequential merges — chained operations
// ===========================================================================

/// Perform two sequential JoinFacesNmt operations on the same edge.
/// Starting from valence-5, merge twice → protected ring should be valence-1.
#[test]
fn sequential_merges_reduce_valence_correctly() {
    let state = TopologyState::empty();
    let mut draft = state.into_mutation();
    let hes = setup_valence_n_edge(&mut draft, 5);

    // First merge: hes[0] and hes[1]. Protected ring: [2, 3, 4] → valence 3.
    draft.execute(
        JoinFacesNmt {
            he_survive: hes[0],
            he_kill: hes[1],
        },
    )
    .unwrap();

    // Verify intermediate state: protected ring has 3 elements.
    let mut count = 0;
    let mut cur = hes[2];
    loop {
        count += 1;
        cur = draft.arena().get_half_edge(cur).unwrap().radial_next();
        if cur == hes[2] {
            break;
        }
        assert!(count < 20);
    }
    assert_eq!(
        count, 3,
        "After first merge: protected ring should have 3 elements"
    );

    // Second merge: hes[2] and hes[3]. Protected ring: [4] → valence 1.
    draft.execute(
        JoinFacesNmt {
            he_survive: hes[2],
            he_kill: hes[3],
        },
    )
    .unwrap();

    // Verify final state.
    assert_eq!(
        draft.arena().get_half_edge(hes[4]).unwrap().radial_next(),
        hes[4],
        "After two merges: sole remaining protected halfedge must self-loop",
    );

    // Both slit pairs are intact.
    assert_eq!(
        draft.arena().get_half_edge(hes[0]).unwrap().radial_next(),
        hes[1]
    );
    assert_eq!(
        draft.arena().get_half_edge(hes[2]).unwrap().radial_next(),
        hes[3]
    );
}

// ===========================================================================
// GENUINELY ADVERSARIAL TESTS
// These go beyond smoke tests — they use real validators, non-trivial
// topologies, and prove assertions are load-bearing.
// ===========================================================================

/// Run real structural validators on the post-op arena.
///
/// This is the most important test: if our pointer surgery left ANY
/// inconsistency in radial rings, prev/next, vertex outgoing, or loop
/// integrity, the validator catches it. Manual pointer checks above could
/// miss a pointer we forgot to check — the validator checks ALL of them.
///
/// Uses `Minimal` level (radial rings + prev consistency + vertex continuity
/// + vertex outgoing) because our raw setup lacks hierarchy for Full.
#[test]
fn post_op_passes_structural_validation_minimal() {
    
    use crate::validate::{validate_topology, ValidationLevel};

    let state = TopologyState::empty();
    let mut draft = state.into_mutation();
    let hes = setup_valence_n_edge(&mut draft, 4);

    // Verify pre-op passes validation too (our setup is valid).
    validate_topology(
        draft.arena(),
        ValidationLevel::Minimal,
    )
    .expect("Pre-op topology must be valid (Minimal+NmtIntermediate)");

    draft.execute(
        JoinFacesNmt {
            he_survive: hes[1],
            he_kill: hes[2],
        },
    )
    .unwrap();

    // This exercises: validate_radial_rings, validate_radial_edge_consistency,
    // validate_prev_consistency, validate_vertex_continuity, validate_vertex_outgoing.
    validate_topology(
        draft.arena(),
        ValidationLevel::Minimal,
    )
    .expect("Post-op topology must pass Minimal+NmtIntermediate validation");
}

/// Validate a non-adjacent merge with Minimal-level structural checks.
/// Uses Minimal (not Intermediate) because placeholder shells prevent
/// validate_hierarchy from passing. validate_loops is covered by the
/// separate `post_op_all_loops_have_consistent_face_membership` test.
#[test]
fn post_op_non_adjacent_merge_passes_minimal_validation() {
    
    use crate::validate::{validate_topology, ValidationLevel};

    let state = TopologyState::empty();
    let mut draft = state.into_mutation();
    let hes = setup_valence_n_edge(&mut draft, 5);

    // Merge non-adjacent pair to stress the ring surgery.
    draft.execute(
        JoinFacesNmt {
            he_survive: hes[0],
            he_kill: hes[3],
        },
    )
    .unwrap();

    validate_topology(
        draft.arena(),
        ValidationLevel::Minimal,
    )
    .expect(
        "Post-op topology must pass Minimal+NmtIntermediate validation after non-adjacent merge",
    );
}

/// Targeted: verify EVERY loop in the arena post-op has all its halfedges
/// pointing to the correct face. This is what validate_loops checks, but
/// we do it manually because our placeholder shells prevent Intermediate level.
#[test]
fn post_op_all_loops_have_consistent_face_membership() {
    let state = TopologyState::empty();
    let mut draft = state.into_mutation();
    let hes = setup_valence_n_edge(&mut draft, 4);

    draft.execute(
        JoinFacesNmt {
            he_survive: hes[1],
            he_kill: hes[2],
        },
    )
    .unwrap();

    // Walk every loop in the arena and verify face consistency.
    for (face_id, face_data) in draft.arena().iter_faces() {
        // Walk outer loop.
        let outer = face_data.outer_loop();
        let outer_seed = draft.arena().get_loop(outer).unwrap().half_edge();
        let mut cur = outer_seed;
        let mut steps = 0;
        loop {
            let he_d = draft.arena().get_half_edge(cur).unwrap();
            assert_eq!(
                he_d.face(),
                face_id,
                "Face {} outer loop: halfedge {} belongs to face {} instead",
                face_id.index(),
                cur.index(),
                he_d.face().index(),
            );
            cur = he_d.next();
            steps += 1;
            if cur == outer_seed {
                break;
            }
            assert!(
                steps < 200,
                "Outer loop of face {} is not closed",
                face_id.index()
            );
        }

        // Walk inner loops.
        for &il_id in face_data.inner_loops() {
            let il_seed = draft.arena().get_loop(il_id).unwrap().half_edge();
            let mut cur = il_seed;
            let mut steps = 0;
            loop {
                let he_d = draft.arena().get_half_edge(cur).unwrap();
                assert_eq!(
                    he_d.face(),
                    face_id,
                    "Face {} inner loop {}: halfedge {} belongs to face {} instead",
                    face_id.index(),
                    il_id.index(),
                    cur.index(),
                    he_d.face().index(),
                );
                cur = he_d.next();
                steps += 1;
                if cur == il_seed {
                    break;
                }
                assert!(steps < 200, "Inner loop {} is not closed", il_id.index());
            }
        }
    }
}

/// Build a killed face with a 4-edge boundary (quad, not 2-gon lune).
/// This tests the outer loop merge when the killed face contributes many
/// halfedges to the merged boundary, not just the trivial 1 return edge.
#[test]
fn non_trivial_killed_face_boundary() {
    let state = TopologyState::empty();
    let mut draft = state.into_mutation();

    // Build 3 faces sharing one edge. Face 0 and 2 are 2-gons.
    // Face 1 (the kill target) is a 4-gon (quad) with 4 distinct edges.
    let v1 = draft.insert_vertex(VertexData::new(ph()));
    let v2 = draft.insert_vertex(VertexData::new(ph()));
    let v3 = draft.insert_vertex(VertexData::new(ph()));
    let v4 = draft.insert_vertex(VertexData::new(ph()));
    let shared_edge = draft.insert_edge(EdgeData::new(ph()));

    // Face 0: simple 2-gon on shared_edge (v1→v2).
    let f0 = draft.insert_face(FaceData::new(placeholder_loop(), placeholder_shell()));
    let ret_e0 = draft.insert_edge(EdgeData::new(ph()));
    let h0_fwd = draft.insert_half_edge(HalfEdgeData::new(ph(), ph(), ph(), f0, v1, shared_edge));
    let h0_ret = draft.insert_half_edge(HalfEdgeData::new(h0_fwd, h0_fwd, h0_fwd, f0, v2, ret_e0));
    draft
        .arena_mut()
        .get_half_edge_mut(h0_fwd)
        .unwrap()
        .set_next(h0_ret);
    draft
        .arena_mut()
        .get_half_edge_mut(h0_fwd)
        .unwrap()
        .set_prev(h0_ret);
    draft
        .arena_mut()
        .get_half_edge_mut(h0_ret)
        .unwrap()
        .set_next(h0_fwd);
    draft
        .arena_mut()
        .get_half_edge_mut(h0_ret)
        .unwrap()
        .set_prev(h0_fwd);
    draft
        .arena_mut()
        .get_half_edge_mut(h0_ret)
        .unwrap()
        .set_radial_next(h0_ret);
    let l0 = draft.insert_loop(LoopData::new(h0_fwd, f0));
    draft
        .arena_mut()
        .get_face_mut(f0)
        .unwrap()
        .set_outer_loop(l0);

    // Face 1: 4-gon (quad) — v1→v2→v3→v4→v1.
    // Edge v1→v2 is shared_edge; the other 3 edges are unique.
    let f1 = draft.insert_face(FaceData::new(placeholder_loop(), placeholder_shell()));
    let e_23 = draft.insert_edge(EdgeData::new(ph()));
    let e_34 = draft.insert_edge(EdgeData::new(ph()));
    let e_41 = draft.insert_edge(EdgeData::new(ph()));
    let h1_12 = draft.insert_half_edge(HalfEdgeData::new(ph(), ph(), ph(), f1, v1, shared_edge));
    let h1_23 = draft.insert_half_edge(HalfEdgeData::new(ph(), ph(), ph(), f1, v2, e_23));
    let h1_34 = draft.insert_half_edge(HalfEdgeData::new(ph(), ph(), ph(), f1, v3, e_34));
    let h1_41 = draft.insert_half_edge(HalfEdgeData::new(ph(), ph(), ph(), f1, v4, e_41));
    // Wire quad loop: h1_12→h1_23→h1_34→h1_41→h1_12.
    draft
        .arena_mut()
        .get_half_edge_mut(h1_12)
        .unwrap()
        .set_next(h1_23);
    draft
        .arena_mut()
        .get_half_edge_mut(h1_12)
        .unwrap()
        .set_prev(h1_41);
    draft
        .arena_mut()
        .get_half_edge_mut(h1_23)
        .unwrap()
        .set_next(h1_34);
    draft
        .arena_mut()
        .get_half_edge_mut(h1_23)
        .unwrap()
        .set_prev(h1_12);
    draft
        .arena_mut()
        .get_half_edge_mut(h1_34)
        .unwrap()
        .set_next(h1_41);
    draft
        .arena_mut()
        .get_half_edge_mut(h1_34)
        .unwrap()
        .set_prev(h1_23);
    draft
        .arena_mut()
        .get_half_edge_mut(h1_41)
        .unwrap()
        .set_next(h1_12);
    draft
        .arena_mut()
        .get_half_edge_mut(h1_41)
        .unwrap()
        .set_prev(h1_34);
    // Radial self-loops for non-shared edges.
    draft
        .arena_mut()
        .get_half_edge_mut(h1_23)
        .unwrap()
        .set_radial_next(h1_23);
    draft
        .arena_mut()
        .get_half_edge_mut(h1_34)
        .unwrap()
        .set_radial_next(h1_34);
    draft
        .arena_mut()
        .get_half_edge_mut(h1_41)
        .unwrap()
        .set_radial_next(h1_41);
    let l1 = draft.insert_loop(LoopData::new(h1_12, f1));
    draft
        .arena_mut()
        .get_face_mut(f1)
        .unwrap()
        .set_outer_loop(l1);

    // Face 2: simple 2-gon on shared_edge (v1→v2).
    let f2 = draft.insert_face(FaceData::new(placeholder_loop(), placeholder_shell()));
    let ret_e2 = draft.insert_edge(EdgeData::new(ph()));
    let h2_fwd = draft.insert_half_edge(HalfEdgeData::new(ph(), ph(), ph(), f2, v1, shared_edge));
    let h2_ret = draft.insert_half_edge(HalfEdgeData::new(h2_fwd, h2_fwd, h2_fwd, f2, v2, ret_e2));
    draft
        .arena_mut()
        .get_half_edge_mut(h2_fwd)
        .unwrap()
        .set_next(h2_ret);
    draft
        .arena_mut()
        .get_half_edge_mut(h2_fwd)
        .unwrap()
        .set_prev(h2_ret);
    draft
        .arena_mut()
        .get_half_edge_mut(h2_ret)
        .unwrap()
        .set_next(h2_fwd);
    draft
        .arena_mut()
        .get_half_edge_mut(h2_ret)
        .unwrap()
        .set_prev(h2_fwd);
    draft
        .arena_mut()
        .get_half_edge_mut(h2_ret)
        .unwrap()
        .set_radial_next(h2_ret);
    let l2 = draft.insert_loop(LoopData::new(h2_fwd, f2));
    draft
        .arena_mut()
        .get_face_mut(f2)
        .unwrap()
        .set_outer_loop(l2);

    // Wire radial ring: h0_fwd → h1_12 → h2_fwd → h0_fwd (valence 3).
    draft
        .arena_mut()
        .get_half_edge_mut(h0_fwd)
        .unwrap()
        .set_radial_next(h1_12);
    draft
        .arena_mut()
        .get_half_edge_mut(h1_12)
        .unwrap()
        .set_radial_next(h2_fwd);
    draft
        .arena_mut()
        .get_half_edge_mut(h2_fwd)
        .unwrap()
        .set_radial_next(h0_fwd);

    // Vertex outgoing pointers.
    draft
        .arena_mut()
        .get_vertex_mut(v1)
        .unwrap()
        .set_primary_disk(h0_fwd);
    draft
        .arena_mut()
        .get_vertex_mut(v2)
        .unwrap()
        .set_primary_disk(h0_ret);
    draft
        .arena_mut()
        .get_vertex_mut(v3)
        .unwrap()
        .set_primary_disk(h1_34);
    draft
        .arena_mut()
        .get_vertex_mut(v4)
        .unwrap()
        .set_primary_disk(h1_41);
    draft
        .arena_mut()
        .get_edge_mut(shared_edge)
        .unwrap()
        .set_half_edge(h0_fwd);

    // NOW: merge face 0 (survive) and face 1 (kill, the quad).
    let out = draft.execute(
        JoinFacesNmt {
            he_survive: h0_fwd,
            he_kill: h1_12,
        },
    )
    .unwrap()
    .into_value();

    // The merged outer loop should contain:
    // - h0_ret (from face 0's boundary)
    //   PLUS the 3 non-shared edges from face 1: h1_23, h1_34, h1_41.
    //   Total: 4 halfedges in the outer loop.
    let outer_loop = draft
        .arena()
        .get_face(out.surviving_face)
        .unwrap()
        .outer_loop();
    let seed = draft.arena().get_loop(outer_loop).unwrap().half_edge();
    let mut outer_hes = vec![];
    let mut cur = seed;
    loop {
        outer_hes.push(cur);
        cur = draft.arena().get_half_edge(cur).unwrap().next();
        if cur == seed {
            break;
        }
        assert!(outer_hes.len() < 50, "Outer loop is not closed");
    }

    assert_eq!(
        outer_hes.len(),
        4,
        "Merged outer loop should have 4 halfedges (1 from face0 + 3 from quad face1), got {}",
        outer_hes.len(),
    );

    // The slit halfedges must NOT be in the outer loop.
    assert!(
        !outer_hes.contains(&h0_fwd),
        "Slit h0_fwd must not be in outer loop"
    );
    assert!(
        !outer_hes.contains(&h1_12),
        "Slit h1_12 must not be in outer loop"
    );

    // All 3 non-shared quad halfedges MUST be in the outer loop.
    assert!(
        outer_hes.contains(&h1_23),
        "quad he h1_23 must be in outer loop"
    );
    assert!(
        outer_hes.contains(&h1_34),
        "quad he h1_34 must be in outer loop"
    );
    assert!(
        outer_hes.contains(&h1_41),
        "quad he h1_41 must be in outer loop"
    );

    // The surviving return edge must be in the outer loop.
    assert!(outer_hes.contains(&h0_ret), "h0_ret must be in outer loop");

    // All halfedges in the outer loop must point to the surviving face.
    for &h in &outer_hes {
        assert_eq!(
            draft.arena().get_half_edge(h).unwrap().face(),
            out.surviving_face,
            "Halfedge {} in outer loop points to wrong face",
            h.index(),
        );
    }
}

/// Verify that the EdgeData entry point reaches the ENTIRE protected ring,
/// not just a subset. If the fix is missing, this would walk only the 2-element
/// slit and report count=2 instead of the correct protected count.
#[test]
fn edge_data_entry_walks_full_protected_ring() {
    
    use crate::validate::{validate_topology, ValidationLevel};

    let state = TopologyState::empty();
    let mut draft = state.into_mutation();
    let hes = setup_valence_n_edge(&mut draft, 6);

    // Merge hes[2] and hes[4] (non-adjacent in a ring of 6).
    draft.execute(
        JoinFacesNmt {
            he_survive: hes[2],
            he_kill: hes[4],
        },
    )
    .unwrap();

    let shared_edge = draft.arena().get_half_edge(hes[0]).unwrap().edge();
    let entry = draft.arena().get_edge(shared_edge).unwrap().half_edge();

    // Collect the entire ring walked from EdgeData.half_edge().
    let mut ring = vec![entry];
    let mut cur = draft.arena().get_half_edge(entry).unwrap().radial_next();
    while cur != entry {
        ring.push(cur);
        cur = draft.arena().get_half_edge(cur).unwrap().radial_next();
        assert!(ring.len() < 20, "Ring is not closed");
    }

    // Protected ring: 6 original - 2 merged = 4 remaining.
    assert_eq!(
        ring.len(),
        4,
        "Protected ring must have 4 elements (6 - 2 merged)"
    );

    // None of the protected ring members should be hes[2] or hes[4] (the slit).
    assert!(
        !ring.contains(&hes[2]),
        "Slit he must not be in protected ring"
    );
    assert!(
        !ring.contains(&hes[4]),
        "Slit he must not be in protected ring"
    );

    // Post-op validation must pass.
    validate_topology(
        draft.arena(),
        ValidationLevel::Minimal,
    )
    .expect("Post-op topology must pass validation after non-adjacent valence-6 merge");
}

// ===========================================================================
// ANTIPARALLEL SETUP — mimics real mesh topology
// ===========================================================================

/// Build a valence-N radial ring with ANTIPARALLEL halfedges.
///
/// In a real mesh, adjacent faces share an edge with opposite orientations:
/// - Even-indexed faces: shared-edge halfedge goes v1→v2
/// - Odd-indexed faces: shared-edge halfedge goes v2→v1
///
/// Each face is a 2-gon (lune). Returns the N shared-edge halfedges.
fn setup_antiparallel_valence_n(
    draft: &mut crate::transactions::MutableDraft,
    n: usize,
) -> (
    Vec<HalfEdgeId>,
    crate::handles::VertexId,
    crate::handles::VertexId,
) {
    let v1 = draft.insert_vertex(VertexData::new(ph()));
    let v2 = draft.insert_vertex(VertexData::new(ph()));
    let shared_edge = draft.insert_edge(EdgeData::new(ph()));

    let mut shared_hes = Vec::with_capacity(n);

    for i in 0..n {
        let f = draft.insert_face(FaceData::new(placeholder_loop(), placeholder_shell()));
        let ret_edge = draft.insert_edge(EdgeData::new(ph()));

        // Alternate direction: even faces go v1→v2, odd faces go v2→v1.
        let (origin, endpoint) = if i % 2 == 0 { (v1, v2) } else { (v2, v1) };

        let h_shared =
            draft.insert_half_edge(HalfEdgeData::new(ph(), ph(), ph(), f, origin, shared_edge));
        let h_ret = draft.insert_half_edge(HalfEdgeData::new(
            h_shared, h_shared, h_shared, f, endpoint, ret_edge,
        ));

        // Wire 2-gon loop: h_shared ↔ h_ret.
        draft
            .arena_mut()
            .get_half_edge_mut(h_shared)
            .unwrap()
            .set_next(h_ret);
        draft
            .arena_mut()
            .get_half_edge_mut(h_shared)
            .unwrap()
            .set_prev(h_ret);
        draft
            .arena_mut()
            .get_half_edge_mut(h_ret)
            .unwrap()
            .set_next(h_shared);
        draft
            .arena_mut()
            .get_half_edge_mut(h_ret)
            .unwrap()
            .set_prev(h_shared);
        draft
            .arena_mut()
            .get_half_edge_mut(h_ret)
            .unwrap()
            .set_radial_next(h_ret);
        draft
            .arena_mut()
            .get_edge_mut(ret_edge)
            .unwrap()
            .set_half_edge(h_ret);

        let l = draft.insert_loop(LoopData::new(h_shared, f));
        draft.arena_mut().get_face_mut(f).unwrap().set_outer_loop(l);

        if i == 0 {
            draft
                .arena_mut()
                .get_vertex_mut(v1)
                .unwrap()
                .set_primary_disk(h_shared);
            draft
                .arena_mut()
                .get_vertex_mut(v2)
                .unwrap()
                .set_primary_disk(h_ret);
            draft
                .arena_mut()
                .get_edge_mut(shared_edge)
                .unwrap()
                .set_half_edge(h_shared);
        }

        shared_hes.push(h_shared);
    }

    // Wire radial ring.
    for i in 0..n {
        let next = (i + 1) % n;
        draft
            .arena_mut()
            .get_half_edge_mut(shared_hes[i])
            .unwrap()
            .set_radial_next(shared_hes[next]);
    }

    (shared_hes, v1, v2)
}

// ===========================================================================
// Test 1: Antiparallel slit has two distinct vertex origins
// ===========================================================================

/// When merging antiparallel halfedges (he_s: v1→v2, he_k: v2→v1),
/// the slit's two halfedges must have DIFFERENT origins (v1 and v2).
/// This is the realistic case — our parallel setup had both origins = v1.
#[test]
fn antiparallel_slit_has_two_distinct_origins() {
    let state = TopologyState::empty();
    let mut draft = state.into_mutation();
    let (hes, v1, v2) = setup_antiparallel_valence_n(&mut draft, 4);

    // hes[0] has origin v1, hes[1] has origin v2 (antiparallel).
    let he_s = hes[0]; // v1→v2
    let he_k = hes[1]; // v2→v1

    assert_ne!(
        draft.arena().get_half_edge(he_s).unwrap().origin(),
        draft.arena().get_half_edge(he_k).unwrap().origin(),
        "Pre-condition: antiparallel halfedges must have different origins",
    );

    draft.execute(
        JoinFacesNmt {
            he_survive: he_s,
            he_kill: he_k,
        },
    )
    .unwrap();

    // Slit halfedges retain their original (different) origins.
    let s_origin = draft.arena().get_half_edge(he_s).unwrap().origin();
    let k_origin = draft.arena().get_half_edge(he_k).unwrap().origin();
    assert_eq!(s_origin, v1, "Slit he_s must retain origin v1");
    assert_eq!(k_origin, v2, "Slit he_k must retain origin v2");
    assert_ne!(
        s_origin, k_origin,
        "Slit origins must be distinct (antiparallel)"
    );
}

// ===========================================================================
// Test 2: Both endpoint vertices get correct outgoing pointers
// ===========================================================================

/// In the antiparallel case, both v1 and v2 are origins of slit halfedges.
/// The operator must fix outgoing for BOTH vertices, not just one.
/// This directly tests the `vertex_k != vertex_s` guard in the operator.
#[test]
fn antiparallel_vertex_outgoing_fixes_both_endpoints() {
    let state = TopologyState::empty();
    let mut draft = state.into_mutation();
    let (hes, v1, v2) = setup_antiparallel_valence_n(&mut draft, 3);

    let he_s = hes[0]; // origin v1
    let he_k = hes[1]; // origin v2

    // Force both vertices to point to their respective slit halfedges.
    draft
        .arena_mut()
        .get_vertex_mut(v1)
        .unwrap()
        .set_primary_disk(he_s);
    draft
        .arena_mut()
        .get_vertex_mut(v2)
        .unwrap()
        .set_primary_disk(he_k);

    draft.execute(
        JoinFacesNmt {
            he_survive: he_s,
            he_kill: he_k,
        },
    )
    .unwrap();

    let v1_out = draft.arena().get_vertex(v1).unwrap().primary_disk();
    let v2_out = draft.arena().get_vertex(v2).unwrap().primary_disk();

    // Neither vertex should point to a slit halfedge.
    assert_ne!(v1_out, he_s, "v1 outgoing must not be slit he_s");
    assert_ne!(v1_out, he_k, "v1 outgoing must not be slit he_k");
    assert_ne!(v2_out, he_s, "v2 outgoing must not be slit he_s");
    assert_ne!(v2_out, he_k, "v2 outgoing must not be slit he_k");

    // Origins must be correct.
    assert_eq!(
        draft.arena().get_half_edge(v1_out).unwrap().origin(),
        v1,
        "v1 outgoing must originate at v1",
    );
    assert_eq!(
        draft.arena().get_half_edge(v2_out).unwrap().origin(),
        v2,
        "v2 outgoing must originate at v2",
    );
}

// ===========================================================================
// Test 3: Structural validation passes on antiparallel post-op topology
// ===========================================================================

/// Run real structural validators on antiparallel post-op arena.
/// This is stronger than the parallel tests because vertex_continuity
/// will see 2 distinct endpoints in the slit ring (not a degenerate self-loop).
#[test]
fn antiparallel_passes_structural_validation() {
    
    use crate::validate::{validate_topology, ValidationLevel};

    let state = TopologyState::empty();
    let mut draft = state.into_mutation();
    let (hes, _, _) = setup_antiparallel_valence_n(&mut draft, 4);

    // Pre-op must validate.
    validate_topology(
        draft.arena(),
        ValidationLevel::Minimal,
    )
    .expect("Pre-op antiparallel topology must be valid");

    draft.execute(
        JoinFacesNmt {
            he_survive: hes[0],
            he_kill: hes[1],
        },
    )
    .unwrap();

    // Post-op must validate. This exercises validate_vertex_continuity
    // with a 2-endpoint slit ring (not a degenerate self-loop).
    validate_topology(
        draft.arena(),
        ValidationLevel::Minimal,
    )
    .expect("Post-op antiparallel topology must pass Minimal+NmtIntermediate validation");
}

// ===========================================================================
// Test 4: EulerDelta matches actual entity count changes
// ===========================================================================

/// Verify entity count changes match the operator's declared EulerDelta.
///
/// `apply_op` already validates declared_delta == actual_delta (returns
/// EulerFormulaViolation if wrong). This test documents the EXPECTED
/// absolute counts after a merge and catches off-by-one errors in the
/// delta declaration itself.
#[test]
fn euler_delta_matches_actual_entity_counts() {
    let state = TopologyState::empty();
    let mut draft = state.into_mutation();
    let hes = setup_valence_n_edge(&mut draft, 4);

    let pre_v = draft.arena().vertex_count();
    let pre_e = draft.arena().edge_count();
    let pre_he = draft.arena().half_edge_count();
    let pre_f = draft.arena().face_count();
    let pre_l = draft.arena().loop_count();

    // apply_op internally validates declared delta == actual delta.
    // If this succeeds, the delta is proven correct.
    draft.execute(
        JoinFacesNmt {
            he_survive: hes[1],
            he_kill: hes[2],
        },
    )
    .unwrap();

    // JoinFacesNmt should: kill 1 face, kill 1 loop, add 1 loop (slit).
    // Net: faces=-1, loops=0, vertices=0, edges=0, halfedges=0.
    assert_eq!(draft.arena().face_count(), pre_f - 1, "One face killed");
    assert_eq!(
        draft.arena().loop_count(),
        pre_l,
        "One loop killed + one slit loop added = net 0"
    );
    assert_eq!(
        draft.arena().vertex_count(),
        pre_v,
        "No vertices created or killed"
    );
    assert_eq!(
        draft.arena().edge_count(),
        pre_e,
        "No edges created or killed"
    );
    assert_eq!(
        draft.arena().half_edge_count(),
        pre_he,
        "No halfedges created or killed"
    );
}

// ===========================================================================
// Test 5: Outer loop bidirectional consistency
// ===========================================================================

/// Walk the merged outer loop forward (next) and backward (prev).
/// Both walks must visit the same halfedges in reverse order.
/// Catches cases where next is correct but prev was wired wrong.
#[test]
fn outer_loop_bidirectional_consistency() {
    let state = TopologyState::empty();
    let mut draft = state.into_mutation();
    let (hes, _, _) = setup_antiparallel_valence_n(&mut draft, 4);

    let out = draft.execute(
        JoinFacesNmt {
            he_survive: hes[0],
            he_kill: hes[1],
        },
    )
    .unwrap()
    .into_value();

    let outer_loop = draft
        .arena()
        .get_face(out.surviving_face)
        .unwrap()
        .outer_loop();
    let seed = draft.arena().get_loop(outer_loop).unwrap().half_edge();

    // Walk forward.
    let mut forward = vec![];
    let mut cur = seed;
    loop {
        forward.push(cur);
        cur = draft.arena().get_half_edge(cur).unwrap().next();
        if cur == seed {
            break;
        }
        assert!(forward.len() < 100, "Forward walk not closed");
    }

    // Walk backward.
    let mut backward = vec![];
    cur = seed;
    loop {
        backward.push(cur);
        cur = draft.arena().get_half_edge(cur).unwrap().prev();
        if cur == seed {
            break;
        }
        assert!(backward.len() < 100, "Backward walk not closed");
    }

    // Same length.
    assert_eq!(
        forward.len(),
        backward.len(),
        "Forward and backward walks have different lengths: {} vs {}",
        forward.len(),
        backward.len(),
    );

    // Backward walk should be the reverse of forward walk (rotated).
    // forward: [seed, a, b, c]
    // backward: [seed, c, b, a]
    // So backward[1..].reverse() should equal forward[1..].
    let mut back_rest: Vec<_> = backward[1..].to_vec();
    back_rest.reverse();
    assert_eq!(
        &forward[1..],
        &back_rest[..],
        "Backward walk is not the reverse of forward walk",
    );
}

// ===========================================================================
// Test 6: Pre-existing inner loops on surviving face survive the merge
// ===========================================================================

/// If the surviving face already has inner loops (holes), those must
/// remain intact after the slit's inner loop is added.
#[test]
fn surviving_face_pre_existing_inner_loops_intact() {
    let state = TopologyState::empty();
    let mut draft = state.into_mutation();
    let hes = setup_valence_n_edge(&mut draft, 3);

    let surviving_face = draft.arena().get_half_edge(hes[0]).unwrap().face();

    // Create a pre-existing inner loop on the surviving face.
    let inner_edge = draft.insert_edge(EdgeData::new(ph()));
    let v_inner = draft.insert_vertex(VertexData::new(ph()));
    let ih = draft.insert_half_edge(HalfEdgeData::new(
        ph(),
        ph(),
        ph(),
        surviving_face,
        v_inner,
        inner_edge,
    ));
    draft
        .arena_mut()
        .get_half_edge_mut(ih)
        .unwrap()
        .set_next(ih);
    draft
        .arena_mut()
        .get_half_edge_mut(ih)
        .unwrap()
        .set_prev(ih);
    draft
        .arena_mut()
        .get_half_edge_mut(ih)
        .unwrap()
        .set_radial_next(ih);
    draft
        .arena_mut()
        .get_vertex_mut(v_inner)
        .unwrap()
        .set_primary_disk(ih);

    let pre_existing_loop = draft.insert_loop(LoopData::new(ih, surviving_face));
    draft
        .arena_mut()
        .get_face_mut(surviving_face)
        .unwrap()
        .add_inner_loop(pre_existing_loop);

    // Verify pre-existing inner loop is registered.
    assert_eq!(
        draft
            .arena()
            .get_face(surviving_face)
            .unwrap()
            .inner_loop_count(),
        1,
        "Pre-condition: surviving face must have 1 inner loop",
    );

    // Merge.
    draft.execute(
        JoinFacesNmt {
            he_survive: hes[0],
            he_kill: hes[1],
        },
    )
    .unwrap();

    // After merge: surviving face should have 2 inner loops
    // (1 pre-existing + 1 slit).
    let inner_loops = draft
        .arena()
        .get_face(surviving_face)
        .unwrap()
        .inner_loops()
        .to_vec();
    assert_eq!(
        inner_loops.len(),
        2,
        "Surviving face must have 2 inner loops (pre-existing + slit), got {}",
        inner_loops.len(),
    );

    // The pre-existing inner loop must still be in the list.
    assert!(
        inner_loops.contains(&pre_existing_loop),
        "Pre-existing inner loop must survive the merge",
    );

    // Walk the pre-existing inner loop — its halfedge must point to surviving face.
    let ih_data = draft.arena().get_half_edge(ih).unwrap();
    assert_eq!(
        ih_data.face(),
        surviving_face,
        "Pre-existing inner loop halfedge must still point to surviving face",
    );

    // The loop entity must still reference the correct face.
    let loop_data = draft.arena().get_loop(pre_existing_loop).unwrap();
    assert_eq!(
        loop_data.face(),
        surviving_face,
        "Pre-existing loop entity must still reference surviving face",
    );
}

// ===========================================================================
// Test 7: Deep slit consistency — EdgeId, vertices, and next().origin()
// ===========================================================================

/// Verify deep invariants of the slit after an antiparallel merge:
/// - Both slit halfedges share the same EdgeId
/// - Origins are distinct (v1 and v2)
/// - next().origin() chain is consistent (slit forms a valid closed loop)
/// - Slit radial ring has exactly 2 elements
#[test]
fn slit_edge_and_vertex_deep_consistency() {
    let state = TopologyState::empty();
    let mut draft = state.into_mutation();
    let (hes, v1, v2) = setup_antiparallel_valence_n(&mut draft, 4);

    let he_s = hes[0]; // origin v1
    let he_k = hes[1]; // origin v2

    draft.execute(
        JoinFacesNmt {
            he_survive: he_s,
            he_kill: he_k,
        },
    )
    .unwrap();

    let s_data = draft.arena().get_half_edge(he_s).unwrap();
    let k_data = draft.arena().get_half_edge(he_k).unwrap();

    // Same EdgeId.
    assert_eq!(
        s_data.edge(),
        k_data.edge(),
        "Slit halfedges must share the same EdgeId",
    );

    // Distinct origins (antiparallel).
    assert_eq!(s_data.origin(), v1);
    assert_eq!(k_data.origin(), v2);

    // next().origin() consistency in the slit loop:
    // he_s (origin=v1) → he_k (origin=v2) → he_s (origin=v1)
    // So he_s.next().origin() = v2 = endpoint of he_s. ✓
    // And he_k.next().origin() = v1 = endpoint of he_k. ✓
    assert_eq!(
        draft.arena().get_half_edge(s_data.next()).unwrap().origin(),
        v2,
        "he_s.next().origin() must be v2 (endpoint of he_s)",
    );
    assert_eq!(
        draft.arena().get_half_edge(k_data.next()).unwrap().origin(),
        v1,
        "he_k.next().origin() must be v1 (endpoint of he_k)",
    );

    // Slit radial ring has exactly 2 elements.
    assert_eq!(s_data.radial_next(), he_k, "Slit radial: he_s → he_k");
    assert_eq!(k_data.radial_next(), he_s, "Slit radial: he_k → he_s");
}

// ===========================================================================
// CATEGORY 1: Symmetry branch tests for slit endpoints
// ===========================================================================

/// Test outgoing == he_kill when vertex_s != vertex_k (antiparallel).
/// This exercises the second branch of the outgoing fix (`vertex_k != vertex_s`).
#[test]
fn antiparallel_outgoing_he_kill_on_vertex_k() {
    let state = TopologyState::empty();
    let mut draft = state.into_mutation();
    let (hes, v1, v2) = setup_antiparallel_valence_n(&mut draft, 3);

    let he_s = hes[0]; // origin v1
    let he_k = hes[1]; // origin v2

    // v1 points to something safe, but v2 points to he_k (the kill halfedge).
    draft
        .arena_mut()
        .get_vertex_mut(v2)
        .unwrap()
        .set_primary_disk(he_k);

    draft.execute(
        JoinFacesNmt {
            he_survive: he_s,
            he_kill: he_k,
        },
    )
    .unwrap();

    let v2_out = draft.arena().get_vertex(v2).unwrap().primary_disk();
    assert_ne!(v2_out, he_s, "v2 outgoing must not be slit he_s");
    assert_ne!(v2_out, he_k, "v2 outgoing must not be slit he_k");
    assert_eq!(
        draft.arena().get_half_edge(v2_out).unwrap().origin(),
        v2,
        "v2 outgoing must originate at v2",
    );
}

/// Test outgoing == he_survive on vertex_k (antiparallel, cross-slit pointer).
/// Vertex v2's outgoing is he_s, which has origin v1 — invalid pre-op too,
/// but the operator must still not leave it on a slit halfedge.
#[test]
fn antiparallel_outgoing_he_survive_on_vertex_k() {
    let state = TopologyState::empty();
    let mut draft = state.into_mutation();
    let (hes, _v1, v2) = setup_antiparallel_valence_n(&mut draft, 3);

    let he_s = hes[0]; // origin v1
    let he_k = hes[1]; // origin v2

    // v2 points to he_s (cross-slit). This is already wrong (origin mismatch),
    // but the operator should still fix it if it detects it's a slit halfedge.
    draft
        .arena_mut()
        .get_vertex_mut(v2)
        .unwrap()
        .set_primary_disk(he_s);

    draft.execute(
        JoinFacesNmt {
            he_survive: he_s,
            he_kill: he_k,
        },
    )
    .unwrap();

    let v2_out = draft.arena().get_vertex(v2).unwrap().primary_disk();
    assert_ne!(v2_out, he_s, "v2 outgoing must not be slit he_s");
    assert_ne!(v2_out, he_k, "v2 outgoing must not be slit he_k");
}

// ===========================================================================
// CATEGORY 2: Selector/order symmetry
// ===========================================================================

/// Swap (he_survive, he_kill) on the same topology.
/// Both orderings must produce valid post-op topology with identical
/// structural invariants. The surviving face may differ, but the slit,
/// ring, and validation must all be correct regardless of which is "survive".
#[test]
fn selector_swap_both_orderings_valid() {
    
    use crate::validate::{validate_topology, ValidationLevel};

    // Run ordering A.
    let state_a = TopologyState::empty();
    let mut draft_a = state_a.into_mutation();
    let (hes_a, _, _) = setup_antiparallel_valence_n(&mut draft_a, 4);

    draft_a.execute(
        JoinFacesNmt {
            he_survive: hes_a[0],
            he_kill: hes_a[1],
        },
    )
    .unwrap();
    validate_topology(
        draft_a.arena(),
        ValidationLevel::Minimal,
    )
    .expect("Ordering A must pass validation");

    // Run ordering B (swapped).
    let state_b = TopologyState::empty();
    let mut draft_b = state_b.into_mutation();
    let (hes_b, _, _) = setup_antiparallel_valence_n(&mut draft_b, 4);

    draft_b.execute(
        JoinFacesNmt {
            he_survive: hes_b[1],
            he_kill: hes_b[0],
        },
    )
    .unwrap();
    validate_topology(
        draft_b.arena(),
        ValidationLevel::Minimal,
    )
    .expect("Ordering B (swapped) must pass validation");

    // Both should have same entity counts.
    assert_eq!(draft_a.arena().face_count(), draft_b.arena().face_count());
    assert_eq!(draft_a.arena().loop_count(), draft_b.arena().loop_count());
    assert_eq!(
        draft_a.arena().half_edge_count(),
        draft_b.arena().half_edge_count()
    );
}

// ===========================================================================
// CATEGORY 3: Valid hierarchy — Intermediate validation
// ===========================================================================

/// Build topology through real Euler operators (MVF → SE → MEF) to get
/// a valid body/lump/region/shell hierarchy, then inflate radial valence
/// and run JoinFacesNmt. Validate at Intermediate level, exercising
/// validate_loops and validate_hierarchy.
#[test]
fn valid_hierarchy_passes_intermediate_validation() {
    use crate::entity_lifecycle::make_edge_face::MakeEdgeFace;
    use crate::entity_lifecycle::make_vertex_face::MakeVertexFace;
    use crate::entity_lifecycle::split_edge::SplitEdge;
    
    use crate::validate::{validate_topology, ValidationLevel};

    let state = TopologyState::empty();
    let mut draft = state.into_mutation();

    // Build a valid 3-face mesh: MVF → SE → SE → MEF → MEF.
    let mvf = draft.execute(MakeVertexFace { shell_kind: ShellKind::Sheet }).unwrap().into_value();
    let se1 = draft.execute(
        SplitEdge {
            edge: mvf.half_edge,
        },
    )
    .unwrap()
    .into_value();
    let se2 = draft.execute(
        SplitEdge {
            edge: se1.he_am,
        },
    )
    .unwrap()
    .into_value();

    // Now we have 3 vertices. Split face to create 2 faces.
    let mef1 = draft.execute(
        MakeEdgeFace {
            vertex_a: mvf.vertex,
            vertex_b: se1.new_vertex,
            face: mvf.face,
        },
    )
    .unwrap()
    .into_value();

    // We now have 2 faces sharing an edge. Find the shared edge.
    let shared_he = mef1.half_edge_ab;
    let shared_edge_id = draft.arena().get_half_edge(shared_he).unwrap().edge();

    // Current valence is 2 (manifold). We need to inflate to 3 by
    // duplicating a face on the shared edge. The simplest way: insert
    // a third face manually sharing the same edge, with the same shell.
    let shell = mvf.shell;
    let existing_face_a = draft.arena().get_half_edge(shared_he).unwrap().face();
    let twin_he = draft
        .arena()
        .get_half_edge(shared_he)
        .unwrap()
        .radial_next();
    let existing_face_b = draft.arena().get_half_edge(twin_he).unwrap().face();

    let v_a = draft.arena().get_half_edge(shared_he).unwrap().origin();
    let v_b = draft.arena().get_half_edge(twin_he).unwrap().origin();

    // Create a third face on the same shell and edge.
    let ret_edge_3 = draft.insert_edge(EdgeData::new(ph()));
    let f3 = draft.insert_face(FaceData::new(placeholder_loop(), shell));
    let h3_fwd =
        draft.insert_half_edge(HalfEdgeData::new(ph(), ph(), ph(), f3, v_a, shared_edge_id));
    let h3_ret = draft.insert_half_edge(HalfEdgeData::new(
        h3_fwd, h3_fwd, h3_fwd, f3, v_b, ret_edge_3,
    ));
    draft
        .arena_mut()
        .get_half_edge_mut(h3_fwd)
        .unwrap()
        .set_next(h3_ret);
    draft
        .arena_mut()
        .get_half_edge_mut(h3_fwd)
        .unwrap()
        .set_prev(h3_ret);
    draft
        .arena_mut()
        .get_half_edge_mut(h3_ret)
        .unwrap()
        .set_next(h3_fwd);
    draft
        .arena_mut()
        .get_half_edge_mut(h3_ret)
        .unwrap()
        .set_prev(h3_fwd);
    draft
        .arena_mut()
        .get_half_edge_mut(h3_ret)
        .unwrap()
        .set_radial_next(h3_ret);
    draft
        .arena_mut()
        .get_edge_mut(ret_edge_3)
        .unwrap()
        .set_half_edge(h3_ret);
    let l3 = draft.insert_loop(LoopData::new(h3_fwd, f3));
    draft
        .arena_mut()
        .get_face_mut(f3)
        .unwrap()
        .set_outer_loop(l3);

    // Wire the radial ring: shared_he → twin_he → h3_fwd → shared_he (valence 3).
    draft
        .arena_mut()
        .get_half_edge_mut(shared_he)
        .unwrap()
        .set_radial_next(twin_he);
    draft
        .arena_mut()
        .get_half_edge_mut(twin_he)
        .unwrap()
        .set_radial_next(h3_fwd);
    draft
        .arena_mut()
        .get_half_edge_mut(h3_fwd)
        .unwrap()
        .set_radial_next(shared_he);

    // Now merge shared_he and twin_he.
    draft.execute(
        JoinFacesNmt {
            he_survive: shared_he,
            he_kill: twin_he,
        },
    )
    .unwrap();

    // Validate at Intermediate level — exercises validate_loops AND validate_hierarchy.
    validate_topology(
        draft.arena(),
        ValidationLevel::Intermediate,
    )
    .expect("Post-op valid-hierarchy topology must pass Intermediate+NmtIntermediate validation");
}

// ===========================================================================
// CATEGORY 4: Handle re-derivation across sequential merges
// ===========================================================================

/// Sequential merges where handles are RE-DERIVED from arena state each step.
/// Guards against callers accidentally reusing stale handles.
#[test]
fn sequential_merges_with_handle_rederivation() {
    let state = TopologyState::empty();
    let mut draft = state.into_mutation();
    let hes = setup_valence_n_edge(&mut draft, 5);
    let shared_edge = draft.arena().get_half_edge(hes[0]).unwrap().edge();

    // First merge: hes[0] and hes[1].
    draft.execute(
        JoinFacesNmt {
            he_survive: hes[0],
            he_kill: hes[1],
        },
    )
    .unwrap();

    // RE-DERIVE: walk the protected ring from EdgeData to find current members.
    let entry = draft.arena().get_edge(shared_edge).unwrap().half_edge();
    let mut protected_ring = vec![entry];
    let mut cur = draft.arena().get_half_edge(entry).unwrap().radial_next();
    while cur != entry {
        protected_ring.push(cur);
        cur = draft.arena().get_half_edge(cur).unwrap().radial_next();
        assert!(protected_ring.len() < 20);
    }
    assert_eq!(
        protected_ring.len(),
        3,
        "After first merge: protected ring should have 3 elements"
    );

    // Pick two adjacent elements from the RE-DERIVED ring for the second merge.
    let he_s2 = protected_ring[0];
    let he_k2 = protected_ring[1];

    // Verify they're on different faces (required precondition).
    assert_ne!(
        draft.arena().get_half_edge(he_s2).unwrap().face(),
        draft.arena().get_half_edge(he_k2).unwrap().face(),
        "Re-derived halfedges must be on different faces",
    );

    draft.execute(
        JoinFacesNmt {
            he_survive: he_s2,
            he_kill: he_k2,
        },
    )
    .unwrap();

    // Final protected ring should have 1 element.
    let entry2 = draft.arena().get_edge(shared_edge).unwrap().half_edge();
    assert_eq!(
        draft.arena().get_half_edge(entry2).unwrap().radial_next(),
        entry2,
        "After two merges: sole remaining protected halfedge must self-loop",
    );
}

// ===========================================================================
// CATEGORY 5: Typed error taxonomy assertions
// ===========================================================================

/// Rejection errors must be KernelError::InvalidInput (not some other variant).
/// This prevents refactors from silently degrading structured error types.
#[test]
fn rejection_errors_are_typed_invalid_input() {
    // Test 1: manifold edge → InvalidInput.
    {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();
        let hes = setup_valence_n_edge(&mut draft, 2);
        let err = draft.execute(
            JoinFacesNmt {
                he_survive: hes[0],
                he_kill: hes[1],
            },
        )
        .unwrap_err();
        assert!(
            matches!(err, KernelError::InvalidInput { .. }),
            "Manifold rejection must be KernelError::InvalidInput, got: {:?}",
            err,
        );
    }

    // Test 2: same face → InvalidInput.
    {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();
        let hes = setup_valence_n_edge(&mut draft, 4);
        let err = draft.execute(
            JoinFacesNmt {
                he_survive: hes[0],
                he_kill: hes[0],
            },
        )
        .unwrap_err();
        assert!(
            matches!(err, KernelError::InvalidInput { .. }),
            "Same-face rejection must be KernelError::InvalidInput, got: {:?}",
            err,
        );
    }

    // Test 3: different edge → InvalidInput.
    {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();

        let v1 = draft.insert_vertex(VertexData::new(ph()));
        let edge_a = draft.insert_edge(EdgeData::new(ph()));
        let edge_b = draft.insert_edge(EdgeData::new(ph()));
        let f1 = draft.insert_face(FaceData::new(placeholder_loop(), placeholder_shell()));
        let f2 = draft.insert_face(FaceData::new(placeholder_loop(), placeholder_shell()));
        let ha = draft.insert_half_edge(HalfEdgeData::new(ph(), ph(), ph(), f1, v1, edge_a));
        let hb = draft.insert_half_edge(HalfEdgeData::new(ph(), ph(), ph(), f2, v1, edge_b));
        draft
            .arena_mut()
            .get_half_edge_mut(ha)
            .unwrap()
            .set_next(ha);
        draft
            .arena_mut()
            .get_half_edge_mut(ha)
            .unwrap()
            .set_prev(ha);
        draft
            .arena_mut()
            .get_half_edge_mut(ha)
            .unwrap()
            .set_radial_next(ha);
        draft
            .arena_mut()
            .get_half_edge_mut(hb)
            .unwrap()
            .set_next(hb);
        draft
            .arena_mut()
            .get_half_edge_mut(hb)
            .unwrap()
            .set_prev(hb);
        draft
            .arena_mut()
            .get_half_edge_mut(hb)
            .unwrap()
            .set_radial_next(hb);

        let err = draft.execute(
            JoinFacesNmt {
                he_survive: ha,
                he_kill: hb,
            },
        )
        .unwrap_err();
        assert!(
            matches!(err, KernelError::InvalidInput { .. }),
            "Edge-mismatch rejection must be KernelError::InvalidInput, got: {:?}",
            err,
        );
    }
}

/// Verify the InvalidInput message content distinguishes each rejection reason.
/// This ensures callers can programmatically distinguish rejection causes.
#[test]
fn rejection_messages_are_distinguishable() {
    use forge_core::KernelError;

    // Manifold.
    let state = TopologyState::empty();
    let mut draft = state.into_mutation();
    let hes = setup_valence_n_edge(&mut draft, 2);
    let err1 = draft.execute(
        JoinFacesNmt {
            he_survive: hes[0],
            he_kill: hes[1],
        },
    )
    .unwrap_err();

    // Same face.
    let state2 = TopologyState::empty();
    let mut draft2 = state2.into_mutation();
    let hes2 = setup_valence_n_edge(&mut draft2, 4);
    let err2 = draft2.execute(
        JoinFacesNmt {
            he_survive: hes2[0],
            he_kill: hes2[0],
        },
    )
    .unwrap_err();

    // Extract messages.
    let msg1 = match &err1 {
        KernelError::InvalidInput { message, .. } => message.clone(),
        e => panic!("wrong type: {e:?}"),
    };
    let msg2 = match &err2 {
        KernelError::InvalidInput { message, .. } => message.clone(),
        e => panic!("wrong type: {e:?}"),
    };

    assert_ne!(
        msg1, msg2,
        "Different rejections must produce different messages"
    );
    assert!(
        msg1.contains("valence") || msg1.contains("> 2"),
        "Manifold message should mention valence"
    );
    assert!(
        msg2.contains("same face"),
        "Same-face message should mention 'same face'"
    );
}

// ===========================================================================
// D2 REGRESSION: shared-origin outgoing == he_kill
// ===========================================================================

/// D2 regression: when vertex_s == vertex_k AND outgoing == he_kill,
/// the old branching code skipped the fixup entirely (only checked he_s
/// in the first branch, and skipped vertex_k because vertex_k == vertex_s).
///
/// This test directly guards the asymmetry bug.
#[test]
fn shared_origin_outgoing_he_kill_is_fixed() {
    let state = TopologyState::empty();
    let mut draft = state.into_mutation();
    let hes = setup_valence_n_edge(&mut draft, 3);

    let he_s = hes[0];
    let he_k = hes[1];

    // Both share v1 as origin (parallel setup).
    let v1 = draft.arena().get_half_edge(he_s).unwrap().origin();
    assert_eq!(
        v1,
        draft.arena().get_half_edge(he_k).unwrap().origin(),
        "Pre-condition: both slit halfedges share origin vertex (parallel setup)",
    );

    // Force v1 outgoing to he_k (the KILL halfedge). This is the exact
    // scenario the old code missed — it only checked `vs_out == he_s` in
    // the first branch, and the second branch was guarded by `vertex_k != vertex_s`.
    draft
        .arena_mut()
        .get_vertex_mut(v1)
        .unwrap()
        .set_primary_disk(he_k);

    draft.execute(
        JoinFacesNmt {
            he_survive: he_s,
            he_kill: he_k,
        },
    )
    .unwrap();

    let v1_out = draft.arena().get_vertex(v1).unwrap().primary_disk();
    assert_ne!(
        v1_out, he_s,
        "D2 regression: v1 outgoing must not be slit he_s after shared-origin merge",
    );
    assert_ne!(
        v1_out, he_k,
        "D2 regression: v1 outgoing must not be slit he_k after shared-origin merge",
    );
    assert_eq!(
        draft.arena().get_half_edge(v1_out).unwrap().origin(),
        v1,
        "D2 regression: v1 outgoing replacement must originate at v1",
    );
}
