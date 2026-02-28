//! Topology validation — Public API.
//!
//! DOMAIN: Re-exports from `structural` and `geometric` submodules.
//! This module owns `ValidationLevel` and the inline tests.
//!
//! DEPENDENCIES: `structural`, `geometric`

/// Validation strictness level.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationLevel {
    /// No checks. Trust the operations blindly (fastest).
    None,
    /// Only check local connectivity invariants (twins, prev/next).
    /// Used for Release builds.
    Minimal,
    /// Intermediate structural checks. Allows temporary non-manifold
    /// topology mid-boolean but checks basic pointer coherence.
    Intermediate,
    /// Full global validity checks (Euler formula, loop closure,
    /// shell-kind-aware manifold enforcement).
    /// Used for Debug/Test builds.
    Full,
}

impl Default for ValidationLevel {
    fn default() -> Self {
        if cfg!(debug_assertions) {
            ValidationLevel::Full
        } else {
            ValidationLevel::Minimal
        }
    }
}

pub use super::structural::validate_manifold_edges;
pub use super::structural::validate_topology;
/// Geometric invariant validation has moved to `forge-spatial::integrity`.
/// Build with forge-spatial as a dependency and call `forge_spatial::integrity::validate_geometric_invariants`.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::arena::TopologyArena;
    use crate::euler::make_vertex_face::MakeVertexFace;
    use crate::euler::split_edge::SplitEdge;
    use crate::handles::HalfEdgeId;
    use crate::operator::apply_op;
    use crate::state::TopologyState;
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

    /// Shell-kind-aware validation rejects valence-3 edges on Sheet shells.
    ///
    /// MVF creates Sheet shells. A valence-3 radial ring violates the
    /// manifold constraint for sheets (max valence 2).
    #[test]
    fn sheet_shell_rejects_valence_3_edge() {
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

        let result = validate_manifold_edges(draft.arena());
        assert!(
            result.is_err(),
            "Sheet shell must reject a valence-3 edge"
        );
        let err = result.unwrap_err();
        assert!(
            matches!(err, KernelError::TopologyViolation { .. }),
            "Expected TopologyViolation from validate_manifold_edges, got: {:?}",
            err
        );
    }

    /// Intermediate validation level skips manifold checks.
    ///
    /// Same valence-3 wiring as above, but validated at Intermediate level
    /// which does NOT run `validate_manifold_edges`. All structural checks
    /// (radial ring closure, prev/next, loop closure, hierarchy) pass.
    #[test]
    fn intermediate_level_skips_manifold_check() {
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

        let result = validate_topology(draft.arena(), ValidationLevel::Intermediate);
        assert!(
            result.is_ok(),
            "Intermediate level must accept valence-3 edge (manifold check skipped): {:?}",
            result.err()
        );
    }

    /// Broken radial rings are always rejected, even at Minimal level.
    #[test]
    fn broken_radial_ring_rejected_at_minimal() {
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

        let result = validate_topology(draft.arena(), ValidationLevel::Minimal);
        assert!(
            result.is_err(),
            "Broken radial ring must be rejected even at Minimal level"
        );
        assert!(
            matches!(result.unwrap_err(), KernelError::TopologyViolation { .. }),
            "Expected TopologyViolation for broken ring"
        );
    }

    /// Default commit() on valid topology passes.
    #[test]
    fn default_commit_passes_valid_topology() {
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
        assert!(
            draft.commit().is_ok(),
            "Default commit on valid topology must pass"
        );
    }

    // ── Adversarial Test Suite ──────────────────────────────────────────

    /// ADVERSARIAL: Edge-entity inconsistency in radial ring.
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

        let edge_am = draft.arena().get_half_edge(he_am).unwrap().edge();
        let edge_mb = draft.arena().get_half_edge(he_mb).unwrap().edge();
        assert_ne!(edge_am, edge_mb, "Test precondition: distinct edge entities");

        let result = validate_topology(draft.arena(), ValidationLevel::Full);
        assert!(
            result.is_err(),
            "Radial ring with mismatched edge entities must be rejected"
        );
    }

    /// validate_manifold_edges directly catches valence-3 on Sheet shell.
    #[test]
    fn validate_manifold_edges_catches_valence_3_directly() {
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

        let result = validate_manifold_edges(draft.arena());
        assert!(
            result.is_err(),
            "validate_manifold_edges must catch valence-3 on Sheet shell"
        );
        assert!(
            matches!(
                result.unwrap_err(),
                KernelError::TopologyViolation {
                    err: forge_core::TopologyError::NonManifoldEdge { .. },
                    ..
                }
            ),
            "Expected NonManifoldEdge error"
        );
    }

    /// ADVERSARIAL: validate_vertex_continuity with cross-edge radial ring.
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

        let orig1 = draft.arena().get_half_edge(se1.he_am).unwrap().origin();
        let orig2 = draft.arena().get_half_edge(se2.he_am).unwrap().origin();
        assert_ne!(orig1, orig2, "Test precondition: different origin vertices");

        let result =
            crate::topology::validators::structural::validate_vertex_continuity(draft.arena());
        assert!(
            result.is_err(),
            "Cross-edge radial ring must be caught by validate_vertex_continuity"
        );
    }

    /// ADVERSARIAL: EntityBitset capacity after entity removal (arena gaps).
    #[test]
    fn adversarial_bitset_capacity_after_entity_removal() {
        use crate::topology::bitset::EntityBitset;

        let state = TopologyState::empty();
        let mut draft = state.into_mutation();

        let mvf1 = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
        let mvf2 = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();

        let face1 = draft.arena().get_half_edge(mvf1.half_edge).unwrap().face();
        let face2 = draft.arena().get_half_edge(mvf2.half_edge).unwrap().face();

        assert_ne!(face1, face2);
        assert_eq!(draft.arena().face_count(), 2);

        draft.arena_mut().remove_face(face1, None).unwrap();

        assert_eq!(draft.arena().face_count(), 1, "One face remains");

        let bs = EntityBitset::for_faces(draft.arena());
        assert!(
            bs.capacity() > face2.index(),
            "Bitset capacity {} must cover remaining face index {}",
            bs.capacity(),
            face2.index()
        );

        let result =
            crate::topology::validators::structural::validate_radial_edge_consistency(draft.arena());
        assert!(
            result.is_ok(),
            "Validation should not panic on missing indices"
        );
    }

    /// ADVERSARIAL: Disjoint radial rings sharing the same EdgeId.
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
            he2_am, he2_mb, he2_am, face2, orig2, shared_edge_id,
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

        let result = validate_manifold_edges(draft.arena());
        assert!(
            result.is_err(),
            "Disjoint radial rings sharing EdgeId must be rejected by validate_manifold_edges"
        );
    }
}
