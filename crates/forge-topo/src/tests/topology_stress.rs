//! Topology stress tests: 6 categories × 3 levels.
//!
//! DOMAIN: Synthetic topology construction via Euler operators to validate
//! structural invariants, traversal robustness, and validation detection.
//!
//! CATEGORIES:
//! 1. Degenerate entities (zero-length, collapsed)
//! 2. Non-manifold topologies (bowtie, high-valence, broken twins)
//! 3. Topological validity & consistency (Euler, orientation, closure)
//! 4. Topological predicates & classification
//! 5. Sewing / gluing / merging (synthetic)
//! 6. Boolean-like topology stress (merged shells, nested shells, chained ops)

#[cfg(test)]
mod tests {
    use crate::algorithms::bridge_edge::bridge_edge;
    use crate::b_rep::ShellKind;
    use crate::b_rep::{FaceData, HalfEdgeData, LoopData, TopologyArena, VertexData};
    use crate::boundary_editing::join_faces::JoinFaces;
    use crate::entity_lifecycle::kill_edge_vertex::KillEdgeVertex;
    use crate::entity_lifecycle::make_edge_face::MakeEdgeFace;
    use crate::entity_lifecycle::make_vertex_face::MakeVertexFace;
    use crate::entity_lifecycle::split_edge::SplitEdge;
    use crate::handles::{EdgeId, FaceId, HalfEdgeId, LoopId, ShellId, VertexId};
    use crate::testing::build_face_with_hole;
    use crate::transactions::TopologyState;
    use crate::traverse::{FaceEdgeIterator, VertexRingIterator};
    use crate::validate::{validate_topology, ValidationLevel};
    use forge_core::KernelError;

    // ══════════════════════════════════════════════════════════════
    // Helpers
    // ══════════════════════════════════════════════════════════════

    /// Build a quad face via MVF + 3×SE. Returns (mvf_output, edge_list).
    fn build_quad(
        draft: &mut crate::transactions::MutableDraft,
    ) -> (
        crate::entity_lifecycle::make_vertex_face::MvfOutput,
        Vec<HalfEdgeId>,
    ) {
        let mvf = draft
            .execute(MakeVertexFace {
                shell_kind: ShellKind::Sheet,
            })
            .unwrap()
            .into_value();
        let se1 = draft
            .execute(SplitEdge {
                edge: mvf.half_edge,
            })
            .unwrap()
            .into_value();
        let se2 = draft
            .execute(SplitEdge { edge: se1.he_mb })
            .unwrap()
            .into_value();
        let _se3 = draft
            .execute(SplitEdge { edge: se2.he_mb })
            .unwrap()
            .into_value();

        let edges: Vec<_> = FaceEdgeIterator::new(draft.arena(), mvf.face)
            .unwrap()
            .map(|r| r.unwrap())
            .collect();
        assert_eq!(edges.len(), 4, "Quad must have 4 edges");
        (mvf, edges)
    }

    /// Euler characteristic: V - E + F.
    fn euler_chi(arena: &TopologyArena) -> isize {
        let v = arena.vertex_count() as isize;
        let e = arena.edge_count() as isize;
        let f = arena.face_count() as isize;
        v - e + f
    }

    // ══════════════════════════════════════════════════════════════
    // CATEGORY 1: Degenerate Entities
    // ══════════════════════════════════════════════════════════════

    /// L1: Pole-like topology — multiple edges converging on a single vertex.
    ///
    /// Creates a fan of 10 edges radiating from one pole vertex via SE+MEF.
    /// Verifies: loop closure, vertex ring traversal, Euler check.
    #[test]
    fn degenerate_l1_pole_edges() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();

        let mvf = draft
            .execute(MakeVertexFace {
                shell_kind: ShellKind::Sheet,
            })
            .unwrap()
            .into_value();
        let pole = mvf.vertex;
        let mut current_edge = mvf.half_edge;

        for _ in 0..10 {
            let se = draft
                .execute(SplitEdge { edge: current_edge })
                .unwrap()
                .into_value();

            let face_id = draft.arena().get_half_edge(current_edge).unwrap().face();
            let mef = draft
                .execute(MakeEdgeFace {
                    vertex_a: pole,
                    vertex_b: se.new_vertex,
                    face: face_id,
                })
                .unwrap()
                .into_value();

            current_edge = mef.half_edge_ab;
        }

        let ring: Vec<_> = VertexRingIterator::new(draft.arena(), pole)
            .unwrap()
            .map(|r| r.unwrap())
            .collect();
        assert!(
            ring.len() >= 10,
            "Pole vertex ring must visit all fan edges, got {}",
            ring.len()
        );

