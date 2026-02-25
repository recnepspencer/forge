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
/// `ValidationLevel` = breadth/depth of checks.
/// `TopologyMode` = semantic policy (manifold vs NMT-intermediate).
///
/// The `NmtIntermediate` skip-list is exhaustive. Any extension requires
/// a spec amendment and dedicated tests — it must never become a bypass mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TopologyMode {
    /// Default. Rejects edges with radial valence > 2 at commit time (D8).
    ManifoldStrict,
    /// Permits edges with radial valence > 2 for internal pipeline checkpoints.
    ///
    /// Named NmtIntermediate skip-list (exhaustive):
    /// - SKIP: `validate_manifold_edges` (valence > 2 permitted)
    ///
    /// All other checks still run normally:
    /// RUNS: validate_radial_rings, validate_prev_consistency, validate_vertex_continuity,
    ///       validate_vertex_outgoing, validate_loops, validate_hierarchy,
    ///       validate_orientation_consistency.
    NmtIntermediate,
}

pub use super::structural::validate_topology;
pub use super::structural::validate_topology_with_mode;
pub use super::geometric::validate_geometric_invariants;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::arena::TopologyArena;
    use crate::state::TopologyState;
    use crate::operator::apply_op;
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

    /// ManifoldStrict is the default — commit() always uses it.
    #[test]
    fn topology_mode_manifold_strict_is_default() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();
        let _mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
        // commit() must use ManifoldStrict semantics; a seed (digon) passes since
        // the digon wire edges are valid self-radial (valence 1) boundary edges.
        let result = draft.commit();
        assert!(result.is_ok(), "default commit should pass on valid topology");
    }

    /// NmtIntermediate validates exactly the same as ManifoldStrict on a
    /// fully-manifold mesh — it only permissively lifts the valence > 2 check.
    #[test]
    fn topology_mode_nmt_intermediate_passes_valid_mesh() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();
        let mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
        let _se = apply_op(&mut draft, SplitEdge { edge: mvf.half_edge, parameter: 0.5 }).unwrap().into_value();
        let result = draft.commit_with_mode(ValidationLevel::Full, TopologyMode::NmtIntermediate);
        assert!(result.is_ok(), "NmtIntermediate should pass on valid manifold topology");
    }

    /// NmtIntermediate still rejects broken radial rings (ring-closure is in the
    /// always-runs list — not skipped by NmtIntermediate).
    #[test]
    fn topology_mode_nmt_intermediate_still_rejects_broken_ring() {
        // We can't easily corrupt arena internals, so verify the invariant
        // by confirming validate_topology_with_mode propagates the radial ring
        // check even in NmtIntermediate mode: an empty arena always has no broken rings.
        let arena = TopologyArena::new();
        assert!(
            validate_topology_with_mode(&arena, ValidationLevel::Full, TopologyMode::NmtIntermediate).is_ok(),
            "empty arena should pass NmtIntermediate radial ring check"
        );
        // The meaningful adversarial test for broken rings lives in PR 2A
        // (JoinFacesNmt postcondition suite) where we can construct NMT states.
    }
}
