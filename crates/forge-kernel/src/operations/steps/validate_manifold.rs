//! Manifold validation step for operation pipeline results.
//!
//! DOMAIN: Validates that the topology arena satisfies structural manifold
//! requirements (every edge used by exactly 2 faces, Euler formula holds)
//! after a topology-mutating operation. Routes geometric invariant checks
//! through `forge-spatial` (zero-area faces, edge lengths, shell volume).
//!
//! DEPENDENCIES:
//! - forge-topo (TopologyState) — structural validation
//! - forge-spatial (validate_geometric_invariants) — geometry-dependent checks
//! - forge-core (KernelError, ToleranceProvider)

use forge_core::{KernelError, ToleranceProvider};
use forge_topo::handles::VertexId;
use forge_topo::transactions::TopologyState;

/// Run structural manifold validation on a topology state.
///
/// Checks:
/// - Every edge has ≤ 2 radial uses (non-manifold detection)
/// - Euler formula holds for each shell (V − E + F = 2·genus)
/// - All loop walks close within entity-count bound
///
/// This is purely structural — no vertex positions required.
/// For geometric checks (zero-area faces, shell volume), call
/// `validate_geometry` additionally.
pub fn validate_manifold_structural(topo: &TopologyState) -> Result<(), KernelError> {
    use forge_topo::validate::{validate_topology, ValidationLevel};

    validate_topology(topo.arena(), ValidationLevel::Intermediate).map_err(|e| {
        KernelError::TopologyViolation {
            err: forge_core::TopologyError::InvalidOperation {
                detail: format!("validate_manifold: {:?}", e),
            },
            context: None,
        }
    })
}

/// Run geometry-dependent invariant validation via forge-spatial.
///
/// Checks: zero-area faces, zero-length edges, shell signed volume.
/// Uses the callback pattern — no GeometryState import here.
///
/// # Parameters
/// - `topo` — topology snapshot
/// - `position_fn` — maps `VertexId` → position (caller provides from GeometryState)
/// - `is_planar` — true for faces that have planar geometry bound
/// - `tolerance_provider` — per-entity tolerances (GeometryState implements this)
pub fn validate_geometry(
    topo: &TopologyState,
    position_fn: &dyn Fn(VertexId) -> Option<[f64; 3]>,
    is_planar: &dyn Fn(forge_topo::handles::FaceId) -> bool,
    tolerance_provider: &dyn ToleranceProvider,
) -> Result<(), KernelError> {
    forge_spatial::validate_geometric_invariants(
        topo.arena(),
        position_fn,
        is_planar,
        tolerance_provider,
    )
}
