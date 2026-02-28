#[cfg(test)]
mod tests {
    use super::validate::*;
    use super::structural::*;
    use crate::arena::TopologyArena;
    use crate::euler::make_vertex_face::MakeVertexFace;
    use crate::euler::split_edge::SplitEdge;
    use crate::handles::HalfEdgeId;
    use crate::operator::apply_op;
    use crate::state::TopologyState;
    use crate::validate;
    use forge_core::KernelError;

    #[test]
    fn empty_arena_validates() {
        let arena = TopologyArena::new();
        assert!(validate_topology(&arena, ValidationLevel::Full).is_ok());
    }

    #[test]
    fn seed_validates() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();
        let _mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
        let state = draft.commit().unwrap();
        assert!(validate_topology(state.arena(), ValidationLevel::Full).is_ok());
    }

    #[test]
    fn split_validates() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();
        let mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
        let _se = apply_op(
            &mut draft,
            SplitEdge {
                edge: mvf.half_edge,
                parameter: 0.5,
            },
        )
        .unwrap()
        .into_value();
        let state = draft.commit().unwrap();
        assert!(validate_topology(state.arena(), ValidationLevel::Full).is_ok());
    }

    #[test]
    fn topology_mode_manifold_strict_rejects_valence_3_edge() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();
        let mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
        let se = apply_op(
            &mut draft,
            SplitEdge {
                edge: mvf.half_edge,
                parameter: 0.5,
            },
        )
        .unwrap()
        .into_value();

        let he_am = se.he_am;
        let he_mb = se.he_mb;

        let face = draft.arena().get_half_edge(he_am).unwrap().face();
        let orig = draft.arena().get_half_edge(he_am).unwrap().origin();
        let edge = draft.arena().get_half_edge(he_am).unwrap().edge();

        draft
            .arena_mut()
            .get_half_edge_mut(he_mb)
            .unwrap()
            .set_edge(edge);

        let ghost = draft.insert_half_edge(crate::arena::HalfEdgeData::new(
            he_am,
            he_mb,
            he_am,
            face, orig, edge,
        ));

        draft
            .arena_mut()
            .get_half_edge_mut(he_am)
            .unwrap()
            .set_next(ghost);
        draft
            .arena_mut()
            .get_half_edge_mut(he_mb)
            .unwrap()
            .set_prev(ghost);

        draft
            .arena_mut()
            .get_half_edge_mut(he_am)
            .unwrap()
            .set_radial_next(he_mb);
        draft
            .arena_mut()
            .get_half_edge_mut(he_mb)
            .unwrap()
            .set_radial_next(ghost);
        draft
            .arena_mut()
            .get_half_edge_mut(ghost)
            .unwrap()
            .set_radial_next(he_am);

        let result = draft.commit_with_mode(
            validate::ValidationLevel::Minimal,
            validate::TopologyMode::ManifoldStrict,
        );
        assert!(result.is_err());
    }

    #[test]
    fn topology_mode_nmt_intermediate_accepts_valence_3_edge() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();
        let mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
        let se = apply_op(
            &mut draft,
            SplitEdge {
                edge: mvf.half_edge,
                parameter: 0.5,
            },
        )
        .unwrap()
        .into_value();

        let he_am = se.he_am;
        let he_mb = se.he_mb;

        let face = draft.arena().get_half_edge(he_am).unwrap().face();
        let orig = draft.arena().get_half_edge(he_am).unwrap().origin();
        let edge = draft.arena().get_half_edge(he_am).unwrap().edge();

        draft
            .arena_mut()
            .get_half_edge_mut(he_mb)
            .unwrap()
            .set_edge(edge);

        let ghost = draft.insert_half_edge(crate::arena::HalfEdgeData::new(
            he_am, he_mb, he_am, face, orig, edge,
        ));

        draft
            .arena_mut()
            .get_half_edge_mut(he_am)
            .unwrap()
            .set_next(ghost);
        draft
            .arena_mut()
            .get_half_edge_mut(he_mb)
            .unwrap()
            .set_prev(ghost);

        draft
            .arena_mut()
            .get_half_edge_mut(he_am)
            .unwrap()
            .set_radial_next(he_mb);
        draft
            .arena_mut()
            .get_half_edge_mut(he_mb)
            .unwrap()
            .set_radial_next(ghost);
        draft
            .arena_mut()
            .get_half_edge_mut(ghost)
            .unwrap()
            .set_radial_next(he_am);

        let result = draft.commit_with_mode(
            validate::ValidationLevel::Full,
            validate::TopologyMode::NmtIntermediate,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn topology_mode_nmt_intermediate_still_rejects_broken_ring() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();
        let mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
        let se = apply_op(
            &mut draft,
            SplitEdge {
                edge: mvf.half_edge,
                parameter: 0.5,
            },
        )
        .unwrap()
        .into_value();

        let stale = HalfEdgeId::from_raw_parts(99_999, 0);
        draft
            .arena_mut()
            .get_half_edge_mut(se.he_am)
            .unwrap()
            .set_radial_next(stale);

        let result = draft.commit_with_mode(
            validate::ValidationLevel::Full,
            validate::TopologyMode::NmtIntermediate,
        );
        assert!(result.is_err());
    }

    #[test]
    fn topology_mode_default_commit_is_always_manifold_strict() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();
        let mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
        let _se = apply_op(
            &mut draft,
            SplitEdge {
                edge: mvf.half_edge,
                parameter: 0.5,
            },
        )
        .unwrap()
        .into_value();
        assert!(draft.commit().is_ok());
    }

    #[test]
    fn adversarial_edge_entity_inconsistency_in_radial_ring() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();
        let mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
        let se = apply_op(
            &mut draft,
            SplitEdge {
                edge: mvf.half_edge,
                parameter: 0.5,
            },
        )
        .unwrap()
        .into_value();

        let he_am = se.he_am;
        let he_mb = se.he_mb;

        draft
            .arena_mut()
            .get_half_edge_mut(he_am)
            .unwrap()
            .set_radial_next(he_mb);
        draft
            .arena_mut()
            .get_half_edge_mut(he_mb)
            .unwrap()
            .set_radial_next(he_am);

        let result = validate_topology(draft.arena(), ValidationLevel::Full);
        assert!(result.is_err());
    }

    #[test]
    fn d8_manifold_strict_unconditional_even_at_level_none() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();
        let mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
        let se = apply_op(
            &mut draft,
            SplitEdge {
                edge: mvf.half_edge,
                parameter: 0.5,
            },
        )
        .unwrap()
        .into_value();

        let he_am = se.he_am;
        let he_mb = se.he_mb;
        let face = draft.arena().get_half_edge(he_am).unwrap().face();
        let orig = draft.arena().get_half_edge(he_am).unwrap().origin();
        let edge = draft.arena().get_half_edge(he_am).unwrap().edge();

        draft
            .arena_mut()
            .get_half_edge_mut(he_mb)
            .unwrap()
            .set_edge(edge);

        let ghost = draft.insert_half_edge(crate::arena::HalfEdgeData::new(
            he_am, he_mb, he_am, face, orig, edge,
        ));
        draft
            .arena_mut()
            .get_half_edge_mut(he_am)
            .unwrap()
            .set_next(ghost);
        draft
            .arena_mut()
            .get_half_edge_mut(he_mb)
            .unwrap()
            .set_prev(ghost);
        draft
            .arena_mut()
            .get_half_edge_mut(he_am)
            .unwrap()
            .set_radial_next(he_mb);
        draft
            .arena_mut()
            .get_half_edge_mut(he_mb)
            .unwrap()
            .set_radial_next(ghost);
        draft
            .arena_mut()
            .get_half_edge_mut(ghost)
            .unwrap()
            .set_radial_next(he_am);

        let result = draft.commit_with_mode(
            validate::ValidationLevel::None,
            validate::TopologyMode::ManifoldStrict,
        );
        assert!(result.is_err());
    }

    #[test]
    fn adversarial_default_commit_rejects_valence_3() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();
        let mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
        let se = apply_op(
            &mut draft,
            SplitEdge {
                edge: mvf.half_edge,
                parameter: 0.5,
            },
        )
        .unwrap()
        .into_value();

        let he_am = se.he_am;
        let he_mb = se.he_mb;
        let face = draft.arena().get_half_edge(he_am).unwrap().face();
        let orig = draft.arena().get_half_edge(he_am).unwrap().origin();
        let edge = draft.arena().get_half_edge(he_am).unwrap().edge();

        draft
            .arena_mut()
            .get_half_edge_mut(he_mb)
            .unwrap()
            .set_edge(edge);

        let ghost = draft.insert_half_edge(crate::arena::HalfEdgeData::new(
            he_am, he_mb, he_am, face, orig, edge,
        ));
        draft
            .arena_mut()
            .get_half_edge_mut(he_am)
            .unwrap()
            .set_next(ghost);
        draft
            .arena_mut()
            .get_half_edge_mut(he_mb)
            .unwrap()
            .set_prev(ghost);
        draft
            .arena_mut()
            .get_half_edge_mut(he_am)
            .unwrap()
            .set_radial_next(he_mb);
        draft
            .arena_mut()
            .get_half_edge_mut(he_mb)
            .unwrap()
            .set_radial_next(ghost);
        draft
            .arena_mut()
            .get_half_edge_mut(ghost)
            .unwrap()
            .set_radial_next(he_am);

        let result = draft.commit();
        assert!(result.is_err());
    }

    #[test]
    fn adversarial_cross_edge_vertex_continuity() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();

        let mvf1 = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
        let se1 = apply_op(
            &mut draft,
            SplitEdge {
                edge: mvf1.half_edge,
                parameter: 0.5,
            },
        )
        .unwrap()
        .into_value();

        let mvf2 = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
        let se2 = apply_op(
            &mut draft,
            SplitEdge {
                edge: mvf2.half_edge,
                parameter: 0.5,
            },
        )
        .unwrap()
        .into_value();

        draft
            .arena_mut()
            .get_half_edge_mut(se1.he_am)
            .unwrap()
            .set_radial_next(se2.he_am);
        draft
            .arena_mut()
            .get_half_edge_mut(se2.he_am)
            .unwrap()
            .set_radial_next(se1.he_am);

        let result = super::loop_wiring::validate_vertex_continuity(draft.arena());
        assert!(result.is_err());
    }

    #[test]
    fn adversarial_bitset_capacity_after_entity_removal() {
        use crate::topology::bitset::EntityBitset;

        let state = TopologyState::empty();
        let mut draft = state.into_mutation();

        let mvf1 = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
        let mvf2 = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();

        let face1 = draft.arena().get_half_edge(mvf1.half_edge).unwrap().face();
        let face2 = draft.arena().get_half_edge(mvf2.half_edge).unwrap().face();

        draft.arena_mut().remove_face(face1, None).unwrap();

        let bs = EntityBitset::for_faces(draft.arena());
        assert!(bs.capacity() > face2.index());

        let result = super::radial_edge::validate_radial_edge_consistency(draft.arena());
        assert!(result.is_ok());
    }

    #[test]
    fn adversarial_disjoint_rings_sharing_edge_id() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();

        let mvf1 = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
        let se1 = apply_op(
            &mut draft,
            SplitEdge {
                edge: mvf1.half_edge,
                parameter: 0.5,
            },
        )
        .unwrap()
        .into_value();

        let shared_edge_id = draft.arena().get_half_edge(se1.he_am).unwrap().edge();
        draft
            .arena_mut()
            .get_half_edge_mut(se1.he_mb)
            .unwrap()
            .set_edge(shared_edge_id);

        draft
            .arena_mut()
            .get_half_edge_mut(se1.he_am)
            .unwrap()
            .set_radial_next(se1.he_mb);
        draft
            .arena_mut()
            .get_half_edge_mut(se1.he_mb)
            .unwrap()
            .set_radial_next(se1.he_am);

        let mvf2 = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
        let se2 = apply_op(
            &mut draft,
            SplitEdge {
                edge: mvf2.half_edge,
                parameter: 0.5,
            },
        )
        .unwrap()
        .into_value();

        let he2_am = se2.he_am;
        let he2_mb = se2.he_mb;
        let face2 = draft.arena().get_half_edge(he2_am).unwrap().face();
        let orig2 = draft.arena().get_half_edge(he2_am).unwrap().origin();

        draft
            .arena_mut()
            .get_half_edge_mut(he2_am)
            .unwrap()
            .set_edge(shared_edge_id);
        draft
            .arena_mut()
            .get_half_edge_mut(he2_mb)
            .unwrap()
            .set_edge(shared_edge_id);

        let ghost = draft.insert_half_edge(crate::arena::HalfEdgeData::new(
            he2_am,
            he2_mb,
            he2_am,
            face2,
            orig2,
            shared_edge_id,
        ));
        draft
            .arena_mut()
            .get_half_edge_mut(he2_am)
            .unwrap()
            .set_next(ghost);
        draft
            .arena_mut()
            .get_half_edge_mut(he2_mb)
            .unwrap()
            .set_prev(ghost);

        draft
            .arena_mut()
            .get_half_edge_mut(he2_am)
            .unwrap()
            .set_radial_next(he2_mb);
        draft
            .arena_mut()
            .get_half_edge_mut(he2_mb)
            .unwrap()
            .set_radial_next(ghost);
        draft
            .arena_mut()
            .get_half_edge_mut(ghost)
            .unwrap()
            .set_radial_next(he2_am);

        let result = validate_topology(draft.arena(), ValidationLevel::Full);
        assert!(result.is_err());
    }
}
