//! Stitch reporting for paired and unpaired halfedges across a shell assembly.
//!
//! DOMAIN: Tracking result stats for twin pointer assembly.
//! This is a pure data structure with no policy or audit awareness.

use forge_core::KernelError;
use forge_topo::transactions::MutableDraft;
use forge_topo::handles::HalfEdgeId;
use crate::geometry_state::GeometryState;
use crate::core::ModelingContext;

/// Structured result from stitching — callers decide if unpaired is acceptable.
pub struct StitchReport {
    /// Total halfedges that were paired in this pass.
    pub paired_count: usize,
    /// Halfedge IDs that remain unpaired after all passes.
    pub unpaired_ids: Vec<HalfEdgeId>,
}

impl StitchReport {
    /// All halfedges were successfully paired.
    pub fn is_fully_paired(&self) -> bool {
        self.unpaired_ids.is_empty()
    }

    /// Require all halfedges paired, or return a generic TopologyViolation error.
    ///
    /// For richer diagnostics (2-ring extraction, lineage dumps), use
    /// `boolean::shared::stitch_err::build_stitch_failure_error` instead.
    pub fn require_fully_paired(
        &self,
        _draft: &MutableDraft,
        _geom: &GeometryState,
        _ctx: &ModelingContext,
    ) -> Result<(), KernelError> {
        if self.is_fully_paired() {
            return Ok(());
        }
        Err(KernelError::TopologyViolation {
            err: forge_core::TopologyError::MissingTwin {
                halfedge_index: self.unpaired_ids[0].index(),
            },
            context: Some(forge_core::ErrorContext {
                scope: forge_core::ErrorScope::Global,
                suggested_fixes: Vec::new(),
                detail: format!(
                    "{} halfedges remain unpaired after stitching",
                    self.unpaired_ids.len(),
                ),
            }),
        })
    }
}
