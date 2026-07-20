use crate::intent_admission::WorthQueryIntentDecisionTraceEnvelope;
use crate::runtime::{
    WorthQueryExistingTruthProbe, WorthQueryExistingTruthProbeReceipt,
    WorthQueryIntentExecutionProvenance,
};

#[derive(Clone, Debug, PartialEq)]
pub struct WorthQueryExistingTruthProbeResult {
    probe: WorthQueryExistingTruthProbe,
    receipt: WorthQueryExistingTruthProbeReceipt,
}

impl WorthQueryExistingTruthProbeResult {
    pub fn probe(&self) -> &WorthQueryExistingTruthProbe {
        &self.probe
    }

    pub fn receipt(&self) -> &WorthQueryExistingTruthProbeReceipt {
        &self.receipt
    }

    pub(in crate::runtime) fn new(
        probe: WorthQueryExistingTruthProbe,
        receipt: WorthQueryExistingTruthProbeReceipt,
    ) -> Self {
        Self { probe, receipt }
    }

    pub(in crate::runtime) fn attach_intent_admission_evidence(
        &mut self,
        decision_trace_envelope: WorthQueryIntentDecisionTraceEnvelope,
        execution_provenance: WorthQueryIntentExecutionProvenance,
    ) {
        self.receipt.decision_trace_envelope = Some(decision_trace_envelope);
        self.receipt.execution_provenance = Some(execution_provenance);
    }
}
