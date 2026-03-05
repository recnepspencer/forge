//! Spatial invariant dispatch — maps `InvariantId` to geometry-dependent validators.
//!
//! DOMAIN: The spatial counterpart to `forge-topo::validator_for()`. Uses an
//! exhaustive match on every `InvariantId` variant to guarantee compile-time
//! coverage when new invariants are added.
//!
//! Structural (combinatorial) invariants return no-ops here — they are
//! dispatched through `forge-topo::validator_for()`.
//!
//! DEPENDENCIES: forge-core (InvariantId, ValidatorCost, KernelError, ToleranceProvider),
//!               forge-topo (TopologyArena, handles).

use forge_core::{InvariantId, KernelError, ToleranceProvider, ValidatorCost};
use forge_topo::b_rep::TopologyArena;
use forge_topo::handles::{FaceId, VertexId};

/// A geometry-dependent validator entry with its cost and check function.
///
/// Unlike `forge-topo::ValidatorEntry` (which takes only `&TopologyArena`),
/// spatial validators require position callbacks and tolerance providers.
pub struct SpatialValidatorEntry {
    /// Algorithmic cost of running this validator.
    pub cost: ValidatorCost,
    /// The validation function. `None` = no-op (invariant validated elsewhere).
    check: Option<SpatialCheckFn>,
}

/// Function signature for spatial validators.
///
/// Parameters:
/// - `arena` — the topology snapshot
/// - `position_fn` — maps `VertexId` → position (caller provides from GeometryState)
/// - `is_planar` — true for faces that have planar geometry bound
/// - `tolerance_provider` — per-entity tolerances
type SpatialCheckFn = fn(
    &TopologyArena,
    &dyn Fn(VertexId) -> Option<[f64; 3]>,
    &dyn Fn(FaceId) -> bool,
    &dyn ToleranceProvider,
) -> Result<(), KernelError>;

impl SpatialValidatorEntry {
    /// Create a validator entry with a check function.
    const fn new(cost: ValidatorCost, check: SpatialCheckFn) -> Self {
        Self { cost, check: Some(check) }
    }

    /// No-op entry for invariants not validated by forge-spatial.
    const fn noop() -> Self {
        Self { cost: ValidatorCost::Cheap, check: None }
    }

    /// Run this validator. Returns `Ok(())` for no-ops.
    pub fn run(
        &self,
        arena: &TopologyArena,
        position_fn: &dyn Fn(VertexId) -> Option<[f64; 3]>,
        is_planar: &dyn Fn(FaceId) -> bool,
        tolerance_provider: &dyn ToleranceProvider,
    ) -> Result<(), KernelError> {
        match self.check {
            Some(check) => check(arena, position_fn, is_planar, tolerance_provider),
            None => Ok(()),
        }
    }
}

// ── Adapter wrappers ────────────────────────────────────────────────────
//
// The existing validators have slightly different signatures (some take
// `is_planar`, some don't). These wrappers normalize them to `SpatialCheckFn`.

fn check_zero_length_edges(
    arena: &TopologyArena,
    position_fn: &dyn Fn(VertexId) -> Option<[f64; 3]>,
    _is_planar: &dyn Fn(FaceId) -> bool,
    tolerance_provider: &dyn ToleranceProvider,
) -> Result<(), KernelError> {
    super::edge_length::validate_zero_length_edges(arena, position_fn, tolerance_provider)
}

fn check_zero_area_faces(
    arena: &TopologyArena,
    position_fn: &dyn Fn(VertexId) -> Option<[f64; 3]>,
    is_planar: &dyn Fn(FaceId) -> bool,
    tolerance_provider: &dyn ToleranceProvider,
) -> Result<(), KernelError> {
    super::area::validate_zero_area_faces(arena, position_fn, is_planar, tolerance_provider)
}

fn check_signed_volume(
    arena: &TopologyArena,
    position_fn: &dyn Fn(VertexId) -> Option<[f64; 3]>,
    _is_planar: &dyn Fn(FaceId) -> bool,
    _tolerance_provider: &dyn ToleranceProvider,
) -> Result<(), KernelError> {
    super::volume::validate_signed_volume(arena, position_fn)
}

fn check_loop_orientation(
    arena: &TopologyArena,
    position_fn: &dyn Fn(VertexId) -> Option<[f64; 3]>,
    is_planar: &dyn Fn(FaceId) -> bool,
    _tolerance_provider: &dyn ToleranceProvider,
) -> Result<(), KernelError> {
    super::loop_orientation::validate_loop_orientation(arena, position_fn, is_planar)
}

