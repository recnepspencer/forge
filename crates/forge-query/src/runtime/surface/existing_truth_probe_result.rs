use crate::intent_admission::ForgeQueryIntentDecisionTraceEnvelope;
use crate::runtime::{
    ForgeQueryExistingTruthProbe, ForgeQueryExistingTruthProbeReceipt,
    ForgeQueryIntentExecutionProvenance,
};

#[derive(Clone, Debug, PartialEq)]
pub struct ForgeQueryExistingTruthProbeResult {
    probe: ForgeQueryExistingTruthProbe,
    receipt: ForgeQueryExistingTruthProbeReceipt,
}

impl ForgeQueryExistingTruthProbeResult {
    pub fn probe(&self) -> &ForgeQueryExistingTruthProbe {
        &self.probe
    }

    pub fn receipt(&self) -> &ForgeQueryExistingTruthProbeReceipt {
        &self.receipt
    }

    pub(in crate::runtime) fn new(
        probe: ForgeQueryExistingTruthProbe,
        receipt: ForgeQueryExistingTruthProbeReceipt,
    ) -> Self {
        Self { probe, receipt }
    }

    pub(in crate::runtime) fn attach_intent_admission_evidence(
        &mut self,
        decision_trace_envelope: ForgeQueryIntentDecisionTraceEnvelope,
        execution_provenance: ForgeQueryIntentExecutionProvenance,
    ) {
        self.receipt.decision_trace_envelope = Some(decision_trace_envelope);
        self.receipt.execution_provenance = Some(execution_provenance);
    }
}
