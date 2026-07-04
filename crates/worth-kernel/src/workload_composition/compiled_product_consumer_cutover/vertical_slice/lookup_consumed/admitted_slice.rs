use worth_spatial::facade::replay_undo_semantic_graph::CurrentReplayUndoSpatialBoundary;

use crate::workload_composition::{LookupConsumedWorkloadDenial, WorkloadCompositionError};

#[derive(Clone, Debug)]
pub(crate) struct LookupConsumedVerticalSlice {
    boundary: CurrentReplayUndoSpatialBoundary,
}

impl LookupConsumedVerticalSlice {
    pub(crate) fn admit(
        boundary: &CurrentReplayUndoSpatialBoundary,
    ) -> Result<Self, WorkloadCompositionError> {
        let handoff = boundary.workload_handoff();
        if boundary.authority().stage_index_identity() != handoff.workload_stage_index_identity() {
            return Err(WorkloadCompositionError::LookupConsumedWorkload(
                LookupConsumedWorkloadDenial::StageIndexIdentityMismatch,
            ));
        }
        if handoff.counters().raw_row_scan_count() != 0
            || handoff.counters().broad_receipt_scan_count() != 0
        {
            return Err(WorkloadCompositionError::LookupConsumedWorkload(
                LookupConsumedWorkloadDenial::BroadEvidenceFallbackScan,
            ));
        }
        if handoff.counters().caller_owned_scan_count() != 0 {
            return Err(WorkloadCompositionError::LookupConsumedWorkload(
                LookupConsumedWorkloadDenial::CallerOwnedLookupScan,
            ));
        }
        Ok(Self {
            boundary: boundary.clone(),
        })
    }

    pub(crate) fn boundary(&self) -> &CurrentReplayUndoSpatialBoundary {
        &self.boundary
    }
}
