use std::sync::Arc;

use super::assembly::{
    BridgeCausalEnvelopeAssemblyRequest, BridgeCausalInspectionAdmissionSummaryKind,
};
use super::binding::BridgeCausalEvidenceBinding;
use super::counters::BridgeCausalEnvelopeCounters;
use super::digest;
use super::identity::BridgeCausalEnvelopeIdentity;
use super::receipt::BridgeCausalEnvelopeReceipt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BridgeCausalExplanationEnvelope {
    identity: BridgeCausalEnvelopeIdentity,
    admission_summary_kind: BridgeCausalInspectionAdmissionSummaryKind,
    admission_summary_digest: Arc<str>,
    request_digest: Arc<str>,
    causal_observation_anchor_digest: Arc<str>,
    bindings: Arc<[BridgeCausalEvidenceBinding]>,
    counters: BridgeCausalEnvelopeCounters,
    receipt: BridgeCausalEnvelopeReceipt,
    envelope_digest: Arc<str>,
}

impl BridgeCausalExplanationEnvelope {
    pub(super) fn new(
        request: BridgeCausalEnvelopeAssemblyRequest,
        bindings: Vec<BridgeCausalEvidenceBinding>,
        counters: BridgeCausalEnvelopeCounters,
    ) -> Self {
        let binding_part = bindings
            .iter()
            .map(BridgeCausalEvidenceBinding::binding_digest)
            .collect::<Vec<_>>()
            .join("|");
        let evidence_binding_digest = digest("bridge-causal-envelope-bindings", &[&binding_part]);
        let identity = BridgeCausalEnvelopeIdentity::new(
            request.request_digest(),
            request.causal_observation_anchor_digest(),
            evidence_binding_digest,
            counters.counter_digest(),
        );
        let envelope_digest = digest(
            "bridge-causal-explanation-envelope",
            &[
                identity.identity_digest(),
                request.request_digest(),
                request.causal_observation_anchor_digest(),
                &binding_part,
                counters.counter_digest(),
            ],
        );
        let receipt = BridgeCausalEnvelopeReceipt::new(&identity, &envelope_digest, &counters);
        Self {
            identity,
            admission_summary_kind: request.admission_summary().kind(),
            admission_summary_digest: Arc::from(request.admission_summary().summary_digest()),
            request_digest: Arc::from(request.request_digest()),
            causal_observation_anchor_digest: Arc::from(request.causal_observation_anchor_digest()),
            bindings: Arc::from(bindings),
            counters,
            receipt,
            envelope_digest: Arc::from(envelope_digest),
        }
    }

    pub fn identity(&self) -> &BridgeCausalEnvelopeIdentity {
        &self.identity
    }

    pub fn admission_summary_kind(&self) -> BridgeCausalInspectionAdmissionSummaryKind {
        self.admission_summary_kind
    }

    pub fn admission_summary_digest(&self) -> &str {
        self.admission_summary_digest.as_ref()
    }

    pub fn request_digest(&self) -> &str {
        self.request_digest.as_ref()
    }

    pub fn causal_observation_anchor_digest(&self) -> &str {
        self.causal_observation_anchor_digest.as_ref()
    }

    pub fn bindings(&self) -> &[BridgeCausalEvidenceBinding] {
        &self.bindings
    }

    pub fn counters(&self) -> &BridgeCausalEnvelopeCounters {
        &self.counters
    }

    pub fn receipt(&self) -> &BridgeCausalEnvelopeReceipt {
        &self.receipt
    }

    pub fn envelope_digest(&self) -> &str {
        self.envelope_digest.as_ref()
    }
}
