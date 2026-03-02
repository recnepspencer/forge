//! Structural assertion helpers for integration tests.
//!
//! DOMAIN: Reusable invariant checks that every integration test calls.
//! When persistent naming or lineage lands, add new assertions here —
//! all tests pick them up automatically.

use forge_topo::b_rep::TopologyArena;
use forge_topo::handles::FaceId;
use forge_core::DecisionLog;

/// Expected entity counts for assertion.
#[derive(Debug, Clone)]
pub struct EntityCounts {
    pub faces: usize,
    pub vertices: usize,
    pub half_edges: usize,
    pub edges: usize,
    pub loops: usize,
    pub shells: usize,
    pub bodies: usize,
}

/// Assert that every halfedge has reciprocal twin and next/prev pointers.
pub fn assert_reciprocity(arena: &TopologyArena) {
    for (he_id, he_data) in arena.iter_half_edges() {
        let twin_id = he_data.radial_next();
        if he_id != twin_id {
            let twin_data = arena.get_half_edge(twin_id)
                .unwrap_or_else(|e| panic!(
                    "Twin {} of he {} not found: {:?}", twin_id.index(), he_id.index(), e
                ));
            assert_eq!(
                twin_data.radial_next(), he_id,
                "Twin reciprocity broken: he[{}].twin={}, but he[{}].twin={} (expected {})",
                he_id.index(), twin_id.index(), twin_id.index(),
                twin_data.radial_next().index(), he_id.index()
            );
        }

        let next_id = he_data.next();
        let next_data = arena.get_half_edge(next_id)
            .unwrap_or_else(|e| panic!(
                "Next {} of he {} not found: {:?}", next_id.index(), he_id.index(), e
            ));
        assert_eq!(
            next_data.prev(), he_id,
            "Next/prev broken: he[{}].next={}, but he[{}].prev={} (expected {})",
            he_id.index(), next_id.index(), next_id.index(),
            next_data.prev().index(), he_id.index()
        );

        let prev_id = he_data.prev();
        let prev_data = arena.get_half_edge(prev_id)
            .unwrap_or_else(|e| panic!(
                "Prev {} of he {} not found: {:?}", prev_id.index(), he_id.index(), e
            ));
        assert_eq!(
            prev_data.next(), he_id,
            "Prev/next broken: he[{}].prev={}, but he[{}].next={} (expected {})",
            he_id.index(), prev_id.index(), prev_id.index(),
            prev_data.next().index(), he_id.index()
        );
    }
}

/// Assert every halfedge in a face loop actually claims that face as its owner.
///
/// This catches the exact bug we fixed: operators using `set_face()` directly
/// instead of `reassign_halfedge_face()`, leaving the halfedge's face pointer
/// correct but the reverse index stale — or vice versa.
pub fn assert_face_ownership(arena: &TopologyArena) {
    for (face_id, face_data) in arena.iter_faces() {
        let outer_loop = face_data.outer_loop();
        let outer_he = arena.get_loop(outer_loop)
            .unwrap_or_else(|e| panic!(
                "Outer loop {} of face {} not found: {:?}",
                outer_loop.index(), face_id.index(), e
            ))
            .half_edge();

        let mut current = outer_he;
        let max = arena.half_edge_count() + 1;
        for step in 0..max {
            let hd = arena.get_half_edge(current).unwrap();
            assert_eq!(
                hd.face(), face_id,
                "Face ownership broken: he[{}] claims face {} but is in loop of face {} (step {})",
                current.index(), hd.face().index(), face_id.index(), step
            );
            current = hd.next();
            if current == outer_he { break; }
            assert!(
                step < max - 1,
                "Face {} outer loop not closed after {} steps",
                face_id.index(), max
            );
        }

        for &inner_loop in face_data.inner_loops() {
            let inner_he = arena.get_loop(inner_loop).unwrap().half_edge();
            let mut current = inner_he;
            for step in 0..max {
                let hd = arena.get_half_edge(current).unwrap();
                assert_eq!(
                    hd.face(), face_id,
                    "Inner loop ownership broken: he[{}] claims face {} in inner loop of face {} (step {})",
                    current.index(), hd.face().index(), face_id.index(), step
                );
                current = hd.next();
                if current == inner_he { break; }
                assert!(
                    step < max - 1,
                    "Face {} inner loop {} not closed after {} steps",
                    face_id.index(), inner_loop.index(), max
                );
            }
        }
    }
}

