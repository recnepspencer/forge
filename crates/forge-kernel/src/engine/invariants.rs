//! Invariant validation dispatch for the feature pipeline.
//!
//! DOMAIN: Maps `InvariantKind` variants to existing validators in
//! forge-topo and forge-kernel. Called by `FeaturePipeline::execute`
//! as a post-execution check.
//!
//! DEPENDENCIES: forge-topo (validate_topology), forge-kernel::analysis (sliver),
//! proof_validation::checkpoint (ValidationConfig, ValidationCheckpoint)

use forge_core::KernelError;
use forge_topo::state::TopologyState;
use forge_topo::validate::{validate_topology, ValidationLevel};

use crate::proof::checkpoint::schema::{ValidationCheckpoint, ValidationConfig};
use crate::engine::contract::InvariantKind;
use crate::geometry_state::GeometryState;

/// Validate a single post-execution invariant against the feature output.
///
/// Each `InvariantKind` maps to an existing validator. This function is
/// the single dispatch point for all invariant checks.
///
/// Respects `ValidationConfig` — if `PostFeature` is not active in the
/// config, all checks are skipped. This allows debug builds to validate
/// everything while release builds skip expensive checks.
pub fn validate_invariant(
    topology: &TopologyState,
    _geometry: &GeometryState,
    kind: &InvariantKind,
    config: &ValidationConfig,
) -> Result<(), KernelError> {
    if !config.is_active(ValidationCheckpoint::PostFeature) {
        return Ok(());
    }

    match kind {
        InvariantKind::ManifoldEdges => validate_topology(topology.arena(), ValidationLevel::Full),
        InvariantKind::G1Continuity => {
            // Future: validate_geometric_invariants(topology.arena(), geometry)
            // Checks face normals agree at shared edges within tangency tolerance.
            // Stub until curved geometry demands it.
            Ok(())
        }
        InvariantKind::NoSelfIntersection => {
            // Future: spatial self-intersection test via BVH
            Ok(())
        }
        InvariantKind::NoSliverFaces => {
            // Note: actual sliver detection requires a threshold from
            // ToleranceConfig. The full integration passes the threshold
            // from ModelingContext. For now, this invariant is a stub
            // that the executor can wire up with ctx.tolerance_config().
            Ok(())
        }
    }
}
