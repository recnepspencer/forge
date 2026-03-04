//! Invariant groups — named subsets of `InvariantId` for feature-level
//! and validation-level composition.
//!
//! DOMAIN: Feature contracts and validation levels reference groups
//! instead of individual invariants. This is the bridge between
//! operator-level `InvariantContract` and feature-level `FeatureContract`.
//!
//! Replaces `InvariantKind` from forge-kernel (migration deferred to post-M0).

use super::invariant_id::InvariantId;

/// Named subsets of `InvariantId` for higher-level consumers.
///
/// Features declare groups; the pipeline resolves them to individual
/// `InvariantId`s and dispatches through `validator_for()`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InvariantGroup {
    /// Radial/next/prev reciprocity, dangling refs, generational freshness.
    PointerCoherence,
    /// Face-has-loop, loop cardinality, duplicates, membership, continuity, endpoints.
    LoopIntegrity,
    /// Single loop owner, no orphan HEs, acyclic containment, inner/outer consistency.
    Ownership,
    /// Radial cycle uniqueness, neighbor consistency, no broken splices.
    RadialEdge,
    /// Face adjacency symmetry, closed boundaries, laminar boundary edges.
    ShellClosure,
    /// Disk entries alive, partition correct, closure, no cross-disk coedges.
    VertexDisk,
    /// Per-component Euler formula.
    EulerFormula,
    /// Side-car and index coherence.
    CacheCoherence,
}

impl InvariantGroup {
    /// Resolve this group to its constituent `InvariantId` variants.
    pub fn invariant_ids(&self) -> &[InvariantId] {
        match self {
            Self::PointerCoherence => &[
                InvariantId::RadialReciprocity,
                InvariantId::NextPrevReciprocity,
                InvariantId::NoDanglingRefs,
                InvariantId::GenerationalFreshness,
            ],
            Self::LoopIntegrity => &[
                InvariantId::FaceHasLoop,
                InvariantId::LoopMinCardinality,
                InvariantId::NoDuplicateCoedges,
                InvariantId::FaceLoopMembership,
                InvariantId::VertexContinuity,
                InvariantId::EdgeEndpointsMatch,
            ],
            Self::Ownership => &[
                InvariantId::SingleLoopOwner,
                InvariantId::NoOrphanHalfEdges,
                InvariantId::AcyclicContainment,
                InvariantId::InnerOuterConsistency,
            ],
            Self::RadialEdge => &[
                InvariantId::RadialCycleUniqueness,
                InvariantId::RadialNeighborConsistency,
                InvariantId::NoBrokenRadialSplices,
            ],
            Self::ShellClosure => &[
                InvariantId::FaceAdjacencyConsistency,
                InvariantId::NoBrokenFaceBoundary,
                InvariantId::BoundaryEdgesLaminar,
            ],
            Self::VertexDisk => &[
                InvariantId::DiskEntriesAlive,
                InvariantId::DiskPartitionCorrect,
                InvariantId::DiskClosure,
                InvariantId::NoCrossDiskCoedges,
            ],
            Self::EulerFormula => &[
                InvariantId::PerComponentEuler,
            ],
            Self::CacheCoherence => &[
                InvariantId::SideCarCoherence,
                InvariantId::IndexCoherence,
            ],
        }
    }

    /// All groups.
    pub const ALL: &[InvariantGroup] = &[
        Self::PointerCoherence,
        Self::LoopIntegrity,
        Self::Ownership,
        Self::RadialEdge,
        Self::ShellClosure,
        Self::VertexDisk,
        Self::EulerFormula,
        Self::CacheCoherence,
    ];
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every group resolves to at least one InvariantId.
    #[test]
    fn all_groups_resolve_to_invariants() {
        for &group in InvariantGroup::ALL {
            let ids = group.invariant_ids();
            assert!(
                !ids.is_empty(),
                "Group {:?} resolves to zero invariants",
                group,
            );
        }
    }

    /// The union of all groups covers every InvariantId.
    #[test]
    fn groups_cover_all_invariant_ids() {
        let mut covered = std::collections::HashSet::new();
        for &group in InvariantGroup::ALL {
            for &id in group.invariant_ids() {
                covered.insert(id);
            }
        }
        for &id in InvariantId::ALL {
            assert!(
                covered.contains(&id),
                "InvariantId::{:?} is not covered by any InvariantGroup",
                id,
            );
        }
    }
}