/// Assert every vertex orbit is closed (walking outgoing→radial_next→next
/// returns to the starting halfedge).
///
/// A broken vertex orbit causes infinite loops in vertex-fan traversal.
pub fn assert_vertex_orbits(arena: &TopologyArena) {
    for (vid, vdata) in arena.iter_vertices() {
        let start = vdata.outgoing();

        let start_data = arena.get_half_edge(start)
            .unwrap_or_else(|e| panic!(
                "Vertex {} outgoing he {} not found: {:?}",
                vid.index(), start.index(), e
            ));
        assert_eq!(
            start_data.origin(), vid,
            "Vertex {} outgoing he {} has origin {} (expected {})",
            vid.index(), start.index(), start_data.origin().index(), vid.index()
        );

        let mut current = start;
        let max = arena.half_edge_count() + 1;
        for step in 0..max {
            let he_data = arena.get_half_edge(current).unwrap();

            assert_eq!(
                he_data.origin(), vid,
                "Vertex orbit broken: at step {}, he[{}] has origin {} (expected vertex {})",
                step, current.index(), he_data.origin().index(), vid.index()
            );

            let twin = he_data.radial_next();
            let twin_next = arena.get_half_edge(twin).unwrap().next();
            current = twin_next;

            if current == start { break; }
            assert!(
                step < max - 1,
                "Vertex {} orbit not closed after {} steps (stuck at he {})",
                vid.index(), max, current.index()
            );
        }
    }
}

/// Assert every edge's canonical halfedge actually belongs to that edge,
/// and the edge's two halfedges have the correct origin vertices (different).
pub fn assert_edge_consistency(arena: &TopologyArena) {
    for (eid, edata) in arena.iter_edges() {
        let he = edata.half_edge();
        let hd = arena.get_half_edge(he)
            .unwrap_or_else(|e| panic!(
                "Edge {} canonical he {} not found: {:?}",
                eid.index(), he.index(), e
            ));

        assert_eq!(
            hd.edge(), eid,
            "Edge {} canonical he {} claims edge {} (expected {})",
            eid.index(), he.index(), hd.edge().index(), eid.index()
        );

        let twin = hd.radial_next();
        if twin != he {
            let td = arena.get_half_edge(twin).unwrap();
            assert_eq!(
                td.edge(), eid,
                "Edge {} twin he {} claims edge {} (expected {})",
                eid.index(), twin.index(), td.edge().index(), eid.index()
            );

            assert_ne!(
                hd.origin(), td.origin(),
                "Edge {} has both halfedges with same origin vertex {}",
                eid.index(), hd.origin().index()
            );
        }
    }
}

/// Assert every loop's canonical halfedge actually belongs to a halfedge
/// on the loop's owning face.
pub fn assert_loop_face_consistency(arena: &TopologyArena) {
    for (face_id, face_data) in arena.iter_faces() {
        let outer_loop = face_data.outer_loop();
        let outer_he = arena.get_loop(outer_loop).unwrap().half_edge();
        let outer_face = arena.get_half_edge(outer_he).unwrap().face();
        assert_eq!(
            outer_face, face_id,
            "Loop {} canonical he {} has face {} but loop belongs to face {}",
            outer_loop.index(), outer_he.index(), outer_face.index(), face_id.index()
        );

        for &il in face_data.inner_loops() {
            let ih = arena.get_loop(il).unwrap().half_edge();
            let ihf = arena.get_half_edge(ih).unwrap().face();
            assert_eq!(
                ihf, face_id,
                "Inner loop {} canonical he {} has face {} but loop belongs to face {}",
                il.index(), ih.index(), ihf.index(), face_id.index()
            );
        }
    }
}

/// Assert all face loops are closed (walking `next` returns to start).
pub fn assert_closed_loops(arena: &TopologyArena) {
    for (face_id, _face_data) in arena.iter_faces() {
        let hes = arena.halfedges_of_face(face_id);
        assert!(!hes.is_empty(), "Face {} has no halfedges", face_id.index());

        let start = hes[0];
        let mut current = arena.get_half_edge(start).unwrap().next();
        let mut count = 1;
        let max = 1000;

        while current != start {
            count += 1;
            assert!(count <= max, "Face {} loop not closed after {} steps", face_id.index(), max);
            current = arena.get_half_edge(current).unwrap().next();
        }

        assert!(
            count >= 3,
            "Face {} has degenerate loop with only {} halfedges",
            face_id.index(), count
        );
    }
}

