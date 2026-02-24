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

pub use super::structural::validate_topology;
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
}
