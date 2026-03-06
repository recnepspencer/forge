//! Manifold validation for committed kernel states.
//!
//! DOMAIN: Provides a higher-level wrapper over `forge-topo` structural
//! validation and `forge-spatial` geometric validation, usable from
//! the kernel's proof/checkpoint system.
//!
//! DEPENDENCIES: forge-topo (TopologyState, ValidationLevel),
//!               forge-spatial (validate_geometric_invariants, GeometryContext),
//!               forge-core (KernelError, ToleranceProvider).
//! INVARIANTS: No mutation — read-only validation.

use forge_core::{KernelError, ToleranceProvider};
use forge_topo::handles::VertexId;
use forge_topo::transactions::TopologyState;
use forge_topo::validate::{validate_topology, ValidationLevel};

/// Validate topology (structural invariants).
pub fn validate_structure(topo: &TopologyState, level: ValidationLevel) -> Result<(), KernelError> {
    validate_topology(topo.arena(), level)
}

/// Validate geometry (spatial invariants) using a `GeometryContext`.
///
/// Delegates to `forge_spatial::validate_geometric_invariants` which runs
/// all geometry-dependent validators through the dispatch system.
pub fn validate_geometry(
    topo: &TopologyState,
    ctx: &forge_spatial::GeometryContext<'_>,
) -> Result<(), KernelError> {
    forge_spatial::validate_geometric_invariants(topo.arena(), ctx)
}
