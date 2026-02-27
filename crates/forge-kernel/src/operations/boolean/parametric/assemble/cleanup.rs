//! Post-assembly degenerate topology cleanup wrapper.
//!
//! DOMAIN: Boolean assembly orchestration delegates structural cleanup to
//! `forge-topo` and keeps kernel-level call sites stable.

use forge_core::KernelError;
use forge_topo::state::MutableDraft;

use crate::geometry_state::GeometryState;

/// Remove degenerate faces and zero-length edges from the draft.
///
/// Geometry is currently unused by the shared topo cleanup, but preserved in the
/// API for caller compatibility while boolean assembly is refactored.
pub fn cleanup_degenerate_topology(
    draft: &mut MutableDraft,
    _geom: &GeometryState,
) -> Result<usize, KernelError> {
    forge_topo::algorithms::simplify::cleanup_degenerate_topology(draft)
}
