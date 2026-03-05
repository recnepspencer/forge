//! Invariant contract system types and compile-time enforcement.
//!
//! DOMAIN: The single registry of all structural B-Rep invariants.
//! Two exhaustive match statements enforce completeness at compile time:
//! 1. Every `TopoOperator`'s `INVARIANT_CONTRACT` closure
//! 2. The `validator_for()` dispatch function
//!
//! Adding a new `InvariantId` variant without updating both = **compile error**.

use crate::b_rep::TopologyArena;
use crate::validators::invariant_group::InvariantGroup;
use forge_core::KernelError;
pub use forge_core::ValidatorCost;

/// Every structural B-Rep invariant in the system.
///
/// Rust's exhaustive pattern matching guarantees that every operator
/// acknowledges every invariant and every invariant has a validator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InvariantId {
    // ── Pointer coherence ───────────────────────────────────────
    /// `radial_next∘radial_next == id` for every half-edge.
    RadialReciprocity,
    /// `next∘prev == id` and `prev∘next == id` for every half-edge.
    NextPrevReciprocity,
    /// No half-edge references point to deleted entities.
    NoDanglingRefs,
    /// No references point to recycled (generation-bumped) slots.
    GenerationalFreshness,

    // ── Loop structure ──────────────────────────────────────────
    /// Every face has at least one loop.
    FaceHasLoop,
    /// Every loop has at least 1 half-edge (self-loop ok).
    LoopMinCardinality,
    /// No duplicate half-edges within a single loop.
    NoDuplicateCoedges,
    /// All half-edges in a loop reference their owning face.
    FaceLoopMembership,
    /// Adjacent half-edges in a loop share a vertex.
    VertexContinuity,
    /// Edge endpoint vertices match the loop vertex wiring.
    EdgeEndpointsMatch,

    // ── Ownership ───────────────────────────────────────────────
    /// Each loop belongs to exactly one face.
    SingleLoopOwner,
    /// Every half-edge belongs to a loop.
    NoOrphanHalfEdges,
    /// Containment hierarchy is a DAG (no cycles).
    AcyclicContainment,
    /// Inner/outer loop nesting is geometrically correct.
    InnerOuterConsistency,

    // ── Radial edge ─────────────────────────────────────────────
    /// Radial ring has no duplicate half-edges.
    RadialCycleUniqueness,
    /// Radial neighbors share the same edge entity.
    RadialNeighborConsistency,
    /// Radial ring continuity (no broken splices).
    NoBrokenRadialSplices,

    // ── Shell closure ───────────────────────────────────────────
    /// Face adjacency through shared edges is symmetric.
    FaceAdjacencyConsistency,
    /// All loops close (next-walk returns to start).
    NoBrokenFaceBoundary,
    /// Boundary edges only appear in non-solid shells.
    BoundaryEdgesLaminar,

    // ── Vertex disk ─────────────────────────────────────────────
    /// Every disk entry references an alive half-edge.
    DiskEntriesAlive,
    /// disk_entries.len() matches the actual disk count.
    DiskPartitionCorrect,
    /// Each disk cycle closes upon itself.
    DiskClosure,
    /// No co-edges cross disk boundaries.
    NoCrossDiskCoedges,

    // ── Euler formula ───────────────────────────────────────────
    /// V − E + F = 2(S − G) per connected component.
    PerComponentEuler,

    // ── Side-car coherence ──────────────────────────────────────
    /// Side-car maps don't reference deleted entities.
    SideCarCoherence,
    /// Cache indexes (face→halfedges, etc.) match ground truth.
    IndexCoherence,
}

