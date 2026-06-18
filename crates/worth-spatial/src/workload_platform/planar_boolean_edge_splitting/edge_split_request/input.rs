use crate::workload_platform::evidence_ledger::WorkloadEvidenceStage;
use crate::workload_platform::evidence_ledger::WorkloadEvidenceStageIndexProduct;
use crate::workload_platform::planar_boolean_edge_splitting::PlanarBooleanCandidateIndexConsumptionGate;
use crate::workload_platform::planar_boolean_events::PlanarBooleanEventLedgerReceipt;

#[derive(Clone, Copy)]
pub struct PlanarBooleanEdgeSplitRequestInput<'a> {
    event_ledger: &'a PlanarBooleanEventLedgerReceipt,
    candidate_index_gate: &'a PlanarBooleanCandidateIndexConsumptionGate,
    stage_index: &'a WorkloadEvidenceStageIndexProduct,
}

impl<'a> PlanarBooleanEdgeSplitRequestInput<'a> {
    pub fn new(
        event_ledger: &'a PlanarBooleanEventLedgerReceipt,
        candidate_index_gate: &'a PlanarBooleanCandidateIndexConsumptionGate,
        stage_index: &'a WorkloadEvidenceStageIndexProduct,
    ) -> Self {
        Self {
            event_ledger,
            candidate_index_gate,
            stage_index,
        }
    }

    pub(crate) fn event_ledger(&self) -> &'a PlanarBooleanEventLedgerReceipt {
        self.event_ledger
    }

    pub(crate) fn candidate_index_gate(&self) -> &'a PlanarBooleanCandidateIndexConsumptionGate {
        self.candidate_index_gate
    }

    pub(crate) fn stage_index(&self) -> &'a WorkloadEvidenceStageIndexProduct {
        self.stage_index
    }

    pub(crate) fn retained_replay_stage_identity(&self) -> Option<&'a str> {
        self.stage_index
            .evidence_for_stage(WorkloadEvidenceStage::RetainedReplay)
    }
}
