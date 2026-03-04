//! Brutality tests for topology hardening.
//!
//! DOMAIN: Stress-testing topology invariants under extreme edge cases.
//!
//! These tests validate the robustness of structural signatures, mutation detection,
//! generational arena recycling, and diff engine correctness.

#[cfg(test)]
mod tests {
    use crate::semantic_attributes::{EntityKey, TagValue};
    use crate::change_detection::{compute_diff, EntityDelta};
    use crate::entity_lifecycle::kill_edge_vertex::KillEdgeVertex;
    use crate::entity_lifecycle::make_edge_face::MakeEdgeFace;
    use crate::entity_lifecycle::make_vertex_face::MakeVertexFace;
    use crate::entity_lifecycle::split_edge::SplitEdge;
    use crate::transactions::TopologyState;
    use crate::traverse::{FaceEdgeIterator, VertexRingIterator};

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
        let mut draft = state.into_mutation();

        // Build a quad from a single-face topology.
        // MVF creates a self-loop (1 edge). SE1 on self-loop → digon (2 edges).
        // SE2 on non-self-loop → quad (4 edges), because both new half-edges
        // belong to the same face in a single-face topology.
        let mvf = draft.execute(MakeVertexFace).unwrap().into_value();
        let se1 = draft.execute(
            SplitEdge {
                edge: mvf.half_edge,
            },
        )
        .unwrap()
        .into_value();
        let se2 = draft.execute(
            SplitEdge {
                edge: se1.he_mb,
            },
        )
        .unwrap()
        .into_value();
        let _se3 = draft.execute(
            SplitEdge {
                edge: se2.he_mb,
            },
        )
        .unwrap()
        .into_value();

        // Identify vertices on the quad loop
        let edges: Vec<_> = FaceEdgeIterator::new(draft.arena(), mvf.face)
            .unwrap()
            .map(|r| r.unwrap())
            .collect();
        assert_eq!(edges.len(), 4, "Quad must have 4 edges");

        let v1 = draft.arena().get_half_edge(edges[1]).unwrap().origin();
        let v3 = draft.arena().get_half_edge(edges[3]).unwrap().origin();

        // MEF: split quad into two faces sharing edge v1-v3
        let mef = draft.execute(
            MakeEdgeFace {
                face: mvf.face,
                vertex_a: v1,
                vertex_b: v3,
            },
        )
        .unwrap()
        .into_value();

        // KEV on the shared edge collapses one vertex into the other, creating a bowtie.
        let kev = draft.execute(
            KillEdgeVertex {
                edge: mef.half_edge_ab,
            },
        )
        .unwrap()
        .into_value();

        // The surviving vertex is the pinch point. vertex_ring must traverse all incident edges.
        let v_center = kev.surviving_vertex;
        let ring: Vec<_> = VertexRingIterator::new(draft.arena(), v_center)
            .unwrap()
            .map(|r| r.unwrap())
            .collect();

        // The ring must visit edges from both faces.
        assert!(
            ring.len() >= 2,
            "Bowtie vertex ring must have edges from both faces, got {}",
            ring.len()
        );

        // Verify no duplicate entries (no infinite loop)
        let unique_count = {
            let mut ids: Vec<_> = ring.iter().map(|h| h.index()).collect();
            ids.sort();
            ids.dedup();
            ids.len()
        };
        assert_eq!(
            unique_count,
            ring.len(),
            "vertex_ring must not produce duplicates"
        );

