//! Brutality tests for topology hardening.
//!
//! DOMAIN: Stress-testing topology invariants under extreme edge cases.
//!
//! These tests validate the robustness of canonical hashing, mutation detection,
//! generational arena recycling, and diff engine correctness.

#[cfg(test)]
mod tests {
    use crate::state::TopologyState;
    use crate::operator::apply_op;
    use crate::euler::make_vertex_face::MakeVertexFace;
    use crate::euler::split_edge::SplitEdge;
    use crate::euler::make_edge_face::MakeEdgeFace;
    use crate::euler::kill_edge_vertex::KillEdgeVertex;
    use crate::traverse::{vertex_ring, face_edges};
    use crate::attributes::{EntityKey, TagValue};
    use crate::diff::{compute_diff, EntityDelta};

    // ─────────────────────────────────────────────────────────────────
    // 1. The "Bowtie" Vertex (Non-Manifold Resistance)
    //
    // Two faces sharing ONLY a single vertex (no shared edge).
    // Constructed by: building a quad, splitting it into two triangles
    // via MEF, then collapsing the shared edge via KEV.
    // After KEV, the two faces touch only at the surviving vertex.
    //
    // Assert: vertex_ring around the pinch vertex visits edges from
    // BOTH faces without infinite-looping.
    // ─────────────────────────────────────────────────────────────────
    #[test]
    fn bowtie_vertex_traversal() {
        let state = TopologyState::empty();
        let mut draft = state.begin_mutation();

        // Build a quad: v0 -> v1 -> v2 -> v3 -> v0
        let mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
        let se1 = apply_op(&mut draft, SplitEdge { edge: mvf.half_edge, parameter: 0.25 }).unwrap().into_value();
        let se2 = apply_op(&mut draft, SplitEdge { edge: se1.he_mb, parameter: 0.33 }).unwrap().into_value();
        let se3 = apply_op(&mut draft, SplitEdge { edge: se2.he_mb, parameter: 0.5 }).unwrap().into_value();

        // Identify vertices on the quad loop
        let edges = face_edges(draft.arena(), mvf.face).unwrap();
        assert_eq!(edges.len(), 4, "Quad must have 4 edges");

        let v0 = draft.arena().get_half_edge(edges[0]).unwrap().origin;
        let v1 = draft.arena().get_half_edge(edges[1]).unwrap().origin;
        let v2 = draft.arena().get_half_edge(edges[2]).unwrap().origin;
        let v3 = draft.arena().get_half_edge(edges[3]).unwrap().origin;

        // MEF: split quad into two triangles sharing edge v0-v2
        let mef = apply_op(&mut draft, MakeEdgeFace {
            face: mvf.face,
            vertex_a: v0,
            vertex_b: v2,
        }).unwrap().into_value();

        // Now we have Face0 (triangle v0-v1-v2) and Face1 (triangle v0-v2-v3).
        // They share edge v0-v2.
        // KEV on that shared edge collapses v2 into v0, creating a bowtie at v0.
        let _kev = apply_op(&mut draft, KillEdgeVertex { edge: mef.half_edge_ab }).unwrap().into_value();

        // v0 is now the pinch vertex. vertex_ring must traverse all incident edges.
        let v_center = _kev.surviving_vertex;
        let ring = vertex_ring(draft.arena(), v_center).unwrap();

        // The ring must visit edges from both faces.
        assert!(ring.len() >= 2, "Bowtie vertex ring must have edges from both faces, got {}", ring.len());

        // Verify no duplicate entries (no infinite loop)
        let unique_count = {
            let mut ids: Vec<_> = ring.iter().map(|h| h.index()).collect();
            ids.sort();
            ids.dedup();
            ids.len()
        };
        assert_eq!(unique_count, ring.len(), "vertex_ring must not produce duplicates");

        // Commit must succeed (topology is valid even if non-manifold at vertex)
        let _state = draft.commit().unwrap();
    }

