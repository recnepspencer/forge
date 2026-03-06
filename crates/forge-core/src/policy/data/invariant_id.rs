//! Invariant contract system — shared types for compile-time enforcement.
//!
//! DOMAIN: The single registry of all B-Rep invariants (structural + geometric).
//! Lives in `forge-core` so both `forge-topo` (combinatorial validators) and
//! `forge-spatial` (geometry-dependent validators) share one contract system.
//!
//! Two exhaustive match statements per consuming crate enforce completeness:
//! 1. Every operator's `INVARIANT_CONTRACT` closure
//! 2. The `validator_for()` / `spatial_validator_for()` dispatch functions
//!
//! Adding a new `InvariantId` variant without updating both = **compile error**.

use super::invariant_group::InvariantGroup;

/// Every B-Rep invariant in the system.
///
/// Rust's exhaustive pattern matching guarantees that every operator
/// acknowledges every invariant and every invariant has a validator.
///
/// Variants are grouped into two tiers:
/// - **Structural** (validated by `forge-topo`): pure combinatorial graph checks
/// - **Geometric** (validated by `forge-spatial`): require vertex positions / tolerances
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

    // ── Geometry-dependent (validated by forge-spatial) ─────────
    /// Edge length below vertex tolerance threshold.
    NoZeroLengthEdges,
    /// Face area below vertex tolerance² threshold.
    NoZeroAreaFaces,
    /// Shell signed volume is negative (normals point inward).
    NoInsideOutShells,
    /// Loop winding matches face sense (outer=CCW, inner=CW).
    LoopOrientationConsistency,
    /// Adjacent face normals across shared edges are compatible.
    ShellOrientationConsistency,
    /// All vertices of a face lie on its supporting surface within tolerance.
    NoVertexOffSurface,
    /// Every face has plane+surface, every edge has curve, every vertex has position.
    GeometryCompleteness,
    /// Edge curve origin/direction/destination match vertex positions.
    EdgeCurveConsistency,
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

            Self::NoZeroLengthEdges | Self::NoZeroAreaFaces
            | Self::NoInsideOutShells | Self::LoopOrientationConsistency
            | Self::ShellOrientationConsistency | Self::NoVertexOffSurface
            | Self::GeometryCompleteness | Self::EdgeCurveConsistency
                => InvariantGroup::Geometry,
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
        // Geometry-dependent:
        Self::NoZeroLengthEdges,
        Self::NoZeroAreaFaces,
        Self::NoInsideOutShells,
        Self::LoopOrientationConsistency,
        Self::ShellOrientationConsistency,
        Self::NoVertexOffSurface,
        Self::GeometryCompleteness,
        Self::EdgeCurveConsistency,
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

/// Compile-time invariant contract for operators.
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
