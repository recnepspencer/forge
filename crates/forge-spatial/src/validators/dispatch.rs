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
//!               forge-topo (TopologyArena, handles), forge-geom (CurveKind, Plane).

use forge_core::{InvariantId, KernelError, ToleranceProvider, ValidatorCost};
use forge_geom::facade::{CurveKind, Plane};
use forge_topo::b_rep::TopologyArena;
use forge_topo::handles::{EdgeId, FaceId, VertexId};

/// Bundles all geometry callbacks into a single parameter for spatial validators.
///
/// Adding a new geometry layer (e.g., coedge_fn in Phase 4) is a one-line
/// field addition here — no signature migration across adapters or callers.
pub struct GeometryContext<'a> {
    /// Maps `VertexId` → exact position.
    pub position_fn: &'a dyn Fn(VertexId) -> Option<[f64; 3]>,
    /// Maps `FaceId` → plane equation.
    pub plane_fn: &'a dyn Fn(FaceId) -> Option<Plane>,
    /// Returns true for faces with planar geometry bound.
    pub is_planar: &'a dyn Fn(FaceId) -> bool,
    /// Maps `EdgeId` → 3D curve kind.
    pub curve_fn: &'a dyn Fn(EdgeId) -> Option<CurveKind>,
    /// Per-entity tolerances.
    pub tolerance_provider: &'a dyn ToleranceProvider,
}

/// A geometry-dependent validator entry with its cost and check function.
///
/// Unlike `forge-topo::ValidatorEntry` (which takes only `&TopologyArena`),
/// spatial validators require geometry callbacks bundled in `GeometryContext`.
pub struct SpatialValidatorEntry {
    /// Algorithmic cost of running this validator.
    pub cost: ValidatorCost,
    /// The validation function. `None` = no-op (invariant validated elsewhere).
    check: Option<SpatialCheckFn>,
}

/// Function signature for spatial validators.
///
/// Takes the topology arena and a `GeometryContext` containing all geometry
/// callbacks. Adding new geometry layers doesn't change this signature.
type SpatialCheckFn = fn(
    &TopologyArena,
    &GeometryContext<'_>,
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
        ctx: &GeometryContext<'_>,
    ) -> Result<(), KernelError> {
        match self.check {
            Some(check) => check(arena, ctx),
            None => Ok(()),
        }
    }
}

// ── Adapter wrappers ────────────────────────────────────────────────────
//
// These wrappers extract what they need from `GeometryContext` and delegate
// to the underlying validators. Adding a new field to GeometryContext
// requires no changes here unless an adapter actually uses it.

fn check_zero_length_edges(
    arena: &TopologyArena,
    ctx: &GeometryContext<'_>,
) -> Result<(), KernelError> {
    super::edge_length::validate_zero_length_edges(arena, ctx.position_fn, ctx.tolerance_provider)
}

fn check_zero_area_faces(
    arena: &TopologyArena,
    ctx: &GeometryContext<'_>,
) -> Result<(), KernelError> {
    super::area::validate_zero_area_faces(arena, ctx.position_fn, ctx.is_planar, ctx.tolerance_provider)
}

fn check_signed_volume(
    arena: &TopologyArena,
    ctx: &GeometryContext<'_>,
) -> Result<(), KernelError> {
    super::volume::validate_signed_volume(arena, ctx.position_fn)
}

fn check_loop_orientation(
    arena: &TopologyArena,
    ctx: &GeometryContext<'_>,
) -> Result<(), KernelError> {
    super::loop_orientation::validate_loop_orientation(arena, ctx.position_fn, ctx.is_planar, ctx.tolerance_provider)
}

fn check_shell_orientation(
    arena: &TopologyArena,
    ctx: &GeometryContext<'_>,
) -> Result<(), KernelError> {
    super::shell_orientation::validate_shell_orientation(arena, ctx.position_fn, ctx.tolerance_provider)
}

fn check_surface_deviation(
    arena: &TopologyArena,
    ctx: &GeometryContext<'_>,
) -> Result<(), KernelError> {
    super::surface_deviation::validate_surface_deviation(arena, ctx.position_fn, ctx.plane_fn, ctx.tolerance_provider)
}

fn check_geometry_completeness(
    arena: &TopologyArena,
    ctx: &GeometryContext<'_>,
) -> Result<(), KernelError> {
    let plane_fn = ctx.plane_fn;
    super::completeness::validate_geometry_completeness(
        arena,
        &|f| plane_fn(f).is_some(),
        &|v| (ctx.position_fn)(v).is_some(),
        Some(&|f| plane_fn(f).is_some()),
        Some(&|e| (ctx.curve_fn)(e).is_some()),
    )
}

fn check_edge_curve_consistency(
    arena: &TopologyArena,
    ctx: &GeometryContext<'_>,
) -> Result<(), KernelError> {
    super::edge_curve_consistency::validate_edge_curve_consistency(
        arena, ctx.position_fn, ctx.curve_fn, ctx.tolerance_provider,
    )
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
        InvariantId::NoVertexOffSurface =>
            SpatialValidatorEntry::new(ValidatorCost::Medium, check_surface_deviation),
        InvariantId::GeometryCompleteness =>
            SpatialValidatorEntry::new(ValidatorCost::Cheap, check_geometry_completeness),
        InvariantId::EdgeCurveConsistency =>
            SpatialValidatorEntry::new(ValidatorCost::Medium, check_edge_curve_consistency),

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
/// Iterates through all `InvariantId` variants in the `Geometry` group
/// and runs each one. Short-circuits on first failure.
pub fn validate_all_spatial_invariants(
    arena: &TopologyArena,
    ctx: &GeometryContext<'_>,
) -> Result<(), KernelError> {
    use forge_core::InvariantGroup;
    use forge_topo::validators::invariant_group::invariant_ids;

    for &id in invariant_ids(InvariantGroup::Geometry) {
        let entry = spatial_validator_for(id);
        entry.run(arena, ctx)?;
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
        let check = spatial_validator_for(InvariantId::GeometryCompleteness);
        assert!(check.check.is_some());
        let check = spatial_validator_for(InvariantId::EdgeCurveConsistency);
        assert!(check.check.is_some());
    }

    /// Structural invariants resolve to no-ops in the spatial dispatcher.
    #[test]
    fn structural_invariants_are_noop() {
        let check = spatial_validator_for(InvariantId::RadialReciprocity);
        assert!(check.check.is_none());
    }
}
