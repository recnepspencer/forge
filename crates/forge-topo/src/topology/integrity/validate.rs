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
    /// Full global validity checks (Euler formula, loop closure).
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

/// Topology manifold policy — controls *what world is allowed* at commit time.
///
/// This is orthogonal to `ValidationLevel` (which controls *how much to check*).
/// `ValidationLevel` = breadth/depth of checks (diagnostics vs speed).
/// `TopologyMode` = semantic policy (manifold vs NMT-intermediate constraints).
///
/// The `NmtIntermediate` skip-list is exhaustive. Any extension requires
/// a spec amendment and dedicated tests — it must never become a bypass mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TopologyMode {
    /// Default. Enforces 2-manifold constraints at commit time (Doctrine D8).
    /// This runs regardless of `ValidationLevel`.
    ManifoldStrict,
    /// Permits edges with radial valence > 2 for internal pipeline checkpoints.
    ///
    /// Named skip-list (exhaustive):
    /// - SKIP: `validate_manifold_edges` (valence > 2 permitted)
    ///
    /// Note: Radial ring validation itself (closure/pointer health) STILL RUNS
    /// (for `ValidationLevel != None`) to prevent data corruption. Only the 
    /// valence threshold is relaxed.
    NmtIntermediate,
}

pub use super::structural::validate_topology;
pub use super::structural::validate_topology_with_mode;
/// Geometric invariant validation has moved to `forge-spatial::integrity`.
/// Build with forge-spatial as a dependency and call `forge_spatial::integrity::validate_geometric_invariants`.


