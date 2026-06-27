use worth_spatial::facade::evidence_lookup_workload_cutover::EvidenceLookupConsumedWorkloadHandoff;

use super::{WorkloadCompositionError, WorthWorkload};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LookupConsumedWorkloadComposition {
    workload: WorthWorkload,
    handoff: EvidenceLookupConsumedWorkloadHandoff,
}

impl LookupConsumedWorkloadComposition {
    pub fn admit(
        workload: &WorthWorkload,
        handoff: &EvidenceLookupConsumedWorkloadHandoff,
    ) -> Result<Self, WorkloadCompositionError> {
        let workload_stage_index_identity =
            workload.evidence_ledger().stage_index().index_identity();
        if workload_stage_index_identity != handoff.workload_stage_index_identity() {
            return Err(WorkloadCompositionError::LookupConsumedWorkload(
                "lookup-consumed workload handoff must match the workload stage-index identity"
                    .to_string(),
            ));
        }
        if handoff.counters().raw_row_scan_count() != 0
            || handoff.counters().broad_receipt_scan_count() != 0
        {
            return Err(WorkloadCompositionError::LookupConsumedWorkload(
                "lookup-consumed workload composition rejects raw evidence and broad receipt fallback"
                    .to_string(),
            ));
        }
        if handoff.counters().caller_owned_scan_count() != 0 {
            return Err(WorkloadCompositionError::LookupConsumedWorkload(
                "lookup-consumed workload composition rejects caller-owned lookup scans"
                    .to_string(),
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
}

impl WorthWorkload {
    pub fn admit_lookup_consumed_workload(
        &self,
        handoff: &EvidenceLookupConsumedWorkloadHandoff,
    ) -> Result<LookupConsumedWorkloadComposition, WorkloadCompositionError> {
        LookupConsumedWorkloadComposition::admit(self, handoff)
    }
}