fn check_shell_orientation(
    arena: &TopologyArena,
    position_fn: &dyn Fn(VertexId) -> Option<[f64; 3]>,
    _is_planar: &dyn Fn(FaceId) -> bool,
    _tolerance_provider: &dyn ToleranceProvider,
) -> Result<(), KernelError> {
    super::shell_orientation::validate_shell_orientation(arena, position_fn)
}

/// Dispatch a single `InvariantId` to its spatial validator.
///
/// Exhaustive match — adding an `InvariantId` variant without coverage
/// is a compile error. Structural invariants return no-ops.
pub fn spatial_validator_for(id: InvariantId) -> SpatialValidatorEntry {
    match id {
        // ── Geometry-dependent (this crate validates) ───────────
        InvariantId::NoZeroLengthEdges =>
            SpatialValidatorEntry::new(ValidatorCost::Cheap, check_zero_length_edges),
        InvariantId::NoZeroAreaFaces =>
            SpatialValidatorEntry::new(ValidatorCost::Medium, check_zero_area_faces),
        InvariantId::NoInsideOutShells =>
            SpatialValidatorEntry::new(ValidatorCost::Expensive, check_signed_volume),
        InvariantId::LoopOrientationConsistency =>
            SpatialValidatorEntry::new(ValidatorCost::Medium, check_loop_orientation),
        InvariantId::ShellOrientationConsistency =>
            SpatialValidatorEntry::new(ValidatorCost::Medium, check_shell_orientation),

        // ── Structural invariants (dispatched by forge-topo) ───
        InvariantId::RadialReciprocity
        | InvariantId::NextPrevReciprocity
        | InvariantId::NoDanglingRefs
        | InvariantId::GenerationalFreshness
        | InvariantId::FaceHasLoop
        | InvariantId::LoopMinCardinality
        | InvariantId::NoDuplicateCoedges
        | InvariantId::FaceLoopMembership
        | InvariantId::VertexContinuity
        | InvariantId::EdgeEndpointsMatch
        | InvariantId::SingleLoopOwner
        | InvariantId::NoOrphanHalfEdges
        | InvariantId::AcyclicContainment
        | InvariantId::InnerOuterConsistency
        | InvariantId::RadialCycleUniqueness
        | InvariantId::RadialNeighborConsistency
        | InvariantId::NoBrokenRadialSplices
        | InvariantId::FaceAdjacencyConsistency
        | InvariantId::NoBrokenFaceBoundary
        | InvariantId::BoundaryEdgesLaminar
        | InvariantId::DiskEntriesAlive
        | InvariantId::DiskPartitionCorrect
        | InvariantId::DiskClosure
        | InvariantId::NoCrossDiskCoedges
        | InvariantId::PerComponentEuler
        | InvariantId::SideCarCoherence
        | InvariantId::IndexCoherence =>
            SpatialValidatorEntry::noop(),
    }
}

/// Run all spatial (geometry-dependent) validators.
///
/// This iterates through all `InvariantId` variants in the `Geometry` group
/// and runs each one. Short-circuits on first failure.
pub fn validate_all_spatial_invariants(
    arena: &TopologyArena,
    position_fn: &dyn Fn(VertexId) -> Option<[f64; 3]>,
    is_planar: &dyn Fn(FaceId) -> bool,
    tolerance_provider: &dyn ToleranceProvider,
) -> Result<(), KernelError> {
    use forge_core::InvariantGroup;
    use forge_topo::validators::invariant_group::invariant_ids;

    for &id in invariant_ids(InvariantGroup::Geometry) {
        let entry = spatial_validator_for(id);
        entry.run(arena, position_fn, is_planar, tolerance_provider)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// CI gate: every InvariantId has a spatial validator entry.
    #[test]
    fn all_invariants_have_spatial_entries() {
        for &id in InvariantId::ALL {
            let entry = spatial_validator_for(id);
            let _ = entry.cost;
        }
    }

    /// Geometry group invariants all resolve to non-noop spatial validators.
    #[test]
    fn geometry_invariants_have_real_validators() {
        let check = spatial_validator_for(InvariantId::NoZeroLengthEdges);
        assert!(check.check.is_some());
        let check = spatial_validator_for(InvariantId::NoZeroAreaFaces);
        assert!(check.check.is_some());
        let check = spatial_validator_for(InvariantId::NoInsideOutShells);
        assert!(check.check.is_some());
        let check = spatial_validator_for(InvariantId::LoopOrientationConsistency);
        assert!(check.check.is_some());
        let check = spatial_validator_for(InvariantId::ShellOrientationConsistency);
        assert!(check.check.is_some());
    }

    /// Structural invariants resolve to no-ops in the spatial dispatcher.
    #[test]
    fn structural_invariants_are_noop() {
        let check = spatial_validator_for(InvariantId::RadialReciprocity);
        assert!(check.check.is_none());
    }
}
