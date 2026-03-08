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

use forge_core::KernelError;
use forge_spec::facade::SpecState;
use forge_topo::projection::{
    ProjectedTopology, validate_projected_per_component_euler, validate_projected_topology_baseline,
};
use forge_topo::transactions::TopologyState;
use forge_topo::validate::{validate_topology, ValidationLevel};

use crate::engine::facade::SpecEnvelope;

/// Validate topology (structural invariants).
pub fn validate_structure(topo: &TopologyState, level: ValidationLevel) -> Result<(), KernelError> {
    validate_topology(topo.arena(), level)
}

/// Validate graph-native spec truth by ensuring it materializes to a valid projected topology.
pub fn validate_spec_structure(spec: &SpecState) -> Result<(), KernelError> {
    SpecEnvelope::from_spec(spec.clone()).validate_structure()
}

/// Validate spec-backed kernel output using its cached projected topology.
pub fn validate_spec_envelope_structure(envelope: &SpecEnvelope) -> Result<(), KernelError> {
    validate_projected_structure(envelope.projection()?)
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

fn validate_projected_structure(projected: &ProjectedTopology) -> Result<(), KernelError> {
    validate_projected_topology_baseline(projected)?;
    validate_projected_per_component_euler(projected)?;
    Ok(())
}