impl InvariantId {
    /// Resolves this invariant back to its higher-level group.
    /// 
    /// Adding a new `InvariantId` will cause a compile error here,
    /// forcing it to be assigned the correct group.
    pub const fn group(&self) -> InvariantGroup {
        match self {
            Self::RadialReciprocity | Self::NextPrevReciprocity
            | Self::NoDanglingRefs | Self::GenerationalFreshness
                => InvariantGroup::PointerCoherence,
            
            Self::FaceHasLoop | Self::LoopMinCardinality
            | Self::NoDuplicateCoedges | Self::FaceLoopMembership
            | Self::VertexContinuity | Self::EdgeEndpointsMatch
                => InvariantGroup::LoopIntegrity,
            
            Self::SingleLoopOwner | Self::NoOrphanHalfEdges
            | Self::AcyclicContainment | Self::InnerOuterConsistency
                => InvariantGroup::Ownership,
            
            Self::RadialCycleUniqueness | Self::RadialNeighborConsistency
            | Self::NoBrokenRadialSplices
                => InvariantGroup::RadialEdge,
            
            Self::FaceAdjacencyConsistency | Self::NoBrokenFaceBoundary
            | Self::BoundaryEdgesLaminar
                => InvariantGroup::ShellClosure,
            
            Self::DiskEntriesAlive | Self::DiskPartitionCorrect
            | Self::DiskClosure | Self::NoCrossDiskCoedges
                => InvariantGroup::VertexDisk,
            
            Self::PerComponentEuler
                => InvariantGroup::EulerFormula,
            
            Self::SideCarCoherence | Self::IndexCoherence
                => InvariantGroup::CacheCoherence,
        }
    }
    /// All invariant variants, listed exhaustively.
    ///
    /// Used by `may_break()`, `requires()`, and CI gate tests.
    pub const ALL: &[InvariantId] = &[
        Self::RadialReciprocity,
        Self::NextPrevReciprocity,
        Self::NoDanglingRefs,
        Self::GenerationalFreshness,
        Self::FaceHasLoop,
        Self::LoopMinCardinality,
        Self::NoDuplicateCoedges,
        Self::FaceLoopMembership,
        Self::VertexContinuity,
        Self::EdgeEndpointsMatch,
        Self::SingleLoopOwner,
        Self::NoOrphanHalfEdges,
        Self::AcyclicContainment,
        Self::InnerOuterConsistency,
        Self::RadialCycleUniqueness,
        Self::RadialNeighborConsistency,
        Self::NoBrokenRadialSplices,
        Self::FaceAdjacencyConsistency,
        Self::NoBrokenFaceBoundary,
        Self::BoundaryEdgesLaminar,
        Self::DiskEntriesAlive,
        Self::DiskPartitionCorrect,
        Self::DiskClosure,
        Self::NoCrossDiskCoedges,
        Self::PerComponentEuler,
        Self::SideCarCoherence,
        Self::IndexCoherence,
    ];
}

/// Declares how an operator relates to a specific invariant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InvariantRelation {
    /// Doesn't read or write state relevant to this invariant.
    Unrelated,

    /// Precondition: assumes the invariant holds on entry.
    Requires,

    /// Postcondition: guarantees the invariant holds on exit.
    Ensures,

    /// Temporarily violates during execution but restores before
    /// returning. Implies both Requires and Ensures.
    TemporarilyViolatesButEnsures,

    /// May leave this invariant violated after execution.
    /// The validator MUST check it post-op.
    MayBreak,
}

/// Compile-time invariant contract for a `TopoOperator`.
///
/// The `relation` function must use an exhaustive `match` on `InvariantId`.
/// Adding a new variant without covering it = compile error.
pub struct InvariantContract {
    /// Maps every invariant to this operator's relation with it.
    pub relation: fn(InvariantId) -> InvariantRelation,
}

impl InvariantContract {
    /// Invariants this operator may leave violated (require post-op validation).
    pub fn may_break(&self) -> impl Iterator<Item = InvariantId> + '_ {
        InvariantId::ALL.iter().copied()
            .filter(|id| matches!((self.relation)(*id), InvariantRelation::MayBreak))
    }

    /// Invariants this operator requires as preconditions.
    pub fn requires(&self) -> impl Iterator<Item = InvariantId> + '_ {
        InvariantId::ALL.iter().copied()
            .filter(|id| matches!(
                (self.relation)(*id),
                InvariantRelation::Requires
                    | InvariantRelation::TemporarilyViolatesButEnsures
            ))
    }

    /// Invariants this operator guarantees on exit.
    pub fn ensures(&self) -> impl Iterator<Item = InvariantId> + '_ {
        InvariantId::ALL.iter().copied()
            .filter(|id| matches!(
                (self.relation)(*id),
                InvariantRelation::Ensures
                    | InvariantRelation::TemporarilyViolatesButEnsures
            ))
    }
}

