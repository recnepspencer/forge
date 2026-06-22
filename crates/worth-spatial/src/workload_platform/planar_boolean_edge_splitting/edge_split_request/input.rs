use crate::workload_platform::evidence_ledger::{
    WorkloadEvidenceBooleanReceiptLookupProduct, WorkloadEvidenceStage,
    WorkloadEvidenceStageLinkSet,
};
use crate::workload_platform::planar_boolean_edge_splitting::PlanarBooleanCandidateIndexConsumptionGate;
use crate::workload_platform::planar_boolean_events::PlanarBooleanEventLedgerReceipt;

#[derive(Clone, Copy)]
pub struct PlanarBooleanEdgeSplitRequestInput<'a> {
    event_ledger: &'a PlanarBooleanEventLedgerReceipt,
    candidate_index_gate: &'a PlanarBooleanCandidateIndexConsumptionGate,
    event_ledger_lookup: &'a WorkloadEvidenceBooleanReceiptLookupProduct,
    retained_replay_stage_links: Option<&'a WorkloadEvidenceStageLinkSet>,
}

impl<'a> PlanarBooleanEdgeSplitRequestInput<'a> {
    pub fn new(
        event_ledger: &'a PlanarBooleanEventLedgerReceipt,
        candidate_index_gate: &'a PlanarBooleanCandidateIndexConsumptionGate,
        event_ledger_lookup: &'a WorkloadEvidenceBooleanReceiptLookupProduct,
        retained_replay_stage_links: Option<&'a WorkloadEvidenceStageLinkSet>,
    ) -> Self {
        Self {
            event_ledger,
            candidate_index_gate,
            event_ledger_lookup,
            retained_replay_stage_links,
        }
    }

    pub(crate) fn event_ledger(&self) -> &'a PlanarBooleanEventLedgerReceipt {
        self.event_ledger
    }

    pub(crate) fn candidate_index_gate(&self) -> &'a PlanarBooleanCandidateIndexConsumptionGate {
        self.candidate_index_gate
    }

    pub(crate) fn event_ledger_lookup(&self) -> &'a WorkloadEvidenceBooleanReceiptLookupProduct {
        self.event_ledger_lookup
    }

    pub(crate) fn retained_replay_stage_links(&self) -> Option<&'a WorkloadEvidenceStageLinkSet> {
        self.retained_replay_stage_links
    }

    pub(crate) fn retained_replay_stage_identity(&self) -> Option<&'a str> {
        self.retained_replay_stage_links
            .and_then(|links| links.link_for_stage(WorkloadEvidenceStage::RetainedReplay))
            .map(|link| link.evidence_identity())
    }
}
