use crate::workload_platform::evidence_ledger::WorkloadEvidenceStageIndexProduct;
use crate::workload_platform::planar_boolean_events::{
    PlanarBooleanEventLedgerReceipt, PlanarBooleanSegmentPairEnumerationReceipt,
};

#[derive(Clone, Copy)]
pub struct PlanarBooleanCandidateIndexConsumptionInput<'a> {
    event_ledger: &'a PlanarBooleanEventLedgerReceipt,
    segment_pair_enumeration: &'a PlanarBooleanSegmentPairEnumerationReceipt,
    stage_index: &'a WorkloadEvidenceStageIndexProduct,
}

impl<'a> PlanarBooleanCandidateIndexConsumptionInput<'a> {
    pub fn new(
        event_ledger: &'a PlanarBooleanEventLedgerReceipt,
        segment_pair_enumeration: &'a PlanarBooleanSegmentPairEnumerationReceipt,
        stage_index: &'a WorkloadEvidenceStageIndexProduct,
    ) -> Self {
        Self {
            event_ledger,
            segment_pair_enumeration,
            stage_index,
        }
    }

    pub(crate) fn event_ledger(&self) -> &'a PlanarBooleanEventLedgerReceipt {
        self.event_ledger
    }

    pub(crate) fn segment_pair_enumeration(
        &self,
    ) -> &'a PlanarBooleanSegmentPairEnumerationReceipt {
        self.segment_pair_enumeration
    }

    pub(crate) fn stage_index(&self) -> &'a WorkloadEvidenceStageIndexProduct {
        self.stage_index
    }
}