#[cfg(test)]
mod tests {
    use super::*;
    use forge_core::KernelError;
    use crate::arena::TopologyArena;
    use crate::handles::HalfEdgeId;
    use crate::state::TopologyState;
    use crate::operator::apply_op;
    use crate::validate;
    use crate::euler::make_vertex_face::MakeVertexFace;
    use crate::euler::split_edge::SplitEdge;

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
        let _se = apply_op(&mut draft, SplitEdge { edge: mvf.half_edge, parameter: 0.5 }).unwrap().into_value();
        let state = draft.commit().unwrap();
        assert!(validate_topology(state.arena(), ValidationLevel::Full).is_ok());
    }

    /// ManifoldStrict rejects a valence-3 radial ring.
    ///
    /// We build a digon (MVF + SplitEdge), then splice a third halfedge into
    /// the face loop AND into the radial ring independently:
    ///
    ///   Face loop:   he_am → ghost → he_mb → he_am  (prev/next consistent)
    ///   Radial ring: he_am → he_mb → ghost → he_am  (valence 3)
    ///
    /// The two structures are orthogonal. ManifoldStrict reaches
    /// `validate_manifold_edges` (not `validate_prev_consistency`) and rejects
    /// because the ring has valence 3.
    ///
    /// Crucially, we prove this is an architectural semantic invariant by
    /// testing it at `ValidationLevel::Minimal`.
    #[test]
    fn topology_mode_manifold_strict_rejects_valence_3_edge() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();
        let mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
        let se = apply_op(&mut draft, SplitEdge {
            edge: mvf.half_edge,
            parameter: 0.5,
        }).unwrap().into_value();

        let he_am = se.he_am;
        let he_mb = se.he_mb;

        // Capture face/origin/edge before any mutation.
        let face  = draft.arena().get_half_edge(he_am).unwrap().face();
        let orig  = draft.arena().get_half_edge(he_am).unwrap().origin();
        let edge  = draft.arena().get_half_edge(he_am).unwrap().edge();

        // SplitEdge gave he_mb a different edge entity. Unify before wiring
        // the 3-ring — radial rings must share one edge entity.
        draft.arena_mut().get_half_edge_mut(he_mb).unwrap().set_edge(edge);

        // Insert ghost, spliced into the face loop: he_am → ghost → he_mb.
        let ghost = draft.insert_half_edge(crate::arena::HalfEdgeData::new(
            he_am,  // radial_next — temporary, overwritten below
            he_mb,  // next: ghost → he_mb
            he_am,  // prev: he_am → ghost
            face, orig, edge,
        ));

        // Splice into face loop: he_am.next = ghost, he_mb.prev = ghost.
        draft.arena_mut().get_half_edge_mut(he_am).unwrap().set_next(ghost);
        draft.arena_mut().get_half_edge_mut(he_mb).unwrap().set_prev(ghost);

        // Wire 3-ring (independent of face loop).
        draft.arena_mut().get_half_edge_mut(he_am).unwrap().set_radial_next(he_mb);
        draft.arena_mut().get_half_edge_mut(he_mb).unwrap().set_radial_next(ghost);
        draft.arena_mut().get_half_edge_mut(ghost).unwrap().set_radial_next(he_am);

        // Proof of Separation (QA Defect Fix):
        // ManifoldStrict is level-independent. Minimal must also reject.
        let result = draft.commit_with_mode(
            validate::ValidationLevel::Minimal,
            validate::TopologyMode::ManifoldStrict,
        );
        assert!(
            result.is_err(),
            "ManifoldStrict must reject a valence-3 edge (non-manifold)"
        );
        let err = result.unwrap_err();
        assert!(
            matches!(err, KernelError::TopologyViolation { .. }),
            "Expected TopologyViolation from validate_manifold_edges, got: {:?}", err
        );
    }

    /// NmtIntermediate accepts the same valence-3 ring that ManifoldStrict rejects.
    ///
    /// Identical wiring to the ManifoldStrict test above. The only difference is
    /// `commit_with_mode(..., NmtIntermediate)`, which skips `validate_manifold_edges`.
    /// All other structural checks (radial ring closure, prev/next consistency,
    /// loop closure, hierarchy) pass because the face loop AND radial ring are
    /// both properly wired as two orthogonal structures.
    #[test]
    fn topology_mode_nmt_intermediate_accepts_valence_3_edge() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();
        let mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
        let se = apply_op(&mut draft, SplitEdge {
            edge: mvf.half_edge,
            parameter: 0.5,
        }).unwrap().into_value();

        let he_am = se.he_am;
        let he_mb = se.he_mb;

        let face  = draft.arena().get_half_edge(he_am).unwrap().face();
        let orig  = draft.arena().get_half_edge(he_am).unwrap().origin();
        let edge  = draft.arena().get_half_edge(he_am).unwrap().edge();

        // Unify edge entity: he_mb must share he_am's edge for a valid ring.
        draft.arena_mut().get_half_edge_mut(he_mb).unwrap().set_edge(edge);

        let ghost = draft.insert_half_edge(crate::arena::HalfEdgeData::new(
            he_am, he_mb, he_am, face, orig, edge,
        ));

        // Splice into face loop.
        draft.arena_mut().get_half_edge_mut(he_am).unwrap().set_next(ghost);
        draft.arena_mut().get_half_edge_mut(he_mb).unwrap().set_prev(ghost);

        // Wire 3-ring.
        draft.arena_mut().get_half_edge_mut(he_am).unwrap().set_radial_next(he_mb);
        draft.arena_mut().get_half_edge_mut(he_mb).unwrap().set_radial_next(ghost);
        draft.arena_mut().get_half_edge_mut(ghost).unwrap().set_radial_next(he_am);

        let result = draft.commit_with_mode(
            validate::ValidationLevel::Full,
            validate::TopologyMode::NmtIntermediate,
        );
        assert!(
            result.is_ok(),
            "NmtIntermediate must accept valence-3 edge: {:?}",
            result.err()
        );
    }


    /// NmtIntermediate still rejects a broken (non-closing) radial ring.
    ///
    /// We wire radial_next to a stale/nonexistent handle so the ring never
    /// closes. radial ring validation runs in NmtIntermediate (it is NOT on
    /// the skip-list), so commit_with_mode must return an error here.
    #[test]
    fn topology_mode_nmt_intermediate_still_rejects_broken_ring() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();
        let mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
        let se = apply_op(&mut draft, SplitEdge {
            edge: mvf.half_edge,
            parameter: 0.5,
        }).unwrap().into_value();

        // Point radial_next to a slot that doesn't exist — the ring cannot close.
        let stale = HalfEdgeId::from_raw_parts(99_999, 0);
        draft.arena_mut()
            .get_half_edge_mut(se.he_am)
            .unwrap()
            .set_radial_next(stale);

        let result = draft.commit_with_mode(
            validate::ValidationLevel::Full,
            validate::TopologyMode::NmtIntermediate,
        );
        assert!(
            result.is_err(),
            "NmtIntermediate must still reject a non-closing radial ring"
        );
        assert!(
            matches!(result.unwrap_err(), KernelError::TopologyViolation { .. }),
            "Expected TopologyViolation for broken ring"
        );
    }

    /// Default commit() always uses ManifoldStrict — there must be no way to
    /// silently change it, even indirectly through a valid topology sequence.
    #[test]
    fn topology_mode_default_commit_is_always_manifold_strict() {
        // Build a valid 2-manifold mesh; default commit must pass.
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();
        let mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
        let _se = apply_op(&mut draft, SplitEdge {
            edge: mvf.half_edge,
            parameter: 0.5,
        }).unwrap().into_value();
        assert!(
            draft.commit().is_ok(),
            "Default commit on valid manifold topology must pass"
        );
    }

    // ── Adversarial Test Suite ──────────────────────────────────────────

    /// ADVERSARIAL #1: Edge-entity inconsistency in radial ring.
    ///
    /// Wire two halfedges with DIFFERENT edge entities into a 2-ring.
    /// `validate_radial_edge_consistency` catches this: all members of a
    /// radial ring must reference the same `EdgeId`.
    #[test]
    fn adversarial_edge_entity_inconsistency_in_radial_ring() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();
        let mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
        let se = apply_op(&mut draft, SplitEdge {
            edge: mvf.half_edge,
            parameter: 0.5,
        }).unwrap().into_value();

        let he_am = se.he_am;
        let he_mb = se.he_mb;

        // SplitEdge creates two separate edges. Wire them into a 2-ring
        // even though they have DIFFERENT edge entities.
        draft.arena_mut().get_half_edge_mut(he_am).unwrap().set_radial_next(he_mb);
        draft.arena_mut().get_half_edge_mut(he_mb).unwrap().set_radial_next(he_am);

        // Confirm the edge entities actually differ — this is the precondition.
        let edge_am = draft.arena().get_half_edge(he_am).unwrap().edge();
        let edge_mb = draft.arena().get_half_edge(he_mb).unwrap().edge();
        assert_ne!(
            edge_am, edge_mb,
            "Test precondition: SplitEdge must produce distinct edge entities"
        );

        let result = validate_topology(draft.arena(), ValidationLevel::Full);
        assert!(
            result.is_err(),
            "Radial ring with mismatched edge entities must be rejected"
        );
        let err = result.unwrap_err();
        assert!(
            matches!(err, KernelError::TopologyViolation { .. }),
            "Expected TopologyViolation for edge-entity inconsistency, got: {:?}", err
        );
    }

    /// D8 PROOF: ManifoldStrict is unconditional — even ValidationLevel::None
    /// cannot bypass it.
    ///
    /// Doctrine D8 declares 2-manifold enforcement as a hard phase-boundary
    /// invariant. `ValidationLevel::None` bypasses diagnostic checks, NOT
    /// semantic policy. If this test ever fails, the kernel has a D8 regression.
    #[test]
    fn d8_manifold_strict_unconditional_even_at_level_none() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();
        let mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
        let se = apply_op(&mut draft, SplitEdge {
            edge: mvf.half_edge,
            parameter: 0.5,
        }).unwrap().into_value();

        let he_am = se.he_am;
        let he_mb = se.he_mb;
        let face  = draft.arena().get_half_edge(he_am).unwrap().face();
        let orig  = draft.arena().get_half_edge(he_am).unwrap().origin();
        let edge  = draft.arena().get_half_edge(he_am).unwrap().edge();

        draft.arena_mut().get_half_edge_mut(he_mb).unwrap().set_edge(edge);

        let ghost = draft.insert_half_edge(crate::arena::HalfEdgeData::new(
            he_am, he_mb, he_am, face, orig, edge,
        ));
        draft.arena_mut().get_half_edge_mut(he_am).unwrap().set_next(ghost);
        draft.arena_mut().get_half_edge_mut(he_mb).unwrap().set_prev(ghost);
        draft.arena_mut().get_half_edge_mut(he_am).unwrap().set_radial_next(he_mb);
        draft.arena_mut().get_half_edge_mut(he_mb).unwrap().set_radial_next(ghost);
        draft.arena_mut().get_half_edge_mut(ghost).unwrap().set_radial_next(he_am);

        // ValidationLevel::None + ManifoldStrict: diagnostics skipped, D8 NOT skipped.
        let result = draft.commit_with_mode(
            validate::ValidationLevel::None,
            validate::TopologyMode::ManifoldStrict,
        );
        assert!(
            result.is_err(),
            "D8 violation: ManifoldStrict MUST reject valence-3 edge even at \
             ValidationLevel::None. Doctrine D8 is non-negotiable."
        );
        let err = result.unwrap_err();
        assert!(
            matches!(err, KernelError::TopologyViolation {
                err: forge_core::TopologyError::NonManifoldEdge { .. }, ..
            }),
            "Expected NonManifoldEdge error, got: {:?}", err
        );
    }

    /// ADVERSARIAL #3: Default commit() rejects valence-3 regardless of build profile.
    ///
    /// This is the regression guard for the Tier 1 reclassification.
    /// commit() uses the default ValidationLevel (Minimal in release, Full in debug)
    /// and ManifoldStrict mode. After our fix, ManifoldStrict runs at ALL levels,
    /// so this test must pass in both --release and debug.
    #[test]
    fn adversarial_default_commit_rejects_valence_3() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();
        let mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
        let se = apply_op(&mut draft, SplitEdge {
            edge: mvf.half_edge,
            parameter: 0.5,
        }).unwrap().into_value();

        let he_am = se.he_am;
        let he_mb = se.he_mb;
        let face  = draft.arena().get_half_edge(he_am).unwrap().face();
        let orig  = draft.arena().get_half_edge(he_am).unwrap().origin();
        let edge  = draft.arena().get_half_edge(he_am).unwrap().edge();

        draft.arena_mut().get_half_edge_mut(he_mb).unwrap().set_edge(edge);

        let ghost = draft.insert_half_edge(crate::arena::HalfEdgeData::new(
            he_am, he_mb, he_am, face, orig, edge,
        ));
        draft.arena_mut().get_half_edge_mut(he_am).unwrap().set_next(ghost);
        draft.arena_mut().get_half_edge_mut(he_mb).unwrap().set_prev(ghost);
        draft.arena_mut().get_half_edge_mut(he_am).unwrap().set_radial_next(he_mb);
        draft.arena_mut().get_half_edge_mut(he_mb).unwrap().set_radial_next(ghost);
        draft.arena_mut().get_half_edge_mut(ghost).unwrap().set_radial_next(he_am);

        // Bare commit() — no explicit mode. Must reject in both debug and release.
        let result = draft.commit();
        assert!(
            result.is_err(),
            "Default commit() must reject valence-3 edge in any build profile \
             (this test guards the Tier 1 reclassification)"
        );
    }

    /// ADVERSARIAL #3: validate_vertex_continuity with cross-edge radial ring.
    ///
    /// Wire halfedges from two different geometric edges into one radial ring.
    /// Since the halfedges have different origin vertices, validate_vertex_continuity
    /// should detect > 2 endpoint vertices and reject.
    ///
    /// We call the specific validator directly to prove it catches the cross-contamination
    /// independently of the newly added validate_radial_edge_consistency.
    #[test]
    fn adversarial_cross_edge_vertex_continuity() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();

        // Create two MVF seeds — each has its own vertex, face, edge, and halfedge.
        let mvf1 = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
        let se1 = apply_op(&mut draft, SplitEdge {
            edge: mvf1.half_edge,
            parameter: 0.5,
        }).unwrap().into_value();

        let mvf2 = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
        let se2 = apply_op(&mut draft, SplitEdge {
            edge: mvf2.half_edge,
            parameter: 0.5,
        }).unwrap().into_value();

        // Wire cross-edge radial ring: se1.he_am ↔ se2.he_am.
        draft.arena_mut().get_half_edge_mut(se1.he_am).unwrap().set_radial_next(se2.he_am);
        draft.arena_mut().get_half_edge_mut(se2.he_am).unwrap().set_radial_next(se1.he_am);

        // Precondition: different original vertices.
        let orig1 = draft.arena().get_half_edge(se1.he_am).unwrap().origin();
        let orig2 = draft.arena().get_half_edge(se2.he_am).unwrap().origin();
        assert_ne!(orig1, orig2, "Test precondition: different origin vertices");

        // Call the specific validator directly to bypass validate_radial_edge_consistency.
        // It should reject because the single edge entity for se1 now appears
        // to have endpoints from both se1 and se2 (total endpoints > 2).
        let result = crate::topology::integrity::structural::validate_vertex_continuity(draft.arena());
        assert!(
            result.is_err(),
            "Cross-edge radial ring must be caught by validate_vertex_continuity — \
             mixed origin vertices produce > 2 endpoints per edge"
        );
    }

    /// ADVERSARIAL #4: EntityBitset capacity after entity removal (arena gaps).
    ///
    /// We explicitly delete an entity from the arena to create an index gap,
    /// proving that EntityBitset sizing (`max_index + 1`) doesn't panic or
    /// truncate when indices are non-contiguous.
    #[test]
    fn adversarial_bitset_capacity_after_entity_removal() {
        use crate::topology::bitset::EntityBitset;

        let state = TopologyState::empty();
        let mut draft = state.into_mutation();

        // Create two disjoint faces
        let mvf1 = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
        let mvf2 = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();

        let face1 = draft.arena().get_half_edge(mvf1.half_edge).unwrap().face();
        let face2 = draft.arena().get_half_edge(mvf2.half_edge).unwrap().face();

        assert_ne!(face1, face2);
        assert_eq!(draft.arena().face_count(), 2);

        // Explicitly DELETE face1 from the arena payload array, creating a gap.
        draft.arena_mut().remove_face(face1, None).unwrap();

        assert_eq!(draft.arena().face_count(), 1, "One face remains");

        // The remaining face (face2) has index 1. Capacity must safely cover it
        // despite index 0 having been deleted.
        let bs = EntityBitset::for_faces(draft.arena());
        assert!(
            bs.capacity() > face2.index(),
            "Bitset capacity {} must cover remaining face index {}",
            bs.capacity(), face2.index()
        );

        // Manual check of the trait implementation directly against the arena gap.
        let result = crate::topology::integrity::structural::validate_radial_edge_consistency(draft.arena());
        assert!(result.is_ok(), "Validation should not panic on missing indices");
    }

    /// ADVERSARIAL #5: Disjoint radial rings sharing the same EdgeId.
    ///
    /// If two structurally separated radial rings both claim the same `EdgeId`,
    /// validators that deduplicate by `EdgeId` (like `validate_manifold_edges`)
    /// will check the first one they encounter, add the `EdgeId` to their
    /// `EntityBitset`, and then SILENTLY SKIP all subsequent rings sharing that ID.
    /// This allows non-manifold or corrupt rings to bypass validation completely.
    #[test]
    fn adversarial_disjoint_rings_sharing_edge_id() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();

        // Construct 1: A perfectly valid, manifold 2-ring.
        let mvf1 = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
        let se1 = apply_op(&mut draft, SplitEdge {
            edge: mvf1.half_edge,
            parameter: 0.5,
        }).unwrap().into_value();
        
        let shared_edge_id = draft.arena().get_half_edge(se1.he_am).unwrap().edge();
        draft.arena_mut().get_half_edge_mut(se1.he_mb).unwrap().set_edge(shared_edge_id);
        
        draft.arena_mut().get_half_edge_mut(se1.he_am).unwrap().set_radial_next(se1.he_mb);
        draft.arena_mut().get_half_edge_mut(se1.he_mb).unwrap().set_radial_next(se1.he_am);

        // Construct 2: A non-manifold 3-ring (valence 3) — should be rejected.
        let mvf2 = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
        let se2 = apply_op(&mut draft, SplitEdge {
            edge: mvf2.half_edge,
            parameter: 0.5,
        }).unwrap().into_value();
        
        // Unify edge entities for Construct 2 (as required for a valid radial ring).
        // BUT ALSO maliciously reuse the edge ID from Construct 1!
        let he2_am = se2.he_am;
        let he2_mb = se2.he_mb;
        let face2  = draft.arena().get_half_edge(he2_am).unwrap().face();
        let orig2  = draft.arena().get_half_edge(he2_am).unwrap().origin();
        
        draft.arena_mut().get_half_edge_mut(he2_am).unwrap().set_edge(shared_edge_id);
        draft.arena_mut().get_half_edge_mut(he2_mb).unwrap().set_edge(shared_edge_id);

        let ghost = draft.insert_half_edge(crate::arena::HalfEdgeData::new(
            he2_am, he2_mb, he2_am, face2, orig2, shared_edge_id,
        ));
        draft.arena_mut().get_half_edge_mut(he2_am).unwrap().set_next(ghost);
        draft.arena_mut().get_half_edge_mut(he2_mb).unwrap().set_prev(ghost);
        
        // Wire the 3-ring for Construct 2.
        draft.arena_mut().get_half_edge_mut(he2_am).unwrap().set_radial_next(he2_mb);
        draft.arena_mut().get_half_edge_mut(he2_mb).unwrap().set_radial_next(ghost);
        draft.arena_mut().get_half_edge_mut(ghost).unwrap().set_radial_next(he2_am);

        // Current state: Construct 2 is valence-3, which ManifoldStrict must reject.
        // However, because it shares `shared_edge_id` with Construct 1 (which is checked first
        // and is manifold), validators might skip Construct 2 entirely.
        
        let result = validate_topology(draft.arena(), ValidationLevel::Full);
        assert!(
            result.is_err(),
            "Disjoint radial rings sharing EdgeId must be rejected by validate_manifold_edges"
        );
        let err = result.unwrap_err();
        assert!(
            matches!(err, KernelError::TopologyViolation { .. }),
            "Expected TopologyViolation for disjoint-ring bypass, got: {:?}", err
        );
    }

}
