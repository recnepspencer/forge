//! Invariant contract system types and compile-time enforcement.
//!
//! DOMAIN: Re-exports the shared invariant types from `forge-core` and provides
//! the `forge-topo`–specific `ValidatorEntry` + `validator_for()` dispatch.
//!
//! The `InvariantId`, `InvariantRelation`, and `InvariantContract` types live in
//! `forge-core` so both `forge-topo` and `forge-spatial` share one contract system.
//! This module re-exports them for backward-compatible import paths and adds the
//! topo-specific dispatcher.

use crate::b_rep::TopologyArena;
use forge_core::KernelError;

// Re-export contract types from forge-core.
// Existing code importing `crate::validators::invariant_id::{InvariantId, ...}` works unchanged.
pub use forge_core::{
    InvariantId, InvariantRelation, InvariantContract,
    ValidatorCost,
};

/// Registry entry mapping an `InvariantId` to its checker function and cost.
///
/// This is the `forge-topo` dispatcher — only structural (combinatorial) validators.
/// Geometry-dependent invariants return no-op entries here; they are dispatched
/// through `forge-spatial::spatial_validator_for()` instead.
pub struct ValidatorEntry {
    /// Algorithmic cost of running this validator.
    pub cost: ValidatorCost,
    /// The validation function. Takes the arena, returns Ok or a structured error.
    pub check: fn(&TopologyArena) -> Result<(), KernelError>,
}

impl ValidatorEntry {
    /// Create a cheap (O(n)) validator entry.
    pub const fn cheap(check: fn(&TopologyArena) -> Result<(), KernelError>) -> Self {
        Self { cost: ValidatorCost::Cheap, check }
    }

    /// Create a medium-cost (O(n log n)) validator entry.
    pub const fn medium(check: fn(&TopologyArena) -> Result<(), KernelError>) -> Self {
        Self { cost: ValidatorCost::Medium, check }
    }

    /// Create an expensive (O(n²) or global) validator entry.
    pub const fn expensive(check: fn(&TopologyArena) -> Result<(), KernelError>) -> Self {
        Self { cost: ValidatorCost::Expensive, check }
    }

    /// No-op validator entry for invariants validated elsewhere (e.g. forge-spatial).
    const fn noop() -> Self {
        Self { cost: ValidatorCost::Cheap, check: |_| Ok(()) }
    }
}