    // ─────────────────────────────────────────────────────────────────
    // 2. The Commutative DAG Fuzzer (Determinism Check)
    //
    // Two independent SplitEdge ops applied in opposite order must
    // yield the same topology hash if hashing is truly canonical.
    //
    // NOTE: This test validates the *aspiration* of canonical hashing.
    // If it fails, our hash is index-dependent and needs upgrading
    // to a permutation-invariant scheme.
    // ─────────────────────────────────────────────────────────────────
    #[test]
    fn commutative_dag_fuzzer() {
        let state = TopologyState::empty();

        // Build seed: a face with 2 edges
        let mut seed_draft = state.begin_mutation();
        let mvf = apply_op(&mut seed_draft, MakeVertexFace).unwrap().into_value();
        let _se = apply_op(&mut seed_draft, SplitEdge { edge: mvf.half_edge, parameter: 0.5 }).unwrap().into_value();
        let seed_state = seed_draft.commit().unwrap();

        let edges = face_edges(seed_state.arena(), mvf.face).unwrap();
        assert_eq!(edges.len(), 2, "Seed must have exactly 2 edges");
        let e1 = edges[0];
        let e2 = edges[1];

        // Draft A: split e1 first, then e2
        let mut draft_a = seed_state.begin_mutation();
        apply_op(&mut draft_a, SplitEdge { edge: e1, parameter: 0.3 }).unwrap();
        apply_op(&mut draft_a, SplitEdge { edge: e2, parameter: 0.7 }).unwrap();
        let state_a = draft_a.commit().unwrap();

        // Draft B: split e2 first, then e1
        let mut draft_b = seed_state.begin_mutation();
        apply_op(&mut draft_b, SplitEdge { edge: e2, parameter: 0.7 }).unwrap();
        apply_op(&mut draft_b, SplitEdge { edge: e1, parameter: 0.3 }).unwrap();
        let state_b = draft_b.commit().unwrap();

        // Both states should have the same entity counts
        assert_eq!(
            state_a.arena().face_count(),
            state_b.arena().face_count(),
            "Face counts must match"
        );
        assert_eq!(
            state_a.arena().vertex_count(),
            state_b.arena().vertex_count(),
            "Vertex counts must match"
        );

        // Canonical hash comparison — this is the key assertion.
        // If this fails, our hashing is index-dependent (known limitation).
        assert_eq!(
            state_a.topology_hash(),
            state_b.topology_hash(),
            "Topology hashes must be identical despite operation order"
        );
    }

    // ─────────────────────────────────────────────────────────────────
    // 3. The Sliver Face Collapse (Topology vs. Geometry Firewall)
    //
    // A geometrically degenerate face (sliver) must survive topologically.
    // Attributes on the sliver face must remain accessible after collapse.
    // ─────────────────────────────────────────────────────────────────
    #[test]
    fn sliver_face_collapse() {
        let state = TopologyState::empty();
        let mut draft = state.begin_mutation();

        // Build a quad
        let mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
        let se1 = apply_op(&mut draft, SplitEdge { edge: mvf.half_edge, parameter: 0.5 }).unwrap().into_value();
        let se2 = apply_op(&mut draft, SplitEdge { edge: se1.he_mb, parameter: 0.5 }).unwrap().into_value();
        let se3 = apply_op(&mut draft, SplitEdge { edge: se2.he_mb, parameter: 0.5 }).unwrap().into_value();

        let edges = face_edges(draft.arena(), mvf.face).unwrap();
        assert_eq!(edges.len(), 4);

        let v0 = draft.arena().get_half_edge(edges[0]).unwrap().origin;
        let v2 = draft.arena().get_half_edge(edges[2]).unwrap().origin;

        // MEF: create a diagonal edge splitting the quad into two faces
        let mef = apply_op(&mut draft, MakeEdgeFace {
            face: mvf.face,
            vertex_a: v0,
            vertex_b: v2,
        }).unwrap().into_value();

        let sliver_face = mef.new_face;

        // Tag the sliver face with an attribute
        draft.arena_mut().get_attribute_store_mut().set_tag(
            EntityKey::Face(sliver_face),
            "quality".to_string(),
            TagValue::Text("sliver".to_string()),
        );

        // KEV: collapse the shared edge, making the sliver geometrically degenerate
        let _kev = apply_op(&mut draft, KillEdgeVertex { edge: mef.half_edge_ab }).unwrap().into_value();

        // The attribute store must NOT panic when queried for the sliver face
        let tag = draft.arena().get_attribute_store().get_tags(EntityKey::Face(sliver_face));
        assert!(tag.is_some(), "Sliver face attributes must survive topological collapse");

        // Commit must succeed — topology is valid even for degenerate geometry
        let final_state = draft.commit().unwrap();
        assert!(final_state.arena().face_count() > 0, "At least one face must survive");
    }