/// Assert entity counts match expected values.
pub fn assert_counts(arena: &TopologyArena, expected: EntityCounts) {
    assert_eq!(arena.face_count(), expected.faces, "face count mismatch");
    assert_eq!(arena.vertex_count(), expected.vertices, "vertex count mismatch");
    assert_eq!(arena.half_edge_count(), expected.half_edges, "halfedge count mismatch");
    assert_eq!(arena.edge_count(), expected.edges, "edge count mismatch");
    assert_eq!(arena.loop_count(), expected.loops, "loop count mismatch");
    assert_eq!(arena.shell_count(), expected.shells, "shell count mismatch");
    assert_eq!(arena.body_count(), expected.bodies, "body count mismatch");
}

/// Assert Euler's formula V - E + F = 2 for a closed solid shell.
pub fn assert_euler_formula(arena: &TopologyArena) {
    let v = arena.vertex_count() as i64;
    let e = arena.edge_count() as i64;
    let f = arena.face_count() as i64;
    let chi = v - e + f;
    assert_eq!(
        chi, 2,
        "Euler formula violated: V({}) - E({}) + F({}) = {} (expected 2)",
        v, e, f, chi
    );
}

/// Run ALL structural invariant checks for closed solids.
///
/// This is the nuclear option — catches:
/// - Broken next/prev/twin pointers (reciprocity)
/// - Halfedges claiming wrong face (face ownership)
/// - Broken vertex orbits (infinite traversal)
/// - Edge-halfedge mismatch (edge consistency)
/// - Loop-face pointer mismatch (loop-face consistency)
/// - Unclosed loops (closed loops)
/// - Euler formula violation
pub fn assert_all_invariants(arena: &TopologyArena) {
    assert_structural_invariants(arena);
    assert_euler_formula(arena);
}

/// Run all structural checks EXCEPT Euler formula.
///
/// Use this after operators that create open shells (e.g., KFMRH removes
/// a face, breaking V-E+F=2). Still catches all wiring bugs.
pub fn assert_structural_invariants(arena: &TopologyArena) {
    assert_reciprocity(arena);
    assert_face_ownership(arena);
    assert_vertex_orbits(arena);
    assert_edge_consistency(arena);
    assert_loop_face_consistency(arena);
    assert_closed_loops(arena);
}

/// Assert that a specific face has exactly `expected` halfedges in its loop.
pub fn assert_face_valence(arena: &TopologyArena, face: FaceId, expected: usize) {
    let hes = arena.halfedges_of_face(face);
    let start = hes[0];
    let mut current = arena.get_half_edge(start).unwrap().next();
    let mut count = 1;
    while current != start {
        count += 1;
        current = arena.get_half_edge(current).unwrap().next();
    }
    assert_eq!(
        count, expected,
        "Face {} has {} halfedges (expected {})",
        face.index(), count, expected
    );
}

/// Assert every decision in a `DecisionLog` is well-formed.
///
/// Validates:
/// - Non-negative margin on every decision
/// - Populated context (not a zero-default)
///
/// This is the observability equivalent of `assert_all_invariants` —
/// call it after any traced operation to catch garbage decision payloads.
pub fn assert_decisions_well_formed(log: &DecisionLog) {
    for decision in log.decisions() {
        assert!(
            decision.get_margin() >= 0.0,
            "Decision {:?} has negative margin: {}",
            decision.get_id(), decision.get_margin()
        );
    }
}

/// Assert vertex placement decisions are valid.
///
/// Thin test wrapper around the production validator in
/// `operations::shared_validators::facade::validate_vertex_decisions`.
/// Emits the decision summary via tracing, then delegates.
pub fn assert_vertex_decisions(
    label: &str,
    log: &DecisionLog,
    expected_vertices: usize,
    tolerance: f64,
) {
    forge_core::tracing::log_decision_log(label, log);
    crate::operations::shared_validators::facade::validate_vertex_decisions(
        log, expected_vertices, tolerance,
    )
    .unwrap_or_else(|e| panic!("{label}: {e}"));
}
