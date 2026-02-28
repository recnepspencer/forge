#[cfg(test)]
mod tests {
    use crate::handles::{FaceId, HalfEdgeId, LoopId, VertexId};
    use crate::transactions::TopologyState;
    use crate::operations::entity_lifecycle::make_edge_face::MakeEdgeFace;
    use crate::operations::entity_lifecycle::make_vertex_face::MakeVertexFace;
    use crate::operations::non_manifold::sew_edge::SewEdge;
    use crate::operations::entity_lifecycle::split_edge::SplitEdge;
    use crate::operator::TopoOperator;
    use forge_core::TopologyError;

    fn build_test_state() -> (crate::transactions::MutableDraft, HalfEdgeId, HalfEdgeId) {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();

        // MVF: creates v0, F0, self-loop he0 (v0->v0)
        // SE: splits he0 into two halfedges: he0 (v0->v1) and he_mb (v1->v0)
        // Both are self-radial boundary edges on the same face, each on its own Edge.
        let mvf = draft.execute(MakeVertexFace).unwrap().into_value();
        let se = draft.execute(
            SplitEdge {
                edge: mvf.half_edge,
                parameter: 0.5,
            },
        )
        .unwrap()
        .into_value();

        let he_v0_v1 = mvf.half_edge; // v0 -> v1
        let he_v1_v0 = se.he_mb; // v1 -> v0

        (draft, he_v0_v1, he_v1_v0)
    }

    #[test]
    fn sew_edge_glues_antiparallel_boundaries() {
        let (mut draft, he_a, he_b) = build_test_state();

        // Assert initial state: both are boundaries
        assert_eq!(
            draft.arena().get_half_edge(he_a).unwrap().radial_next(),
            he_a
        );
        assert_eq!(
            draft.arena().get_half_edge(he_b).unwrap().radial_next(),
            he_b
        );
        assert_eq!(draft.arena().edge_count(), 2);

        // Sew them together
        let op = SewEdge { he_a, he_b };
        let result = draft.execute(op).unwrap().into_value();

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
        draft.execute(SewEdge { he_a, he_b }).unwrap();

        // Try sewing them again
        let res = draft.execute(SewEdge { he_a, he_b });
        assert!(matches!(
            res.unwrap_err(),
            forge_core::KernelError::TopologyViolation {
                err: forge_core::TopologyError::BoundaryEdgeInSolid { .. },
                ..
            }
        ));
    }

    #[test]
    fn sew_edge_fails_on_non_antiparallel_edges() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();

        let mvf = draft.execute(MakeVertexFace).unwrap().into_value();
        let se1 = draft.execute(
            SplitEdge {
                edge: mvf.half_edge,
                parameter: 0.5,
            },
        )
        .unwrap()
        .into_value();
        let _se2 = draft.execute(
            SplitEdge {
                edge: se1.he_mb,
                parameter: 0.5,
            },
        )
        .unwrap()
        .into_value();

        // Now we have v0->v1->v2->v0.
        // If we try to sew (v0->v1) to (v1->v2), they share v1 but are not antiparallel.
        let he_a = mvf.half_edge; // v0->v1
        let he_b = draft.arena().get_half_edge(he_a).unwrap().next(); // v1->v2

        let op = SewEdge { he_a, he_b };
        let res = draft.execute(op);

        assert!(res.is_err(), "Must fail when edges aren't antiparallel");
    }
}
