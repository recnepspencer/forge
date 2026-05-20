use crate::intent_admission::ForgeQueryIntentDecisionTraceEnvelope;
use crate::runtime::{
    ForgeQueryExistingTruthProbe, ForgeQueryIntentConsumerInspection,
    ForgeQueryIntentExecutionProvenance,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryExistingTruthProbeReceipt {
    authoritative_identity: String,
    binding_digest: String,
    request_digest: String,
    probe_digest: String,
    snapshot_token: String,
    field_count: usize,
    pub(super) decision_trace_envelope: Option<ForgeQueryIntentDecisionTraceEnvelope>,
    pub(super) execution_provenance: Option<ForgeQueryIntentExecutionProvenance>,
}

impl ForgeQueryExistingTruthProbeReceipt {
    pub(in crate::runtime) fn from_probe(
        request: &crate::runtime::ForgeQueryExistingTruthProbeRequest,
        probe: &ForgeQueryExistingTruthProbe,
        snapshot_token: String,
    ) -> Self {
        Self {
            authoritative_identity: request.binding().authoritative_identity().to_string(),
            binding_digest: request.binding().binding_digest(),
            request_digest: request.request_digest().to_string(),
            probe_digest: probe.probe_digest().to_string(),
            snapshot_token,
            field_count: probe.fields().len(),
            decision_trace_envelope: None,
            execution_provenance: None,
        }
    }

    pub fn authoritative_identity(&self) -> &str {
        &self.authoritative_identity
    }

    pub fn binding_digest(&self) -> &str {
        &self.binding_digest
    }

    pub fn request_digest(&self) -> &str {
        &self.request_digest
    }

    pub fn probe_digest(&self) -> &str {
        &self.probe_digest
    }

    pub fn snapshot_token(&self) -> &str {
        &self.snapshot_token
    }

    pub fn field_count(&self) -> usize {
        self.field_count
    }

    pub fn decision_trace_envelope(&self) -> Option<&ForgeQueryIntentDecisionTraceEnvelope> {
        self.decision_trace_envelope.as_ref()
    }

    pub fn execution_provenance(&self) -> Option<&ForgeQueryIntentExecutionProvenance> {
        self.execution_provenance.as_ref()
    }

    pub fn execution_provenance_chain_digest(&self) -> Option<&str> {
        self.execution_provenance
            .as_ref()
            .map(|provenance| provenance.execution_provenance_chain_digest())
    }

    pub fn consumer_inspection(&self) -> Option<ForgeQueryIntentConsumerInspection<'_>> {
        Some(ForgeQueryIntentConsumerInspection::from_existing_truth_probe_receipt(self))
    }
}
