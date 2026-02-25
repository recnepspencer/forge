//! Curved merge orchestration — placeholder.
//!
//! DOMAIN: Documents the execution contract for future implementation.
//! Currently returns `KernelError::NotImplemented`.
//!
//! EXECUTION CONTRACT (from REGION_MERGE_SPEC §6.6.1):
//!
//! 1. Entry point consumes `KernelState` and returns
//!    `Result<OperationResult<KernelState>, KernelError>`.
//!
//! 2. Internal execution creates a local `KernelDraft` and mutates
//!    topology + geometry through `MutableDraft` + `GeometryPatch`.
//!
//! 3. On success: `draft.commit()` finalizes topo + geom atomically.
//!
//! 4. On failure: dropping the local `KernelDraft` discards both
//!    topology and geometry mutations (fail-fast, no bleed).
//!
//! ELIGIBILITY PRECONDITIONS (all four must hold):
//!
//! (a) SurfaceRelation::Coincident between all selected face surfaces.
//! (b) Shared trims match/resolve in UV space.
//! (c) Resulting UV loops certify Simple or WeaklySimple (same
//!     certifier architecture as Epic A, UV backend).
//! (d) Normal/tangent continuity checks pass per policy.
//!
//! All decisions MUST be emitted as TracedDecisions in the
//! OperationResult's DecisionLog.

use forge_core::{KernelError, OperationResult};
use crate::core::KernelState;
use super::schema::CurvedMergeSelection;

/// Execute a curved same-support surface merge.
///
/// DESIGN TARGET: Not yet implemented. Returns `NotImplemented`.
///
/// When implemented, this function will:
/// 1. Validate that all selected faces share the same SurfaceRef.
/// 2. Classify the surface pair as Coincident (reject if not).
/// 3. Extract and project trim boundaries into UV space.
/// 4. Certify UV boundary via the boundary certifier (Epic A, UV backend).
/// 5. Build a deterministic CurvedMergePlan.
/// 6. Execute the plan inside a KernelDraft (D6 atomic transactionality).
/// 7. Clean stale face/coedge/curve bindings via GeometryPatch.
/// 8. Commit and return OperationResult with full trace.
pub fn execute_curved_merge(
    _state: KernelState,
    _selection: &CurvedMergeSelection,
) -> Result<OperationResult<KernelState>, KernelError> {
    Err(KernelError::InternalError {
        message: "Curved same-support merge is not yet implemented (Epic C scaffold only)".into(),
        context: None,
    })
}
