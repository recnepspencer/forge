use worth_spatial::facade::evidence_lookup_execution::EvidenceLookupExecutionReceipt;
use worth_spatial::facade::evidence_lookup_workload_cutover::EvidenceLookupConsumedWorkloadHandoff;
use worth_spatial::facade::workload_vocabulary::SpatialGeometryEvidenceTouchAuthority;

use super::{LookupConsumedWorkloadDenial, WorkloadCompositionError, WorthWorkload};
use crate::workload_composition::{
    admit_spatial_conflict_input, AdmittedSpatialConflictInput, SpatialConflictInputRequest,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LookupConsumedWorkloadComposition {
    workload: WorthWorkload,
    handoff: EvidenceLookupConsumedWorkloadHandoff,
}

impl LookupConsumedWorkloadComposition {
    pub(crate) fn admit(
        workload: &WorthWorkload,
        handoff: &EvidenceLookupConsumedWorkloadHandoff,
    ) -> Result<Self, WorkloadCompositionError> {
        let workload_stage_index_identity =
            workload.evidence_ledger().stage_index().index_identity();
        if workload_stage_index_identity != handoff.workload_stage_index_identity() {
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
            workload: workload.clone(),
            handoff: handoff.clone(),
        })
    }

    pub fn workload(&self) -> &WorthWorkload {
        &self.workload
    }

    pub fn handoff(&self) -> &EvidenceLookupConsumedWorkloadHandoff {
        &self.handoff
    }

    pub fn admit_spatial_conflict_input<'a>(
        &'a self,
        authority: &'a SpatialGeometryEvidenceTouchAuthority,
        execution_receipt: &'a EvidenceLookupExecutionReceipt,
    ) -> Result<AdmittedSpatialConflictInput<'a>, WorkloadCompositionError> {
        admit_spatial_conflict_input(
            SpatialConflictInputRequest::new(authority)
                .with_evidence_lookup(self.handoff(), execution_receipt),
        )
    }
}

impl WorthWorkload {
    pub(crate) fn admit_lookup_consumed_workload(
        &self,
        handoff: &EvidenceLookupConsumedWorkloadHandoff,
    ) -> Result<LookupConsumedWorkloadComposition, WorkloadCompositionError> {
        LookupConsumedWorkloadComposition::admit(self, handoff)
    }
}