/// Dispatch every `InvariantId` to its validator function and cost tier.
///
/// Exhaustive match — adding an `InvariantId` variant without a validator
/// is a compile error.
///
/// Geometry-dependent invariants (`NoZeroLengthEdges`, etc.) return no-ops
/// here because they require vertex positions and are dispatched through
/// `forge-spatial::spatial_validator_for()` instead.
pub fn validator_for(id: InvariantId) -> ValidatorEntry {
    use super::cache_index;
    use super::euler_genus;
    use super::loop_wiring;
    use super::radial_edge;
    use super::reference_integrity;
    use super::shell_closure;
    use super::vertex_disk;

    match id {
        // ── Pointer coherence ───────────────────────────────────
        InvariantId::RadialReciprocity =>
            ValidatorEntry::cheap(radial_edge::validate_radial_rings),
        InvariantId::NextPrevReciprocity =>
            ValidatorEntry::cheap(loop_wiring::validate_prev_consistency),
        InvariantId::NoDanglingRefs =>
            ValidatorEntry::cheap(reference_integrity::validate_no_dangling_half_edge_refs),
        InvariantId::GenerationalFreshness =>
            ValidatorEntry::cheap(reference_integrity::validate_generational_id_freshness),

        // ── Loop structure ──────────────────────────────────────
        InvariantId::FaceHasLoop =>
            ValidatorEntry::cheap(reference_integrity::validate_face_has_at_least_one_loop),
        InvariantId::LoopMinCardinality =>
            ValidatorEntry::cheap(loop_wiring::validate_loop_minimum_cardinality),
        InvariantId::NoDuplicateCoedges =>
            ValidatorEntry::cheap(loop_wiring::validate_no_duplicate_coedges_in_loop),
        InvariantId::FaceLoopMembership =>
            ValidatorEntry::medium(loop_wiring::validate_face_loop_membership_complete),
        InvariantId::VertexContinuity =>
            ValidatorEntry::cheap(loop_wiring::validate_vertex_continuity),
        InvariantId::EdgeEndpointsMatch =>
            ValidatorEntry::medium(loop_wiring::validate_edge_endpoints_match_loop_vertices),

        // ── Ownership ───────────────────────────────────────────
        InvariantId::SingleLoopOwner =>
            ValidatorEntry::medium(reference_integrity::validate_single_owner_per_loop),
        InvariantId::NoOrphanHalfEdges =>
            ValidatorEntry::medium(reference_integrity::validate_no_orphan_half_edges),
        InvariantId::AcyclicContainment =>
            ValidatorEntry::medium(reference_integrity::validate_acyclic_containment),
        InvariantId::InnerOuterConsistency =>
            ValidatorEntry::medium(reference_integrity::validate_inner_outer_loop_consistency),

        // ── Radial edge ─────────────────────────────────────────
        InvariantId::RadialCycleUniqueness =>
            ValidatorEntry::cheap(radial_edge::validate_radial_cycle_uniqueness),
        InvariantId::RadialNeighborConsistency =>
            ValidatorEntry::expensive(radial_edge::validate_radial_neighbor_consistency),
        InvariantId::NoBrokenRadialSplices =>
            ValidatorEntry::expensive(radial_edge::validate_no_broken_radial_splices),

        // ── Shell closure ───────────────────────────────────────
        InvariantId::FaceAdjacencyConsistency =>
            ValidatorEntry::expensive(shell_closure::validate_face_adjacency_consistency),
        InvariantId::NoBrokenFaceBoundary =>
            ValidatorEntry::expensive(shell_closure::validate_no_broken_face_boundary),
        InvariantId::BoundaryEdgesLaminar =>
            ValidatorEntry::expensive(shell_closure::validate_boundary_edges_laminar_only),

        // ── Vertex disk ─────────────────────────────────────────
        InvariantId::DiskEntriesAlive =>
            ValidatorEntry::cheap(vertex_disk::validate_vertex_outgoing),
        InvariantId::DiskPartitionCorrect =>
            ValidatorEntry::expensive(vertex_disk::validate_vertex_disk_partition),
        InvariantId::DiskClosure =>
            ValidatorEntry::expensive(vertex_disk::validate_disk_closure),
        InvariantId::NoCrossDiskCoedges =>
            ValidatorEntry::expensive(vertex_disk::validate_no_cross_disk_coedges),

        // ── Euler formula ───────────────────────────────────────
        InvariantId::PerComponentEuler =>
            ValidatorEntry::expensive(euler_genus::validate_per_component_euler),

        // ── Side-car coherence ──────────────────────────────────
        InvariantId::SideCarCoherence =>
            ValidatorEntry::cheap(cache_index::validate_index_coherence),
        InvariantId::IndexCoherence =>
            ValidatorEntry::cheap(cache_index::validate_index_coherence),

        // ── Geometry-dependent (dispatched via forge-spatial) ───
        InvariantId::NoZeroLengthEdges
        | InvariantId::NoZeroAreaFaces
        | InvariantId::NoInsideOutShells
        | InvariantId::LoopOrientationConsistency
        | InvariantId::ShellOrientationConsistency
        | InvariantId::NoVertexOffSurface
        | InvariantId::GeometryCompleteness
        | InvariantId::EdgeCurveConsistency =>
            ValidatorEntry::noop(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// CI gate: every InvariantId has a validator — exercised at runtime.
    #[test]
    fn all_invariants_have_validators() {
        for &id in InvariantId::ALL {
            let entry = validator_for(id);
            // The exhaustive match guarantees coverage, but this exercises
            // the runtime path and ensures no panic.
            let _ = entry.cost;
        }
    }

    /// Verify that ALL contains every variant (count check).
    #[test]
    fn all_constant_covers_every_variant() {
        assert_eq!(
            InvariantId::ALL.len(),
            35,
            "ALL should contain all 35 InvariantId variants (27 structural + 8 geometric)"
        );
    }

    /// Contract helpers return correct invariants.
    #[test]
    fn may_break_returns_correct_invariants() {
        let contract = InvariantContract {
            relation: |id| match id {
                InvariantId::RadialReciprocity => InvariantRelation::MayBreak,
                InvariantId::NextPrevReciprocity => InvariantRelation::MayBreak,
                InvariantId::NoDanglingRefs => InvariantRelation::Ensures,
                InvariantId::GenerationalFreshness => InvariantRelation::Ensures,
                _ => InvariantRelation::Unrelated,
            },
        };

        let may_break: Vec<_> = contract.may_break().collect();
        assert_eq!(may_break, vec![
            InvariantId::RadialReciprocity,
            InvariantId::NextPrevReciprocity,
        ]);

        let requires: Vec<_> = contract.requires().collect();
        assert!(requires.is_empty());

        let ensures: Vec<_> = contract.ensures().collect();
        assert_eq!(ensures, vec![
            InvariantId::NoDanglingRefs,
            InvariantId::GenerationalFreshness,
        ]);
    }

    /// Verify ValidatorCost ordering (used for cost-tier filtering).
    #[test]
    fn validator_cost_ordering() {
        assert!(ValidatorCost::Cheap < ValidatorCost::Medium);
        assert!(ValidatorCost::Medium < ValidatorCost::Expensive);
    }
}
