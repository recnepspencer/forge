//! Invariant groups — bridge between forge-core `InvariantGroup` contract type
//! and forge-topo `InvariantId` variants.
//!
//! DOMAIN: The `InvariantGroup` enum lives in `forge-core` (shared contract type).
//! This module re-exports it and provides the `invariant_ids()` free function
//! that resolves groups to their constituent `InvariantId`s.
//!
//! `invariant_ids()` is a free function (not a method) because Rust orphan rules
//! prevent adding inherent methods to types from external crates.

pub use forge_core::InvariantGroup;
pub use forge_core::InvariantTier;

use super::invariant_id::InvariantId;

/// Resolve a group to its constituent `InvariantId` variants.
///
/// `InvariantId` now lives in `forge-core` (shared contract type).
/// This function maps groups to their constituent IDs for the
/// topo-specific scheduling and validation scheduling.
pub fn invariant_ids(group: InvariantGroup) -> &'static [InvariantId] {
    match group {
        InvariantGroup::PointerCoherence => &[
            InvariantId::RadialReciprocity,
            InvariantId::NextPrevReciprocity,
            InvariantId::NoDanglingRefs,
            InvariantId::GenerationalFreshness,
        ],
        InvariantGroup::LoopIntegrity => &[
            InvariantId::FaceHasLoop,
            InvariantId::LoopMinCardinality,
            InvariantId::NoDuplicateCoedges,
            InvariantId::FaceLoopMembership,
            InvariantId::VertexContinuity,
            InvariantId::EdgeEndpointsMatch,
        ],
        InvariantGroup::Ownership => &[
            InvariantId::SingleLoopOwner,
            InvariantId::NoOrphanHalfEdges,
            InvariantId::AcyclicContainment,
            InvariantId::InnerOuterConsistency,
        ],
        InvariantGroup::RadialEdge => &[
            InvariantId::RadialCycleUniqueness,
            InvariantId::RadialNeighborConsistency,
            InvariantId::NoBrokenRadialSplices,
        ],
        InvariantGroup::ShellClosure => &[
            InvariantId::FaceAdjacencyConsistency,
            InvariantId::NoBrokenFaceBoundary,
            InvariantId::BoundaryEdgesLaminar,
        ],
        InvariantGroup::VertexDisk => &[
            InvariantId::DiskEntriesAlive,
            InvariantId::DiskPartitionCorrect,
            InvariantId::DiskClosure,
            InvariantId::NoCrossDiskCoedges,
        ],
        InvariantGroup::EulerFormula => &[
            InvariantId::PerComponentEuler,
        ],
        InvariantGroup::CacheCoherence => &[
            InvariantId::SideCarCoherence,
            InvariantId::IndexCoherence,
        ],
        InvariantGroup::Geometry => &[
            InvariantId::NoZeroLengthEdges,
            InvariantId::NoZeroAreaFaces,
            InvariantId::NoInsideOutShells,
            InvariantId::LoopOrientationConsistency,
            InvariantId::ShellOrientationConsistency,
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every group resolves to at least one InvariantId.
    #[test]
    fn all_groups_resolve_to_invariants() {
        for &group in InvariantGroup::ALL {
            let ids = invariant_ids(group);
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
            for &id in invariant_ids(group) {
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

    /// `InvariantId::group()` round-trips correctly through `invariant_ids()`.
    /// This ensures that the two match statements stay perfectly in sync.
    #[test]
    fn group_roundtrips_with_invariant_ids() {
        for &id in InvariantId::ALL {
            let group = id.group();
            let ids = invariant_ids(group);
            assert!(
                ids.contains(&id),
                "InvariantId::{:?} claims to be in InvariantGroup::{:?}, but the group does not list it.",
                id, group
            );
        }
    }

    /// Every InvariantGroup has a valid tier.
    #[test]
    fn all_groups_have_tiers() {
        for &group in InvariantGroup::ALL {
            let tier = group.tier();
            // Just verify it doesn't panic and the tier is one of the expected values
            match tier {
                InvariantTier::Topology | InvariantTier::Semantic | InvariantTier::Cache => {}
            }
        }
    }

    /// Bitmask round-trips: each group has a unique, non-zero mask.
    #[test]
    fn bitmask_uniqueness() {
        let mut seen = 0u32;
        for &group in InvariantGroup::ALL {
            let mask = group.mask();
            assert_ne!(mask, 0, "Group {:?} has zero mask", group);
            assert_eq!(seen & mask, 0, "Group {:?} mask collides", group);
            seen |= mask;
        }
    }
}