    // ─────────────────────────────────────────────────────────────────
    // 4. The Self-Intersecting "Spaghetti" Split
    //
    // Split the initial self-loop 1000 times, then undo all 1000 via KEV.
    // The arena must recycle slots correctly without leaking.
    // ─────────────────────────────────────────────────────────────────
    #[test]
    fn self_intersecting_spaghetti_split() {
        let state = TopologyState::empty();
        let mut draft = state.begin_mutation();

        let mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();

        let initial_face_count = draft.arena().face_count();
        let initial_vertex_count = draft.arena().vertex_count();

        let iterations = 1000;
        let mut current_edge = mvf.half_edge;
        let mut stack = Vec::with_capacity(iterations);

        // Split 1000 times — always splitting the newly created edge
        for _ in 0..iterations {
            let se = apply_op(&mut draft, SplitEdge { edge: current_edge, parameter: 0.5 }).unwrap().into_value();
            current_edge = se.he_mb;
            stack.push(current_edge);
        }

        assert_eq!(draft.arena().vertex_count(), initial_vertex_count + iterations);

        // Undo all 1000 splits via KEV (LIFO order)
        while let Some(edge_to_kill) = stack.pop() {
            apply_op(&mut draft, KillEdgeVertex { edge: edge_to_kill }).unwrap();
        }

        // Must be back to seed topology
        assert_eq!(draft.arena().face_count(), initial_face_count, "Face count must return to initial");
        assert_eq!(draft.arena().vertex_count(), initial_vertex_count, "Vertex count must return to initial");

        // Commit succeeds
        let final_state = draft.commit().unwrap();
        assert_eq!(final_state.arena().face_count(), initial_face_count);
        assert_eq!(final_state.arena().vertex_count(), initial_vertex_count);
    }

    // ─────────────────────────────────────────────────────────────────
    // 5. The "Ship of Theseus" Rewire (Diff Engine Stress Test)
    //
    // Every pointer on a face is modified without deleting the face.
    // The diff engine MUST report it as Modified (not unchanged).
    // ─────────────────────────────────────────────────────────────────
    #[test]
    fn ship_of_theseus_rewire() {
        let state = TopologyState::empty();
        let mut draft = state.begin_mutation();

        // Build a quad
        let mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
        let se1 = apply_op(&mut draft, SplitEdge { edge: mvf.half_edge, parameter: 0.5 }).unwrap().into_value();
        let se2 = apply_op(&mut draft, SplitEdge { edge: se1.he_mb, parameter: 0.5 }).unwrap().into_value();
        let _se3 = apply_op(&mut draft, SplitEdge { edge: se2.he_mb, parameter: 0.5 }).unwrap().into_value();

        let original_face = mvf.face;
        let state_before = draft.commit().unwrap();

        // Rewire: split every edge on the face (modifies all face pointers)
        let mut draft_mod = state_before.begin_mutation();
        let edges = face_edges(state_before.arena(), original_face).unwrap();

        for e in edges {
            apply_op(&mut draft_mod, SplitEdge { edge: e, parameter: 0.5 }).unwrap();
        }

        let state_after = draft_mod.commit().unwrap();

        let diff = compute_diff(
            state_before.arena(),
            state_after.arena(),
            state_before.epoch(),
            state_after.epoch(),
        );

        // The face must appear as Modified
        let face_modified = diff.faces.iter().any(|delta| {
            matches!(delta, EntityDelta::Modified { index, .. } if *index == original_face.index() as usize)
        });

        assert!(face_modified, "Face MUST be marked as Modified after boundary rewiring");
    }
}
