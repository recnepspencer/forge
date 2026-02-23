#[cfg(test)]
mod tests {
    use crate::EulerOperator;
    use crate::handles::{FaceId, HalfEdgeId, LoopId, VertexId};
    use crate::operator::apply_op;
    use crate::state::TopologyState;
    use crate::topology::operations::euler::make_vertex_face::MakeVertexFace;
    use crate::topology::operations::euler::split_edge::SplitEdge;
    use crate::topology::operations::euler::make_edge_face::MakeEdgeFace;
    use crate::topology::operations::euler::sew_edge::SewEdge;
    use forge_core::TopologyError;

    fn build_test_state() -> (crate::state::MutableDraft, HalfEdgeId, HalfEdgeId) {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();

        // 1. MVF: creates v0, F0, self-loop he0
        let mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
        
        // 2. SE: splits he0, creating v1. Now F0 is a digon (v0->v1, v1->v0)
        let se = apply_op(&mut draft, SplitEdge { edge: mvf.half_edge, parameter: 0.5 })
            .unwrap().into_value();
        
        // 3. MEF: splits F0 with a new edge connecting v0 and v1.
        // It's a new bridge between existing vertices, splitting the digon into two digons.
        let mef = apply_op(&mut draft, MakeEdgeFace {
            vertex_a: mvf.vertex,
            vertex_b: se.new_vertex,
            face: mvf.face,
        }).unwrap().into_value();

        // MEF redirects next pointers, breaking the vertex-pair match between twins
        //. This causes `repair_edge_after_next_change` to split the Edge entity.
        // Now mef.half_edge_ab is on F0, and its twin (actually it's now self-radial) is somewhere else?
        // Wait, NO. We want to test gluing TWO DIFFERENT edges together.
        // If MEF already split the face and split the edges, we might have two boundary edges spanning the same vertices.

        // Actually, the easiest way to get two open boundary edges spanning the same vertices
        // is to use KEF (KillEdgeFace) or similar, but we don't have that.
        // How do we get two independent edges spanning v0 and v1?
        // MVF -> F0 (v0, he0)
        // SE(he0) -> v1. Now F0 has v0->v1 and v1->v0. Note: these are radial twins (they share an Edge).
        // Wait, no. SE gives them the SAME edge if they were one edge. But they are consecutive halfedges.
        // Ah, he0 is v0->v0. SE splits it into he0 (v0->v1) and he_new (v1->v0). They are NOT twins.
        // They are consecutive around F0, both self-radial.
        
        // Let's SE again: SE(v1->v0) -> v2. Now we have v0->v1->v2->v0.
        // Then MEF(v0, v1, F0) -> creates a new bridge.
        
        // How to set up an antiparallel pair of boundary edges without SewEdge/MEF?
        // 1. MVF: F0, v0 (he0 is v0->v0, E0)
        // 2. SE1(he0): v1. he0 is v0->v1 (E0). he1 is v1->v0 (E1). Both are self-radial on F0.
        // Wait, SE splits the edge. A self-loop edge split becomes 2 edges.
        let mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
        let se = apply_op(&mut draft, SplitEdge { edge: mvf.half_edge, parameter: 0.5 }).unwrap().into_value();
        
        let he_v0_v1 = mvf.half_edge;  // v0 -> v1
        let he_v1_v0 = se.he_mb; // v1 -> v0
        
        (draft, he_v0_v1, he_v1_v0)
    }

    #[test]
    fn sew_edge_glues_antiparallel_boundaries() {
        let (mut draft, he_a, he_b) = build_test_state();
        
        // Assert initial state: both are boundaries
        assert_eq!(draft.arena().get_half_edge(he_a).unwrap().radial_next(), he_a);
        assert_eq!(draft.arena().get_half_edge(he_b).unwrap().radial_next(), he_b);
        assert_eq!(draft.arena().edge_count(), 2);
        
        // Sew them together
        let op = SewEdge { he_a, he_b };
        let result = apply_op(&mut draft, op).unwrap().into_value();
        
        // Assert final state
        let he_a_data = draft.arena().get_half_edge(he_a).unwrap();
        let he_b_data = draft.arena().get_half_edge(he_b).unwrap();
        
        // 1. Radial pointers point to each other
        assert_eq!(he_a_data.radial_next(), he_b);
        assert_eq!(he_b_data.radial_next(), he_a);
        
        // 2. They share the same Edge entity
        let edge = he_a_data.edge();
        assert_eq!(edge, he_b_data.edge());
        assert_eq!(edge, result.edge);
        
        // 3. One edge entity was removed
        assert_eq!(draft.arena().edge_count(), 1);
        assert!(draft.arena().get_edge(result.removed_edge).is_err());
    }

    #[test]
    fn sew_edge_fails_on_already_sewn_edges() {
        let (mut draft, he_a, he_b) = build_test_state();
        apply_op(&mut draft, SewEdge { he_a, he_b }).unwrap();
        
        // Try sewing them again
        let res = apply_op(&mut draft, SewEdge { he_a, he_b });
        assert!(matches!(
            res.unwrap_err(),
            forge_core::KernelError::TopologyViolation { err: forge_core::TopologyError::BoundaryEdgeInSolid { .. }, .. }
        ));
    }

    #[test]
    fn sew_edge_fails_on_non_antiparallel_edges() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();

        let mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
        let se1 = apply_op(&mut draft, SplitEdge { edge: mvf.half_edge, parameter: 0.5 }).unwrap().into_value();
        let _se2 = apply_op(&mut draft, SplitEdge { edge: se1.he_mb, parameter: 0.5 }).unwrap().into_value();
        
        // Now we have v0->v1->v2->v0.
        // If we try to sew (v0->v1) to (v1->v2), they share v1 but are not antiparallel.
        let he_a = mvf.half_edge; // v0->v1
        let he_b = draft.arena().get_half_edge(he_a).unwrap().next(); // v1->v2
        
        let op = SewEdge { he_a, he_b };
        let res = apply_op(&mut draft, op);
        
        assert!(res.is_err(), "Must fail when edges aren't antiparallel");
    }
}
