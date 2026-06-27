use super::{
    EvidenceLookupConsumedWorkloadHandoff, EvidenceLookupMilestoneTwelveSeed,
    EvidenceLookupWorkloadCutoverError, EvidenceLookupWorkloadCutoverErrorKind,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct EvidenceLookupMilestoneTwelveReplayPlanningSeed {
    milestone_twelve_seed: EvidenceLookupMilestoneTwelveSeed,
    stage_receipt_identity: String,
    workload_stage_index_identity: String,
    indexed_lookup_count: usize,
    topology_receipt_ref_count: usize,
}

impl EvidenceLookupMilestoneTwelveReplayPlanningSeed {
    pub(crate) fn admit_from_handoff(
        handoff: &EvidenceLookupConsumedWorkloadHandoff,
    ) -> Result<Self, EvidenceLookupWorkloadCutoverError> {
        if handoff.counters().raw_row_scan_count() != 0
            || handoff.counters().broad_receipt_scan_count() != 0
            || handoff.counters().caller_owned_scan_count() != 0
        {
            return Err(EvidenceLookupWorkloadCutoverError::new(
                EvidenceLookupWorkloadCutoverErrorKind::RawEvidenceFallbackDenied,
                "milestone twelve replay planning seed cannot admit from fallback-backed lookup consumption",
            ));
        }
        Ok(Self {
            milestone_twelve_seed: handoff.milestone_twelve_seed().clone(),
            stage_receipt_identity: handoff.stage_receipt_identity().to_string(),
            workload_stage_index_identity: handoff.workload_stage_index_identity().to_string(),
            indexed_lookup_count: handoff.counters().indexed_lookup_count(),
            topology_receipt_ref_count: handoff.counters().topology_receipt_ref_count(),
        })
    }

    pub(crate) fn milestone_twelve_seed(&self) -> &EvidenceLookupMilestoneTwelveSeed {
        &self.milestone_twelve_seed
    }

    pub(crate) fn stage_receipt_identity(&self) -> &str {
        &self.stage_receipt_identity
    }

    pub(crate) fn workload_stage_index_identity(&self) -> &str {
        &self.workload_stage_index_identity
    }

    pub(crate) fn indexed_lookup_count(&self) -> usize {
        self.indexed_lookup_count
    }

    pub(crate) fn topology_receipt_ref_count(&self) -> usize {
        self.topology_receipt_ref_count
    }
}