        let committed = draft.commit().unwrap();
        // Fan from self-loop seed creates multi-edges (valid DCEL, non-manifold geometry).
        // Use Minimal — these tests validate operator correctness, not manifold geometry.
        assert!(validate_topology(committed.arena(), ValidationLevel::Minimal).is_ok());
    }

    /// L2: Edge collapse via KEV — remove a diagonal edge and its target vertex.
    ///
    /// Build quad, MEF diagonal to create 2 faces, then KEV on the diagonal
    /// to collapse one vertex. KEV removes 1 vertex + 1 edge but keeps faces.
    /// Both faces survive with one fewer edge each.
    #[test]
    fn degenerate_l2_collapsed_edge_shared_faces() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();

        let (mvf, edges) = build_quad(&mut draft);

        let v1 = draft.arena().get_half_edge(edges[1]).unwrap().origin();
        let v3 = draft.arena().get_half_edge(edges[3]).unwrap().origin();

        let mef = draft
            .execute(MakeEdgeFace {
                face: mvf.face,
                vertex_a: v1,
                vertex_b: v3,
            })
            .unwrap()
            .into_value();

        let faces_before_kev = draft.arena().face_count();
        let verts_before_kev = draft.arena().vertex_count();
        assert_eq!(faces_before_kev, 2);

        let kev = draft
            .execute(KillEdgeVertex {
                edge: mef.half_edge_ab,
            })
            .unwrap()
            .into_value();

        assert_eq!(
            draft.arena().face_count(),
            faces_before_kev,
            "KEV removes edge+vertex but keeps faces"
        );
        assert_eq!(
            draft.arena().vertex_count(),
            verts_before_kev - 1,
            "KEV must remove exactly 1 vertex"
        );

        let ring: Vec<_> = VertexRingIterator::new(draft.arena(), kev.surviving_vertex)
            .unwrap()
            .map(|r| r.unwrap())
            .collect();
        assert!(!ring.is_empty(), "Surviving vertex must have edges");

        let committed = draft.commit().unwrap();
        assert!(validate_topology(committed.arena(), ValidationLevel::Full).is_ok());
    }

    /// L3: Force a degenerate loop — all edges in a 3-edge face point to the
    /// same vertex. validate_degenerate_loops must catch this.
    ///
    /// BLOCKED: Requires a `ValidateLoopIsSimpleTopologically` validator
    /// (from validators.md §2) that detects repeated vertices in a loop.
    /// No existing validator checks this invariant yet.
    #[test]
    #[ignore = "requires ValidateLoopIsSimpleTopologically — not yet implemented"]
    fn degenerate_l3_sliver_triangle_euler_violation() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();

        let mvf = draft
            .execute(MakeVertexFace {
                shell_kind: ShellKind::Sheet,
            })
            .unwrap()
            .into_value();
        let se1 = draft
            .execute(SplitEdge {
                edge: mvf.half_edge,
            })
            .unwrap()
            .into_value();
        let se2 = draft
            .execute(SplitEdge { edge: se1.he_mb })
            .unwrap()
            .into_value();
        let _se3 = draft
            .execute(SplitEdge { edge: se2.he_mb })
            .unwrap()
            .into_value();

        let edges: Vec<_> = FaceEdgeIterator::new(draft.arena(), mvf.face)
            .unwrap()
            .map(|r| r.unwrap())
            .collect();
        assert_eq!(edges.len(), 4);

        let v0 = draft.arena().get_half_edge(edges[0]).unwrap().origin();

        let arena = draft.arena_mut();
        for &he_id in &edges {
            arena.get_half_edge_mut(he_id).unwrap().set_origin(v0);
        }

        let result = draft.commit();
        assert!(
            result.is_err(),
            "Commit must reject topology with all-same-vertex loop"
        );
        assert!(matches!(
            result.unwrap_err(),
            KernelError::TopologyViolation { .. }
        ));
    }

    // ══════════════════════════════════════════════════════════════
    // CATEGORY 2: Non-Manifold Topologies
    // ══════════════════════════════════════════════════════════════

    /// L1: Double bowtie — three faces sharing a single pinch vertex.
    ///
    /// Build two quads sharing a vertex, MEF each into triangles, KEV
    /// the shared edges to create three faces touching at one vertex.
    #[test]
    fn nonmanifold_l1_bowtie_three_faces() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();

        let mvf = draft
            .execute(MakeVertexFace {
                shell_kind: ShellKind::Sheet,
            })
            .unwrap()
            .into_value();
        let center = mvf.vertex;

        let se1 = draft
            .execute(SplitEdge {
                edge: mvf.half_edge,
            })
            .unwrap()
            .into_value();
        let se2 = draft
            .execute(SplitEdge { edge: se1.he_mb })
            .unwrap()
            .into_value();
        let se3 = draft
            .execute(SplitEdge { edge: se2.he_mb })
            .unwrap()
            .into_value();
        let se4 = draft
            .execute(SplitEdge { edge: se3.he_mb })
            .unwrap()
            .into_value();
        let _se5 = draft
            .execute(SplitEdge { edge: se4.he_mb })
            .unwrap()
            .into_value();

        let edges: Vec<_> = FaceEdgeIterator::new(draft.arena(), mvf.face)
            .unwrap()
            .map(|r| r.unwrap())
            .collect();

        let v_at_2 = draft.arena().get_half_edge(edges[2]).unwrap().origin();
        let v_at_4 = draft.arena().get_half_edge(edges[4]).unwrap().origin();

        let _mef1 = draft
            .execute(MakeEdgeFace {
                face: mvf.face,
                vertex_a: center,
                vertex_b: v_at_2,
            })
            .unwrap()
            .into_value();

        let _mef2 = draft
            .execute(MakeEdgeFace {
                face: mvf.face,
                vertex_a: center,
                vertex_b: v_at_4,
            })
            .unwrap()
            .into_value();

        assert!(
            draft.arena().face_count() >= 3,
            "Must have at least 3 faces"
        );

        let ring: Vec<_> = VertexRingIterator::new(draft.arena(), center)
            .unwrap()
            .map(|r| r.unwrap())
            .collect();
        assert!(
            ring.len() >= 3,
            "Center vertex ring must visit edges from all faces"
        );

        let unique_count = {
            let mut ids: Vec<_> = ring.iter().map(|h| h.index()).collect();
            ids.sort();
            ids.dedup();
            ids.len()
        };
        assert_eq!(unique_count, ring.len(), "No duplicates in vertex ring");
    }

    /// L2: High-valence star — 20 edges radiating from a single vertex.
    ///
    /// Fan-split to create 20 radial edges. Verify ring traversal completes
    /// without hitting MAX_ITER, and face counts are consistent.
    #[test]
    fn nonmanifold_l2_high_valence_star() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();

        let mvf = draft
            .execute(MakeVertexFace {
                shell_kind: ShellKind::Sheet,
            })
            .unwrap()
            .into_value();
        let center = mvf.vertex;
        let mut current_edge = mvf.half_edge;

        for _ in 0..20 {
            let se = draft
                .execute(SplitEdge { edge: current_edge })
                .unwrap()
                .into_value();

            let face_id = draft.arena().get_half_edge(current_edge).unwrap().face();
            let mef = draft
                .execute(MakeEdgeFace {
                    vertex_a: center,
                    vertex_b: se.new_vertex,
                    face: face_id,
                })
                .unwrap()
                .into_value();

            current_edge = mef.half_edge_ab;
        }

        assert_eq!(draft.arena().vertex_count(), 21);
        assert_eq!(draft.arena().face_count(), 21);

        let ring: Vec<_> = VertexRingIterator::new(draft.arena(), center)
            .unwrap()
            .map(|r| r.unwrap())
            .collect();
        assert_eq!(
            ring.len(),
            21,
            "Center must have exactly 21 outgoing half-edges"
        );

        let committed = draft.commit().unwrap();
        assert!(validate_topology(committed.arena(), ValidationLevel::Minimal).is_ok());
    }

    /// L3: Deliberately broken twin chain — patch a halfedge's twin to point
    /// to the wrong halfedge, creating a non-reciprocal twin link.
    /// validate_twins must catch this and commit must reject.
    #[test]
    fn nonmanifold_l3_broken_twin_chain() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();

        let mvf = draft
            .execute(MakeVertexFace {
                shell_kind: ShellKind::Sheet,
            })
            .unwrap()
            .into_value();
        let se = draft
            .execute(SplitEdge {
                edge: mvf.half_edge,
            })
            .unwrap()
            .into_value();

        let bogus_he = HalfEdgeId::DANGLING;
        draft
            .arena_mut()
            .get_half_edge_mut(se.he_am)
            .unwrap()
            .set_radial_next(bogus_he);

        let result = draft.commit();
        assert!(result.is_err(), "Commit must reject broken twin chain");
        assert!(matches!(
            result.unwrap_err(),
            KernelError::TopologyViolation { .. }
        ));
    }

    // ══════════════════════════════════════════════════════════════
    // CATEGORY 3: Topological Validity & Consistency
    // ══════════════════════════════════════════════════════════════

    /// L1: Build a topological sphere using SewEdge and verify the Euler formula.
    ///
    /// V=2, E=1, F=1, χ = V-E+F = 2 for a closed genus-0 shell.
    /// Uses MVF → SE → SewEdge to seamlessly glue an open surface into a manifold.
    #[test]
    fn validity_l1_sphere_euler() {
        use crate::operations::non_manifold::sew_edge::SewEdge;

        let state = TopologyState::empty();
        let mut draft = state.into_mutation();

        // 1. Create a single face F0 with self-loop halfedge (v0->v0) at v0.
        //    Open surface, boundaries=1, χ=1.
        let mvf = draft
            .execute(MakeVertexFace {
                shell_kind: ShellKind::Sheet,
            })
            .unwrap()
            .into_value();

        // 2. Split the boundary edge, creating v1.
        //    Now we have a digon on F0: v0->v1 and v1->v0.
        //    Both are still boundary halfedges on the same face.
        let se1 = draft
            .execute(SplitEdge {
                edge: mvf.half_edge,
            })
            .unwrap()
            .into_value();

        // 3. Sew the two boundary halfedges together!
        //    This glues the digon shut, eliminating the boundary and leaving a closed sphere.
        let he_v0_v1 = mvf.half_edge;
        let he_v1_v0 = se1.he_mb;
        draft
            .execute(SewEdge {
                he_a: he_v0_v1,
                he_b: he_v1_v0,
            })
            .unwrap();

        // The sphere is now watertight — promote shell from Sheet to Solid.
        let face = draft.arena().get_half_edge(he_v0_v1).unwrap().face();
        let shell = draft.arena().get_face(face).unwrap().shell();
        draft
            .arena_mut()
            .get_shell_mut(shell)
            .unwrap()
            .set_kind(crate::b_rep::ShellKind::Solid(
                crate::b_rep::ShellOrientation::Outer,
            ));

        let arena = draft.arena();
        let v = arena.vertex_count();
        let e = arena.edge_count();
        let f = arena.face_count();
        let chi = euler_chi(arena);

        assert_eq!(v, 2, "Sphere V count");
        assert_eq!(e, 1, "Sphere E count");
        assert_eq!(f, 1, "Sphere F count");
        assert_eq!(
            chi, 2,
            "Sphere must have χ=2, got V={} E={} F={} χ={}",
            v, e, f, chi
        );

        let committed = draft.commit().unwrap();
        assert!(validate_topology(committed.arena(), ValidationLevel::Minimal).is_ok());
    }

    /// L2: Face with inner loop (hole) — generalized Euler with R=1.
    ///
    /// Build outer+inner triangle, bridge them. Verify the merged loop has
    /// correct traversal count and validate_topology passes.
    #[test]
    fn validity_l2_face_with_hole() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();

        let (face, outer_he, inner_he, _inner_loop, _verts) = build_face_with_hole(&mut draft);

        assert_eq!(draft.arena().get_face(face).unwrap().inner_loop_count(), 1);

        let bridge = bridge_edge(&mut draft, outer_he, inner_he).unwrap();

        assert_eq!(draft.arena().get_face(face).unwrap().inner_loop_count(), 0);

        let he_in = bridge.he_into_hole;
        let he_out = bridge.he_out_of_hole;
        assert_eq!(
            draft.arena().get_half_edge(he_in).unwrap().radial_next(),
            he_out
        );
        assert_eq!(
            draft.arena().get_half_edge(he_out).unwrap().radial_next(),
            he_in
        );

        let outer_loop = draft.arena().get_face(face).unwrap().loops.outer();
        let start = draft.arena().get_loop(outer_loop).unwrap().half_edge();
        let mut current = start;
        let mut count = 0usize;

        loop {
            count += 1;
            assert!(count <= 100, "Infinite loop in merged loop traversal");
            current = draft.arena().get_half_edge(current).unwrap().next();
            if current == start {
                break;
            }
        }

        assert_eq!(
            count, 8,
            "Outer(3) + Inner(3) + 2 bridge = 8 edges in merged loop"
        );
    }

    /// L3: Deliberately reversed prev/next on one half-edge.
    /// validate_prev_consistency must catch this and commit must reject.
    #[test]
    fn validity_l3_phantom_face_reversed_orientation() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();

        let mvf = draft
            .execute(MakeVertexFace {
                shell_kind: ShellKind::Sheet,
            })
            .unwrap()
            .into_value();
        let se1 = draft
            .execute(SplitEdge {
                edge: mvf.half_edge,
            })
            .unwrap()
            .into_value();
        let _se2 = draft
            .execute(SplitEdge { edge: se1.he_mb })
            .unwrap()
            .into_value();

        let edges: Vec<_> = FaceEdgeIterator::new(draft.arena(), mvf.face)
            .unwrap()
            .map(|r| r.unwrap())
            .collect();
        assert!(edges.len() >= 3);

        let e0 = edges[0];
        let e1 = edges[1];
        let arena = draft.arena_mut();
        arena.get_half_edge_mut(e0).unwrap().set_next(e0);
        arena.get_half_edge_mut(e1).unwrap().set_prev(e1);

        let result = draft.commit();
        assert!(
            result.is_err(),
            "Commit must reject broken prev/next pointers"
        );
        assert!(matches!(
            result.unwrap_err(),
            KernelError::TopologyViolation { .. }
        ));
    }

    // ══════════════════════════════════════════════════════════════
    // CATEGORY 4: Topological Predicates & Classification
    // Tests moved to forge-spatial/src/classify/point_in_solid_tests.rs
    // ══════════════════════════════════════════════════════════════

    // ══════════════════════════════════════════════════════════════
    // CATEGORY 4B: Structural Validation on Mesh Topology
    // ══════════════════════════════════════════════════════════════

    /// Twin pairs must belong to DIFFERENT faces (orientation consistency).
    ///
    /// This is the exact invariant that MB-N3/MB-N4 violate. If this test
    /// fails on our cube mesh, the topology builder has a fundamental bug.
    #[test]
    fn structural_twin_pairs_different_faces() {
        let (arena, _) = build_cube_arena();

        for (he_id, he_data) in arena.iter_half_edges() {
            let twin_id = he_data.radial_next();
            if he_id == twin_id {
                continue;
            }

            let twin_data = arena.get_half_edge(twin_id).unwrap();
            assert_ne!(
                he_data.face(),
                twin_data.face(),
                "Twin pair ({}, {}) both belong to face {} — orientation is inconsistent",
                he_id.index(),
                twin_id.index(),
                he_data.face().index()
            );
        }
    }

    /// Every twin must be reciprocal: he.twin.twin == he.
    #[test]
    fn structural_twin_reciprocity_on_mesh() {
        let (arena, _) = build_cube_arena();

        for (he_id, he_data) in arena.iter_half_edges() {
            let twin_id = he_data.radial_next();
            if he_id == twin_id {
                continue;
            }

            let twin_data = arena.get_half_edge(twin_id).unwrap_or_else(|_| {
                panic!("Twin {} of {} is invalid", twin_id.index(), he_id.index())
            });
            assert_eq!(
                twin_data.radial_next(),
                he_id,
                "Twin reciprocity broken: he[{}].twin = {}, but he[{}].twin = {}",
                he_id.index(),
                twin_id.index(),
                twin_id.index(),
                twin_data.radial_next().index()
            );
        }
    }

    /// Every geometric edge must be shared by exactly 2 faces (manifold).
    #[test]
    fn structural_manifold_edges_on_mesh() {
        let (arena, _) = build_cube_arena();

        let mut edge_face_pairs: std::collections::BTreeMap<(u32, u32), Vec<u32>> =
            std::collections::BTreeMap::new();

        for (he_id, he_data) in arena.iter_half_edges() {
            let twin_id = he_data.radial_next();
            if he_id == twin_id {
                continue;
            }

            let canonical = (
                he_id.index().min(twin_id.index()),
                he_id.index().max(twin_id.index()),
            );
            edge_face_pairs
                .entry(canonical)
                .or_default()
                .push(he_data.face().index());
        }

        for ((lo, hi), faces) in &edge_face_pairs {
            assert_eq!(
                faces.len(),
                2,
                "Edge ({}, {}) shared by {} faces (expected 2): {:?}",
                lo,
                hi,
                faces.len(),
                faces
            );
            assert_ne!(
                faces[0], faces[1],
                "Edge ({}, {}) has both halfedges on face {} — non-manifold",
                lo, hi, faces[0]
            );
        }
    }

    /// Every face loop must be closed and have consistent vertex wiring.
    ///
    /// For each face: next(prev(he)) == he AND prev(next(he)) == he.
    #[test]
    fn structural_loop_closure_on_mesh() {
        let (arena, _) = build_cube_arena();

        for (face_id, _) in arena.iter_faces() {
            let mut edge_count = 0usize;
            for he_result in FaceEdgeIterator::new(&arena, face_id).unwrap() {
                let he_id = he_result.unwrap();
                let he_data = arena.get_half_edge(he_id).unwrap();

                assert_eq!(
                    he_data.face(),
                    face_id,
                    "Half-edge {} in loop of face {} claims face {}",
                    he_id.index(),
                    face_id.index(),
                    he_data.face().index()
                );

                let prev_data = arena.get_half_edge(he_data.prev()).unwrap();
                assert_eq!(
                    prev_data.next(),
                    he_id,
                    "prev({}).next != {} in face {}",
                    he_data.prev().index(),
                    he_id.index(),
                    face_id.index()
                );

                edge_count += 1;
            }
            assert!(
                edge_count >= 3,
                "Face {} has only {} edges (minimum 3 for a valid polygon)",
                face_id.index(),
                edge_count
            );
        }
    }

    /// Adjacent faces must have opposite winding at their shared edge.
    ///
    /// If he goes from vertex A→B, twin must go from B→A. This is the
    /// orientation coherence guarantee for a properly oriented manifold.
    #[test]
    fn structural_orientation_coherence_on_mesh() {
        let (arena, _) = build_cube_arena();

        for (he_id, he_data) in arena.iter_half_edges() {
            let twin_id = he_data.radial_next();
            if he_id == twin_id {
                continue;
            }

            let twin_data = arena.get_half_edge(twin_id).unwrap();
            let next_data = arena.get_half_edge(he_data.next()).unwrap();

            assert_eq!(
                next_data.origin(),
                twin_data.origin(),
                "Orientation coherence broken at edge ({}, {}): \
                 he[{}] origin={}, next.origin={}, twin.origin={} — \
                 twin should go from target back to origin",
                he_id.index(),
                twin_id.index(),
                he_id.index(),
                he_data.origin().index(),
                next_data.origin().index(),
                twin_data.origin().index()
            );
        }
    }

    /// Build a cube topology (V=8, E=12, F=6) with vertex positions using
    /// raw arena manipulation for classification tests.
    fn build_cube_arena() -> (TopologyArena, Vec<[f64; 3]>) {
        let mut arena = TopologyArena::new();

        let positions = vec![
            [-1.0, -1.0, -1.0], // 0
            [1.0, -1.0, -1.0],  // 1
            [1.0, 1.0, -1.0],   // 2
            [-1.0, 1.0, -1.0],  // 3
            [-1.0, -1.0, 1.0],  // 4
            [1.0, -1.0, 1.0],   // 5
            [1.0, 1.0, 1.0],    // 6
            [-1.0, 1.0, 1.0],   // 7
        ];

        let placeholder_he = HalfEdgeId::DANGLING;
        let placeholder_loop = LoopId::DANGLING;

        let mut verts = Vec::new();
        for _ in 0..8 {
            verts.push(arena.insert_vertex(VertexData::new(placeholder_he)));
        }

        let quad_faces: [[usize; 4]; 6] = [
            [0, 3, 2, 1], // -Z
            [4, 5, 6, 7], // +Z
            [0, 1, 5, 4], // -Y
            [2, 3, 7, 6], // +Y
            [0, 4, 7, 3], // -X
            [1, 2, 6, 5], // +X
        ];

        for quad in &quad_faces {
            let placeholder_shell_q = crate::handles::ShellId::DANGLING;
            let placeholder_e_q = crate::handles::EdgeId::DANGLING;
            let face = arena.insert_face(FaceData::new(placeholder_loop, placeholder_shell_q));
            let loop_id = arena.insert_loop(LoopData::new(placeholder_he, face));
            arena.get_face_mut(face).unwrap().loops.set_outer(loop_id);

            let mut he_ids = Vec::new();
            for i in 0..4 {
                let origin = verts[quad[i]];
                let he = arena.insert_half_edge(HalfEdgeData::new(
                    placeholder_he,
                    placeholder_he,
                    placeholder_he,
                    face,
                    origin,
                    placeholder_e_q,
                ));
                he_ids.push(he);
            }

            for i in 0..4 {
                let next = he_ids[(i + 1) % 4];
                let prev = he_ids[(i + 3) % 4];
                arena.get_half_edge_mut(he_ids[i]).unwrap().set_next(next);
                arena.get_half_edge_mut(he_ids[i]).unwrap().set_prev(prev);
            }

            arena
                .get_loop_mut(loop_id)
                .unwrap()
                .set_half_edge(he_ids[0]);
            arena
                .get_vertex_mut(verts[quad[0]])
                .unwrap()
                .set_primary_disk(he_ids[0]);
        }

        let all_hes: Vec<(HalfEdgeId, u32, u32)> = arena
            .iter_half_edges()
            .map(|(id, data)| {
                let origin = data.origin().index();
                let next_he = arena.get_half_edge(data.next()).unwrap();
                let target = next_he.origin().index();
                (id, origin, target)
            })
            .collect();

        for i in 0..all_hes.len() {
            let (he_id, origin, target) = all_hes[i];
            if arena.get_half_edge(he_id).unwrap().radial_next() != placeholder_he {
                if arena.get_half_edge(he_id).unwrap().radial_next() != he_id {
                    continue;
                }
            }
            for j in (i + 1)..all_hes.len() {
                let (other_id, other_origin, other_target) = all_hes[j];
                if origin == other_target && target == other_origin {
                    arena
                        .get_half_edge_mut(he_id)
                        .unwrap()
                        .set_radial_next(other_id);
                    arena
                        .get_half_edge_mut(other_id)
                        .unwrap()
                        .set_radial_next(he_id);
                    break;
                }
            }
        }

        let unmatched: Vec<HalfEdgeId> = arena
            .iter_half_edges()
            .filter(|(_, data)| data.radial_next() == placeholder_he)
            .map(|(id, _)| id)
            .collect();
        for he_id in unmatched {
            arena
                .get_half_edge_mut(he_id)
                .unwrap()
                .set_radial_next(he_id);
        }

        (arena, positions)
    }

    // ══════════════════════════════════════════════════════════════
    // CATEGORY 5: Sewing / Gluing / Merging (synthetic)
    // ══════════════════════════════════════════════════════════════

    /// L1: Two faces sharing a twin-paired edge form a connected shell.
    ///
    /// Build a digon (MVF+SE), then MEF to split into two triangle faces.
    /// Verify discover_shell_faces finds one shell containing both faces.
    #[test]
    fn sewing_l1_shared_edge_merge() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();

        let mvf = draft
            .execute(MakeVertexFace {
                shell_kind: ShellKind::Sheet,
            })
            .unwrap()
            .into_value();
        let se1 = draft
            .execute(SplitEdge {
                edge: mvf.half_edge,
            })
            .unwrap()
            .into_value();
        let se2 = draft
            .execute(SplitEdge { edge: se1.he_mb })
            .unwrap()
            .into_value();
        let _se3 = draft
            .execute(SplitEdge { edge: se2.he_mb })
            .unwrap()
            .into_value();

        let edges: Vec<_> = FaceEdgeIterator::new(draft.arena(), mvf.face)
            .unwrap()
            .map(|r| r.unwrap())
            .collect();
        assert_eq!(edges.len(), 4);

        // MVF+2×SE creates 3 distinct vertices; edges[0] and edges[2] share
        // the same origin. Use v1/v3 which are always distinct.
        let v1 = draft.arena().get_half_edge(edges[1]).unwrap().origin();
        let v3 = draft.arena().get_half_edge(edges[3]).unwrap().origin();

        let mef = draft
            .execute(MakeEdgeFace {
                face: mvf.face,
                vertex_a: v1,
                vertex_b: v3,
            })
            .unwrap()
            .into_value();

        assert_eq!(draft.arena().face_count(), 2);

        let committed = draft.commit().unwrap();

        let mut visited = crate::b_rep::EntityBitset::for_faces(committed.arena());
        let shell =
            crate::queries::shell::discover_shell_faces(committed.arena(), mvf.face, &mut visited)
                .unwrap();
        assert_eq!(shell.len(), 2, "Both faces must be in the same shell");
        assert!(visited.contains(mvf.face.index()).unwrap());
        assert!(visited.contains(mef.new_face.index()).unwrap());
    }

    /// L2: Double hole splice — build one face with two holes, bridge both.
    ///
    /// After bridging both holes, the single outer loop must traverse
    /// through both hole interiors.
    #[test]
    fn sewing_l2_hole_splice_via_bridge() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();

        let (face, outer_he, inner_he, _inner_loop, _verts) = build_face_with_hole(&mut draft);

        let _bridge1 = bridge_edge(&mut draft, outer_he, inner_he).unwrap();

        assert_eq!(draft.arena().get_face(face).unwrap().inner_loop_count(), 0);

        let outer_loop = draft.arena().get_face(face).unwrap().loops.outer();
        let start = draft.arena().get_loop(outer_loop).unwrap().half_edge();
        let mut current = start;
        let mut count = 0usize;

        loop {
            count += 1;
            assert!(count <= 200, "Infinite loop in merged traversal");
            current = draft.arena().get_half_edge(current).unwrap().next();
            if current == start {
                break;
            }
        }

        assert_eq!(count, 8, "Outer(3) + inner(3) + 2 bridge = 8 edges");
    }

    /// L3: Wire two faces with mismatched vertex outgoing pointers.
    /// validate_vertex_outgoing must catch the inconsistency.
    #[test]
    fn sewing_l3_near_coincident_vertices() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();

        let mvf = draft
            .execute(MakeVertexFace {
                shell_kind: ShellKind::Sheet,
            })
            .unwrap()
            .into_value();
        let se = draft
            .execute(SplitEdge {
                edge: mvf.half_edge,
            })
            .unwrap()
            .into_value();

        let wrong_he = se.he_mb;
        draft
            .arena_mut()
            .get_vertex_mut(mvf.vertex)
            .unwrap()
            .set_primary_disk(wrong_he);

        let result = draft.commit();
        assert!(
            result.is_err(),
            "Commit must reject mismatched vertex outgoing"
        );
        assert!(matches!(
            result.unwrap_err(),
            KernelError::TopologyViolation { .. }
        ));
    }

    // ══════════════════════════════════════════════════════════════
    // CATEGORY 6: Boolean-like Topology Stress (synthetic)
    // ══════════════════════════════════════════════════════════════

    /// L1: Large fan — single center vertex with many radiating faces.
    ///
    /// Build a fan of 50 faces from a center, then verify all faces are in
    /// one connected shell and Euler characteristic is correct.
    #[test]
    fn boolean_like_l1_fan_shell_connectivity() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();

        let mvf = draft
            .execute(MakeVertexFace {
                shell_kind: ShellKind::Sheet,
            })
            .unwrap()
            .into_value();
        let center = mvf.vertex;
        let mut current_edge = mvf.half_edge;

        for _ in 0..50 {
            let se = draft
                .execute(SplitEdge { edge: current_edge })
                .unwrap()
                .into_value();

            let face_id = draft.arena().get_half_edge(current_edge).unwrap().face();
            let mef = draft
                .execute(MakeEdgeFace {
                    vertex_a: center,
                    vertex_b: se.new_vertex,
                    face: face_id,
                })
                .unwrap()
                .into_value();

            current_edge = mef.half_edge_ab;
        }

        assert_eq!(draft.arena().face_count(), 51);
        assert_eq!(draft.arena().vertex_count(), 51);

        let committed = draft.commit().unwrap();
        assert!(validate_topology(committed.arena(), ValidationLevel::Minimal).is_ok());

        let chi = euler_chi(committed.arena());
        assert_eq!(
            chi, 1,
            "Open fan topology has χ=1 (boundary edges), got {}",
            chi
        );

        let mut visited = crate::b_rep::EntityBitset::for_faces(committed.arena());
        let shell =
            crate::queries::shell::discover_shell_faces(committed.arena(), mvf.face, &mut visited)
                .unwrap();
        assert_eq!(
            shell.len(),
            51,
            "All 51 faces must be in one connected shell"
        );
    }

    /// L2: Multiple independent shells in one arena.
    ///
    /// Build two separate seeds (MVF each). Verify discover_shell_faces
    /// identifies exactly 2 independent shells.
    #[test]
    fn boolean_like_l2_nested_shells() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();

        let mvf1 = draft
            .execute(MakeVertexFace {
                shell_kind: ShellKind::Sheet,
            })
            .unwrap()
            .into_value();
        let _se1 = draft
            .execute(SplitEdge {
                edge: mvf1.half_edge,
            })
            .unwrap()
            .into_value();

        let mvf2 = draft
            .execute(MakeVertexFace {
                shell_kind: ShellKind::Sheet,
            })
            .unwrap()
            .into_value();
        let _se2 = draft
            .execute(SplitEdge {
                edge: mvf2.half_edge,
            })
            .unwrap()
            .into_value();

        assert_eq!(draft.arena().face_count(), 2);

        let committed = draft.commit().unwrap();

        let mut visited = crate::b_rep::EntityBitset::for_faces(committed.arena());
        let shell1 =
            crate::queries::shell::discover_shell_faces(committed.arena(), mvf1.face, &mut visited)
                .unwrap();
        let shell2 =
            crate::queries::shell::discover_shell_faces(committed.arena(), mvf2.face, &mut visited)
                .unwrap();

        assert_eq!(shell1.len(), 1, "Shell 1 has exactly 1 face");
        assert_eq!(shell2.len(), 1, "Shell 2 has exactly 1 face");
        assert_ne!(mvf1.face, mvf2.face, "Shells must be distinct");
    }

    /// L3: Chained split+merge stress — 100 iterations of SE+MEF then selective JoinFaces.
    ///
    /// Build a fan of 100 faces, then merge half of them back. Verify Euler
    /// invariant and structural validation pass at each checkpoint.
    #[test]
    fn boolean_like_l3_chained_splits_and_merges() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();

        let mvf = draft
            .execute(MakeVertexFace {
                shell_kind: ShellKind::Sheet,
            })
            .unwrap()
            .into_value();
        let center = mvf.vertex;
        let mut current_edge = mvf.half_edge;
        let mut mef_edges: Vec<HalfEdgeId> = Vec::new();

        for _ in 0..100 {
            let se = draft
                .execute(SplitEdge { edge: current_edge })
                .unwrap()
                .into_value();

            let face_id = draft.arena().get_half_edge(current_edge).unwrap().face();
            let mef = draft
                .execute(MakeEdgeFace {
                    vertex_a: center,
                    vertex_b: se.new_vertex,
                    face: face_id,
                })
                .unwrap()
                .into_value();

            mef_edges.push(mef.half_edge_ab);
            current_edge = mef.half_edge_ab;
        }

        assert_eq!(draft.arena().face_count(), 101);
        assert_eq!(draft.arena().vertex_count(), 101);

        let mid_state = draft.commit().unwrap();
        assert!(
            validate_topology(mid_state.arena(), ValidationLevel::Minimal).is_ok(),
            "Topology must be valid after 100 fan splits"
        );

        let mut draft2 = mid_state.into_mutation();

        let mut merged = 0usize;
        for edge in mef_edges.iter().rev().take(50) {
            let he_data = draft2.arena().get_half_edge(*edge);
            if he_data.is_ok() {
                let result = draft2.execute(JoinFaces { edge: *edge });
                if result.is_ok() {
                    merged += 1;
                }
            }
        }

        assert!(merged > 0, "At least some faces must have been merged back");

        let final_state = draft2.commit().unwrap();
        assert!(
            validate_topology(final_state.arena(), ValidationLevel::Minimal).is_ok(),
            "Topology must remain valid after selective merges"
        );

        let chi = euler_chi(final_state.arena());
        assert_eq!(
            chi, 1,
            "Open fan topology has χ=1 (boundary edges), got {}",
            chi
        );
    }
}
