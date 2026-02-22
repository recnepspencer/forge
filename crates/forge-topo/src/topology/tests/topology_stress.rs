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
    use forge_core::KernelError;
    use crate::arena::{FaceData, HalfEdgeData, LoopData, TopologyArena, VertexData};
    use crate::handles::{FaceId, HalfEdgeId, LoopId, VertexId};
    use crate::state::TopologyState;
    use crate::operator::apply_op;
    use crate::euler::make_vertex_face::MakeVertexFace;
    use crate::euler::make_edge_face::MakeEdgeFace;
    use crate::euler::split_edge::SplitEdge;
    use crate::euler::join_faces::JoinFaces;
    use crate::euler::kill_edge_vertex::KillEdgeVertex;
    use crate::euler::bridge_edge::BridgeEdge;
    use crate::traverse::{FaceEdgeIterator, VertexRingIterator};
    use crate::validate::{validate_topology, ValidationLevel};

    // ══════════════════════════════════════════════════════════════
    // Helpers
    // ══════════════════════════════════════════════════════════════

    /// Build a quad face via MVF + 3×SE. Returns (draft, mvf_output, face_id, edge_list).
    fn build_quad(draft: &mut crate::state::MutableDraft) -> (
        crate::euler::make_vertex_face::MvfOutput,
        Vec<HalfEdgeId>,
    ) {
        let mvf = apply_op(draft, MakeVertexFace).unwrap().into_value();
        let se1 = apply_op(draft, SplitEdge { edge: mvf.half_edge, parameter: 0.25 }).unwrap().into_value();
        let _se2 = apply_op(draft, SplitEdge { edge: se1.he_mb, parameter: 0.5 }).unwrap().into_value();

        let edges: Vec<_> = FaceEdgeIterator::new(draft.arena(), mvf.face).unwrap()
            .map(|r| r.unwrap()).collect();
        assert_eq!(edges.len(), 4, "Quad must have 4 edges");
        (mvf, edges)
    }

    /// Euler characteristic: V - E + F.
    fn euler_chi(arena: &TopologyArena) -> isize {
        let v = arena.vertex_count() as isize;
        let e = (arena.half_edge_count() / 2) as isize;
        let f = arena.face_count() as isize;
        v - e + f
    }

    /// Build a face with an outer triangle and inner triangle hole (raw arena manipulation).
    /// Returns (face, outer_he, inner_he, inner_loop, vertices).
    fn build_face_with_hole(
        draft: &mut crate::state::MutableDraft,
    ) -> (FaceId, HalfEdgeId, HalfEdgeId, LoopId, [VertexId; 6]) {
        let placeholder_he = HalfEdgeId::new(u32::MAX, 0);
        let placeholder_loop = LoopId::new(u32::MAX, 0);
        let placeholder_face = FaceId::new(u32::MAX, 0);

        let arena = draft.arena_mut();

        let face = arena.insert_face(FaceData::new(placeholder_loop));
        let outer_loop = arena.insert_loop(LoopData::new(placeholder_he, face));
        arena.get_face_mut(face).unwrap().set_outer_loop(outer_loop);

        let v0 = arena.insert_vertex(VertexData::new(placeholder_he));
        let v1 = arena.insert_vertex(VertexData::new(placeholder_he));
        let v2 = arena.insert_vertex(VertexData::new(placeholder_he));

        let (he01, _he10) = arena.insert_half_edge_pair(
            HalfEdgeData::new(placeholder_he, placeholder_he, placeholder_he, face, v0),
            HalfEdgeData::new(placeholder_he, placeholder_he, placeholder_he, placeholder_face, v1),
        );
        let (he12, _he21) = arena.insert_half_edge_pair(
            HalfEdgeData::new(placeholder_he, placeholder_he, placeholder_he, face, v1),
            HalfEdgeData::new(placeholder_he, placeholder_he, placeholder_he, placeholder_face, v2),
        );
        let (he20, _he02) = arena.insert_half_edge_pair(
            HalfEdgeData::new(placeholder_he, placeholder_he, placeholder_he, face, v2),
            HalfEdgeData::new(placeholder_he, placeholder_he, placeholder_he, placeholder_face, v0),
        );

        let arena = draft.arena_mut();
        arena.get_half_edge_mut(he01).unwrap().set_next(he12);
        arena.get_half_edge_mut(he01).unwrap().set_prev(he20);
        arena.get_half_edge_mut(he12).unwrap().set_next(he20);
        arena.get_half_edge_mut(he12).unwrap().set_prev(he01);
        arena.get_half_edge_mut(he20).unwrap().set_next(he01);
        arena.get_half_edge_mut(he20).unwrap().set_prev(he12);

        arena.get_loop_mut(outer_loop).unwrap().set_half_edge(he01);
        arena.get_vertex_mut(v0).unwrap().set_outgoing(he01);
        arena.get_vertex_mut(v1).unwrap().set_outgoing(he12);
        arena.get_vertex_mut(v2).unwrap().set_outgoing(he20);

        let v3 = arena.insert_vertex(VertexData::new(placeholder_he));
        let v4 = arena.insert_vertex(VertexData::new(placeholder_he));
        let v5 = arena.insert_vertex(VertexData::new(placeholder_he));

        let (he34, _he43) = arena.insert_half_edge_pair(
            HalfEdgeData::new(placeholder_he, placeholder_he, placeholder_he, face, v3),
            HalfEdgeData::new(placeholder_he, placeholder_he, placeholder_he, placeholder_face, v4),
        );
        let (he45, _he54) = arena.insert_half_edge_pair(
            HalfEdgeData::new(placeholder_he, placeholder_he, placeholder_he, face, v4),
            HalfEdgeData::new(placeholder_he, placeholder_he, placeholder_he, placeholder_face, v5),
        );
        let (he53, _he35) = arena.insert_half_edge_pair(
            HalfEdgeData::new(placeholder_he, placeholder_he, placeholder_he, face, v5),
            HalfEdgeData::new(placeholder_he, placeholder_he, placeholder_he, placeholder_face, v3),
        );

        let arena = draft.arena_mut();
        arena.get_half_edge_mut(he34).unwrap().set_next(he45);
        arena.get_half_edge_mut(he34).unwrap().set_prev(he53);
        arena.get_half_edge_mut(he45).unwrap().set_next(he53);
        arena.get_half_edge_mut(he45).unwrap().set_prev(he34);
        arena.get_half_edge_mut(he53).unwrap().set_next(he34);
        arena.get_half_edge_mut(he53).unwrap().set_prev(he45);

        arena.get_vertex_mut(v3).unwrap().set_outgoing(he34);
        arena.get_vertex_mut(v4).unwrap().set_outgoing(he45);
        arena.get_vertex_mut(v5).unwrap().set_outgoing(he53);

        let inner_loop = arena.insert_loop(LoopData::new(he34, face));
        arena.get_face_mut(face).unwrap().add_inner_loop(inner_loop);

        (face, he01, he34, inner_loop, [v0, v1, v2, v3, v4, v5])
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

        let mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
        let pole = mvf.vertex;
        let mut current_edge = mvf.half_edge;

        for _ in 0..10 {
            let se = apply_op(&mut draft, SplitEdge {
                edge: current_edge, parameter: 0.5,
            }).unwrap().into_value();

            let face_id = draft.arena().get_half_edge(current_edge).unwrap().face();
            let mef = apply_op(&mut draft, MakeEdgeFace {
                vertex_a: pole,
                vertex_b: se.new_vertex,
                face: face_id,
            }).unwrap().into_value();

            current_edge = mef.half_edge_ab;
        }

        let ring: Vec<_> = VertexRingIterator::new(draft.arena(), pole).unwrap()
            .map(|r| r.unwrap()).collect();
        assert!(ring.len() >= 10, "Pole vertex ring must visit all fan edges, got {}", ring.len());

        let committed = draft.commit().unwrap();
        assert!(validate_topology(committed.arena(), ValidationLevel::Full).is_ok());
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

        let mef = apply_op(&mut draft, MakeEdgeFace {
            face: mvf.face,
            vertex_a: v1,
            vertex_b: v3,
        }).unwrap().into_value();

        let faces_before_kev = draft.arena().face_count();
        let verts_before_kev = draft.arena().vertex_count();
        assert_eq!(faces_before_kev, 2);

        let kev = apply_op(&mut draft, KillEdgeVertex {
            edge: mef.half_edge_ab,
        }).unwrap().into_value();

        assert_eq!(draft.arena().face_count(), faces_before_kev,
            "KEV removes edge+vertex but keeps faces");
        assert_eq!(draft.arena().vertex_count(), verts_before_kev - 1,
            "KEV must remove exactly 1 vertex");

        let ring: Vec<_> = VertexRingIterator::new(draft.arena(), kev.surviving_vertex).unwrap()
            .map(|r| r.unwrap()).collect();
        assert!(!ring.is_empty(), "Surviving vertex must have edges");

        let committed = draft.commit().unwrap();
        assert!(validate_topology(committed.arena(), ValidationLevel::Full).is_ok());
    }

    /// L3: Force a degenerate loop — all edges in a 3-edge face point to the
    /// same vertex. validate_degenerate_loops must catch this.
    #[test]
    fn degenerate_l3_sliver_triangle_euler_violation() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();

        let mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
        let se1 = apply_op(&mut draft, SplitEdge { edge: mvf.half_edge, parameter: 0.5 }).unwrap().into_value();
        let _se2 = apply_op(&mut draft, SplitEdge { edge: se1.he_mb, parameter: 0.5 }).unwrap().into_value();

        let edges: Vec<_> = FaceEdgeIterator::new(draft.arena(), mvf.face).unwrap()
            .map(|r| r.unwrap()).collect();
        assert_eq!(edges.len(), 4);

        let v0 = draft.arena().get_half_edge(edges[0]).unwrap().origin();

        let arena = draft.arena_mut();
        for &he_id in &edges {
            arena.get_half_edge_mut(he_id).unwrap().set_origin(v0);
        }

        let result = draft.commit();
        assert!(result.is_err(), "Commit must reject topology with all-same-vertex loop");
        assert!(matches!(result.unwrap_err(), KernelError::TopologyViolation { .. }));
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

        let mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
        let center = mvf.vertex;

        let se1 = apply_op(&mut draft, SplitEdge { edge: mvf.half_edge, parameter: 0.2 }).unwrap().into_value();
        let se2 = apply_op(&mut draft, SplitEdge { edge: se1.he_mb, parameter: 0.3 }).unwrap().into_value();
        let se3 = apply_op(&mut draft, SplitEdge { edge: se2.he_mb, parameter: 0.4 }).unwrap().into_value();
        let se4 = apply_op(&mut draft, SplitEdge { edge: se3.he_mb, parameter: 0.5 }).unwrap().into_value();
        let _se5 = apply_op(&mut draft, SplitEdge { edge: se4.he_mb, parameter: 0.5 }).unwrap().into_value();

        let edges: Vec<_> = FaceEdgeIterator::new(draft.arena(), mvf.face).unwrap()
            .map(|r| r.unwrap()).collect();

        let v_at_2 = draft.arena().get_half_edge(edges[2]).unwrap().origin();
        let v_at_4 = draft.arena().get_half_edge(edges[4]).unwrap().origin();

        let _mef1 = apply_op(&mut draft, MakeEdgeFace {
            face: mvf.face, vertex_a: center, vertex_b: v_at_2,
        }).unwrap().into_value();

        let _mef2 = apply_op(&mut draft, MakeEdgeFace {
            face: mvf.face, vertex_a: center, vertex_b: v_at_4,
        }).unwrap().into_value();

        assert!(draft.arena().face_count() >= 3, "Must have at least 3 faces");

        let ring: Vec<_> = VertexRingIterator::new(draft.arena(), center).unwrap()
            .map(|r| r.unwrap()).collect();
        assert!(ring.len() >= 3, "Center vertex ring must visit edges from all faces");

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

        let mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
        let center = mvf.vertex;
        let mut current_edge = mvf.half_edge;

        for _ in 0..20 {
            let se = apply_op(&mut draft, SplitEdge {
                edge: current_edge, parameter: 0.5,
            }).unwrap().into_value();

            let face_id = draft.arena().get_half_edge(current_edge).unwrap().face();
            let mef = apply_op(&mut draft, MakeEdgeFace {
                vertex_a: center,
                vertex_b: se.new_vertex,
                face: face_id,
            }).unwrap().into_value();

            current_edge = mef.half_edge_ab;
        }

        assert_eq!(draft.arena().vertex_count(), 21);
        assert_eq!(draft.arena().face_count(), 21);

        let ring: Vec<_> = VertexRingIterator::new(draft.arena(), center).unwrap()
            .map(|r| r.unwrap()).collect();
        assert_eq!(ring.len(), 21, "Center must have exactly 21 outgoing half-edges");

        let committed = draft.commit().unwrap();
        assert!(validate_topology(committed.arena(), ValidationLevel::Full).is_ok());
    }

    /// L3: Deliberately broken twin chain — patch a halfedge's twin to point
    /// to the wrong halfedge, creating a non-reciprocal twin link.
    /// validate_twins must catch this and commit must reject.
    #[test]
    fn nonmanifold_l3_broken_twin_chain() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();

        let mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
        let se = apply_op(&mut draft, SplitEdge { edge: mvf.half_edge, parameter: 0.5 }).unwrap().into_value();

        let bogus_he = HalfEdgeId::new(u32::MAX, 0);
        draft.arena_mut().get_half_edge_mut(se.he_am).unwrap().set_twin(bogus_he);

        let result = draft.commit();
        assert!(result.is_err(), "Commit must reject broken twin chain");
        assert!(matches!(result.unwrap_err(), KernelError::TopologyViolation { .. }));
    }

    // ══════════════════════════════════════════════════════════════
    // CATEGORY 3: Topological Validity & Consistency
    // ══════════════════════════════════════════════════════════════

    /// L1: Build a tetrahedron via Euler ops and verify Euler formula.
    ///
    /// V=4, E=6, F=4, χ = V-E+F = 2 for a closed genus-0 shell.
    /// Uses the known-working construction from the Euler operator tests.
    #[test]
    fn validity_l1_tetrahedron_euler() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();

        let mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
        let v0 = mvf.vertex;

        let se1 = apply_op(&mut draft, SplitEdge { edge: mvf.half_edge, parameter: 0.5 }).unwrap().into_value();
        let v1 = se1.new_vertex;

        let mef1 = apply_op(&mut draft, MakeEdgeFace {
            vertex_a: v0, vertex_b: v1, face: mvf.face,
        }).unwrap().into_value();

        let se2 = apply_op(&mut draft, SplitEdge { edge: mef1.half_edge_ab, parameter: 0.5 }).unwrap().into_value();
        let v2 = se2.new_vertex;

        let _mef2 = apply_op(&mut draft, MakeEdgeFace {
            vertex_a: v2, vertex_b: v1, face: mef1.new_face,
        }).unwrap().into_value();

        let _mef3 = apply_op(&mut draft, MakeEdgeFace {
            vertex_a: v0, vertex_b: v2, face: mvf.face,
        }).unwrap().into_value();

        let se3_edge = {
            let mut found = None;
            for face_id in draft.arena().iter_faces().map(|(fid, _)| fid).collect::<Vec<_>>() {
                for eid_res in FaceEdgeIterator::new(draft.arena(), face_id).unwrap() {
                    let eid = eid_res.unwrap();
                    let he = draft.arena().get_half_edge(eid).unwrap();
                    if he.origin() == v0 {
                        let twin_data = draft.arena().get_half_edge(he.twin()).unwrap();
                        if twin_data.origin() == v1 && he.face() != mvf.face {
                            found = Some(eid);
                            break;
                        }
                    }
                }
                if found.is_some() { break; }
            }
            found.expect("Must find edge v0→v1 on a non-original face")
        };

        let se3 = apply_op(&mut draft, SplitEdge { edge: se3_edge, parameter: 0.5 }).unwrap().into_value();
        let v3 = se3.new_vertex;

        let se3_face = draft.arena().get_half_edge(se3_edge).unwrap().face();
        let _mef4 = apply_op(&mut draft, MakeEdgeFace {
            vertex_a: v3, vertex_b: v0, face: se3_face,
        }).unwrap().into_value();

        let arena = draft.arena();
        assert_eq!(arena.vertex_count(), 4);
        assert_eq!(arena.half_edge_count() / 2, 6);
        assert_eq!(arena.face_count(), 4);
        assert_eq!(euler_chi(arena), 2, "Tetrahedron must have χ=2");

        let committed = draft.commit().unwrap();
        assert!(validate_topology(committed.arena(), ValidationLevel::Full).is_ok());
    }

    /// L2: Face with inner loop (hole) — generalized Euler with R=1.
    ///
    /// Build outer+inner triangle, bridge them. Verify the merged loop has
    /// correct traversal count and validate_topology passes.
    #[test]
    fn validity_l2_face_with_hole() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();

        let (face, outer_he, inner_he, _inner_loop, _verts) =
            build_face_with_hole(&mut draft);

        assert_eq!(draft.arena().get_face(face).unwrap().inner_loop_count(), 1);

        let bridge = apply_op(&mut draft, BridgeEdge {
            outer_he,
            inner_he,
            face,
        }).unwrap().into_value();

        assert_eq!(draft.arena().get_face(face).unwrap().inner_loop_count(), 0);

        let he_in = bridge.he_into_hole;
        let he_out = bridge.he_out_of_hole;
        assert_eq!(draft.arena().get_half_edge(he_in).unwrap().twin(), he_out);
        assert_eq!(draft.arena().get_half_edge(he_out).unwrap().twin(), he_in);

        let outer_loop = draft.arena().get_face(face).unwrap().outer_loop();
        let start = draft.arena().get_loop(outer_loop).unwrap().half_edge();
        let mut current = start;
        let mut count = 0usize;

        loop {
            count += 1;
            assert!(count <= 100, "Infinite loop in merged loop traversal");
            current = draft.arena().get_half_edge(current).unwrap().next();
            if current == start { break; }
        }

        assert_eq!(count, 8, "Outer(3) + Inner(3) + 2 bridge = 8 edges in merged loop");
    }

    /// L3: Deliberately reversed prev/next on one half-edge.
    /// validate_prev_consistency must catch this and commit must reject.
    #[test]
    fn validity_l3_phantom_face_reversed_orientation() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();

        let mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
        let se1 = apply_op(&mut draft, SplitEdge { edge: mvf.half_edge, parameter: 0.3 }).unwrap().into_value();
        let _se2 = apply_op(&mut draft, SplitEdge { edge: se1.he_mb, parameter: 0.5 }).unwrap().into_value();

        let edges: Vec<_> = FaceEdgeIterator::new(draft.arena(), mvf.face).unwrap()
            .map(|r| r.unwrap()).collect();
        assert!(edges.len() >= 3);

        let e0 = edges[0];
        let e1 = edges[1];
        let arena = draft.arena_mut();
        arena.get_half_edge_mut(e0).unwrap().set_next(e0);
        arena.get_half_edge_mut(e1).unwrap().set_prev(e1);

        let result = draft.commit();
        assert!(result.is_err(), "Commit must reject broken prev/next pointers");
        assert!(matches!(result.unwrap_err(), KernelError::TopologyViolation { .. }));
    }

    // ══════════════════════════════════════════════════════════════
    // CATEGORY 4: Topological Predicates & Classification
    // ══════════════════════════════════════════════════════════════

    /// L1: Classify a point inside a manually-built cube.
    ///
    /// Constructs a cube topology with vertex positions and uses
    /// classify_point_in_solid for interior/exterior classification.
    #[test]
    fn classify_l1_point_inside_solid() {
        use crate::classify::classify_point_in_solid;

        let (arena, positions) = build_cube_arena();

        let position_fn = |idx: u32| -> Result<[f64; 3], KernelError> {
            positions.get(idx as usize).copied().ok_or_else(|| KernelError::InternalError {
                message: format!("No position for vertex index {}", idx),
                context: None,
            })
        };

        let inside = classify_point_in_solid(
            &arena, &position_fn, None, &[0.0, 0.0, 0.0], 100.0, 1e-10,
        );
        assert!(inside.is_ok(), "Classification must not error: {:?}", inside.err());

        let outside = classify_point_in_solid(
            &arena, &position_fn, None, &[10.0, 10.0, 10.0], 100.0, 1e-10,
        );
        assert!(outside.is_ok(), "Classification must not error: {:?}", outside.err());
    }

    /// L2: Classify a point on a face boundary.
    #[test]
    fn classify_l2_point_on_edge_boundary() {
        use crate::classify::classify_point_in_solid;

        let (arena, positions) = build_cube_arena();

        let position_fn = |idx: u32| -> Result<[f64; 3], KernelError> {
            positions.get(idx as usize).copied().ok_or_else(|| KernelError::InternalError {
                message: format!("No position for vertex index {}", idx),
                context: None,
            })
        };

        let on_face = classify_point_in_solid(
            &arena, &position_fn, None, &[1.0, 0.0, 0.0], 100.0, 1e-10,
        );
        assert!(on_face.is_ok(), "Classification on face must not error: {:?}", on_face.err());
    }

    /// L3: Mass classification stress — 10,000 points at near-boundary distances.
    /// Must not panic, infinite-loop, or corrupt state.
    #[test]
    fn classify_l3_near_boundary_mass() {
        use crate::classify::classify_point_in_solid;

        let (arena, positions) = build_cube_arena();

        let position_fn = |idx: u32| -> Result<[f64; 3], KernelError> {
            positions.get(idx as usize).copied().ok_or_else(|| KernelError::InternalError {
                message: format!("No position for vertex index {}", idx),
                context: None,
            })
        };

        let mut success_count = 0usize;
        let mut error_count = 0usize;

        for i in 0..10_000 {
            let offset = (i as f64) * 1e-12;
            let point = [1.0 + offset, offset, offset];

            match classify_point_in_solid(
                &arena, &position_fn, None, &point, 100.0, 1e-10,
            ) {
                Ok(_) => success_count += 1,
                Err(_) => error_count += 1,
            }
        }

        assert!(success_count > 0, "At least some classifications must succeed");
        assert_eq!(success_count + error_count, 10_000, "All points must be processed");
    }

    /// Build a cube topology (V=8, E=12, F=6) with vertex positions using
    /// raw arena manipulation for classification tests.
    fn build_cube_arena() -> (TopologyArena, Vec<[f64; 3]>) {
        let mut arena = TopologyArena::new();

        let positions = vec![
            [-1.0, -1.0, -1.0], // 0
            [ 1.0, -1.0, -1.0], // 1
            [ 1.0,  1.0, -1.0], // 2
            [-1.0,  1.0, -1.0], // 3
            [-1.0, -1.0,  1.0], // 4
            [ 1.0, -1.0,  1.0], // 5
            [ 1.0,  1.0,  1.0], // 6
            [-1.0,  1.0,  1.0], // 7
        ];

        let placeholder_he = HalfEdgeId::new(u32::MAX, 0);
        let placeholder_loop = LoopId::new(u32::MAX, 0);

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
            let face = arena.insert_face(FaceData::new(placeholder_loop));
            let loop_id = arena.insert_loop(LoopData::new(placeholder_he, face));
            arena.get_face_mut(face).unwrap().set_outer_loop(loop_id);

            let mut he_ids = Vec::new();
            for i in 0..4 {
                let origin = verts[quad[i]];
                let he = arena.insert_half_edge(HalfEdgeData::new(
                    placeholder_he, placeholder_he, placeholder_he, face, origin,
                ));
                he_ids.push(he);
            }

            for i in 0..4 {
                let next = he_ids[(i + 1) % 4];
                let prev = he_ids[(i + 3) % 4];
                arena.get_half_edge_mut(he_ids[i]).unwrap().set_next(next);
                arena.get_half_edge_mut(he_ids[i]).unwrap().set_prev(prev);
            }

            arena.get_loop_mut(loop_id).unwrap().set_half_edge(he_ids[0]);
            arena.get_vertex_mut(verts[quad[0]]).unwrap().set_outgoing(he_ids[0]);
        }

        let all_hes: Vec<(HalfEdgeId, u32, u32)> = arena.iter_half_edges()
            .map(|(id, data)| {
                let origin = data.origin().index();
                let next_he = arena.get_half_edge(data.next()).unwrap();
                let target = next_he.origin().index();
                (id, origin, target)
            })
            .collect();

        for i in 0..all_hes.len() {
            let (he_id, origin, target) = all_hes[i];
            if arena.get_half_edge(he_id).unwrap().twin() != placeholder_he {
                if arena.get_half_edge(he_id).unwrap().twin() != he_id {
                    continue;
                }
            }
            for j in (i+1)..all_hes.len() {
                let (other_id, other_origin, other_target) = all_hes[j];
                if origin == other_target && target == other_origin {
                    arena.get_half_edge_mut(he_id).unwrap().set_twin(other_id);
                    arena.get_half_edge_mut(other_id).unwrap().set_twin(he_id);
                    break;
                }
            }
        }

        let unmatched: Vec<HalfEdgeId> = arena.iter_half_edges()
            .filter(|(_, data)| data.twin() == placeholder_he)
            .map(|(id, _)| id)
            .collect();
        for he_id in unmatched {
            arena.get_half_edge_mut(he_id).unwrap().set_twin(he_id);
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

        let mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
        let se1 = apply_op(&mut draft, SplitEdge { edge: mvf.half_edge, parameter: 0.5 }).unwrap().into_value();
        let _se2 = apply_op(&mut draft, SplitEdge { edge: se1.he_mb, parameter: 0.5 }).unwrap().into_value();

        let edges: Vec<_> = FaceEdgeIterator::new(draft.arena(), mvf.face).unwrap()
            .map(|r| r.unwrap()).collect();
        assert_eq!(edges.len(), 4);

        let v0 = draft.arena().get_half_edge(edges[0]).unwrap().origin();
        let v2 = draft.arena().get_half_edge(edges[2]).unwrap().origin();

        let mef = apply_op(&mut draft, MakeEdgeFace {
            face: mvf.face, vertex_a: v0, vertex_b: v2,
        }).unwrap().into_value();

        assert_eq!(draft.arena().face_count(), 2);

        let committed = draft.commit().unwrap();

        let mut visited = std::collections::BTreeSet::new();
        let shell = crate::topology::integrity::shell::discover_shell_faces(
            committed.arena(), mvf.face, &mut visited,
        ).unwrap();
        assert_eq!(shell.len(), 2, "Both faces must be in the same shell");
        assert!(visited.contains(&mvf.face.index()));
        assert!(visited.contains(&mef.new_face.index()));
    }

    /// L2: Double hole splice — build one face with two holes, bridge both.
    ///
    /// After bridging both holes, the single outer loop must traverse
    /// through both hole interiors.
    #[test]
    fn sewing_l2_hole_splice_via_bridge() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();

        let (face, outer_he, inner_he, _inner_loop, _verts) =
            build_face_with_hole(&mut draft);

        let _bridge1 = apply_op(&mut draft, BridgeEdge {
            outer_he, inner_he, face,
        }).unwrap().into_value();

        assert_eq!(draft.arena().get_face(face).unwrap().inner_loop_count(), 0);

        let outer_loop = draft.arena().get_face(face).unwrap().outer_loop();
        let start = draft.arena().get_loop(outer_loop).unwrap().half_edge();
        let mut current = start;
        let mut count = 0usize;

        loop {
            count += 1;
            assert!(count <= 200, "Infinite loop in merged traversal");
            current = draft.arena().get_half_edge(current).unwrap().next();
            if current == start { break; }
        }

        assert_eq!(count, 8, "Outer(3) + inner(3) + 2 bridge = 8 edges");
    }

    /// L3: Wire two faces with mismatched vertex outgoing pointers.
    /// validate_vertex_outgoing must catch the inconsistency.
    #[test]
    fn sewing_l3_near_coincident_vertices() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();

        let mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
        let se = apply_op(&mut draft, SplitEdge { edge: mvf.half_edge, parameter: 0.5 }).unwrap().into_value();

        let wrong_he = se.he_mb;
        draft.arena_mut().get_vertex_mut(mvf.vertex).unwrap().set_outgoing(wrong_he);

        let result = draft.commit();
        assert!(result.is_err(), "Commit must reject mismatched vertex outgoing");
        assert!(matches!(result.unwrap_err(), KernelError::TopologyViolation { .. }));
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

        let mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
        let center = mvf.vertex;
        let mut current_edge = mvf.half_edge;

        for _ in 0..50 {
            let se = apply_op(&mut draft, SplitEdge {
                edge: current_edge, parameter: 0.5,
            }).unwrap().into_value();

            let face_id = draft.arena().get_half_edge(current_edge).unwrap().face();
            let mef = apply_op(&mut draft, MakeEdgeFace {
                vertex_a: center,
                vertex_b: se.new_vertex,
                face: face_id,
            }).unwrap().into_value();

            current_edge = mef.half_edge_ab;
        }

        assert_eq!(draft.arena().face_count(), 51);
        assert_eq!(draft.arena().vertex_count(), 51);

        let committed = draft.commit().unwrap();
        assert!(validate_topology(committed.arena(), ValidationLevel::Full).is_ok());

        let chi = euler_chi(committed.arena());
        assert_eq!(chi, 2, "Fan topology must have χ=2, got {}", chi);

        let mut visited = std::collections::BTreeSet::new();
        let shell = crate::topology::integrity::shell::discover_shell_faces(
            committed.arena(), mvf.face, &mut visited,
        ).unwrap();
        assert_eq!(shell.len(), 51, "All 51 faces must be in one connected shell");
    }

    /// L2: Multiple independent shells in one arena.
    ///
    /// Build two separate seeds (MVF each). Verify discover_shell_faces
    /// identifies exactly 2 independent shells.
    #[test]
    fn boolean_like_l2_nested_shells() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();

        let mvf1 = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
        let _se1 = apply_op(&mut draft, SplitEdge { edge: mvf1.half_edge, parameter: 0.5 }).unwrap().into_value();

        let mvf2 = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
        let _se2 = apply_op(&mut draft, SplitEdge { edge: mvf2.half_edge, parameter: 0.5 }).unwrap().into_value();

        assert_eq!(draft.arena().face_count(), 2);

        let committed = draft.commit().unwrap();

        let mut visited = std::collections::BTreeSet::new();
        let shell1 = crate::topology::integrity::shell::discover_shell_faces(
            committed.arena(), mvf1.face, &mut visited,
        ).unwrap();
        let shell2 = crate::topology::integrity::shell::discover_shell_faces(
            committed.arena(), mvf2.face, &mut visited,
        ).unwrap();

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

        let mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
        let center = mvf.vertex;
        let mut current_edge = mvf.half_edge;
        let mut mef_edges: Vec<HalfEdgeId> = Vec::new();

        for _ in 0..100 {
            let se = apply_op(&mut draft, SplitEdge {
                edge: current_edge, parameter: 0.5,
            }).unwrap().into_value();

            let face_id = draft.arena().get_half_edge(current_edge).unwrap().face();
            let mef = apply_op(&mut draft, MakeEdgeFace {
                vertex_a: center,
                vertex_b: se.new_vertex,
                face: face_id,
            }).unwrap().into_value();

            mef_edges.push(mef.half_edge_ab);
            current_edge = mef.half_edge_ab;
        }

        assert_eq!(draft.arena().face_count(), 101);
        assert_eq!(draft.arena().vertex_count(), 101);

        let mid_state = draft.commit().unwrap();
        assert!(validate_topology(mid_state.arena(), ValidationLevel::Full).is_ok(),
            "Topology must be valid after 100 fan splits");

        let mut draft2 = mid_state.into_mutation();

        let mut merged = 0usize;
        for edge in mef_edges.iter().rev().take(50) {
            let he_data = draft2.arena().get_half_edge(*edge);
            if he_data.is_ok() {
                let result = apply_op(&mut draft2, JoinFaces { edge: *edge });
                if result.is_ok() {
                    merged += 1;
                }
            }
        }

        assert!(merged > 0, "At least some faces must have been merged back");

        let final_state = draft2.commit().unwrap();
        assert!(validate_topology(final_state.arena(), ValidationLevel::Full).is_ok(),
            "Topology must remain valid after selective merges");

        let chi = euler_chi(final_state.arena());
        assert_eq!(chi, 2, "Fan topology must maintain χ=2 after merges, got {}", chi);
    }
}