/// Generate a conservative `InvariantContract` that maps every invariant
/// to `MayBreak`.
///
/// **Phase 1 scaffold**: Use this for operators whose precise invariant
/// relations haven't been analyzed yet. In Phase 2, replace with an
/// explicit exhaustive match per operator.
///
/// Adding a new `InvariantId` variant will cause a compile error here,
/// forcing the macro to be updated — preserving exhaustiveness.
#[macro_export]
macro_rules! conservative_contract {
    () => {
        $crate::validators::invariant_id::InvariantContract {
            relation: |id| {
                match id {
                    $crate::validators::invariant_id::InvariantId::RadialReciprocity
                    | $crate::validators::invariant_id::InvariantId::NextPrevReciprocity
                    | $crate::validators::invariant_id::InvariantId::NoDanglingRefs
                    | $crate::validators::invariant_id::InvariantId::GenerationalFreshness
                    | $crate::validators::invariant_id::InvariantId::FaceHasLoop
                    | $crate::validators::invariant_id::InvariantId::LoopMinCardinality
                    | $crate::validators::invariant_id::InvariantId::NoDuplicateCoedges
                    | $crate::validators::invariant_id::InvariantId::FaceLoopMembership
                    | $crate::validators::invariant_id::InvariantId::VertexContinuity
                    | $crate::validators::invariant_id::InvariantId::EdgeEndpointsMatch
                    | $crate::validators::invariant_id::InvariantId::SingleLoopOwner
                    | $crate::validators::invariant_id::InvariantId::NoOrphanHalfEdges
                    | $crate::validators::invariant_id::InvariantId::AcyclicContainment
                    | $crate::validators::invariant_id::InvariantId::InnerOuterConsistency
                    | $crate::validators::invariant_id::InvariantId::RadialCycleUniqueness
                    | $crate::validators::invariant_id::InvariantId::RadialNeighborConsistency
                    | $crate::validators::invariant_id::InvariantId::NoBrokenRadialSplices
                    | $crate::validators::invariant_id::InvariantId::FaceAdjacencyConsistency
                    | $crate::validators::invariant_id::InvariantId::NoBrokenFaceBoundary
                    | $crate::validators::invariant_id::InvariantId::BoundaryEdgesLaminar
                    | $crate::validators::invariant_id::InvariantId::DiskEntriesAlive
                    | $crate::validators::invariant_id::InvariantId::DiskPartitionCorrect
                    | $crate::validators::invariant_id::InvariantId::DiskClosure
                    | $crate::validators::invariant_id::InvariantId::NoCrossDiskCoedges
                    | $crate::validators::invariant_id::InvariantId::PerComponentEuler
                    | $crate::validators::invariant_id::InvariantId::SideCarCoherence
                    | $crate::validators::invariant_id::InvariantId::IndexCoherence
                    => $crate::validators::invariant_id::InvariantRelation::MayBreak,
                }
            },
        }
    };
}

// ValidatorCost is re-exported from forge-core (see top of file).
// It was moved there so forge-kernel's GroupPolicyConfig can reference it.

/// Registry entry mapping an `InvariantId` to its checker function and cost.
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
}

/// Dispatch every `InvariantId` to its validator function and cost tier.
///
/// Exhaustive match — adding an `InvariantId` variant without a validator
/// is a compile error.
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
        // SideCarCoherence uses IndexCoherence as a proxy until M1
        // introduces dedicated side-car maps.
        InvariantId::SideCarCoherence =>
            ValidatorEntry::cheap(cache_index::validate_index_coherence),
        InvariantId::IndexCoherence =>
            ValidatorEntry::cheap(cache_index::validate_index_coherence),
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
            27,
            "ALL should contain all 27 InvariantId variants"
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
                InvariantId::FaceHasLoop => InvariantRelation::Unrelated,
                InvariantId::LoopMinCardinality => InvariantRelation::Unrelated,
                InvariantId::NoDuplicateCoedges => InvariantRelation::Unrelated,
                InvariantId::FaceLoopMembership => InvariantRelation::Unrelated,
                InvariantId::VertexContinuity => InvariantRelation::Unrelated,
                InvariantId::EdgeEndpointsMatch => InvariantRelation::Unrelated,
                InvariantId::SingleLoopOwner => InvariantRelation::Unrelated,
                InvariantId::NoOrphanHalfEdges => InvariantRelation::Unrelated,
                InvariantId::AcyclicContainment => InvariantRelation::Unrelated,
                InvariantId::InnerOuterConsistency => InvariantRelation::Unrelated,
                InvariantId::RadialCycleUniqueness => InvariantRelation::Unrelated,
                InvariantId::RadialNeighborConsistency => InvariantRelation::Unrelated,
                InvariantId::NoBrokenRadialSplices => InvariantRelation::Unrelated,
                InvariantId::FaceAdjacencyConsistency => InvariantRelation::Unrelated,
                InvariantId::NoBrokenFaceBoundary => InvariantRelation::Unrelated,
                InvariantId::BoundaryEdgesLaminar => InvariantRelation::Unrelated,
                InvariantId::DiskEntriesAlive => InvariantRelation::Unrelated,
                InvariantId::DiskPartitionCorrect => InvariantRelation::Unrelated,
                InvariantId::DiskClosure => InvariantRelation::Unrelated,
                InvariantId::NoCrossDiskCoedges => InvariantRelation::Unrelated,
                InvariantId::PerComponentEuler => InvariantRelation::Unrelated,
                InvariantId::SideCarCoherence => InvariantRelation::Unrelated,
                InvariantId::IndexCoherence => InvariantRelation::Unrelated,
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
