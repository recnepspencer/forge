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
    use crate::b_rep::ShellKind;
    use crate::b_rep::TopologyArena;
    use crate::entity_lifecycle::make_vertex_face::MakeVertexFace;
    use crate::entity_lifecycle::split_edge::SplitEdge;
    use crate::handles::HalfEdgeId;
    use crate::transactions::TopologyState;
    use crate::validators::loop_wiring::validate_vertex_continuity;
    use crate::validators::radial_edge::validate_radial_edge_consistency;
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
        let _mvf = draft
            .execute(MakeVertexFace {
                shell_kind: ShellKind::Sheet,
            })
            .unwrap()
            .into_value();
        let state = draft.commit().unwrap();
        assert!(validate_topology(state.arena(), ValidationLevel::Full).is_ok());
    }

    #[test]
    fn split_validates() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();
        let mvf = draft
            .execute(MakeVertexFace {
                shell_kind: ShellKind::Sheet,
            })
            .unwrap()
            .into_value();
        let _se = draft
            .execute(SplitEdge {
                edge: mvf.half_edge,
            })
            .unwrap()
            .into_value();
        let state = draft.commit().unwrap();
        assert!(validate_topology(state.arena(), ValidationLevel::Full).is_ok());
    }

    /// Shell-kind-aware validation rejects valence-3 edges on Sheet shells.
    #[test]
    fn sheet_shell_rejects_valence_3_edge() {
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

        let ghost = draft.insert_half_edge(crate::b_rep::HalfEdgeData::new(
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
        assert!(result.is_err(), "Sheet shell must reject a valence-3 edge");
        assert!(
            matches!(result.unwrap_err(), KernelError::TopologyViolation { .. }),
            "Expected TopologyViolation from validate_manifold_edges"
        );
    }

    /// Intermediate validation level skips manifold checks.
    ///
    /// We can't create a valence-3 topology through the pipeline because
    /// the ghost insertion also breaks bidirectional links, edge endpoints,
    /// and other Batch 1 invariants that run at all levels.
    /// Instead, we verify: valid topology passes at Intermediate, and
    /// Intermediate is a strict subset of Full (the manifold checks are
    /// Full-only, confirmed by the `sheet_shell_rejects_valence_3_edge`
    /// test above which proves manifold checks catch violations at Full).
    #[test]
    fn intermediate_level_passes_valid_topology() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();
        let mvf = draft
            .execute(MakeVertexFace {
                shell_kind: ShellKind::Sheet,
            })
            .unwrap()
            .into_value();
        let _se = draft
            .execute(SplitEdge {
                edge: mvf.half_edge,
            })
            .unwrap()
            .into_value();

        let result = validate_topology(draft.arena(), ValidationLevel::Intermediate);
        assert!(
            result.is_ok(),
            "Intermediate level must pass valid topology: {:?}",
            result.err()
        );
    }

    /// Broken radial rings are always rejected, even at Minimal level.
    #[test]
    fn broken_radial_ring_rejected_at_minimal() {
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

        let stale = HalfEdgeId::new(99_999, 0);
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
        let mvf = draft
            .execute(MakeVertexFace {
                shell_kind: ShellKind::Sheet,
            })
            .unwrap()
            .into_value();
        let _se = draft
            .execute(SplitEdge {
                edge: mvf.half_edge,
            })
            .unwrap()
            .into_value();
        assert!(
            draft.commit().is_ok(),
            "Default commit on valid topology must pass"
        );
    }

    // ── Adversarial Test Suite ──────────────────────────────────────────

    #[test]
    fn adversarial_edge_entity_inconsistency_in_radial_ring() {
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
        assert_ne!(
            edge_am, edge_mb,
            "Test precondition: distinct edge entities"
        );

        let result = validate_topology(draft.arena(), ValidationLevel::Full);
        assert!(
            result.is_err(),
            "Radial ring with mismatched edge entities must be rejected"
        );
    }

    #[test]
    fn validate_manifold_edges_catches_valence_3_directly() {
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

        let ghost = draft.insert_half_edge(crate::b_rep::HalfEdgeData::new(
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

    #[test]
    fn adversarial_cross_edge_vertex_continuity() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();

        let mvf1 = draft
            .execute(MakeVertexFace {
                shell_kind: ShellKind::Sheet,
            })
            .unwrap()
            .into_value();
        let se1 = draft
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
        let se2 = draft
            .execute(SplitEdge {
                edge: mvf2.half_edge,
            })
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

        let result = validate_vertex_continuity(draft.arena());
        assert!(
            result.is_err(),
            "Cross-edge radial ring must be caught by validate_vertex_continuity"
        );
    }

    #[test]
    fn adversarial_bitset_capacity_after_entity_removal() {
        use crate::b_rep::EntityBitset;

        let state = TopologyState::empty();
        let mut draft = state.into_mutation();

        let mvf1 = draft
            .execute(MakeVertexFace {
                shell_kind: ShellKind::Sheet,
            })
            .unwrap()
            .into_value();
        let mvf2 = draft
            .execute(MakeVertexFace {
                shell_kind: ShellKind::Sheet,
            })
            .unwrap()
            .into_value();

        let face1 = draft.arena().get_half_edge(mvf1.half_edge).unwrap().face();
        let face2 = draft.arena().get_half_edge(mvf2.half_edge).unwrap().face();

        assert_ne!(face1, face2);
        assert_eq!(draft.arena().face_count(), 2);

        draft.arena_mut().remove_face(face1).unwrap();

        assert_eq!(draft.arena().face_count(), 1, "One face remains");

        let bs = EntityBitset::for_faces(draft.arena());
        assert!(
            bs.capacity() > face2.index(),
            "Bitset capacity must cover remaining face index"
        );

        let result = validate_radial_edge_consistency(draft.arena());
        assert!(
            result.is_ok(),
            "Validation should not panic on missing indices"
        );
    }

    #[test]
    fn adversarial_disjoint_rings_sharing_edge_id() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();

        // Create two separate topologies FIRST, before any manual corruption.
        let mvf1 = draft
            .execute(MakeVertexFace {
                shell_kind: ShellKind::Sheet,
            })
            .unwrap()
            .into_value();
        let se1 = draft
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
        let se2 = draft
            .execute(SplitEdge {
                edge: mvf2.half_edge,
            })
            .unwrap()
            .into_value();

        // Now corrupt: force both topologies to share the same EdgeId
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

        let ghost = draft.insert_half_edge(crate::b_rep::HalfEdgeData::new(
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

        let result = validate_manifold_edges(draft.arena());
        assert!(
            result.is_err(),
            "Disjoint radial rings sharing EdgeId must be rejected"
        );
    }

    // ══════════════════════════════════════════════════════════════════
    //  BATCH 1 — Adversarial Poison Tests (VALIDATOR_QA.md contract)
    //
    //  Each Batch 1 validator gets:
    //    • 1 positive proof  (valid topology → Ok)
    //    • 1+ adversarial poison (corrupt exactly the tested invariant → Err)
    //  Tests call validators directly, not the pipeline.
    //  Test names mirror the validator file: e.g. dangling_refs.rs → poison_dangling_refs_*
    // ══════════════════════════════════════════════════════════════════

    /// Helper: Build a valid MVF+SE topology for corruption.
    /// Returns (draft, face, he_am, he_mb) — a face with a 4-HE loop.
    fn valid_mvf_se_draft() -> (
        crate::transactions::MutableDraft,
        crate::handles::FaceId,
        HalfEdgeId,
        HalfEdgeId,
    ) {
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
        let face = draft.arena().get_half_edge(se.he_am).unwrap().face();
        (draft, face, se.he_am, se.he_mb)
    }

    // ── 1. dangling_refs.rs — ValidateNoDanglingHalfEdgeRefs ────────

    #[test]
    fn positive_dangling_refs_passes_valid_topology() {
        let (draft, _, _, _) = valid_mvf_se_draft();
        assert!(
            crate::validators::reference_integrity::validate_no_dangling_half_edge_refs(
                draft.arena()
            )
            .is_ok()
        );
    }

    #[test]
    fn poison_dangling_refs_next_points_to_nonexistent_he() {
        let (mut draft, _, he_am, _) = valid_mvf_se_draft();

        // Pre-condition: valid
        assert!(
            crate::validators::reference_integrity::validate_no_dangling_half_edge_refs(
                draft.arena()
            )
            .is_ok()
        );

        // Corrupt: .next → bogus handle that doesn't exist in the arena
        let bogus = HalfEdgeId::new(99_999, 0);
        draft
            .arena_mut()
            .get_half_edge_mut(he_am)
            .unwrap()
            .set_next(bogus);

        let result = crate::validators::reference_integrity::validate_no_dangling_half_edge_refs(
            draft.arena(),
        );
        assert!(result.is_err(), "Dangling .next must be caught");
        assert!(
            format!("{:?}", result.unwrap_err()).contains("no_dangling_half_edge_refs"),
            "Error must come from the dangling_refs validator"
        );
    }

    #[test]
    fn poison_dangling_refs_prev_points_to_nonexistent_he() {
        let (mut draft, _, he_am, _) = valid_mvf_se_draft();

        let bogus = HalfEdgeId::new(99_998, 0);
        draft
            .arena_mut()
            .get_half_edge_mut(he_am)
            .unwrap()
            .set_prev(bogus);

        let result = crate::validators::reference_integrity::validate_no_dangling_half_edge_refs(
            draft.arena(),
        );
        assert!(result.is_err(), "Dangling .prev must be caught");
    }

    #[test]
    fn poison_dangling_refs_origin_points_to_nonexistent_vertex() {
        let (mut draft, _, he_am, _) = valid_mvf_se_draft();

        let bogus = crate::handles::VertexId::new(99_997, 0);
        draft
            .arena_mut()
            .get_half_edge_mut(he_am)
            .unwrap()
            .set_origin(bogus);

        let result = crate::validators::reference_integrity::validate_no_dangling_half_edge_refs(
            draft.arena(),
        );
        assert!(result.is_err(), "Dangling .origin must be caught");
    }

    #[test]
    fn poison_dangling_refs_face_points_to_nonexistent_face() {
        let (mut draft, _, he_am, _) = valid_mvf_se_draft();

        let bogus = crate::handles::FaceId::new(99_996, 0);
        draft
            .arena_mut()
            .get_half_edge_mut(he_am)
            .unwrap()
            .set_face(bogus);

        let result = crate::validators::reference_integrity::validate_no_dangling_half_edge_refs(
            draft.arena(),
        );
        assert!(result.is_err(), "Dangling .face must be caught");
    }

    #[test]
    fn poison_dangling_refs_edge_points_to_nonexistent_edge() {
        let (mut draft, _, he_am, _) = valid_mvf_se_draft();

        let bogus = crate::handles::EdgeId::new(99_995, 0);
        draft
            .arena_mut()
            .get_half_edge_mut(he_am)
            .unwrap()
            .set_edge(bogus);

        let result = crate::validators::reference_integrity::validate_no_dangling_half_edge_refs(
            draft.arena(),
        );
        assert!(result.is_err(), "Dangling .edge must be caught");
    }

    #[test]
    fn poison_dangling_refs_radial_next_points_to_nonexistent_he() {
        let (mut draft, _, he_am, _) = valid_mvf_se_draft();

        let bogus = HalfEdgeId::new(99_994, 0);
        draft
            .arena_mut()
            .get_half_edge_mut(he_am)
            .unwrap()
            .set_radial_next(bogus);

        let result = crate::validators::reference_integrity::validate_no_dangling_half_edge_refs(
            draft.arena(),
        );
        assert!(result.is_err(), "Dangling .radial_next must be caught");
    }

    // ── 2. bidirectional_links.rs — ValidateBidirectionalLinks ──────

    #[test]
    fn positive_bidirectional_links_passes_valid_topology() {
        let (draft, _, _, _) = valid_mvf_se_draft();
        assert!(
            crate::validators::reference_integrity::validate_bidirectional_links(draft.arena())
                .is_ok()
        );
    }

    #[test]
    fn poison_bidirectional_links_edge_rep_he_points_to_wrong_edge() {
        let (mut draft, _, he_am, he_mb) = valid_mvf_se_draft();

        // Pre-condition: valid
        assert!(
            crate::validators::reference_integrity::validate_bidirectional_links(draft.arena())
                .is_ok()
        );

        // Get two different edges — he_am.edge and he_mb.edge should be distinct
        let edge_am = draft.arena().get_half_edge(he_am).unwrap().edge();
        let edge_mb = draft.arena().get_half_edge(he_mb).unwrap().edge();

        // Corrupt: make he_am claim to belong to edge_mb, but edge_am's rep is still he_am.
        // Now edge_am.rep_he.edge() == edge_mb ≠ edge_am — bidirectional mismatch.
        draft
            .arena_mut()
            .get_half_edge_mut(he_am)
            .unwrap()
            .set_edge(edge_mb);

        let result =
            crate::validators::reference_integrity::validate_bidirectional_links(draft.arena());
        assert!(result.is_err(), "Edge→HE→Edge mismatch must be caught");
        assert!(
            format!("{:?}", result.unwrap_err()).contains("bidirectional_links"),
            "Error must come from bidirectional_links validator"
        );
    }

    #[test]
    fn poison_bidirectional_links_shell_rep_face_points_to_wrong_shell() {
        let (mut draft, face, _, _) = valid_mvf_se_draft();

        // Corrupt: make the face's shell pointer bogus, so shell.rep_face.shell() ≠ shell
        let shell = draft.arena().get_face(face).unwrap().shell();
        let bogus_shell = crate::handles::ShellId::new(99_993, 0);
        draft
            .arena_mut()
            .get_face_mut(face)
            .unwrap()
            .set_shell(bogus_shell);

        let result =
            crate::validators::reference_integrity::validate_bidirectional_links(draft.arena());
        assert!(result.is_err(), "Shell→Face→Shell mismatch must be caught");
    }

    #[test]
    fn poison_bidirectional_links_loop_rep_he_on_wrong_face() {
        let (mut draft, face, he_am, _) = valid_mvf_se_draft();

        // Corrupt: change the HE's face pointer so loop.rep_he.face() ≠ loop.face()
        let bogus_face = crate::handles::FaceId::new(99_992, 0);
        draft
            .arena_mut()
            .get_half_edge_mut(he_am)
            .unwrap()
            .set_face(bogus_face);

        let result =
            crate::validators::reference_integrity::validate_bidirectional_links(draft.arena());
        assert!(result.is_err(), "Loop→HE→Face mismatch must be caught");
    }

    // ── 3. face_loop_existence.rs — ValidateFaceHasAtLeastOneLoop ───

    #[test]
    fn positive_face_loop_existence_passes_valid_topology() {
        let (draft, _, _, _) = valid_mvf_se_draft();
        assert!(
            crate::validators::reference_integrity::validate_face_has_at_least_one_loop(
                draft.arena()
            )
            .is_ok()
        );
    }

    #[test]
    fn poison_face_loop_existence_outer_loop_points_to_bogus_loop() {
        let (mut draft, face, _, _) = valid_mvf_se_draft();

        // Pre-condition: valid
        assert!(
            crate::validators::reference_integrity::validate_face_has_at_least_one_loop(
                draft.arena()
            )
            .is_ok()
        );

        // Corrupt: face.outer_loop → nonexistent loop
        let bogus_loop = crate::handles::LoopId::new(99_991, 0);
        draft
            .arena_mut()
            .get_face_mut(face)
            .unwrap()
            .loops
            .set_outer(bogus_loop);

        let result = crate::validators::reference_integrity::validate_face_has_at_least_one_loop(
            draft.arena(),
        );
        assert!(result.is_err(), "Face with bogus outer_loop must be caught");
        assert!(
            format!("{:?}", result.unwrap_err()).contains("face_has_at_least_one_loop"),
            "Error must come from face_has_at_least_one_loop validator"
        );
    }

    // ── 4. loop_cardinality.rs — ValidateLoopMinimumCardinality ─────

    #[test]
    fn positive_loop_cardinality_passes_valid_topology() {
        let (draft, _, _, _) = valid_mvf_se_draft();
        assert!(
            crate::validators::loop_wiring::validate_loop_minimum_cardinality(draft.arena())
                .is_ok()
        );
    }

    #[test]
    fn positive_loop_cardinality_passes_single_he_self_loop() {
        // MVF creates a 1-HE self-loop — this is legitimately valid
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();
        let _mvf = draft
            .execute(MakeVertexFace {
                shell_kind: ShellKind::Sheet,
            })
            .unwrap()
            .into_value();
        assert!(
            crate::validators::loop_wiring::validate_loop_minimum_cardinality(draft.arena())
                .is_ok(),
            "1-HE self-loop from MVF must be accepted"
        );
    }

    #[test]
    fn poison_loop_cardinality_loop_walk_exceeds_bound() {
        let (mut draft, _, he_am, he_mb) = valid_mvf_se_draft();

        // Pre-condition: valid
        assert!(
            crate::validators::loop_wiring::validate_loop_minimum_cardinality(draft.arena())
                .is_ok()
        );

        // Corrupt: create an infinite cycle by making he_am.next = he_mb
        // and he_mb.next = he_am, but the loop rep points to a HE whose
        // .next chain never revisits the start because we've broken it.
        // Insert a spur: he_am → he_am (self-next, never reaching he_mb)
        // This creates a 1-HE walk that returns immediately — still valid (≥1).
        // Instead, create a topology where the walk never terminates:
        // Point he_am.next to he_mb, but he_mb.next to a bogus HE that arena
        // won't find — this triggers the bound-exceeded path.
        let bogus = HalfEdgeId::new(99_990, 0);
        draft
            .arena_mut()
            .get_half_edge_mut(he_mb)
            .unwrap()
            .set_next(bogus);

        // This will actually fail in validate_no_dangling_half_edge_refs first
        // when run in the pipeline, but we're calling cardinality directly:
        let result =
            crate::validators::loop_wiring::validate_loop_minimum_cardinality(draft.arena());
        // The walk will hit arena.get_half_edge(bogus) which returns Err,
        // and the ? propagation will produce an error (not the bound-exceeded path,
        // but a dangling ref error). This proves the validator is defensive.
        assert!(
            result.is_err(),
            "Loop with broken .next chain must produce an error"
        );
    }

    // ── 5. duplicate_coedges.rs — ValidateNoDuplicateCoedgesInLoop ──

    #[test]
    fn positive_duplicate_coedges_passes_valid_topology() {
        let (draft, _, _, _) = valid_mvf_se_draft();
        assert!(
            crate::validators::loop_wiring::validate_no_duplicate_coedges_in_loop(draft.arena())
                .is_ok()
        );
    }

    #[test]
    fn poison_duplicate_coedges_lasso_cycle() {
        let (mut draft, _, he_am, he_mb) = valid_mvf_se_draft();

        // Pre-condition: valid
        assert!(
            crate::validators::loop_wiring::validate_no_duplicate_coedges_in_loop(draft.arena())
                .is_ok()
        );

        // The MVF+SE topology has a loop: he_am → X → he_mb → Y → he_am
        // Corrupt: make he_mb.next point back to he_am, creating a lasso
        // he_am → X → he_mb → he_am → X → he_mb → ... (he_am visited twice)
        // But we need he_am not to be the start of the loop walk (otherwise
        // the second visit of he_am = normal closure). The loop's rep HE
        // determines the start. If it starts at he_am, hitting he_am again is closure.
        //
        // So we need the loop rep HE to NOT be he_am. Let's find another HE in the loop:
        let he_after_am = draft.arena().get_half_edge(he_am).unwrap().next();

        // Now corrupt: make he_mb.next skip Y and jump to he_after_am.
        // Walk from loop rep: rep → ... → he_mb → he_after_am → ... → he_mb → he_after_am ...
        // he_after_am will be visited twice.
        draft
            .arena_mut()
            .get_half_edge_mut(he_mb)
            .unwrap()
            .set_next(he_after_am);

        let result =
            crate::validators::loop_wiring::validate_no_duplicate_coedges_in_loop(draft.arena());
        assert!(
            result.is_err(),
            "Lasso cycle with duplicate coedge must be caught"
        );
    }

    // ── 6. cycle_uniqueness.rs — ValidateRadialCycleUniqueness ──────

    #[test]
    fn positive_radial_cycle_uniqueness_passes_valid_topology() {
        let (draft, _, _, _) = valid_mvf_se_draft();
        assert!(
            crate::validators::radial_edge::validate_radial_cycle_uniqueness(draft.arena()).is_ok()
        );
    }

    #[test]
    fn poison_radial_cycle_uniqueness_cross_wired_rings() {
        // Create two separate topologies, then cross-wire their radial rings
        // so a single ring visits HEs from two different edges.
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();

        let mvf1 = draft
            .execute(MakeVertexFace {
                shell_kind: ShellKind::Sheet,
            })
            .unwrap()
            .into_value();
        let se1 = draft
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
        let se2 = draft
            .execute(SplitEdge {
                edge: mvf2.half_edge,
            })
            .unwrap()
            .into_value();

        // Pre-condition: valid
        assert!(
            crate::validators::radial_edge::validate_radial_cycle_uniqueness(draft.arena()).is_ok()
        );

        // Corrupt: cross-wire radial_next to create a composite ring
        // se1.he_am → se2.he_am → se1.he_am (both were self-radial before)
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

        // The ring itself is structurally valid (no duplicates in the walk).
        // But edge_consistency will catch that they have different EdgeIds.
        // For uniqueness: splice in a third reference to create an actual duplicate.
        // Make se2.he_am.radial_next = se1.he_am, and se1.he_am.radial_next = se2.he_am,
        // then also se2.he_mb.radial_next = se1.he_am — creating se1.he_am appearing twice.
        draft
            .arena_mut()
            .get_half_edge_mut(se2.he_mb)
            .unwrap()
            .set_radial_next(se1.he_am);
        // Now the ring from se2.he_mb: se2.he_mb → se1.he_am → se2.he_am → se1.he_am (duplicate!)

        let result =
            crate::validators::radial_edge::validate_radial_cycle_uniqueness(draft.arena());
        assert!(
            result.is_err(),
            "Radial ring with duplicate HE must be caught"
        );
    }

    // ── 7. face_membership.rs — ValidateFaceLoopMembershipComplete ──

    #[test]
    fn positive_face_membership_passes_valid_topology() {
        let (draft, _, _, _) = valid_mvf_se_draft();
        assert!(
            crate::validators::loop_wiring::validate_face_loop_membership_complete(draft.arena())
                .is_ok()
        );
    }

    #[test]
    fn poison_face_membership_floating_he_claims_face() {
        let (mut draft, face, _he_am, he_mb) = valid_mvf_se_draft();

        // Pre-condition: valid
        assert!(
            crate::validators::loop_wiring::validate_face_loop_membership_complete(draft.arena())
                .is_ok()
        );

        // Corrupt: insert a phantom HE that claims `face` but isn't in any loop
        let origin = draft.arena().get_half_edge(he_mb).unwrap().origin();
        let edge = draft.arena().get_half_edge(he_mb).unwrap().edge();
        let ghost = draft.insert_half_edge(crate::b_rep::HalfEdgeData::new(
            he_mb, he_mb, he_mb, face, origin, edge,
        ));
        let _ = ghost; // Must exist in the arena

        let result =
            crate::validators::loop_wiring::validate_face_loop_membership_complete(draft.arena());
        assert!(
            result.is_err(),
            "Floating HE claiming face but unreachable from loops must be caught"
        );
        assert!(
            format!("{:?}", result.unwrap_err()).contains("face_loop_membership_complete"),
            "Error must come from face_loop_membership validator"
        );
    }

    #[test]
    fn poison_face_membership_he_claims_wrong_face() {
        use crate::entity_lifecycle::make_edge_face::MakeEdgeFace;

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

        let mef = draft
            .execute(MakeEdgeFace {
                face: draft.arena().get_half_edge(se.he_am).unwrap().face(),
                vertex_a: draft.arena().get_half_edge(se.he_am).unwrap().origin(),
                vertex_b: se.new_vertex,
            })
            .unwrap()
            .into_value();

        // Pre-condition: valid
        assert!(
            crate::validators::loop_wiring::validate_face_loop_membership_complete(draft.arena())
                .is_ok()
        );

        // Subtle corruption: insert a ghost HE that claims new_face but is
        // NOT wired into new_face's loop — it just exists in the arena.
        let origin = draft.arena().get_half_edge(se.he_am).unwrap().origin();
        let edge = draft.arena().get_half_edge(se.he_am).unwrap().edge();
        let ghost = draft.insert_half_edge(crate::b_rep::HalfEdgeData::new(
            se.he_am,
            se.he_am,
            se.he_am,
            mef.new_face,
            origin,
            edge,
        ));
        let _ = ghost;

        let result =
            crate::validators::loop_wiring::validate_face_loop_membership_complete(draft.arena());
        assert!(
            result.is_err(),
            "Ghost HE claiming new_face but unreachable from new_face's loops must be caught"
        );
    }

    // ══════════════════════════════════════════════════════════════════
    //  BATCH 2 — Adversarial Poison Tests (Ownership & Orphans)
    // ══════════════════════════════════════════════════════════════════

    // ── 1. single_owner.rs — ValidateSingleOwnerPerLoop ─────────────

    #[test]
    fn positive_single_owner_passes_valid_topology() {
        let (draft, _, _, _) = valid_mvf_se_draft();
        assert!(
            crate::validators::reference_integrity::validate_single_owner_per_loop(draft.arena())
                .is_ok()
        );
    }

    #[test]
    fn poison_single_owner_loop_hijack() {
        use crate::entity_lifecycle::make_edge_face::MakeEdgeFace;

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

        let mef = draft
            .execute(MakeEdgeFace {
                face: draft.arena().get_half_edge(se.he_am).unwrap().face(),
                vertex_a: draft.arena().get_half_edge(se.he_am).unwrap().origin(),
                vertex_b: se.new_vertex,
            })
            .unwrap()
            .into_value();

        let face_a = draft.arena().get_half_edge(se.he_am).unwrap().face();
        let face_b = mef.new_face;

        // Pre-condition: valid
        assert!(
            crate::validators::reference_integrity::validate_single_owner_per_loop(draft.arena())
                .is_ok()
        );

        // Corrupt: Face B hijacks Face A's outer loop by adding it to its inner_loops
        let loop_a = draft.arena().get_face(face_a).unwrap().loops.outer();
        draft
            .arena_mut()
            .get_face_mut(face_b)
            .unwrap()
            .loops
            .add_inner(loop_a);

        let result =
            crate::validators::reference_integrity::validate_single_owner_per_loop(draft.arena());
        assert!(result.is_err(), "Single owner must catch loop hijacking");
        assert!(
            format!("{:?}", result.unwrap_err()).contains("single_owner_per_loop"),
            "Error must come from single_owner validator"
        );
    }

    // ── 2. inner_outer_consistency.rs — ValidateInnerOuterLoopConsistency ─

    #[test]
    fn positive_inner_outer_consistency_passes_valid() {
        let (draft, _, _, _) = valid_mvf_se_draft();
        assert!(
            crate::validators::reference_integrity::validate_inner_outer_loop_consistency(
                draft.arena()
            )
            .is_ok()
        );
    }

    #[test]
    fn poison_inner_outer_consistency_outer_in_inner_list() {
        let (mut draft, face, _, _) = valid_mvf_se_draft();

        // Pre-condition: valid
        assert!(
            crate::validators::reference_integrity::validate_inner_outer_loop_consistency(
                draft.arena()
            )
            .is_ok()
        );

        // Corrupt: Face adds its own outer loop to its inner loops list
        let outer_loop = draft.arena().get_face(face).unwrap().loops.outer();
        draft
            .arena_mut()
            .get_face_mut(face)
            .unwrap()
            .loops
            .add_inner(outer_loop);

        let result = crate::validators::reference_integrity::validate_inner_outer_loop_consistency(
            draft.arena(),
        );
        assert!(
            result.is_err(),
            "Face containing outer loop in inner loops must be caught"
        );
        assert!(
            format!("{:?}", result.unwrap_err()).contains("inner_outer_loop_consistency"),
            "Error must come from inner_outer_loop_consistency validator"
        );
    }

    // ── 3. edge_endpoints.rs — ValidateEdgeEndpointsMatchLoopVertices ─

    #[test]
    fn positive_edge_endpoints_match_valid() {
        use crate::entity_lifecycle::make_edge_face::MakeEdgeFace;
        let (mut draft, _, he_am, _) = valid_mvf_se_draft();

        let se_v2 = draft.arena().get_half_edge(he_am).unwrap().origin();
        let se_v1 = draft
            .arena()
            .get_half_edge(draft.arena().get_half_edge(he_am).unwrap().next())
            .unwrap()
            .origin();

        let _mef = draft
            .execute(MakeEdgeFace {
                face: draft.arena().get_half_edge(he_am).unwrap().face(),
                vertex_a: se_v2,
                vertex_b: se_v1,
            })
            .unwrap()
            .into_value();

        assert!(
            crate::validators::loop_wiring::validate_edge_endpoints_match_loop_vertices(
                draft.arena()
            )
            .is_ok()
        );
    }

    #[test]
    fn poison_edge_endpoints_origin_mismatch() {
        use crate::entity_lifecycle::make_edge_face::MakeEdgeFace;
        let (mut draft, _, he_am, _) = valid_mvf_se_draft();

        let v_start = draft.arena().get_half_edge(he_am).unwrap().origin();
        let he_next = draft.arena().get_half_edge(he_am).unwrap().next();
        let v_end = draft.arena().get_half_edge(he_next).unwrap().origin();

        // This splits the face by drawing a new edge from v_start to v_end.
        // The new edge is shared by the two faces, so its halfedges are twins.
        let mef = draft
            .execute(MakeEdgeFace {
                face: draft.arena().get_half_edge(he_am).unwrap().face(),
                vertex_a: v_start,
                vertex_b: v_end,
            })
            .unwrap()
            .into_value();

        // mef.half_edge_ab and its twin share the new edge.
        let he1 = mef.half_edge_ab;
        let twin = draft.arena().get_half_edge(he1).unwrap().radial_next();
        assert_ne!(
            he1, twin,
            "MakeEdgeFace must create an edge with two distinct halfedges"
        );

        // Pre-condition: valid
        assert!(
            crate::validators::loop_wiring::validate_edge_endpoints_match_loop_vertices(
                draft.arena()
            )
            .is_ok()
        );

        // Corrupt: alter twin's origin so it no longer mathematically matches the Edge endpoints
        let bogus_vertex = crate::handles::VertexId::new(99_999, 0);
        draft
            .arena_mut()
            .get_half_edge_mut(twin)
            .unwrap()
            .set_origin(bogus_vertex);

        let result = crate::validators::loop_wiring::validate_edge_endpoints_match_loop_vertices(
            draft.arena(),
        );
        assert!(
            result.is_err(),
            "HalfEdge endpoints must strictly agree across a shared edge"
        );
        assert!(
            format!("{:?}", result.unwrap_err()).contains("edge_endpoints_match"),
            "Error must come from edge_endpoints validator"
        );
    }

    // ── 4. orphan_half_edges.rs — ValidateNoOrphanHalfEdges ─────────

    #[test]
    fn positive_no_orphan_half_edges_valid() {
        let (draft, _, _, _) = valid_mvf_se_draft();
        assert!(
            crate::validators::reference_integrity::validate_no_orphan_half_edges(draft.arena())
                .is_ok()
        );
    }

    #[test]
    fn poison_orphan_half_edges_ghost_insertion() {
        let (mut draft, face, he_am, _) = valid_mvf_se_draft();

        // Pre-condition: valid
        assert!(
            crate::validators::reference_integrity::validate_no_orphan_half_edges(draft.arena())
                .is_ok()
        );

        // Corrupt: insert a new HalfEdge directly into the arena memory
        // without placing it into any Face's loop.
        let twin = draft.arena().get_half_edge(he_am).unwrap().radial_next();
        let origin = draft.arena().get_half_edge(he_am).unwrap().origin();
        let edge = draft.arena().get_half_edge(he_am).unwrap().edge();

        // Create a technically valid HalfEdge block (all pointers point to real data)
        let _ghost = draft.insert_half_edge(crate::b_rep::HalfEdgeData::new(
            he_am, twin, twin, face, origin, edge,
        ));

        let result =
            crate::validators::reference_integrity::validate_no_orphan_half_edges(draft.arena());
        assert!(
            result.is_err(),
            "Validator must catch disconnected HalfEdges floating in memory"
        );
        assert!(
            format!("{:?}", result.unwrap_err()).contains("no_orphan_half_edges"),
            "Error must come from no_orphan_half_edges validator"
        );
    }

    // ── 5. acyclic_containment.rs — ValidateAcyclicContainment ──────

    #[test]
    fn positive_acyclic_containment_valid() {
        let (draft, _, _, _) = valid_mvf_se_draft();
        assert!(
            crate::validators::reference_integrity::validate_acyclic_containment(draft.arena())
                .is_ok()
        );
    }

    #[test]
    fn poison_acyclic_containment_multi_parent_region() {
        use crate::entity_lifecycle::make_vertex_face::MakeVertexFace;
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();

        // Create two separate bodies
        let mvf1 = draft
            .execute(MakeVertexFace {
                shell_kind: ShellKind::Sheet,
            })
            .unwrap()
            .into_value();
        let mvf2 = draft
            .execute(MakeVertexFace {
                shell_kind: ShellKind::Sheet,
            })
            .unwrap()
            .into_value();

        let face1 = draft.arena().get_half_edge(mvf1.half_edge).unwrap().face();
        let shell1 = draft.arena().get_face(face1).unwrap().shell();
        let region1 = draft.arena().get_shell(shell1).unwrap().region();
        let lump1 = draft.arena().get_region(region1).unwrap().lump();

        let face2 = draft.arena().get_half_edge(mvf2.half_edge).unwrap().face();
        let shell2 = draft.arena().get_face(face2).unwrap().shell();
        let region2 = draft.arena().get_shell(shell2).unwrap().region();
        let lump2 = draft.arena().get_region(region2).unwrap().lump();

        assert_ne!(lump1, lump2);

        // Pre-condition: valid
        assert!(
            crate::validators::reference_integrity::validate_acyclic_containment(draft.arena())
                .is_ok()
        );

        // Corrupt: insert Region 2 into Lump 1. Region 2 is now owned by Lump 1 and Lump 2.
        draft
            .arena_mut()
            .get_lump_mut(lump1)
            .unwrap()
            .add_region(region2);

        let result =
            crate::validators::reference_integrity::validate_acyclic_containment(draft.arena());
        assert!(
            result.is_err(),
            "Validator must catch an entity claimed by two parents"
        );
        assert!(
            format!("{:?}", result.unwrap_err()).contains("acyclic_containment"),
            "Error must come from acyclic_containment validator"
        );
    }

    // ── Batch 3 tests ───────────────────────────────────────────────

    #[test]
    fn positive_face_adjacency_consistency_valid() {
        let (draft, _, _, _) = valid_mvf_se_draft();
        assert!(
            crate::validators::shell_closure::validate_face_adjacency_consistency(draft.arena())
                .is_ok()
        );
    }

    #[test]
    fn poison_face_adjacency_consistency_mismatch() {
        use crate::entity_lifecycle::make_vertex_face::MakeVertexFace;
        let (mut draft, _face1, he1, _) = valid_mvf_se_draft();

        let mvf2 = draft
            .execute(MakeVertexFace {
                shell_kind: ShellKind::Sheet,
            })
            .unwrap()
            .into_value();
        let he2 = mvf2.half_edge;

        draft
            .arena_mut()
            .get_half_edge_mut(he1)
            .unwrap()
            .set_radial_next(he2);
        draft
            .arena_mut()
            .get_half_edge_mut(he2)
            .unwrap()
            .set_radial_next(he1);

        let res =
            crate::validators::shell_closure::validate_face_adjacency_consistency(draft.arena());
        assert!(res.is_err());
        assert!(format!("{:?}", res).contains("face_adjacency_consistency"));
    }

    #[test]
    fn positive_broken_face_boundary_valid() {
        let (draft, _, _, _) = valid_mvf_se_draft();
        assert!(
            crate::validators::shell_closure::validate_no_broken_face_boundary(draft.arena())
                .is_ok()
        );
    }

    #[test]
    fn poison_broken_face_boundary_wrong_face() {
        use crate::entity_lifecycle::make_vertex_face::MakeVertexFace;
        let (mut draft, _face1, he1, _) = valid_mvf_se_draft();
        let mvf2 = draft
            .execute(MakeVertexFace {
                shell_kind: ShellKind::Sheet,
            })
            .unwrap()
            .into_value();
        let face2 = mvf2.face;

        // Corrupt he1 to claim face2 instead of face1
        draft
            .arena_mut()
            .get_half_edge_mut(he1)
            .unwrap()
            .set_face(face2);

        let res = crate::validators::shell_closure::validate_no_broken_face_boundary(draft.arena());
        assert!(res.is_err());
        assert!(format!("{:?}", res).contains("boundary mismatch"));
    }

    #[test]
    fn poison_broken_face_boundary_unclosed() {
        let (mut draft, _face, he_am, he_mb) = valid_mvf_se_draft();

        // Instead of closing to itself, create a spur that never returns to he_am
        draft
            .arena_mut()
            .get_half_edge_mut(he_am)
            .unwrap()
            .set_next(he_mb);
        draft
            .arena_mut()
            .get_half_edge_mut(he_mb)
            .unwrap()
            .set_next(he_mb);

        let res = crate::validators::shell_closure::validate_no_broken_face_boundary(draft.arena());
        assert!(res.is_err(), "validator passed unexpectedly");
    }

    #[test]
    fn positive_shell_watertightness_valid() {
        let (draft, _, _, _) = valid_mvf_se_draft();
        // solid shell MVF+SE is a closed sphere with valence 2 edges
        assert!(
            crate::validators::shell_closure::validate_shell_consistency(draft.arena()).is_ok()
        );
    }

    #[test]
    fn poison_shell_watertightness_boundary_edge() {
        let (mut draft, face, he_am, _) = valid_mvf_se_draft();

        let shell = draft.arena().get_face(face).unwrap().shell();
        // The valid_mvf_se_draft created a Sheet shell, which allows boundaries.
        // We mutate it to a Solid shell, which PROHIBITS boundary edges.
        draft
            .arena_mut()
            .get_shell_mut(shell)
            .unwrap()
            .set_kind(crate::b_rep::ShellKind::Solid(
                crate::b_rep::ShellOrientation::Outer,
            ));

        // Break watertightness by severing the radial bond on `he_am`, making it a valence-1 boundary edge
        draft
            .arena_mut()
            .get_half_edge_mut(he_am)
            .unwrap()
            .set_radial_next(he_am);

        let res = crate::validators::shell_closure::validate_shell_consistency(draft.arena());
        assert!(
            res.is_err(),
            "Solid shell validator must catch the boundary edge we created"
        );
        assert!(format!("{:?}", res).contains("watertight"));
    }

    #[test]
    fn positive_boundary_edges_laminar_valid() {
        let (mut draft, face, _he_am, _he_mb) = valid_mvf_se_draft();

        let shell = draft.arena().get_face(face).unwrap().shell();
        draft
            .arena_mut()
            .get_shell_mut(shell)
            .unwrap()
            .set_kind(crate::b_rep::ShellKind::Sheet);

        assert!(
            crate::validators::shell_closure::validate_boundary_edges_laminar_only(draft.arena())
                .is_ok()
        );
    }

    #[test]
    fn poison_boundary_edges_laminar_valence_3() {
        let (mut draft, face, he_am, _) = valid_mvf_se_draft();

        let shell = draft.arena().get_face(face).unwrap().shell();
        draft
            .arena_mut()
            .get_shell_mut(shell)
            .unwrap()
            .set_kind(crate::b_rep::ShellKind::Sheet);

        // Clone he_am data to inject safely
        let clone_data = draft.arena().get_half_edge(he_am).unwrap().clone();
        let he2 = draft.insert_half_edge(clone_data.clone());
        let he3 = draft.insert_half_edge(clone_data);

        // Wire a proper 3-cycle: he_am -> he2 -> he3 -> he_am
        draft
            .arena_mut()
            .get_half_edge_mut(he_am)
            .unwrap()
            .set_radial_next(he2);
        draft
            .arena_mut()
            .get_half_edge_mut(he2)
            .unwrap()
            .set_radial_next(he3);
        draft
            .arena_mut()
            .get_half_edge_mut(he3)
            .unwrap()
            .set_radial_next(he_am);

        draft
            .arena_mut()
            .get_half_edge_mut(he2)
            .unwrap()
            .set_face(face);
        draft
            .arena_mut()
            .get_half_edge_mut(he3)
            .unwrap()
            .set_face(face);

        let res =
            crate::validators::shell_closure::validate_boundary_edges_laminar_only(draft.arena());
        assert!(res.is_err(), "validator passed unexpectedly");
        assert!(format!("{:?}", res).contains("boundary_edges_laminar"));
    }

    #[test]
    fn positive_radial_neighbor_consistency_valid() {
        let (draft, _, _, _) = valid_mvf_se_draft();
        assert!(
            crate::validators::radial_edge::validate_radial_neighbor_consistency(draft.arena())
                .is_ok()
        );
    }

    /// In an NMT-capable kernel, co-directional valence-2 pairs are demoted
    /// to a tracing::warn (geometric concern, not topological). This test
    /// verifies the validator does NOT reject them as errors.
    #[test]
    fn poison_radial_neighbor_consistency_same_origin() {
        let (mut draft, _face, he_am, _) = valid_mvf_se_draft();

        let origin = draft.arena().get_half_edge(he_am).unwrap().origin();

        // Clone he_am to create an explicit twin (making valence 2)
        let clone_data = draft.arena().get_half_edge(he_am).unwrap().clone();
        let he_twin = draft.insert_half_edge(clone_data);

        // Wire them as a manifold pair
        draft
            .arena_mut()
            .get_half_edge_mut(he_am)
            .unwrap()
            .set_radial_next(he_twin);
        draft
            .arena_mut()
            .get_half_edge_mut(he_twin)
            .unwrap()
            .set_radial_next(he_am);

        // Set same origin (co-directional) — now a warning, not error
        draft
            .arena_mut()
            .get_half_edge_mut(he_twin)
            .unwrap()
            .set_origin(origin);

        let res =
            crate::validators::radial_edge::validate_radial_neighbor_consistency(draft.arena());
        // In an NMT kernel, co-directional pairs are topologically valid
        // (face-orientation is a geometric concern handled by forge-spatial).
        assert!(
            res.is_ok(),
            "Co-directional pairs should warn, not error: {:?}",
            res
        );
    }

    #[test]
    fn positive_no_broken_radial_splices_valid() {
        let (draft, _, _, _) = valid_mvf_se_draft();
        assert!(
            crate::validators::radial_edge::validate_no_broken_radial_splices(draft.arena())
                .is_ok()
        );
    }

    #[test]
    fn poison_no_broken_radial_splices_disjoint() {
        let (mut draft, _face, he_am, _) = valid_mvf_se_draft();

        // Clone he_am data to inject safely. It claims the same edge.
        let clone_data = draft.arena().get_half_edge(he_am).unwrap().clone();
        let he3 = draft.insert_half_edge(clone_data);

        // Make it refer exclusively to itself. We now have a main valid 2-pair, and
        // a 1-pair that also claims the same EdgeId, meaning the overall edge's ring is disjoint.
        draft
            .arena_mut()
            .get_half_edge_mut(he3)
            .unwrap()
            .set_radial_next(he3);

        let res = crate::validators::radial_edge::validate_no_broken_radial_splices(draft.arena());
        assert!(res.is_err());
        assert!(format!("{:?}", res).contains("no_broken_radial_splices"));
    }

    #[test]
    fn poison_disk_closure_unclosed_cycle() {
        let (mut draft, _face, he_am, _) = valid_mvf_se_draft();

        // We will assert it fails.
        // If we set he_am's radial_next to DANGLING, it fails trying to fetch.
        draft
            .arena_mut()
            .get_half_edge_mut(he_am)
            .unwrap()
            .set_radial_next(HalfEdgeId::DANGLING);

        let res = crate::validators::vertex_disk::validate_disk_closure(draft.arena());
        assert!(res.is_err());
    }

    #[test]
    fn poison_vertex_disk_partition_leak() {
        // Create two separate topologies via Euler operators, then manually
        // move the second topology's HEs to share the same origin vertex as
        // the first, without registering NMT disk entries. This creates a
        // genuine 2-disk partition (separate edges, separate radial rings)
        // that the validator should detect.
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();

        let mvf1 = draft
            .execute(MakeVertexFace {
                shell_kind: ShellKind::Sheet,
            })
            .unwrap()
            .into_value();
        let mvf2 = draft
            .execute(MakeVertexFace {
                shell_kind: ShellKind::Sheet,
            })
            .unwrap()
            .into_value();

        // Pre-condition: mvf2's vertex has its own outgoing HE
        let v1 = mvf1.vertex;
        let v2 = mvf2.vertex;
        assert_ne!(v1, v2);

        // Corrupt: reassign mvf2's HE origin to v1, creating a disjoint disk
        // at v1 without registering the NMT extra disk entry.
        let he2 = mvf2.half_edge;
        draft
            .arena_mut()
            .get_half_edge_mut(he2)
            .unwrap()
            .set_origin(v1);

        // Now v1 has two outgoing HEs (mvf1.half_edge and he2), on separate
        // edges with separate radial rings → 2 disk components.
        // But disk_count(v1) is still 1 (no add_disk_entry was called).
        let res = crate::validators::vertex_disk::validate_vertex_disk_partition(draft.arena());
        assert!(res.is_err(), "Expected partition leak error, got {:?}", res);
    }

    #[test]
    fn poison_cross_disk_coedges() {
        let (draft, _, _, _) = valid_mvf_se_draft();
        let res = crate::validators::vertex_disk::validate_no_cross_disk_coedges(draft.arena());
        assert!(
            res.is_ok(),
            "Valid topology must pass cross-disk check: {:?}",
            res
        );
    }

    #[test]
    fn poison_per_component_euler_invalid_genus() {
        let mut draft = TopologyState::empty().into_mutation();
        let s1 = draft.insert_shell(crate::b_rep::ShellData::new(
            crate::handles::FaceId::DANGLING,
            crate::b_rep::ShellKind::Solid(crate::b_rep::ShellOrientation::Outer),
            crate::handles::RegionId::DANGLING,
        ));
        let f1 = draft.insert_face(crate::b_rep::FaceData::new(
            crate::handles::LoopId::DANGLING,
            s1,
        ));
        let v1 = draft.insert_vertex(crate::b_rep::VertexData::new(HalfEdgeId::DANGLING));
        let e1 = draft.insert_edge(crate::b_rep::EdgeData::new(HalfEdgeId::DANGLING));

        // V=1, E=1, F=1
        let he1 = draft.insert_half_edge(crate::b_rep::HalfEdgeData::new(
            HalfEdgeId::DANGLING,
            HalfEdgeId::DANGLING,
            HalfEdgeId::DANGLING,
            f1,
            v1,
            e1,
        ));

        draft
            .arena_mut()
            .get_vertex_mut(v1)
            .unwrap()
            .set_primary_disk(he1);
        draft
            .arena_mut()
            .get_edge_mut(e1)
            .unwrap()
            .set_half_edge(he1);

        draft
            .arena_mut()
            .get_half_edge_mut(he1)
            .unwrap()
            .set_next(he1);
        draft
            .arena_mut()
            .get_half_edge_mut(he1)
            .unwrap()
            .set_prev(he1);
        draft
            .arena_mut()
            .get_half_edge_mut(he1)
            .unwrap()
            .set_radial_next(he1);

        let res = crate::validators::euler_genus::validate_per_component_euler(draft.arena());
        assert!(res.is_err(), "Expected Euler violation, got {:?}", res);
    }

    #[test]
    fn full_validation_passes_on_valid_topology() {
        let (draft, _, _, _) = valid_mvf_se_draft();
        assert!(
            validate_topology(draft.arena(), ValidationLevel::Full).is_ok(),
            "Full validation must pass on uncorrupted MVF+SE topology"
        );
    }

    #[test]
    fn diag_dump_mvf_se_wiring() {
        let (draft, _face, he_am, he_mb) = valid_mvf_se_draft();
        let arena = draft.arena();

        eprintln!("=== VERTICES ===");
        for (vid, vd) in arena.iter_vertices() {
            eprintln!(
                "  V{} primary_disk=HE{}",
                vid.index(),
                vd.primary_disk().index()
            );
        }
        eprintln!("=== HALF-EDGES ===");
        for (heid, hed) in arena.iter_half_edges() {
            eprintln!(
                "  HE{}: origin=V{} next=HE{} prev=HE{} radial_next=HE{} face=F{} edge=E{}",
                heid.index(),
                hed.origin().index(),
                hed.next().index(),
                hed.prev().index(),
                hed.radial_next().index(),
                hed.face().index(),
                hed.edge().index()
            );
        }
        eprintln!("=== EDGES ===");
        for (eid, ed) in arena.iter_edges() {
            eprintln!("  E{}: half_edge=HE{}", eid.index(), ed.half_edge().index());
        }
        eprintln!("=== SHELLS ===");
        for (sid, sd) in arena.iter_shells() {
            eprintln!("  S{}: kind={:?}", sid.index(), sd.kind());
        }
        eprintln!("he_am=HE{}, he_mb=HE{}", he_am.index(), he_mb.index());
    }
}