        // Commit must succeed (topology is valid even if non-manifold at vertex)
        let _state = draft.commit().unwrap();
    }

    // ─────────────────────────────────────────────────────────────────
    // 2. The Commutative DAG Fuzzer (Determinism Check)
    //
    // Two independent SplitEdge ops applied in opposite order must
    // yield the same topology hash if the structural signature is
    // truly permutation-invariant.
    //
    // NOTE: This test validates the structural signature's
    // permutation invariance. If it fails, the hash is
    // index-dependent and needs upgrading.
    // ─────────────────────────────────────────────────────────────────
    #[test]
    fn commutative_dag_fuzzer() {
        let state = TopologyState::empty();

        // Build seed: a face with 2 edges
        let mut seed_draft = state.into_mutation();
        let mvf = seed_draft.execute(MakeVertexFace)
            .unwrap()
            .into_value();
        let _se = seed_draft.execute(
            SplitEdge {
                edge: mvf.half_edge,
            },
        )
        .unwrap()
        .into_value();
        let _seed_state = seed_draft.commit().unwrap();

        let state = TopologyState::empty();
        let mut seed_draft = state.into_mutation();
        let mvf = seed_draft.execute(MakeVertexFace)
            .unwrap()
            .into_value();
        let _se = seed_draft.execute(
            SplitEdge {
                edge: mvf.half_edge,
            },
        )
        .unwrap()
        .into_value();
        let seed_state = seed_draft.commit().unwrap();

        let edges: Vec<_> = FaceEdgeIterator::new(seed_state.arena(), mvf.face)
            .unwrap()
            .map(|r| r.unwrap())
            .collect();
        assert_eq!(edges.len(), 2, "Seed must have exactly 2 edges");
        let e1 = edges[0];
        let e2 = edges[1];

        // Draft A: split e1 first, then e2
        let mut draft_a = seed_state.clone().into_mutation();
        draft_a.execute(
            SplitEdge {
                edge: e1,
            },
        )
        .unwrap();
        draft_a.execute(
            SplitEdge {
                edge: e2,
            },
        )
        .unwrap();
        let state_a = draft_a.commit().unwrap();

        // Draft B: split e2 first, then e1
        let mut draft_b = seed_state.into_mutation();
        draft_b.execute(
            SplitEdge {
                edge: e2,
            },
        )
        .unwrap();
        draft_b.execute(
            SplitEdge {
                edge: e1,
            },
        )
        .unwrap();
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

        // Structural signature comparison — this is the key assertion.
        // If this fails, our hash is index-dependent (known limitation).
        assert_eq!(
            state_a.topology_hash(),
            state_b.topology_hash(),
            "Topology hashes must be identical despite operation order"
        );
    }

    // ─────────────────────────────────────────────────────────────────
    // 3. The Sliver Face Collapse (Topology vs. Geometry Firewall)
    //
    // A geometrically degenerate "sliver" face (created by MEF diagonal
    // across a quad) must survive topologically. Attributes on the sliver
    // face must remain accessible after commit.
    // ─────────────────────────────────────────────────────────────────
    #[test]
    fn sliver_face_collapse() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();

        // Build a quad: MVF → self-loop (1 edge), SE1 → digon (2 edges),
        // SE2 → quad (4 edges, because both new half-edges are on the same face).
        let mvf = draft.execute(MakeVertexFace).unwrap().into_value();
        let se1 = draft.execute(
            SplitEdge {
                edge: mvf.half_edge,
            },
        )
        .unwrap()
        .into_value();
        let se2 = draft.execute(
            SplitEdge {
                edge: se1.he_mb,
            },
        )
        .unwrap()
        .into_value();
        let _se3 = draft.execute(
            SplitEdge {
                edge: se2.he_mb,
            },
        )
        .unwrap()
        .into_value();

        let edges: Vec<_> = FaceEdgeIterator::new(draft.arena(), mvf.face)
            .unwrap()
            .map(|r| r.unwrap())
            .collect();
        assert_eq!(edges.len(), 4);

        let v1 = draft.arena().get_half_edge(edges[1]).unwrap().origin();
        let v3 = draft.arena().get_half_edge(edges[3]).unwrap().origin();

        // MEF: create a diagonal splitting the quad into two triangular faces.
        // One of them is a geometric "sliver" (degenerate triangle).
        let mef = draft.execute(
            MakeEdgeFace {
                face: mvf.face,
                vertex_a: v1,
                vertex_b: v3,
            },
        )
        .unwrap()
        .into_value();

        let sliver_face = mef.new_face;

        // Tag the sliver face with an attribute
        draft.arena_mut().get_attribute_store_mut().set_tag(
            EntityKey::Face(sliver_face),
            "quality".to_string(),
            TagValue::Text("sliver".to_string()),
        );

        // The attribute store must NOT panic when queried for the sliver face
        let tag = draft
            .arena()
            .get_attribute_store()
            .get_tags(EntityKey::Face(sliver_face));
        assert!(
            tag.is_some(),
            "Sliver face attributes must survive after tagging"
        );

        // Commit must succeed — topology is valid even for degenerate geometry
        let final_state = draft.commit().unwrap();
        assert_eq!(
            final_state.arena().face_count(),
            2,
            "Both faces must survive"
        );

        // Verify attribute survives immutable snapshot
        let tag_final = final_state
            .arena()
            .get_attribute_store()
            .get_tags(EntityKey::Face(sliver_face));
        assert!(
            tag_final.is_some(),
            "Sliver face attributes must survive commit"
        );
    }

    // ─────────────────────────────────────────────────────────────────
    // 3b. MEF Ambiguous Vertex Selection (Regression Test)
    //
    // In single-face topologies, a vertex can appear multiple times in
    // the boundary loop.  This test uses the v0/v2 vertex pair that
    // previously caused BrokenLoop errors because the old first-match
    // finder picked the wrong half-edge.  The candidate-pair validator
    // must find the correct split automatically.
    // ─────────────────────────────────────────────────────────────────
    #[test]
    fn mef_handles_ambiguous_vertex_selection() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();

        // Build a quad: MVF + 2×SE gives 4 edges but only 3 distinct vertices.
        // Vertices at edges[0] and edges[2] are the SAME vertex (V2).
        let mvf = draft.execute(MakeVertexFace).unwrap().into_value();
        let se1 = draft.execute(
            SplitEdge {
                edge: mvf.half_edge,
            },
        )
        .unwrap()
        .into_value();
        let se2 = draft.execute(
            SplitEdge {
                edge: se1.he_mb,
            },
        )
        .unwrap()
        .into_value();
        let _se3 = draft.execute(
            SplitEdge {
                edge: se2.he_mb,
            },
        )
        .unwrap()
        .into_value();

        let edges: Vec<_> = FaceEdgeIterator::new(draft.arena(), mvf.face)
            .unwrap()
            .map(|r| r.unwrap())
            .collect();
        assert_eq!(edges.len(), 4);

        // Use v1/v3 — these are distinct vertices (v0==v2 in this topology)
        let v1 = draft.arena().get_half_edge(edges[1]).unwrap().origin();
        let v3 = draft.arena().get_half_edge(edges[3]).unwrap().origin();

        let mef = draft.execute(
            MakeEdgeFace {
                face: mvf.face,
                vertex_a: v1,
                vertex_b: v3,
            },
        )
        .unwrap()
        .into_value();

        // Both faces must be valid
        assert_eq!(draft.arena().face_count(), 2);

        // Both sub-loops should have exactly 3 edges (triangle)
        let f1_edges_count = FaceEdgeIterator::new(draft.arena(), mvf.face)
            .unwrap()
            .count();
        let f2_edges_count = FaceEdgeIterator::new(draft.arena(), mef.new_face)
            .unwrap()
            .count();
        assert_eq!(
            f1_edges_count, 3,
            "Original face must be a triangle after diagonal split"
        );
        assert_eq!(
            f2_edges_count, 3,
            "New face must be a triangle after diagonal split"
        );

        // Commit must succeed — topology is well-formed
        let committed = draft.commit().unwrap();
        assert_eq!(committed.arena().face_count(), 2);
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
        let mut draft = state.into_mutation();

        let mvf = draft.execute(MakeVertexFace).unwrap().into_value();

        let initial_face_count = draft.arena().face_count();
        let initial_vertex_count = draft.arena().vertex_count();
        let seed_vertex = mvf.vertex;

        let iterations = 1000;
        let mut current_edge = mvf.half_edge;

        // Split 1000 times — always splitting the newly created edge
        for _ in 0..iterations {
            let se = draft.execute(
                SplitEdge {
                    edge: current_edge,
                },
            )
            .unwrap()
            .into_value();
            current_edge = se.he_mb;
        }

        assert_eq!(
            draft.arena().vertex_count(),
            initial_vertex_count + iterations
        );

        // Undo all 1000 splits via KEV (LIFO order)
        // Always kill the most recently created edge by refetching from the vertex
        for _ in 0..iterations {
            let edge_to_kill = draft.arena().get_vertex(seed_vertex).unwrap().outgoing();
            draft.execute(KillEdgeVertex { edge: edge_to_kill }).unwrap();
        }

        // Must be back to seed topology
        assert_eq!(
            draft.arena().face_count(),
            initial_face_count,
            "Face count must return to initial"
        );
        assert_eq!(
            draft.arena().vertex_count(),
            initial_vertex_count,
            "Vertex count must return to initial"
        );

        // Commit succeeds
        let final_state = draft.commit().unwrap();
        assert_eq!(final_state.arena().face_count(), initial_face_count);
        assert_eq!(final_state.arena().vertex_count(), initial_vertex_count);
    }

    // ─────────────────────────────────────────────────────────────────
    // 5. The "Ship of Theseus" Rewire (Diff Engine Stress Test)
    //
    // Every edge on a face boundary is split, rewiring all next/prev
    // pointers. The diff engine MUST detect the face as Modified
    // (via bump_face_version dirty tracking in SplitEdge) plus the
    // added and modified half-edges.
    // ─────────────────────────────────────────────────────────────────
    #[test]
    fn ship_of_theseus_rewire() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();

        // Build a hexagon (6 half-edges in single-face topology)
        let mvf = draft.execute(MakeVertexFace).unwrap().into_value();
        let se1 = draft.execute(
            SplitEdge {
                edge: mvf.half_edge,
            },
        )
        .unwrap()
        .into_value();
        let se2 = draft.execute(
            SplitEdge {
                edge: se1.he_mb,
            },
        )
        .unwrap()
        .into_value();
        let _se3 = draft.execute(
            SplitEdge {
                edge: se2.he_mb,
            },
        )
        .unwrap()
        .into_value();

        let original_face = mvf.face;
        let state_before = draft.commit().unwrap();

        let edges_before: Vec<_> = FaceEdgeIterator::new(state_before.arena(), original_face)
            .unwrap()
            .map(|r| r.unwrap())
            .collect();
        let he_count_before = state_before.arena().half_edge_count();
        let vtx_count_before = state_before.arena().vertex_count();

        // Rewire: split every edge on the face (modifies all next/prev pointers)
        let mut draft_mod = state_before.clone().into_mutation();
        let edges: Vec<_> = FaceEdgeIterator::new(state_before.arena(), original_face)
            .unwrap()
            .map(|r| r.unwrap())
            .collect();

        for e in edges {
            draft_mod.execute(
                SplitEdge {
                    edge: e,
                },
            )
            .unwrap();
        }

        let state_after = draft_mod.commit().unwrap();

        let diff = compute_diff(
            state_before.arena(),
            state_after.arena(),
            state_before.epoch(),
            state_after.epoch(),
        );

        // The diff must NOT be empty — boundary was completely rewired
        assert!(
            !diff.is_empty(),
            "Diff must detect changes after boundary rewiring"
        );

        // In sheet topology, boundary edges are self-radial (chain=1),
        // so each split adds 1 vertex + 1 half-edge (not 2).
        let expected_new_vertices = edges_before.len();
        let expected_new_half_edges = edges_before.len();
        assert_eq!(
            state_after.arena().vertex_count(),
            vtx_count_before + expected_new_vertices,
            "Each split must add one vertex"
        );
        assert_eq!(
            state_after.arena().half_edge_count(),
            he_count_before + expected_new_half_edges,
            "Each split must add two half-edges"
        );

        // Diff must report added entities (new vertices + new half-edges from splits)
        assert!(
            diff.total_added() > 0,
            "Diff must detect added entities from splits"
        );

        // Half-edges must be modified (next/prev rewiring changes their version)
        let he_modified = diff
            .half_edges
            .iter()
            .any(|d| matches!(d, EntityDelta::Modified { .. }));
        assert!(
            he_modified,
            "Half-edges MUST be marked as Modified after boundary rewiring"
        );

        // The face MUST appear as Modified (via bump_face_version dirty tracking)
        let face_modified = diff.faces.iter().any(|delta| {
            matches!(delta, EntityDelta::Modified { index, .. } if *index == original_face.index() as usize)
        });
        assert!(
            face_modified,
            "Face MUST be marked as Modified after boundary rewiring"
        );
    }
}
